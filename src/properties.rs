use std::fs;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};


/// Game properties file, stored at `properties.toml`
#[derive(Serialize, Deserialize)]
pub struct GameProperties {
  /// RPC port the server binds to, default is 1337
  pub rpc_port: u32,
  /// Tick increment per frame, default is 1
  pub tick_speed: u8,
}

impl Default for GameProperties {
  fn default() -> Self {
    Self { 
      rpc_port: 1337, 
      tick_speed: 1
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
    let exists = fs::try_exists(Self::LOCATION).map_err(GamePropertiesError::FileError)?;
    if exists {
      Err(GamePropertiesError::AlreadyExists)
    } else {
      let config = GameProperties::default();
      fs::write(Self::LOCATION, toml::to_string_pretty(&config).unwrap()).map_err(GamePropertiesError::FileError)?;
      Ok(config)
    }
  }
}

pub struct PropertiesPluign;

impl Plugin for PropertiesPluign {
  fn build(&self, app: &mut App) {
    let config = GameProperties::from_file().map_err(|err| match err {
      GamePropertiesError::FileError(err) => format!("A file error occured while loading properties.toml: {}", err),
      GamePropertiesError::ParsingError(err) => format!("A parsing error occured while loading properties.toml: {}", err),
      _ => unimplemented!(),
    }).unwrap();

    info!("properties.toml loaded");
    app.insert_resource(config);
  }
}
