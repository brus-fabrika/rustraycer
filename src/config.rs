use serde_derive::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub(crate) struct Diffuse {
    pub(crate) albedo: [f32; 3],
}

#[derive(Debug, Deserialize)]
pub(crate) struct Metal {
    pub(crate) albedo: [f32; 3],
    pub(crate) fuzz: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Dielectric {
    pub(crate) refraction: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Ground {
    pub(crate) color: [f32; 3],
    pub(crate) center: [f32; 3],
    pub(crate) radius: f32,
    pub(crate) material: String,
    pub(crate) diffuse: Option<Diffuse>,
    pub(crate) metal: Option<Metal>,
    pub(crate) dielectric: Option<Dielectric>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub width: u16,
    pub samples_per_pixel: u16,
    pub max_depth: u8,
    pub ground: Ground,
} 

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;

        s.try_deserialize()
    }
}
