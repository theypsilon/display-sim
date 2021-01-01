/* Copyright (c) 2019-2021 José manuel Barroso Galindo <theypsilon@gmail.com>
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

use crate::app_events::AppEventDispatcher;
use crate::general_types::IncDec;
use crate::simulation_context::SimulationContext;
use crate::simulation_core_state::MainState;
use crate::ui_controller::{EncodedValue, UiController};
use app_error::AppResult;
use std::str::FromStr;

#[derive(Default, Clone)]
pub struct FilterPreset {
    input: IncDec<bool>,
    event: Option<FilterPresetOptions>,
    pub value: FilterPresetOptions,
}

impl From<FilterPresetOptions> for FilterPreset {
    fn from(value: FilterPresetOptions) -> Self {
        FilterPreset {
            input: Default::default(),
            event: None,
            value,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FilterPresetOptions {
    Sharp1,
    CrtApertureGrille1,
    CrtShadowMask1,
    CrtShadowMask2,
    DemoFlight1,
    Custom,
}

impl std::fmt::Display for FilterPresetOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FilterPresetOptions::Sharp1 => write!(f, "sharp-1"),
            FilterPresetOptions::CrtApertureGrille1 => write!(f, "crt-aperture-grille-1"),
            FilterPresetOptions::CrtShadowMask1 => write!(f, "crt-shadow-mask-1"),
            FilterPresetOptions::CrtShadowMask2 => write!(f, "crt-shadow-mask-2"),
            FilterPresetOptions::DemoFlight1 => write!(f, "demo-1"),
            FilterPresetOptions::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for FilterPresetOptions {
    type Err = String;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "sharp-1" => Ok(Self::Sharp1),
            "crt-aperture-grille-1" => Ok(Self::CrtApertureGrille1),
            "crt-shadow-mask-1" => Ok(Self::CrtShadowMask1),
            "crt-shadow-mask-2" => Ok(Self::CrtShadowMask2),
            "demo-1" => Ok(Self::DemoFlight1),
            "custom" => Ok(Self::Custom),
            _ => Err("Unknown name for a preset".into()),
        }
    }
}

impl FilterPresetOptions {
    pub fn get_description(&self) -> &str {
        match self {
            FilterPresetOptions::Sharp1 => "Sharp 1",
            FilterPresetOptions::CrtApertureGrille1 => "CRT Aperture Grille 1",
            FilterPresetOptions::CrtShadowMask1 => "CRT Shadow Mask 1",
            FilterPresetOptions::CrtShadowMask2 => "CRT Shadow Mask 2",
            FilterPresetOptions::DemoFlight1 => "Flight Demo",
            FilterPresetOptions::Custom => "Custom",
        }
    }
}

#[cfg(test)]
mod filter_presets_tests {
    use super::FilterPresetOptions;
    use app_error::AppResult;
    use std::str::FromStr;
    #[test]
    fn test_from_str_to_str() -> AppResult<()> {
        // @TODO ensure a way to have this array correctly updated automatically
        let presets: [FilterPresetOptions; 6] = [
            FilterPresetOptions::Sharp1,
            FilterPresetOptions::CrtApertureGrille1,
            FilterPresetOptions::CrtShadowMask1,
            FilterPresetOptions::CrtShadowMask2,
            FilterPresetOptions::DemoFlight1,
            FilterPresetOptions::Custom,
        ];
        for preset in presets.iter() {
            assert_eq!(FilterPresetOptions::from_str(preset.to_string().as_ref())?, *preset);
        }
        Ok(())
    }
}

impl Default for FilterPresetOptions {
    fn default() -> Self {
        Self::CrtApertureGrille1
    }
}

impl UiController for FilterPreset {
    fn event_tag(&self) -> &'static str {
        "front2back:filter-presets-selected"
    }
    fn keys_inc(&self) -> &[&'static str] {
        &[]
    }
    fn keys_dec(&self) -> &[&'static str] {
        &[]
    }
    fn update(&mut self, _: &MainState, _: &dyn SimulationContext) -> bool {
        false
    }
    fn apply_event(&mut self) {
        if let Some(v) = self.event {
            self.value = v;
        }
    }
    fn reset_inputs(&mut self) {
        self.event = None;
        self.input.increase = false;
        self.input.decrease = false;
    }
    fn read_event(&mut self, encoded: &dyn EncodedValue) -> AppResult<()> {
        self.event = Some(FilterPresetOptions::from_str(&encoded.to_string()?)?);
        Ok(())
    }
    fn read_key_inc(&mut self, pressed: bool) {
        self.input.increase = pressed;
    }
    fn read_key_dec(&mut self, pressed: bool) {
        self.input.decrease = pressed;
    }
    fn dispatch_event(&self, dispatcher: &dyn AppEventDispatcher) {
        dispatcher.dispatch_string_event("back2front:preset_selected_name", &self.value.to_string());
    }
    fn pre_process_input(&mut self) {}
    fn post_process_input(&mut self) {
        self.event = None;
    }
}
