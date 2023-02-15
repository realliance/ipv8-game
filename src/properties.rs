use std::fs;
use std::path::Path;

use bevy::prelude::*;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

/// Game properties file, stored at `properties.toml`
#[derive(Serialize, Deserialize, Resource)]
pub struct GameProperties {
  /// RPC port the server binds to, default is 1337
  pub rpc_port: u32,
  /// Tick increment per frame, default is 1
  pub tick_speed: u32,
  /// Seed of the world
  pub seed: i64,
}

impl Default for GameProperties {
  fn default() -> Self {
    let mut rng = thread_rng();
    Self {
      rpc_port: 1337,
      tick_speed: 1,
      seed: rng.gen(),
    }
  }
}

#[derive(Debug)]
pub enum GamePropertiesError {
  AlreadyExists,
  FileError(std::io::Error),
  ParsingError(toml::de::Error),
}

impl GameProperties {
  pub const LOCATION: &'static str = "properties.toml";

  pub fn from_file() -> Result<Self, GamePropertiesError> {
    let config = fs::read_to_string(Self::LOCATION).map_err(GamePropertiesError::FileError)?;
    toml::from_str(&config).map_err(GamePropertiesError::ParsingError)
  }

  pub fn generate_default_config() -> Result<Self, GamePropertiesError> {
    if Path::new(Self::LOCATION).exists() {
      Err(GamePropertiesError::AlreadyExists)
    } else {
      let config = GameProperties::default();
      fs::write(Self::LOCATION, toml::to_string_pretty(&config).unwrap()).map_err(GamePropertiesError::FileError)?;
      Ok(config)
    }
  }
}

pub struct PropertiesPlugin;

impl Plugin for PropertiesPlugin {
  fn build(&self, app: &mut App) {
    let config = GameProperties::from_file()
      .map_err(|err| match err {
        GamePropertiesError::FileError(err) => format!("A file error occured while loading properties.toml: {}", err),
        GamePropertiesError::ParsingError(err) => {
          format!("A parsing error occured while loading properties.toml: {}", err)
        },
        _ => unimplemented!(),
      })
      .unwrap();

    info!("properties.toml loaded");
    app.insert_resource(config);
  }
}
