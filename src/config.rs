use serde_derive::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
struct Diffuse {
    albedo: [f32; 3],
}

#[derive(Debug, Deserialize)]
struct Metal {
    albedo: [f32; 3],
    fuzz: f32,
}

#[derive(Debug, Deserialize)]
struct Dielectric {
    refraction: f32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Ground {
    color: [f32; 3],
    pub(crate) center: [f32; 3],
    pub(crate) radius: f32,
    material: String,
    diffuse: Option<Diffuse>,
    metal: Option<Metal>,
    dielectric: Option<Dielectric>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
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
