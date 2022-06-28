//! Race types

use errors::UnknownVariant;

#[cfg(feature = "clans")]
use clans::Clan;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The playable races in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum Race {
    AuRa,
    Elezen,
    Hrothgar,
    Hyur,
    Lalafell,
    Miqote,
    Roegadyn,
    Viera,
}

impl Race {
    #[cfg(feature = "all_const")]
    pub const ALL: [Race; 8] = [
        Race::AuRa,
        Race::Elezen,
        Race::Hrothgar,
        Race::Hyur,
        Race::Lalafell,
        Race::Miqote,
        Race::Roegadyn,
        Race::Viera,
    ];

    /// Returns the string variant of this world.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Race::AuRa => "AuRa",
            Race::Elezen => "Elezen",
            Race::Hrothgar => "Hrothgar",
            Race::Hyur => "Hyur",
            Race::Lalafell => "Lalafell",
            Race::Miqote => "Miqote",
            Race::Roegadyn => "Roegadyn",
            Race::Viera => "Viera",
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Race::AuRa => "Au Ra",
            Race::Elezen => "Elezen",
            Race::Hrothgar => "Hrothgar",
            Race::Hyur => "Hyur",
            Race::Lalafell => "Lalafell",
            Race::Miqote => "Miqo'te",
            Race::Roegadyn => "Roegadyn",
            Race::Viera => "Viera",
        }
    }

    #[cfg(feature = "clans")]
    pub fn clans(&self) -> [Clan; 2] {
        match *self {
            Race::AuRa => [Clan::Raen, Clan::Xaela],
            Race::Elezen => [Clan::Duskwight, Clan::Wildwood],
            Race::Hrothgar => [Clan::Helions, Clan::TheLost],
            Race::Hyur => [Clan::Highlander, Clan::Midlander],
            Race::Lalafell => [Clan::Dunesfolk, Clan::Plainsfolk],
            Race::Miqote => [Clan::KeeperOfTheMoon, Clan::SeekerOfTheSun],
            Race::Roegadyn => [Clan::Hellsguard, Clan::SeaWolf],
            Race::Viera => [Clan::Rava, Clan::Veena],
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
            "hrothgar" => Race::Hrothgar,
            "hyur" => Race::Hyur,
            "lalafell" => Race::Lalafell,
            "miqote" | "miqo'te" => Race::Miqote,
            "roegadyn" => Race::Roegadyn,
            "viera" => Race::Viera,
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
