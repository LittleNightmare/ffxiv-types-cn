//! Race types

use errors::UnknownVariant;

#[cfg(feature = "clans")]
use clans::Clan;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The playable races in the game.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum Race {
  AuRa,
  Elezen,
  Hyur,
  Lalafell,
  Miqote,
  Roegadyn,
}

impl Race {
  #[cfg(feature = "all_const")]
  pub const ALL: [Race; 6] = [
    Race::AuRa,
    Race::Elezen,
    Race::Hyur,
    Race::Lalafell,
    Race::Miqote,
    Race::Roegadyn,
  ];

  /// Returns the string variant of this world.
  pub fn as_str(&self) -> &'static str {
    match *self {
      Race::AuRa => "AuRa",
      Race::Elezen => "Elezen",
      Race::Hyur => "Hyur",
      Race::Lalafell => "Lalafell",
      Race::Miqote => "Miqote",
      Race::Roegadyn => "Roegadyn",
    }
  }

  pub fn name(&self) -> &'static str {
    match *self {
      Race::AuRa => "Au Ra",
      Race::Elezen => "Elezen",
      Race::Hyur => "Hyur",
      Race::Lalafell => "Lalafell",
      Race::Miqote => "Miqo'te",
      Race::Roegadyn => "Roegadyn",
    }
  }

  #[cfg(feature = "clans")]
  pub fn clans(&self) -> [Clan; 2] {
    match *self {
      Race::AuRa => [Clan::Raen, Clan::Xaela],
      Race::Elezen => [Clan::Duskwight, Clan::Wildwood],
      Race::Hyur => [Clan::Highlander, Clan::Midlander],
      Race::Lalafell => [Clan::Dunesfolk, Clan::Plainsfolk],
      Race::Miqote => [Clan::SeekerOfTheMoon, Clan::SeekerOfTheSun],
      Race::Roegadyn => [Clan::Hellsguard, Clan::SeaWolf],
    }
  }
}

impl FromStr for Race {
  type Err = UnknownVariant;

  /// Parses a string `s` to return a value of this type.
  ///
  /// This is case-insensitive.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let world = match s.to_lowercase().as_str() {
      "aura" | "au ra" => Race::AuRa,
      "elezen" => Race::Elezen,
      "hyur" => Race::Hyur,
      "lalafell" => Race::Lalafell,
      "miqote" | "miqo'te" => Race::Miqote,
      "roegadyn" => Race::Roegadyn,
      _ => return Err(UnknownVariant("Race", s.into()))
    };

    Ok(world)
  }
}

impl Display for Race {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.name())
  }
}
