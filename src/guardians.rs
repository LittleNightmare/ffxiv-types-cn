//! Guardian deity types

use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The guardian deities in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum Guardian {
  Althyk,
  Azeyma,
  Byregot,
  Halone,
  Llymlaen,
  Menphina,
  NaldThal,
  Nophica,
  Nymeia,
  Oschon,
  Rhalgr,
  Thaliak,
}

impl Guardian {
  #[cfg(feature = "all_const")]
  pub const ALL: [Guardian; 12] = [
    Guardian::Althyk,
    Guardian::Azeyma,
    Guardian::Byregot,
    Guardian::Halone,
    Guardian::Llymlaen,
    Guardian::Menphina,
    Guardian::NaldThal,
    Guardian::Nophica,
    Guardian::Nymeia,
    Guardian::Oschon,
    Guardian::Rhalgr,
    Guardian::Thaliak,
  ];

  /// Returns the string variant of this world.
  pub fn as_str(&self) -> &'static str {
    match *self {
      Guardian::Althyk => "Althyk",
      Guardian::Azeyma => "Azeyma",
      Guardian::Byregot => "Byregot",
      Guardian::Halone => "Halone",
      Guardian::Llymlaen => "Llymlaen",
      Guardian::Menphina => "Menphina",
      Guardian::NaldThal => "NaldThal",
      Guardian::Nophica => "Nophica",
      Guardian::Nymeia => "Nymeia",
      Guardian::Oschon => "Oschon",
      Guardian::Rhalgr => "Rhalgr",
      Guardian::Thaliak => "Thaliak",
    }
  }

  pub fn name(&self) -> &'static str {
    match *self {
      Guardian::Althyk => "Althyk",
      Guardian::Azeyma => "Azeyma",
      Guardian::Byregot => "Byregot",
      Guardian::Halone => "Halone",
      Guardian::Llymlaen => "Llymlaen",
      Guardian::Menphina => "Menphina",
      Guardian::NaldThal => "Nald'thal",
      Guardian::Nophica => "Nophica",
      Guardian::Nymeia => "Nymeia",
      Guardian::Oschon => "Oschon",
      Guardian::Rhalgr => "Rhalgr",
      Guardian::Thaliak => "Thaliak",
    }
  }

  pub fn epithet(&self) -> &'static str {
    match *self {
      Guardian::Althyk => "the Keeper",
      Guardian::Azeyma => "the Warden",
      Guardian::Byregot => "the Builder",
      Guardian::Halone => "the Fury",
      Guardian::Llymlaen => "the Navigator",
      Guardian::Menphina => "the Lover",
      Guardian::NaldThal => "the Traders",
      Guardian::Nophica => "the Matron",
      Guardian::Nymeia => "the Spinner",
      Guardian::Oschon => "the Wanderer",
      Guardian::Rhalgr => "the Destroyer",
      Guardian::Thaliak => "the Scholar",
    }
  }
}

impl FromStr for Guardian {
  type Err = UnknownVariant;

  /// Parses a string `s` to return a value of this type.
  ///
  /// This is case-insensitive.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let guardian = match s.to_lowercase().as_str() {
      "althyk" => Guardian::Althyk,
      "azeyma" => Guardian::Azeyma,
      "byregot" => Guardian::Byregot,
      "halone" => Guardian::Halone,
      "llymlaen" => Guardian::Llymlaen,
      "menphina" => Guardian::Menphina,
      "naldthal" | "nald'thal" => Guardian::NaldThal,
      "nophica" => Guardian::Nophica,
      "nymeia" => Guardian::Nymeia,
      "oschon" => Guardian::Oschon,
      "rhalgr" => Guardian::Rhalgr,
      "thaliak" => Guardian::Thaliak,
      _ => return Err(UnknownVariant("Guardian", s.into()))
    };

    Ok(guardian)
  }
}

impl Display for Guardian {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.name())
  }
}
