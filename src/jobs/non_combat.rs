//! Non-combat job types

#[cfg(feature = "job_classifications")]
use jobs::classification::Classification;
use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The Disciple of the Land and Disciple of the Hand jobs available in the game.
#[derive(Debug, Clone, Copy)]
pub enum NonCombatJob {
  // Gatherers
  Botanist,
  Fisher,
  Miner,

  // Crafters
  Alchemist,
  Armorer,
  Blacksmith,
  Carpenter,
  Culinarian,
  Goldsmith,
  Leatherworker,
  Weaver,
}

impl NonCombatJob {
  #[cfg(feature = "all_const")]
  pub const ALL: [NonCombatJob; 11] = [
    NonCombatJob::Botanist,
    NonCombatJob::Fisher,
    NonCombatJob::Miner,

    NonCombatJob::Alchemist,
    NonCombatJob::Armorer,
    NonCombatJob::Blacksmith,
    NonCombatJob::Carpenter,
    NonCombatJob::Culinarian,
    NonCombatJob::Goldsmith,
    NonCombatJob::Leatherworker,
    NonCombatJob::Weaver,
  ];

  pub fn as_str(&self) -> &'static str {
    match *self {
      NonCombatJob::Botanist => "Botanist",
      NonCombatJob::Fisher => "Fisher",
      NonCombatJob::Miner => "Miner",

      NonCombatJob::Alchemist => "Alchemist",
      NonCombatJob::Armorer => "Armorer",
      NonCombatJob::Blacksmith => "Blacksmith",
      NonCombatJob::Carpenter => "Carpenter",
      NonCombatJob::Culinarian => "Culinarian",
      NonCombatJob::Goldsmith => "Goldsmith",
      NonCombatJob::Leatherworker => "Leatherworker",
      NonCombatJob::Weaver => "Weaver",
    }
  }

  #[cfg(feature = "job_classifications")]
  pub fn classification(&self) -> Classification {
    match *self {
      NonCombatJob::Botanist |
      NonCombatJob::Fisher |
      NonCombatJob::Miner => Classification::Land,

      NonCombatJob::Alchemist |
      NonCombatJob::Armorer |
      NonCombatJob::Blacksmith |
      NonCombatJob::Carpenter |
      NonCombatJob::Culinarian |
      NonCombatJob::Goldsmith |
      NonCombatJob::Leatherworker |
      NonCombatJob::Weaver => Classification::Hand,
    }
  }
}

impl FromStr for NonCombatJob {
  type Err = UnknownVariant;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let job = match s.to_lowercase().as_str() {
      "botanist" => NonCombatJob::Botanist,
      "fisher" => NonCombatJob::Fisher,
      "miner" => NonCombatJob::Miner,

      "alchemist" => NonCombatJob::Alchemist,
      "armorer" => NonCombatJob::Armorer,
      "blacksmith" => NonCombatJob::Blacksmith,
      "carpenter" => NonCombatJob::Carpenter,
      "culinarian" => NonCombatJob::Culinarian,
      "goldsmith" => NonCombatJob::Goldsmith,
      "leatherworker" => NonCombatJob::Leatherworker,
      "weaver" => NonCombatJob::Weaver,
      _ => return Err(UnknownVariant("NonCombatJob", s.into()))
    };

    Ok(job)
  }
}

impl Display for NonCombatJob {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.as_str())
  }
}
