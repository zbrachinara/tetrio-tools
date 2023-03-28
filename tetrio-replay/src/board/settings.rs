use ttrm::event::GameOptions;

pub struct Settings {
    pub gravity: GravitySettings,
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
            gravity: GravitySettings::default(),
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
            gravity: options.into(),
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

pub enum GravitySettings {
    Leveled {
        level_speed: f32,
        base_gravity: f32,
    },
    Continuous {
        gravity: f32,
        gravity_increase: Option<f32>,
    },
}

impl Default for GravitySettings {
    fn default() -> Self {
        Self::Continuous {
            gravity: 0.02,
            gravity_increase: Some(0.0035),
        }
    }
}

impl<'a, 'b> From<&'a GameOptions<'b>> for GravitySettings {
    fn from(options: &'a GameOptions<'b>) -> Self {
        if options.levels.unwrap_or(false) {
            Self::Leveled {
                level_speed: options.level_speed.unwrap(),
                base_gravity: options.gravity_base.unwrap(),
            }
        } else {
            Self::Continuous {
                gravity: options.gravity.unwrap(),
                gravity_increase: options.gravity_increase,
            }
        }
    }
}

impl GravitySettings {
    pub fn current_gravity(&self, level: u32, subframe: u32) -> f32 {
        match self {
            GravitySettings::Leveled {
                level_speed,
                base_gravity,
            } => todo!(),
            GravitySettings::Continuous {
                gravity,
                gravity_increase,
            } => f32::max(*gravity, 0.05),
        }
    }
}
