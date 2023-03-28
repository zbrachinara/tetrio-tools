use ttrm::event::GameOptions;

pub struct Settings {
    pub gravity: f64,
    pub gravity_increase: f64,
    /// Measured in frames, not subframes
    pub garbage_speed: u32,
    pub garbage_cap: u16,
    pub das: u32,
    pub arr: u32,
    pub sdf: u32,
    pub dcd: u32,
    pub lock_delay: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: 0.01,
            gravity_increase: 0.0,
            garbage_speed: 20,
            garbage_cap: 8,
            das: 100,
            arr: 20,
            sdf: 60,
            dcd: 10,
            lock_delay: 30,
        }
    }
}

impl<'a, 'b> From<&'a GameOptions<'b>> for Settings {
    fn from(options: &'a GameOptions<'b>) -> Self {
        let mut settings = Self {
            gravity: options.gravity,
            gravity_increase: options.gravity_increase.unwrap_or(0.0),
            lock_delay: options.lock_time.unwrap_or(30),
            garbage_speed: options.garbage_speed,
            garbage_cap: options.garbage_cap,
            ..Default::default()
        };

        if let Some(ref handling) = options.handling {
            settings.das = (handling.das * 10.).round() as u32;
            settings.arr = (handling.arr * 10.).round() as u32;
            settings.sdf = handling.sdf as u32;
            settings.dcd = (handling.dcd * 10.).round() as u32;
        }

        settings
    }
}
