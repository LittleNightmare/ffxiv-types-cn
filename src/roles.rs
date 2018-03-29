//! Job role types

use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The roles available in the game.
///
/// Each [`Job`] has a role attached to it.
///
/// [`Job`]: ::jobs::Job
#[derive(Debug, Clone, Copy)]
pub enum Role {
  Dps,
  Healer,
  Tank,
}

impl Role {
  #[cfg(feature = "all_const")]
  pub const ALL: [Role; 3] = [Role::Dps, Role::Healer, Role::Tank];

  pub fn as_str(&self) -> &'static str {
    match *self {
      Role::Dps => "DPS",
      Role::Healer => "Healer",
      Role::Tank => "Tank",
    }
  }
}

impl FromStr for Role {
  type Err = UnknownVariant;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let role = match s.to_lowercase().as_str() {
      "dps" => Role::Dps,
      "healer" => Role::Healer,
      "tank" => Role::Tank,
      _ => return Err(UnknownVariant("Role", s.into()))
    };

    Ok(role)
  }
}

impl Display for Role {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.as_str())
  }
}
