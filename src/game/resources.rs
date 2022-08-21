use serde::Deserialize;

/// A Resource in the game.
#[derive(Deserialize, Clone, Copy)]
pub enum Resource {
  /// The basic unit of energy.
  Watt,
  /// The basic unit of money. Generated by Headquarters.
  Credit,
}

impl Resource {
  /// A shorthand function to easily create a [ResourceDelta]
  pub fn d(self, value: i32) -> ResourceDelta {
    ResourceDelta { resource: self, value }
  }
}

/// Represents information about how a resource changes. May be used as a
/// producer (positive number) or consumer (negative number).
#[derive(Deserialize, Clone, Copy)]
pub struct ResourceDelta {
  pub resource: Resource,
  pub value: i32,
}
