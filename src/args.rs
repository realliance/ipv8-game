use clap::{Parser, Subcommand};
use tracing::{error, info};

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

  /// Enables a debug window viewer to display the current world
  DebugView,
}

#[derive(PartialEq, Eq)]
/// Side effects from processing arg commands that need to be consumed
/// elsewhere.
pub enum ArgsSideEffect {
  Exit,
  AddDebuggingWindowPlugins,
}

pub fn process_command(command: Option<Commands>) -> Option<ArgsSideEffect> {
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

        Some(ArgsSideEffect::Exit)
      },
      Commands::DebugView => Some(ArgsSideEffect::AddDebuggingWindowPlugins),
    }
  } else {
    None
  }
}
