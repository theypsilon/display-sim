/* Copyright (c) 2019-2024 José manuel Barroso Galindo <theypsilon@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use crate::camera::CameraChange;
use app_util::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const SIMULATION_RECORDING_FORMAT_VERSION: u32 = 1;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pressed {
    Yes,
    No,
}

impl Pressed {
    pub fn from_bool(pressed: bool) -> Self {
        if pressed {
            Self::Yes
        } else {
            Self::No
        }
    }
}

/// A serializable value accepted by a simulation UI controller.
///
/// Keeping this type in the core removes platform-specific encoded-value
/// wrappers and makes controller assignments recordable like every other
/// simulation command.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ControllerValue {
    Number(f64),
    Text(String),
}

impl ControllerValue {
    fn number(&self) -> AppResult<f64> {
        match self {
            Self::Number(value) => Ok(*value),
            Self::Text(_) => Err(AppError::new("text controller value is not numeric".into())),
        }
    }

    pub fn to_f64(&self) -> AppResult<f64> {
        self.number()
    }

    pub fn to_f32(&self) -> AppResult<f32> {
        Ok(self.number()? as f32)
    }

    pub fn to_u32(&self) -> AppResult<u32> {
        Ok(self.number()? as u32)
    }

    pub fn to_i32(&self) -> AppResult<i32> {
        Ok(self.number()? as i32)
    }

    pub fn to_usize(&self) -> AppResult<usize> {
        Ok(self.number()? as usize)
    }

    pub fn to_text(&self) -> AppResult<&str> {
        match self {
            Self::Number(_) => Err(AppError::new("numeric controller value is not text".into())),
            Self::Text(value) => Ok(value),
        }
    }
}

/// The platform-neutral ingress contract for every mutation of a running
/// simulation. UI implementations and operating-system adapters only emit
/// commands; the core ticker is their sole consumer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "command", content = "value", rename_all = "snake_case")]
pub enum SimulationCommand {
    Keyboard { pressed: Pressed, key: String },
    MouseClick(Pressed),
    MouseMove { x: i32, y: i32 },
    MouseWheel(f32),
    BlurredWindow,

    ControllerSet { event_tag: String, value: ControllerValue },
    PixelWidth(f32),
    Camera(CameraChange),
    CustomScalingResolutionWidth(f32),
    CustomScalingResolutionHeight(f32),
    CustomScalingAspectRatioX(f32),
    CustomScalingAspectRatioY(f32),
    CustomScalingStretchNearest(bool),
    ViewportResize(u32, u32),
}

impl SimulationCommand {
    pub fn controller_set(event_tag: impl Into<String>, value: ControllerValue) -> Self {
        Self::ControllerSet {
            event_tag: event_tag.into(),
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordedSimulationCommand {
    /// Tick relative to the start of recording.
    pub tick: u64,
    /// FIFO position within the tick.
    pub order: u64,
    pub command: SimulationCommand,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationRecording {
    pub format_version: u32,
    /// Number of simulation ticks observed while recording, including ticks
    /// with no newly emitted commands. This keeps trailing held-input time and
    /// entirely idle recordings playable without inventing an end point.
    pub total_ticks: u64,
    pub commands: Vec<RecordedSimulationCommand>,
}

impl Default for SimulationRecording {
    fn default() -> Self {
        Self {
            format_version: SIMULATION_RECORDING_FORMAT_VERSION,
            total_ticks: 0,
            commands: Vec::new(),
        }
    }
}

impl SimulationRecording {
    pub fn validate(&self) -> AppResult<()> {
        if self.format_version != SIMULATION_RECORDING_FORMAT_VERSION {
            return Err(AppError::new(format!(
                "unsupported simulation recording format {}; expected {}",
                self.format_version, SIMULATION_RECORDING_FORMAT_VERSION
            )));
        }

        let mut current_tick = None;
        let mut expected_order = 0;
        for recorded in &self.commands {
            if recorded.tick >= self.total_ticks {
                return Err(AppError::new(format!(
                    "simulation recording command tick {} is outside its {} recorded ticks",
                    recorded.tick, self.total_ticks
                )));
            }
            match current_tick {
                None => {
                    current_tick = Some(recorded.tick);
                    expected_order = 0;
                }
                Some(tick) if recorded.tick < tick => {
                    return Err(AppError::new("simulation recording ticks are not ordered".into()));
                }
                Some(tick) if recorded.tick > tick => {
                    current_tick = Some(recorded.tick);
                    expected_order = 0;
                }
                Some(_) => {}
            }
            if recorded.order != expected_order {
                return Err(AppError::new(format!(
                    "simulation recording tick {} has command order {}; expected {}",
                    recorded.tick, recorded.order, expected_order
                )));
            }
            expected_order += 1;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ActiveRecording {
    start_tick: u64,
    recording: SimulationRecording,
}

/// One ordered queue shared by all command producers.
#[derive(Debug, Default)]
pub struct SimulationCommandBus {
    pending: VecDeque<SimulationCommand>,
    /// Commands postponed by the core so opposite keyboard edges are sampled
    /// on different ticks. They remain on the same bus, but are not recorded
    /// twice when they are consumed on a later tick.
    deferred: VecDeque<SimulationCommand>,
    next_tick: u64,
    recording: Option<ActiveRecording>,
}

impl SimulationCommandBus {
    pub fn emit(&mut self, command: SimulationCommand) {
        self.pending.push_back(command);
    }

    pub fn emit_all(&mut self, commands: impl IntoIterator<Item = SimulationCommand>) {
        self.pending.extend(commands);
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len() + self.deferred.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.deferred.is_empty()
    }

    pub fn start_recording(&mut self) {
        self.recording = Some(ActiveRecording {
            start_tick: self.next_tick,
            recording: SimulationRecording::default(),
        });
    }

    pub fn is_recording(&self) -> bool {
        self.recording.is_some()
    }

    pub fn finish_recording(&mut self) -> Option<SimulationRecording> {
        self.recording.take().map(|active| active.recording)
    }

    pub(crate) fn drain_for_tick(&mut self) -> Vec<SimulationCommand> {
        let tick = self.next_tick;
        self.next_tick += 1;
        let new_commands: Vec<_> = self.pending.drain(..).collect();
        if let Some(active) = &mut self.recording {
            let relative_tick = tick - active.start_tick;
            active.recording.total_ticks += 1;
            active
                .recording
                .commands
                .extend(new_commands.iter().cloned().enumerate().map(|(order, command)| RecordedSimulationCommand {
                    tick: relative_tick,
                    order: order as u64,
                    command,
                }));
        }
        let mut commands: Vec<_> = self.deferred.drain(..).collect();
        commands.extend(new_commands);
        commands
    }

    pub(crate) fn defer(&mut self, commands: impl IntoIterator<Item = SimulationCommand>) {
        self.deferred.extend(commands);
    }
}

/// Tick-driven playback helper. It deliberately feeds the same command bus as
/// live input so replay exercises the exact production processing path.
#[derive(Clone, Debug)]
pub struct SimulationCommandPlayer {
    recording: SimulationRecording,
    cursor: usize,
    next_tick: u64,
}

impl SimulationCommandPlayer {
    pub fn new(recording: SimulationRecording) -> AppResult<Self> {
        recording.validate()?;
        Ok(Self {
            recording,
            cursor: 0,
            next_tick: 0,
        })
    }

    /// Emits the commands scheduled for the next recorded tick and advances
    /// playback by exactly one tick, including empty ticks.
    pub fn emit_next_tick(&mut self, bus: &mut SimulationCommandBus) -> usize {
        if self.is_finished() {
            return 0;
        }
        let start = self.cursor;
        while let Some(recorded) = self.recording.commands.get(self.cursor) {
            if recorded.tick != self.next_tick {
                break;
            }
            bus.emit(recorded.command.clone());
            self.cursor += 1;
        }
        self.next_tick += 1;
        self.cursor - start
    }

    pub fn is_finished(&self) -> bool {
        self.next_tick >= self.recording.total_ticks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controller_values_are_strict_and_preserve_numeric_casts() {
        let numeric = ControllerValue::Number(42.75);
        assert_eq!(numeric.to_f64().unwrap(), 42.75);
        assert_eq!(numeric.to_i32().unwrap(), 42);
        let text = ControllerValue::Text("17".into());
        assert_eq!(text.to_text().unwrap(), "17");
        assert!(text.to_usize().is_err());
        assert!(numeric.to_text().is_err());
    }

    #[test]
    fn recording_round_trips_and_replays_tick_order() {
        let mut bus = SimulationCommandBus::default();
        bus.start_recording();
        bus.emit(SimulationCommand::Keyboard {
            pressed: Pressed::Yes,
            key: "w".into(),
        });
        assert_eq!(bus.drain_for_tick().len(), 1);
        assert!(bus.drain_for_tick().is_empty());
        bus.emit(SimulationCommand::MouseMove { x: 3, y: -2 });
        bus.emit(SimulationCommand::Keyboard {
            pressed: Pressed::No,
            key: "w".into(),
        });
        assert_eq!(bus.drain_for_tick().len(), 2);

        let recording = bus.finish_recording().unwrap();
        assert_eq!(recording.total_ticks, 3);
        let json = serde_json::to_string(&recording).unwrap();
        let decoded: SimulationRecording = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, recording);
        decoded.validate().unwrap();

        let mut player = SimulationCommandPlayer::new(decoded).unwrap();
        let mut replay_bus = SimulationCommandBus::default();
        assert_eq!(player.emit_next_tick(&mut replay_bus), 1);
        assert_eq!(replay_bus.drain_for_tick()[0], recording.commands[0].command);
        assert_eq!(player.emit_next_tick(&mut replay_bus), 0);
        assert!(replay_bus.drain_for_tick().is_empty());
        assert_eq!(player.emit_next_tick(&mut replay_bus), 2);
        assert_eq!(replay_bus.drain_for_tick().len(), 2);
        assert!(player.is_finished());
        assert_eq!(player.emit_next_tick(&mut replay_bus), 0);
    }

    #[test]
    fn every_command_variant_round_trips_through_the_recording_format() {
        let commands = vec![
            SimulationCommand::Keyboard {
                pressed: Pressed::Yes,
                key: "w".into(),
            },
            SimulationCommand::MouseClick(Pressed::No),
            SimulationCommand::MouseMove { x: -3, y: 7 },
            SimulationCommand::MouseWheel(-120.0),
            SimulationCommand::BlurredWindow,
            SimulationCommand::controller_set("front2back:blur-level", ControllerValue::Number(2.0)),
            SimulationCommand::controller_set("front2back:filter-presets-selected", ControllerValue::Text("demo-flight-1".into())),
            SimulationCommand::PixelWidth(1.25),
            SimulationCommand::Camera(CameraChange::DirectionY(-0.5)),
            SimulationCommand::CustomScalingResolutionWidth(320.0),
            SimulationCommand::CustomScalingResolutionHeight(240.0),
            SimulationCommand::CustomScalingAspectRatioX(4.0),
            SimulationCommand::CustomScalingAspectRatioY(3.0),
            SimulationCommand::CustomScalingStretchNearest(true),
            SimulationCommand::ViewportResize(1_920, 1_080),
        ];

        let json = serde_json::to_string(&commands).unwrap();
        assert_eq!(serde_json::from_str::<Vec<SimulationCommand>>(&json).unwrap(), commands);
    }

    #[test]
    fn core_deferred_commands_stay_on_the_bus_without_being_recorded_twice() {
        let press = SimulationCommand::Keyboard {
            pressed: Pressed::Yes,
            key: "w".into(),
        };
        let release = SimulationCommand::Keyboard {
            pressed: Pressed::No,
            key: "w".into(),
        };
        let mut bus = SimulationCommandBus::default();
        bus.start_recording();
        bus.emit_all([press.clone(), release.clone()]);
        assert_eq!(bus.drain_for_tick(), vec![press, release.clone()]);
        bus.defer([release.clone()]);
        assert_eq!(bus.pending_len(), 1);
        assert_eq!(bus.drain_for_tick(), vec![release]);

        let recording = bus.finish_recording().unwrap();
        assert_eq!(recording.commands.len(), 2);
        assert!(recording.commands.iter().all(|command| command.tick == 0));
    }

    #[test]
    fn recording_validation_rejects_non_fifo_order() {
        let recording = SimulationRecording {
            format_version: SIMULATION_RECORDING_FORMAT_VERSION,
            total_ticks: 1,
            commands: vec![RecordedSimulationCommand {
                tick: 0,
                order: 1,
                command: SimulationCommand::BlurredWindow,
            }],
        };
        assert!(recording.validate().is_err());
    }

    #[test]
    fn playback_preserves_trailing_empty_ticks() {
        let mut bus = SimulationCommandBus::default();
        bus.start_recording();
        bus.emit(SimulationCommand::Keyboard {
            pressed: Pressed::Yes,
            key: "w".into(),
        });
        bus.drain_for_tick();
        bus.drain_for_tick();
        bus.drain_for_tick();

        let recording = bus.finish_recording().unwrap();
        assert_eq!(recording.total_ticks, 3);
        let mut player = SimulationCommandPlayer::new(recording).unwrap();
        let mut replay_bus = SimulationCommandBus::default();
        assert_eq!(player.emit_next_tick(&mut replay_bus), 1);
        assert!(!player.is_finished());
        assert_eq!(player.emit_next_tick(&mut replay_bus), 0);
        assert!(!player.is_finished());
        assert_eq!(player.emit_next_tick(&mut replay_bus), 0);
        assert!(player.is_finished());
    }

    #[test]
    fn recording_validation_rejects_unknown_versions_and_out_of_range_ticks() {
        let unknown_version = SimulationRecording {
            format_version: SIMULATION_RECORDING_FORMAT_VERSION + 1,
            total_ticks: 0,
            commands: Vec::new(),
        };
        assert!(unknown_version.validate().is_err());

        let out_of_range = SimulationRecording {
            format_version: SIMULATION_RECORDING_FORMAT_VERSION,
            total_ticks: 1,
            commands: vec![RecordedSimulationCommand {
                tick: 1,
                order: 0,
                command: SimulationCommand::BlurredWindow,
            }],
        };
        assert!(out_of_range.validate().is_err());
    }
}
