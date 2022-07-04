//! World types

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[cfg(feature = "data_centers")]
use data_centers::DataCenter;
use errors::UnknownVariant;

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
    Phantom,
    Ragnarok,
    Sagittarius,
    Spriggan,

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
    Tonberry,
    Typhon,

    // Gaia
    Alexander,
    Bahamut,
    Durandal,
    Fenrir,
    Ifrit,
    Ridill,
    Tiamat,
    Ultima,

    // Light
    Alpha,
    Lich,
    Odin,
    Phoenix,
    Raiden,
    Shiva,
    Twintania,
    Zodiark,

    // Mana
    Anima,
    Asura,
    Chocobo,
    Hades,
    Ixion,
    Masamune,
    Pandaemonium,
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

    // Materia
    Bismarck,
    Ravana,
    Sephirot,
    Sophia,
    Zurvan,

    // Meteor
    Belias,
    Mandragora,
    Ramuh,
    Shinryu,
    Unicorn,
    Valefor,
    Yojimbo,
    Zeromus,
}

impl World {
    #[cfg(feature = "all_const")]
    pub const ALL: [World; 77] = [
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
        World::Phantom,
        World::Ragnarok,
        World::Sagittarius,
        World::Spriggan,

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
        World::Tonberry,
        World::Typhon,

        // Gaia
        World::Alexander,
        World::Bahamut,
        World::Durandal,
        World::Fenrir,
        World::Ifrit,
        World::Ridill,
        World::Tiamat,
        World::Ultima,

        // Light
        World::Alpha,
        World::Lich,
        World::Odin,
        World::Phoenix,
        World::Raiden,
        World::Shiva,
        World::Twintania,
        World::Zodiark,

        // Mana
        World::Anima,
        World::Asura,
        World::Chocobo,
        World::Hades,
        World::Ixion,
        World::Masamune,
        World::Pandaemonium,
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

        // Materia
        World::Bismarck,
        World::Ravana,
        World::Sephirot,
        World::Sophia,
        World::Zurvan,

        // Meteor
        World::Belias,
        World::Mandragora,
        World::Ramuh,
        World::Shinryu,
        World::Unicorn,
        World::Valefor,
        World::Yojimbo,
        World::Zeromus,
    ];

