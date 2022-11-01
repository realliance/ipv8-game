use clap::{Parser, Subcommand};
use tracing::{info, error};

use crate::properties::{GameProperties, GamePropertiesError};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Generates a default properties.toml file
  GenConfig,
}

pub fn process_command(command: Option<Commands>) -> bool {
  if let Some(command) = command {
    match command {
      Commands::GenConfig => {
        info!("Generating default config file...");
        if let Err(err) = GameProperties::generate_default_config().map_err(|err| match err {
          GamePropertiesError::AlreadyExists => "Properties file already exists".to_string(),
          GamePropertiesError::FileError(err) => err.to_string(),
          GamePropertiesError::ParsingError(err) => err.to_string(),
        }) {
          error!("Error: {}", err);
        }
      },
    }

    true
  } else {
    false
  }
}
