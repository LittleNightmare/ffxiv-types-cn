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
}

impl DataCenter {
    #[cfg(feature = "all_const")]
    pub const ALL: [DataCenter; 10] = [
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
