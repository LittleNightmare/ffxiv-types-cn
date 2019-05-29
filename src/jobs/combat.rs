//! Combat job types

use errors::UnknownVariant;
#[cfg(feature = "job_classifications")]
use jobs::classification::Classification;
#[cfg(feature = "roles")]
use roles::Role;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The Disciple of War and Disciple of Magic jobs available in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum Job {
  // DPS
  Bard,
  BlackMage,
  Dancer,
  Dragoon,
  Machinist,
  Monk,
  Ninja,
  RedMage,
  Samurai,
  Summoner,

  // Healer
  Astrologian,
  Scholar,
  WhiteMage,

  // Tank
  DarkKnight,
  Gunbreaker,
  Paladin,
  Warrior,

  // Limited
  BlueMage,
}

impl Job {
  #[cfg(feature = "all_const")]
  pub const ALL: [Job; 18] = [
    // DPS
    Job::Bard,
    Job::BlackMage,
    Job::Dancer,
    Job::Dragoon,
    Job::Machinist,
    Job::Monk,
    Job::Ninja,
    Job::RedMage,
    Job::Samurai,
    Job::Summoner,

    // Healer
    Job::Astrologian,
    Job::Scholar,
    Job::WhiteMage,

    // Tank
    Job::DarkKnight,
    Job::Gunbreaker,
    Job::Paladin,
    Job::Warrior,

    // Limited
    Job::BlueMage,
  ];

  /// Returns the string representation of this variant.
  pub fn as_str(&self) -> &'static str {
    match *self {
      Job::Bard => "Bard",
      Job::BlackMage => "BlackMage",
      Job::Dancer => "Dancer",
      Job::Dragoon => "Dragoon",
      Job::Machinist => "Machinist",
      Job::Monk => "Monk",
      Job::Ninja => "Ninja",
      Job::RedMage => "RedMage",
      Job::Samurai => "Samurai",
      Job::Summoner => "Summoner",

      Job::Astrologian => "Astrologian",
      Job::Scholar => "Scholar",
      Job::WhiteMage => "WhiteMage",

      Job::DarkKnight => "DarkKnight",
      Job::Gunbreaker => "Gunbreaker",
      Job::Paladin => "Paladin",
      Job::Warrior => "Warrior",

      Job::BlueMage => "Blue Mage",
    }
  }

  /// Returns the name of this job.
  ///
  /// Names are title-cased and have spaces between words (e.g. "Bard" and "Black Mage").
  pub fn name(&self) -> &'static str {
    match *self {
      Job::Bard => "Bard",
      Job::BlackMage => "Black Mage",
      Job::Dancer => "Dancer",
      Job::Dragoon => "Dragoon",
      Job::Machinist => "Machinist",
      Job::Monk => "Monk",
      Job::Ninja => "Ninja",
      Job::RedMage => "Red Mage",
      Job::Samurai => "Samurai",
      Job::Summoner => "Summoner",

      Job::Astrologian => "Astrologian",
      Job::Scholar => "Scholar",
      Job::WhiteMage => "White Mage",

      Job::DarkKnight => "Dark Knight",
      Job::Gunbreaker => "Gunbreaker",
      Job::Paladin => "Paladin",
      Job::Warrior => "Warrior",

      Job::BlueMage => "Blue Mage",
    }
  }

  /// Returns the short code of this job.
  ///
  /// Short codes are fully capitalized (e.g. "BRD", "BLM").
  pub fn code(&self) -> &'static str {
    match *self {
      Job::Bard => "BRD",
      Job::BlackMage => "BLM",
      Job::Dancer => "DNC",
      Job::Dragoon => "DRG",
      Job::Machinist => "MCH",
      Job::Monk => "MNK",
      Job::Ninja => "NIN",
      Job::RedMage => "RDM",
      Job::Samurai => "SAM",
      Job::Summoner => "SMN",

      Job::Astrologian => "AST",
      Job::Scholar => "SCH",
      Job::WhiteMage => "WHM",

      Job::DarkKnight => "DRK",
      Job::Gunbreaker => "GNB",
      Job::Paladin => "PLD",
      Job::Warrior => "WAR",

      Job::BlueMage => "BLU",
    }
  }

  /// Returns the [`Role`] for this job.
  #[cfg(feature = "roles")]
  pub fn role(&self) -> Role {
    match *self {
      Job::Bard |
      Job::BlackMage |
      Job::Dancer |
      Job::Dragoon |
      Job::Machinist |
      Job::Monk |
      Job::Ninja |
      Job::RedMage |
      Job::Samurai |
      Job::Summoner |
      Job::BlueMage => Role::Dps,

      Job::Astrologian |
      Job::Scholar |
      Job::WhiteMage => Role::Healer,

      Job::DarkKnight |
      Job::Gunbreaker |
      Job::Paladin |
      Job::Warrior => Role::Tank,
    }
  }

  /// Returns the [`Classification`] for this job.
  #[cfg(feature = "job_classifications")]
  pub fn classification(&self) -> Classification {
    match *self {
      Job::Bard |
      Job::Dancer |
      Job::DarkKnight |
      Job::Dragoon |
      Job::Gunbreaker |
      Job::Machinist |
      Job::Monk |
      Job::Ninja |
      Job::Paladin |
      Job::Samurai |
      Job::Warrior |
      Job::BlueMage => Classification::War,

      Job::Astrologian |
      Job::BlackMage |
      Job::RedMage |
      Job::Scholar |
      Job::Summoner |
      Job::WhiteMage => Classification::Magic,
    }
  }
}

impl FromStr for Job {
  type Err = UnknownVariant;

  /// Parses a string `s` to return a value of this type.
  ///
  /// This accepts the name of the variant as a string, the name of the variant as a string with
  /// spaces between words, and the shortened job code for each variant (e.g. "BLM" for Black Mage).
  ///
  /// This is case-insensitive.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let job = match s.to_lowercase().as_str() {
      "bard" | "brd" => Job::Bard,
      "black mage" | "blackmage" | "blm" => Job::BlackMage,
      "dancer" | "dnc" => Job::Dancer,
      "dragoon" | "drg" => Job::Dragoon,
      "machinist" | "mch" => Job::Machinist,
      "monk" | "mnk" => Job::Monk,
      "ninja" | "nin" => Job::Ninja,
      "red mage" | "redmage" | "rdm" => Job::RedMage,
      "samurai" | "sam" => Job::Samurai,
      "summoner" | "smn" => Job::Summoner,

      "astrologian" | "ast" => Job::Astrologian,
      "scholar" | "sch" => Job::Scholar,
      "white mage" | "whitemage" | "whm" => Job::WhiteMage,

      "dark knight" | "darkknight" | "drk" => Job::DarkKnight,
      "gunbreaker" | "gnb" => Job::Gunbreaker,
      "paladin" | "pld" => Job::Paladin,
      "warrior" | "war" => Job::Warrior,

      "blue mage" | "bluemage" | "blu" => Job::BlueMage,

      _ => return Err(UnknownVariant("Job", s.into()))
    };

    Ok(job)
  }
}

impl Display for Job {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.name())
  }
}
