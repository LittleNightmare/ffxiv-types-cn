//! World types

#[cfg(feature = "data_centers")]
use data_centers::DataCenter;
use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The worlds, sometimes called servers, in the game.
///
/// Each [`DataCenter`] has multiple worlds attached to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum World {
  // Aether
  Adamantoise,
  Cactuar,
  Faerie,
  Gilgamesh,
  Jenova,
  Midgardsormr,
  Sargatanas,
  Siren,

  // Chaos
  Cerberus,
  Louisoix,
  Moogle,
  Omega,
  Ragnarok,

  // Crystal
  Balmung,
  Brynhildr,
  Coeurl,
  Diabolos,
  Goblin,
  Malboro,
  Mateus,
  Zalera,

  // Elemental
  Aegis,
  Atomos,
  Carbuncle,
  Garuda,
  Gungnir,
  Kujata,
  Ramuh,
  Tonberry,
  Typhon,
  Unicorn,

  // Gaia
  Alexander,
  Bahamut,
  Durandal,
  Fenrir,
  Ifrit,
  Ridill,
  Tiamat,
  Ultima,
  Valefor,
  Yojimbo,
  Zeromus,

  // Light
  Lich,
  Odin,
  Phoenix,
  Shiva,
  Zodiark,

  // Mana
  Anima,
  Asura,
  Belias,
  Chocobo,
  Hades,
  Ixion,
  Mandragora,
  Masamune,
  Pandaemonium,
  Shinryu,
  Titan,

  // Primal
  Behemoth,
  Excalibur,
  Exodus,
  Famfrit,
  Hyperion,
  Lamia,
  Leviathan,
  Ultros,
}

impl World {
  #[cfg(feature = "all_const")]
  pub const ALL: [World; 66] = [
    // Aether
    World::Adamantoise,
    World::Cactuar,
    World::Faerie,
    World::Gilgamesh,
    World::Jenova,
    World::Midgardsormr,
    World::Sargatanas,
    World::Siren,

    // Chaos
    World::Cerberus,
    World::Louisoix,
    World::Moogle,
    World::Omega,
    World::Ragnarok,

    // Crystal
    World::Balmung,
    World::Brynhildr,
    World::Coeurl,
    World::Diabolos,
    World::Goblin,
    World::Malboro,
    World::Mateus,
    World::Zalera,

    // Elemental
    World::Aegis,
    World::Atomos,
    World::Carbuncle,
    World::Garuda,
    World::Gungnir,
    World::Kujata,
    World::Ramuh,
    World::Tonberry,
    World::Typhon,
    World::Unicorn,

    // Gaia
    World::Alexander,
    World::Bahamut,
    World::Durandal,
    World::Fenrir,
    World::Ifrit,
    World::Ridill,
    World::Tiamat,
    World::Ultima,
    World::Valefor,
    World::Yojimbo,
    World::Zeromus,

    // Light
    World::Lich,
    World::Odin,
    World::Phoenix,
    World::Shiva,
    World::Zodiark,

    // Mana
    World::Anima,
    World::Asura,
    World::Belias,
    World::Chocobo,
    World::Hades,
    World::Ixion,
    World::Mandragora,
    World::Masamune,
    World::Pandaemonium,
    World::Shinryu,
    World::Titan,

    // Primal
    World::Behemoth,
    World::Excalibur,
    World::Exodus,
    World::Famfrit,
    World::Hyperion,
    World::Lamia,
    World::Leviathan,
    World::Ultros,
  ];

