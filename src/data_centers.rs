//! Data center types

use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum DataCenter {
    Aether,
    Chaos,
    Crystal,
    Elemental,
    Gaia,
    Light,
    Mana,
    Materia,
    Meteor,
    Primal,
    Dynamis,
    陆行鸟,
    莫古力,
    猫小胖,
    豆豆柴,
}

impl DataCenter {
    #[cfg(feature = "all_const")]
    pub const ALL: [DataCenter; 15] = [
        DataCenter::Aether,
        DataCenter::Chaos,
        DataCenter::Crystal,
        DataCenter::Elemental,
        DataCenter::Gaia,
        DataCenter::Light,
        DataCenter::Mana,
        DataCenter::Materia,
        DataCenter::Meteor,
        DataCenter::Primal,
        DataCenter::Dynamis,
        DataCenter::陆行鸟,
        DataCenter::莫古力,
        DataCenter::猫小胖,
        DataCenter::豆豆柴,
    ];

    pub fn as_str(&self) -> &'static str {
        match *self {
            DataCenter::Aether => "Aether",
            DataCenter::Chaos => "Chaos",
            DataCenter::Crystal => "Crystal",
            DataCenter::Elemental => "Elemental",
            DataCenter::Gaia => "Gaia",
            DataCenter::Light => "Light",
            DataCenter::Mana => "Mana",
            DataCenter::Materia => "Materia",
            DataCenter::Meteor => "Meteor",
            DataCenter::Primal => "Primal",
            DataCenter::Dynamis => "Dynamis",
            DataCenter::陆行鸟 => "陆行鸟",
            DataCenter::莫古力 => "莫古力",
            DataCenter::猫小胖 => "猫小胖",
            DataCenter::豆豆柴 => "豆豆柴",
        }
    }

    pub fn name(&self) -> &'static str {
        // if any variants with spaces are added, this must be changed
        self.as_str()
    }
}

impl FromStr for DataCenter {
    type Err = UnknownVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_center = match s.to_lowercase().as_str() {
            "aether" => DataCenter::Aether,
            "chaos" => DataCenter::Chaos,
            "crystal" => DataCenter::Crystal,
            "elemental" => DataCenter::Elemental,
            "gaia" => DataCenter::Gaia,
            "light" => DataCenter::Light,
            "mana" => DataCenter::Mana,
            "materia" => DataCenter::Materia,
            "meteor" => DataCenter::Meteor,
            "primal" => DataCenter::Primal,
            "dynamis" => DataCenter::Dynamis,
            "陆行鸟" => DataCenter::陆行鸟,
            "莫古力" => DataCenter::莫古力,
            "猫小胖" => DataCenter::猫小胖,
            "豆豆柴" => DataCenter::豆豆柴,
            _ => return Err(UnknownVariant("DataCenter", s.into()))
        };

        Ok(data_center)
    }
}

impl Display for DataCenter {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name())
    }
}
