use crate::general_types::Size2D;

pub struct InternalResolution {
    pub multiplier: f64,
    pub minimum_reached: bool,
    pub maximium_reached: bool,
    backup_multiplier: f64,
    max_texture_size: i32,
    pub viewport: Size2D<u32>,
}

impl InternalResolution {
    pub fn new(multiplier: f64) -> InternalResolution {
        InternalResolution {
            multiplier,
            minimum_reached: false,
            maximium_reached: false,
            backup_multiplier: multiplier,
            viewport: Size2D { width: 0, height: 0 },
            max_texture_size: 16384,
        }
    }
    pub fn initialize(&mut self, viewport: Size2D<u32>, max_texture_size: i32) {
        self.viewport = viewport;
        self.max_texture_size = max_texture_size;
    }
    pub fn increase(&mut self) {
        self.minimum_reached = false;
        let new_height = match self.height() {
            720 => (self.backup_multiplier * f64::from(self.viewport.height)) as i32,
            486 => 720,
            480 => 486,
            243 => 480,
            240 => 243,
            224 => 240,
            160 => 224,
            152 => 160,
            144 => 152,
            102 => 144,
            51..=101 => 102,
            height => height * 2,
        };
        self.set_resolution(new_height);
    }
    pub fn decrease(&mut self) {
        self.maximium_reached = false;
        let new_height = match self.height() {
            721..=1440 => {
                self.backup_multiplier = self.multiplier;
                720
            }
            720 => 486,
            486 => 480,
            480 => 243,
            243 => 240,
            240 => 224,
            224 => 160,
            160 => 152,
            152 => 144,
            144 => 102,
            height @ 0..=4 => {
                self.minimum_reached = true;
                height
            }
            height => height / 2,
        };
        self.set_resolution(new_height);
    }
    fn set_resolution(&mut self, resolution: i32) {
        self.multiplier = f64::from(resolution) / f64::from(self.viewport.height);
        if self.width() > self.max_texture_size || self.height() > self.max_texture_size {
            self.decrease();
            self.maximium_reached = true;
        }
    }

    pub fn width(&self) -> i32 {
        (f64::from(self.viewport.width) * self.multiplier) as i32
    }
    pub fn height(&self) -> i32 {
        (f64::from(self.viewport.height) * self.multiplier) as i32
    }
    pub fn to_label(&self) -> String {
        let height = self.height();
        if height <= 1080 {
            format!("{}p", height)
        } else {
            format!("{}K", height / 540)
        }
    }
}
