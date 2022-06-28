//! Clan types

use errors::UnknownVariant;

#[cfg(feature = "races")]
use races::Race;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The clans of the playable races in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum Clan {
    // Au Ra
    Raen,
    Xaela,
    // Elezen
    Duskwight,
    Wildwood,
    // Hrothgar
    Helions,
    TheLost,
    // Hyur
    Highlander,
    Midlander,
    // Lalafell
    Dunesfolk,
    Plainsfolk,
    // Miqo'te
    KeeperOfTheMoon,
    SeekerOfTheSun,
    // Roegadyn
    Hellsguard,
    SeaWolf,
    // Viera
    Rava,
    Veena,
}

impl Clan {
    #[cfg(feature = "all_const")]
    pub const ALL: [Clan; 16] = [
        Clan::Raen,
        Clan::Xaela,
        Clan::Duskwight,
        Clan::Wildwood,
        Clan::Helions,
        Clan::TheLost,
        Clan::Highlander,
        Clan::Midlander,
        Clan::Dunesfolk,
        Clan::Plainsfolk,
        Clan::KeeperOfTheMoon,
        Clan::SeekerOfTheSun,
        Clan::Hellsguard,
        Clan::SeaWolf,
        Clan::Rava,
        Clan::Veena,
    ];

    /// Returns the string variant of this world.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Clan::Raen => "Raen",
            Clan::Xaela => "Xaela",
            Clan::Duskwight => "Duskwight",
            Clan::Wildwood => "Wildwood",
            Clan::Helions => "Helions",
            Clan::TheLost => "TheLost",
            Clan::Highlander => "Highlander",
            Clan::Midlander => "Midlander",
            Clan::Dunesfolk => "Dunesfolk",
            Clan::Plainsfolk => "Plainsfolk",
            Clan::KeeperOfTheMoon => "KeeperOfTheMoon",
            Clan::SeekerOfTheSun => "SeekerOfTheSun",
            Clan::Hellsguard => "Hellsguard",
            Clan::SeaWolf => "SeaWolf",
            Clan::Rava => "Rava",
            Clan::Veena => "Veena",
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Clan::Raen => "Raen",
            Clan::Xaela => "Xaela",
            Clan::Duskwight => "Duskwight",
            Clan::Wildwood => "Wildwood",
            Clan::Helions => "Helions",
            Clan::TheLost => "The Lost",
            Clan::Highlander => "Highlander",
            Clan::Midlander => "Midlander",
            Clan::Dunesfolk => "Dunesfolk",
            Clan::Plainsfolk => "Plainsfolk",
            Clan::KeeperOfTheMoon => "Keeper of the Moon",
            Clan::SeekerOfTheSun => "Seeker of the Sun",
            Clan::Hellsguard => "Hellsguard",
            Clan::SeaWolf => "Sea Wolf",
            Clan::Rava => "Rava",
            Clan::Veena => "Veena",
        }
    }

    #[cfg(feature = "races")]
    pub fn race(&self) -> Race {
        match *self {
            Clan::Raen | Clan::Xaela => Race::AuRa,
            Clan::Duskwight | Clan::Wildwood => Race::Elezen,
            Clan::Helions | Clan::TheLost => Race::Hrothgar,
            Clan::Highlander | Clan::Midlander => Race::Hyur,
            Clan::Dunesfolk | Clan::Plainsfolk => Race::Lalafell,
            Clan::KeeperOfTheMoon | Clan::SeekerOfTheSun => Race::Miqote,
            Clan::Hellsguard | Clan::SeaWolf => Race::Roegadyn,
            Clan::Rava | Clan::Veena => Race::Viera,
        }
    }
}

impl FromStr for Clan {
    type Err = UnknownVariant;

    /// Parses a string `s` to return a value of this type.
    ///
    /// This is case-insensitive.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let world = match s.to_lowercase().as_str() {
            "raen" => Clan::Raen,
            "xaela" => Clan::Xaela,
            "duskwight" => Clan::Duskwight,
            "wildwood" => Clan::Wildwood,
            "helions" => Clan::Helions,
            "thelost" | "the lost" => Clan::TheLost,
            "highlander" => Clan::Highlander,
            "midlander" => Clan::Midlander,
            "dunesfolk" => Clan::Dunesfolk,
            "plainsfolk" => Clan::Plainsfolk,
            "keeperofthemoon" | "keeper of the moon" => Clan::KeeperOfTheMoon,
            "seekerofthesun" | "seeker of the sun" => Clan::SeekerOfTheSun,
            "hellsguard" => Clan::Hellsguard,
            "seawolf" | "sea wolf" => Clan::SeaWolf,
            "rava" => Clan::Rava,
            "veena" => Clan::Veena,
            _ => return Err(UnknownVariant("Clan", s.into()))
        };

        Ok(world)
    }
}

impl Display for Clan {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name())
    }
}