    /// Returns the string variant of this world.
    pub fn as_str(&self) -> &'static str {
        match *self {
            // Aether
            World::Adamantoise => "Adamantoise",
            World::Cactuar => "Cactuar",
            World::Faerie => "Faerie",
            World::Gilgamesh => "Gilgamesh",
            World::Jenova => "Jenova",
            World::Midgardsormr => "Midgardsormr",
            World::Sargatanas => "Sargatanas",
            World::Siren => "Siren",

            // Chaos
            World::Cerberus => "Cerberus",
            World::Louisoix => "Louisoix",
            World::Moogle => "Moogle",
            World::Omega => "Omega",
            World::Phantom => "Phantom",
            World::Ragnarok => "Ragnarok",
            World::Sagittarius => "Sagittarius",
            World::Spriggan => "Spriggan",

            // Crystal
            World::Balmung => "Balmung",
            World::Brynhildr => "Brynhildr",
            World::Coeurl => "Coeurl",
            World::Diabolos => "Diabolos",
            World::Goblin => "Goblin",
            World::Malboro => "Malboro",
            World::Mateus => "Mateus",
            World::Zalera => "Zalera",

            // Elemental
            World::Aegis => "Aegis",
            World::Atomos => "Atomos",
            World::Carbuncle => "Carbuncle",
            World::Garuda => "Garuda",
            World::Gungnir => "Gungnir",
            World::Kujata => "Kujata",
            World::Tonberry => "Tonberry",
            World::Typhon => "Typhon",

            // Gaia
            World::Alexander => "Alexander",
            World::Bahamut => "Bahamut",
            World::Durandal => "Durandal",
            World::Fenrir => "Fenrir",
            World::Ifrit => "Ifrit",
            World::Ridill => "Ridill",
            World::Tiamat => "Tiamat",
            World::Ultima => "Ultima",

            // Light
            World::Alpha => "Alpha",
            World::Lich => "Lich",
            World::Odin => "Odin",
            World::Phoenix => "Phoenix",
            World::Raiden => "Raiden",
            World::Shiva => "Shiva",
            World::Twintania => "Twintania",
            World::Zodiark => "Zodiark",

            // Mana
            World::Anima => "Anima",
            World::Asura => "Asura",
            World::Chocobo => "Chocobo",
            World::Hades => "Hades",
            World::Ixion => "Ixion",
            World::Masamune => "Masamune",
            World::Pandaemonium => "Pandaemonium",
            World::Titan => "Titan",

            // Primal
            World::Behemoth => "Behemoth",
            World::Excalibur => "Excalibur",
            World::Exodus => "Exodus",
            World::Famfrit => "Famfrit",
            World::Hyperion => "Hyperion",
            World::Lamia => "Lamia",
            World::Leviathan => "Leviathan",
            World::Ultros => "Ultros",

            // Materia
            World::Bismarck => "Bismarck",
            World::Ravana => "Ravana",
            World::Sephirot => "Sephirot",
            World::Sophia => "Sophia",
            World::Zurvan => "Zurvan",

            // Meteor
            World::Belias => "Belias",
            World::Mandragora => "Mandragora",
            World::Ramuh => "Ramuh",
            World::Shinryu => "Shinryu",
            World::Unicorn => "Unicorn",
            World::Valefor => "Valefor",
            World::Yojimbo => "Yojimbo",
            World::Zeromus => "Zeromus",
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
            // Aether
            World::Adamantoise |
            World::Cactuar |
            World::Faerie |
            World::Gilgamesh |
            World::Jenova |
            World::Midgardsormr |
            World::Sargatanas |
            World::Siren => DataCenter::Aether,

            // Chaos
            World::Cerberus |
            World::Louisoix |
            World::Moogle |
            World::Omega |
            World::Phantom |
            World::Ragnarok |
            World::Sagittarius |
            World::Spriggan => DataCenter::Chaos,

            // Crystal
            World::Balmung |
            World::Brynhildr |
            World::Coeurl |
            World::Diabolos |
            World::Goblin |
            World::Malboro |
            World::Mateus |
            World::Zalera => DataCenter::Crystal,

            // Elemental
            World::Aegis |
            World::Atomos |
            World::Carbuncle |
            World::Garuda |
            World::Gungnir |
            World::Kujata |
            World::Tonberry |
            World::Typhon => DataCenter::Elemental,

            // Gaia
            World::Alexander |
            World::Bahamut |
            World::Durandal |
            World::Fenrir |
            World::Ifrit |
            World::Ridill |
            World::Tiamat |
            World::Ultima => DataCenter::Gaia,

            // Light
            World::Alpha |
            World::Lich |
            World::Odin |
            World::Phoenix |
            World::Raiden |
            World::Shiva |
            World::Twintania |
            World::Zodiark => DataCenter::Light,

            // Mana
            World::Anima |
            World::Asura |
            World::Chocobo |
            World::Hades |
            World::Ixion |
            World::Masamune |
            World::Pandaemonium |
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

            // Materia
            World::Bismarck |
            World::Ravana |
            World::Sephirot |
            World::Sophia |
            World::Zurvan => DataCenter::Materia,

            // Meteor
            World::Belias |
            World::Mandragora |
            World::Ramuh |
            World::Shinryu |
            World::Unicorn |
            World::Valefor |
            World::Yojimbo |
            World::Zeromus => DataCenter::Meteor,
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
            // Aether
            "adamantoise" => World::Adamantoise,
            "cactuar" => World::Cactuar,
            "faerie" => World::Faerie,
            "gilgamesh" => World::Gilgamesh,
            "jenova" => World::Jenova,
            "midgardsormr" => World::Midgardsormr,
            "sargatanas" => World::Sargatanas,
            "siren" => World::Siren,

            // Chaos
            "cerberus" => World::Cerberus,
            "louisoix" => World::Louisoix,
            "moogle" => World::Moogle,
            "omega" => World::Omega,
            "phantom" => World::Phantom,
            "ragnarok" => World::Ragnarok,
            "sagittarius" => World::Sagittarius,
            "spriggan" => World::Spriggan,

            // Crystal
            "balmung" => World::Balmung,
            "brynhildr" => World::Brynhildr,
            "coeurl" => World::Coeurl,
            "diabolos" => World::Diabolos,
            "goblin" => World::Goblin,
            "malboro" => World::Malboro,
            "mateus" => World::Mateus,
            "zalera" => World::Zalera,

            // Elemental
            "aegis" => World::Aegis,
            "atomos" => World::Atomos,
            "carbuncle" => World::Carbuncle,
            "garuda" => World::Garuda,
            "gungnir" => World::Gungnir,
            "kujata" => World::Kujata,
            "tonberry" => World::Tonberry,
            "typhon" => World::Typhon,

            // Gaia
            "alexander" => World::Alexander,
            "bahamut" => World::Bahamut,
            "durandal" => World::Durandal,
            "fenrir" => World::Fenrir,
            "ifrit" => World::Ifrit,
            "ridill" => World::Ridill,
            "tiamat" => World::Tiamat,
            "ultima" => World::Ultima,

            // Light
            "alpha" => World::Alpha,
            "lich" => World::Lich,
            "odin" => World::Odin,
            "phoenix" => World::Phoenix,
            "raiden" => World::Raiden,
            "shiva" => World::Shiva,
            "twintania" => World::Twintania,
            "zodiark" => World::Zodiark,

            // Mana
            "anima" => World::Anima,
            "asura" => World::Asura,
            "chocobo" => World::Chocobo,
            "hades" => World::Hades,
            "ixion" => World::Ixion,
            "masamune" => World::Masamune,
            "pandaemonium" => World::Pandaemonium,
            "titan" => World::Titan,

            // Primal
            "behemoth" => World::Behemoth,
            "excalibur" => World::Excalibur,
            "exodus" => World::Exodus,
            "famfrit" => World::Famfrit,
            "hyperion" => World::Hyperion,
            "lamia" => World::Lamia,
            "leviathan" => World::Leviathan,
            "ultros" => World::Ultros,

            // Materia
            "bismarck" => World::Bismarck,
            "ravana" => World::Ravana,
            "sephirot" => World::Sephirot,
            "sophia" => World::Sophia,
            "zurvan" => World::Zurvan,

            // Meteor
            "belias" => World::Belias,
            "mandragora" => World::Mandragora,
            "ramuh" => World::Ramuh,
            "shinryu" => World::Shinryu,
            "unicorn" => World::Unicorn,
            "valefor" => World::Valefor,
            "yojimbo" => World::Yojimbo,
            "zeromus" => World::Zeromus,

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
