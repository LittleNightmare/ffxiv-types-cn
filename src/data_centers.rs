//! Data center types

use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum DataCenter {
  Aether,
  Chaos,
  Elemental,
  Gaia,
  Mana,
  Primal,
}

impl DataCenter {
  pub const ALL: [DataCenter; 6] = [
    DataCenter::Aether,
    DataCenter::Chaos,
    DataCenter::Elemental,
    DataCenter::Gaia,
    DataCenter::Mana,
    DataCenter::Primal,
  ];

  pub fn as_str(&self) -> &'static str {
    match *self {
      DataCenter::Aether => "Aether",
      DataCenter::Chaos => "Chaos",
      DataCenter::Elemental => "Elemental",
      DataCenter::Gaia => "Gaia",
      DataCenter::Mana => "Mana",
      DataCenter::Primal => "Primal",
    }
  }
}

impl FromStr for DataCenter {
  type Err = UnknownVariant;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let data_center = match s.to_lowercase().as_str() {
      "aether" => DataCenter::Aether,
      "chaos" => DataCenter::Chaos,
      "elemental" => DataCenter::Elemental,
      "gaia" => DataCenter::Gaia,
      "mana" => DataCenter::Mana,
      "primal" => DataCenter::Primal,
      _ => return Err(UnknownVariant("DataCenter", s.into()))
    };

    Ok(data_center)
  }
}

impl Display for DataCenter {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.as_str())
  }
}
