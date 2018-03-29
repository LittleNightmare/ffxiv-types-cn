use errors::UnknownVariant;
use roles::Role;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Job {
  // DPS
  Bard,
  BlackMage,
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
  Paladin,
  Warrior,
}

impl Job {
  pub const ALL: [Job; 15] = [
    // DPS
    Job::Bard,
    Job::BlackMage,
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
    Job::Paladin,
    Job::Warrior,
  ];

  pub fn as_str(&self) -> &'static str {
    match *self {
      Job::Bard => "Bard",
      Job::BlackMage => "Black Mage",
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
      Job::Paladin => "Paladin",
      Job::Warrior => "Warrior",
    }
  }

  pub fn role(&self) -> Role {
    match *self {
      Job::Bard |
      Job::BlackMage |
      Job::Dragoon |
      Job::Machinist |
      Job::Monk |
      Job::Ninja |
      Job::RedMage |
      Job::Samurai |
      Job::Summoner => Role::Dps,

      Job::Astrologian |
      Job::Scholar |
      Job::WhiteMage => Role::Healer,

      Job::DarkKnight |
      Job::Paladin |
      Job::Warrior => Role::Tank,
    }
  }
}

impl FromStr for Job {
  type Err = UnknownVariant;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let job = match s.to_lowercase().as_str() {
      "bard" | "brd" => Job::Bard,
      "black mage" | "blackmage" | "blm" => Job::BlackMage,
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
      "paladin" | "pld" => Job::Paladin,
      "warrior" | "war" => Job::Warrior,

      _ => return Err(UnknownVariant("Job", s.into()))
    };

    Ok(job)
  }
}

impl Display for Job {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.as_str())
  }
}
