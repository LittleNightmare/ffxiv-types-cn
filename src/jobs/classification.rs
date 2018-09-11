//! Job classification types

use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The classification of jobs available in the game.
///
/// `{War, Magic}` refer to [`Job`], while `{Land, Hand}` refer to [`NonCombatJob`].
///
/// [`Job`]: ::jobs::Job
/// [`NonCombatJob`]: ::jobs::NonCombatJob
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "with_serde", serde(rename_all = "snake_case"))]
pub enum Classification {
  War,
  Magic,
  Land,
  Hand,
}

impl Classification {
  #[cfg(feature = "all_const")]
  pub const ALL: [Classification; 4] = [
    Classification::War,
    Classification::Magic,
    Classification::Land,
    Classification::Hand,
  ];

  pub fn as_str(&self) -> &'static str {
    match *self {
      Classification::War => "War",
      Classification::Magic => "Magic",
      Classification::Land => "Land",
      Classification::Hand => "Hand",
    }
  }

  pub fn name(&self) -> &'static str {
    match *self {
      Classification::War => "Disciple of War",
      Classification::Magic => "Disciple of Magic",
      Classification::Land => "Disciple of the Land",
      Classification::Hand => "Disciple of the Hand",
    }
  }
}

impl FromStr for Classification {
  type Err = UnknownVariant;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let class = match s.to_lowercase().as_str() {
      "disciple of war" | "war" => Classification::War,
      "disciple of magic" | "magic" => Classification::Magic,
      "disciple of the land" | "land" => Classification::Land,
      "disciple of the hand" | "hand" => Classification::Hand,
      _ => return Err(UnknownVariant("Classification", s.into()))
    };

    Ok(class)
  }
}

impl Display for Classification {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.name())
  }
}
