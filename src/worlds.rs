//! World types

#[cfg(feature = "data_centers")]
use data_centers::DataCenter;
use errors::UnknownVariant;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The worlds, sometimes called servers, in the game.
///
/// Each [`DataCenter`] has multiple worlds attached to it.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum World {
  // Aether
  Adamantoise,
  Balmung,
  Cactuar,
  Coeurl,
  Faerie,
  Gilgamesh,
  Goblin,
  Jenova,
  Mateus,
  Midgardsormr,
  Sargatanas,
  Siren,
  Zalera,

  // Chaos
  Cerberus,
  Lich,
  Louisoix,
  Moogle,
  Odin,
  Omega,
  Phoenix,
  Ragnarok,
  Shiva,
  Zodiark,

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
  Brynhildr,
  Diabolos,
  Excalibur,
  Exodus,
  Famfrit,
  Hyperion,
  Lamia,
  Leviathan,
  Malboro,
  Ultros,
}

impl World {
  #[cfg(feature = "all_const")]
  pub const ALL: [World; 66] = [
    // Aether
    World::Adamantoise,
    World::Balmung,
    World::Cactuar,
    World::Coeurl,
    World::Faerie,
    World::Gilgamesh,
    World::Goblin,
    World::Jenova,
    World::Mateus,
    World::Midgardsormr,
    World::Sargatanas,
    World::Siren,
    World::Zalera,

    // Chaos
    World::Cerberus,
    World::Lich,
    World::Louisoix,
    World::Moogle,
    World::Odin,
    World::Omega,
    World::Phoenix,
    World::Ragnarok,
    World::Shiva,
    World::Zodiark,

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
    World::Brynhildr,
    World::Diabolos,
    World::Excalibur,
    World::Exodus,
    World::Famfrit,
    World::Hyperion,
    World::Lamia,
    World::Leviathan,
    World::Malboro,
    World::Ultros,
  ];

  /// Returns the string variant of this world.
  pub fn as_str(&self) -> &'static str {
    match *self {
      World::Adamantoise => "Adamantoise",
      World::Balmung => "Balmung",
      World::Cactuar => "Cactuar",
      World::Coeurl => "Coeurl",
      World::Faerie => "Faerie",
      World::Gilgamesh => "Gilgamesh",
      World::Goblin => "Goblin",
      World::Jenova => "Jenova",
      World::Mateus => "Mateus",
      World::Midgardsormr => "Midgardsormr",
      World::Sargatanas => "Sargatanas",
      World::Siren => "Siren",
      World::Zalera => "Zalera",

      World::Cerberus => "Cerberus",
      World::Lich => "Lich",
      World::Louisoix => "Louisoix",
      World::Moogle => "Moogle",
      World::Odin => "Odin",
      World::Omega => "Omega",
      World::Phoenix => "Phoenix",
      World::Ragnarok => "Ragnarok",
      World::Shiva => "Shiva",
      World::Zodiark => "Zodiark",

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
      World::Brynhildr => "Brynhildr",
      World::Diabolos => "Diabolos",
      World::Excalibur => "Excalibur",
      World::Exodus => "Exodus",
      World::Famfrit => "Famfrit",
      World::Hyperion => "Hyperion",
      World::Lamia => "Lamia",
      World::Leviathan => "Leviathan",
      World::Malboro => "Malboro",
      World::Ultros => "Ultros",
    }
  }

  /// Returns the [`DataCenter`] this world is on.
  #[cfg(feature = "data_centers")]
  pub fn data_center(&self) -> DataCenter {
    match *self {
      World::Adamantoise |
      World::Balmung |
      World::Cactuar |
      World::Coeurl |
      World::Faerie |
      World::Gilgamesh |
      World::Goblin |
      World::Jenova |
      World::Mateus |
      World::Midgardsormr |
      World::Sargatanas |
      World::Siren |
      World::Zalera => DataCenter::Aether,

      World::Cerberus |
      World::Lich |
      World::Louisoix |
      World::Moogle |
      World::Odin |
      World::Omega |
      World::Phoenix |
      World::Ragnarok |
      World::Shiva |
      World::Zodiark => DataCenter::Chaos,

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

      World::Behemoth |
      World::Brynhildr |
      World::Diabolos |
      World::Excalibur |
      World::Exodus |
      World::Famfrit |
      World::Hyperion |
      World::Lamia |
      World::Leviathan |
      World::Malboro |
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
      "balmung" => World::Balmung,
      "cactuar" => World::Cactuar,
      "coeurl" => World::Coeurl,
      "faerie" => World::Faerie,
      "gilgamesh" => World::Gilgamesh,
      "goblin" => World::Goblin,
      "jenova" => World::Jenova,
      "mateus" => World::Mateus,
      "midgardsormr" => World::Midgardsormr,
      "sargatanas" => World::Sargatanas,
      "siren" => World::Siren,
      "zalera" => World::Zalera,

      "cerberus" => World::Cerberus,
      "lich" => World::Lich,
      "louisoix" => World::Louisoix,
      "moogle" => World::Moogle,
      "odin" => World::Odin,
      "omega" => World::Omega,
      "phoenix" => World::Phoenix,
      "ragnarok" => World::Ragnarok,
      "shiva" => World::Shiva,
      "zodiark" => World::Zodiark,

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
      "brynhildr" => World::Brynhildr,
      "diabolos" => World::Diabolos,
      "excalibur" => World::Excalibur,
      "exodus" => World::Exodus,
      "famfrit" => World::Famfrit,
      "hyperion" => World::Hyperion,
      "lamia" => World::Lamia,
      "leviathan" => World::Leviathan,
      "malboro" => World::Malboro,
      "ultros" => World::Ultros,

      _ => return Err(UnknownVariant("World", s.into()))
    };

    Ok(world)
  }
}

impl Display for World {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.as_str())
  }
}