  /// Returns the string variant of this world.
  pub fn as_str(&self) -> &'static str {
    match *self {
      World::Adamantoise => "Adamantoise",
      World::Cactuar => "Cactuar",
      World::Faerie => "Faerie",
      World::Gilgamesh => "Gilgamesh",
      World::Jenova => "Jenova",
      World::Midgardsormr => "Midgardsormr",
      World::Sargatanas => "Sargatanas",
      World::Siren => "Siren",

      World::Cerberus => "Cerberus",
      World::Louisoix => "Louisoix",
      World::Moogle => "Moogle",
      World::Omega => "Omega",
      World::Ragnarok => "Ragnarok",

      World::Balmung => "Balmung",
      World::Brynhildr => "Brynhildr",
      World::Coeurl => "Coeurl",
      World::Diabolos => "Diabolos",
      World::Goblin => "Goblin",
      World::Malboro => "Malboro",
      World::Mateus => "Mateus",
      World::Zalera => "Zalera",

      World::Aegis => "Aegis",
      World::Atomos => "Atomos",
      World::Carbuncle => "Carbuncle",
      World::Garuda => "Garuda",
      World::Gungnir => "Gungnir",
      World::Kujata => "Kujata",
      World::Ramuh => "Ramuh",
      World::Tonberry => "Tonberry",
      World::Typhon => "Typhon",
      World::Unicorn => "Unicorn",

      World::Alexander => "Alexander",
      World::Bahamut => "Bahamut",
      World::Durandal => "Durandal",
      World::Fenrir => "Fenrir",
      World::Ifrit => "Ifrit",
      World::Ridill => "Ridill",
      World::Tiamat => "Tiamat",
      World::Ultima => "Ultima",
      World::Valefor => "Valefor",
      World::Yojimbo => "Yojimbo",
      World::Zeromus => "Zeromus",

      World::Lich => "Lich",
      World::Odin => "Odin",
      World::Phoenix => "Phoenix",
      World::Shiva => "Shiva",
      World::Zodiark => "Zodiark",

      World::Anima => "Anima",
      World::Asura => "Asura",
      World::Belias => "Belias",
      World::Chocobo => "Chocobo",
      World::Hades => "Hades",
      World::Ixion => "Ixion",
      World::Mandragora => "Mandragora",
      World::Masamune => "Masamune",
      World::Pandaemonium => "Pandaemonium",
      World::Shinryu => "Shinryu",
      World::Titan => "Titan",

      World::Behemoth => "Behemoth",
      World::Excalibur => "Excalibur",
      World::Exodus => "Exodus",
      World::Famfrit => "Famfrit",
      World::Hyperion => "Hyperion",
      World::Lamia => "Lamia",
      World::Leviathan => "Leviathan",
      World::Ultros => "Ultros",
    }
  }

  pub fn name(&self) -> &'static str {
    // if any variants with spaces are added, this must be changed
    self.as_str()
  }

  /// Returns the [`DataCenter`] this world is on.
  #[cfg(feature = "data_centers")]
  pub fn data_center(&self) -> DataCenter {
    match *self {
      World::Adamantoise |
      World::Cactuar |
      World::Faerie |
      World::Gilgamesh |
      World::Jenova |
      World::Midgardsormr |
      World::Sargatanas |
      World::Siren => DataCenter::Aether,

      World::Cerberus |
      World::Louisoix |
      World::Moogle |
      World::Omega |
      World::Ragnarok => DataCenter::Chaos,

      World::Balmung |
      World::Brynhildr |
      World::Coeurl |
      World::Diabolos |
      World::Goblin |
      World::Malboro |
      World::Mateus |
      World::Zalera => DataCenter::Crystal,

      World::Aegis |
      World::Atomos |
      World::Carbuncle |
      World::Garuda |
      World::Gungnir |
      World::Kujata |
      World::Ramuh |
      World::Tonberry |
      World::Typhon |
      World::Unicorn => DataCenter::Elemental,

      World::Alexander |
      World::Bahamut |
      World::Durandal |
      World::Fenrir |
      World::Ifrit |
      World::Ridill |
      World::Tiamat |
      World::Ultima |
      World::Valefor |
      World::Yojimbo |
      World::Zeromus => DataCenter::Gaia,

      World::Lich |
      World::Odin |
      World::Phoenix |
      World::Shiva |
      World::Zodiark => DataCenter::Light,

      World::Anima |
      World::Asura |
      World::Belias |
      World::Chocobo |
      World::Hades |
      World::Ixion |
      World::Mandragora |
      World::Masamune |
      World::Pandaemonium |
      World::Shinryu |
      World::Titan => DataCenter::Mana,

      // Primal
      World::Behemoth |
      World::Excalibur |
      World::Exodus |
      World::Famfrit |
      World::Hyperion |
      World::Lamia |
      World::Leviathan |
      World::Ultros => DataCenter::Primal,
    }
  }
}

impl FromStr for World {
  type Err = UnknownVariant;

  /// Parses a string `s` to return a value of this type.
  ///
  /// This is case-insensitive.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let world = match s.to_lowercase().as_str() {
      "adamantoise" => World::Adamantoise,
      "cactuar" => World::Cactuar,
      "faerie" => World::Faerie,
      "gilgamesh" => World::Gilgamesh,
      "jenova" => World::Jenova,
      "midgardsormr" => World::Midgardsormr,
      "sargatanas" => World::Sargatanas,
      "siren" => World::Siren,

      "cerberus" => World::Cerberus,
      "louisoix" => World::Louisoix,
      "moogle" => World::Moogle,
      "omega" => World::Omega,
      "ragnarok" => World::Ragnarok,

      "balmung" => World::Balmung,
      "brynhildr" => World::Brynhildr,
      "coeurl" => World::Coeurl,
      "diabolos" => World::Diabolos,
      "goblin" => World::Goblin,
      "malboro" => World::Malboro,
      "mateus" => World::Mateus,
      "zalera" => World::Zalera,

      "aegis" => World::Aegis,
      "atomos" => World::Atomos,
      "carbuncle" => World::Carbuncle,
      "garuda" => World::Garuda,
      "gungnir" => World::Gungnir,
      "kujata" => World::Kujata,
      "ramuh" => World::Ramuh,
      "tonberry" => World::Tonberry,
      "typhon" => World::Typhon,
      "unicorn" => World::Unicorn,

      "alexander" => World::Alexander,
      "bahamut" => World::Bahamut,
      "durandal" => World::Durandal,
      "fenrir" => World::Fenrir,
      "ifrit" => World::Ifrit,
      "ridill" => World::Ridill,
      "tiamat" => World::Tiamat,
      "ultima" => World::Ultima,
      "valefor" => World::Valefor,
      "yojimbo" => World::Yojimbo,
      "zeromus" => World::Zeromus,

      "lich" => World::Lich,
      "odin" => World::Odin,
      "phoenix" => World::Phoenix,
      "shiva" => World::Shiva,
      "zodiark" => World::Zodiark,

      "anima" => World::Anima,
      "asura" => World::Asura,
      "belias" => World::Belias,
      "chocobo" => World::Chocobo,
      "hades" => World::Hades,
      "ixion" => World::Ixion,
      "mandragora" => World::Mandragora,
      "masamune" => World::Masamune,
      "pandaemonium" => World::Pandaemonium,
      "shinryu" => World::Shinryu,
      "titan" => World::Titan,

      "behemoth" => World::Behemoth,
      "excalibur" => World::Excalibur,
      "exodus" => World::Exodus,
      "famfrit" => World::Famfrit,
      "hyperion" => World::Hyperion,
      "lamia" => World::Lamia,
      "leviathan" => World::Leviathan,
      "ultros" => World::Ultros,

      _ => return Err(UnknownVariant("World", s.into()))
    };

    Ok(world)
  }
}

impl Display for World {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.name())
  }
}
