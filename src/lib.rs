//! Types for use in FFXIV-related projects.

#[cfg(feature = "with_serde")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "clans")]
pub mod clans;
#[cfg(feature = "data_centers")]
pub mod data_centers;
pub mod errors;
#[cfg(feature = "guardians")]
pub mod guardians;
pub mod jobs;
#[cfg(feature = "races")]
pub mod races;
#[cfg(feature = "roles")]
pub mod roles;
#[cfg(feature = "worlds")]
pub mod worlds;

#[cfg(feature = "clans")]
pub use self::clans::Clan;
#[cfg(feature = "data_centers")]
pub use self::data_centers::DataCenter;
#[cfg(feature = "guardians")]
pub use self::guardians::Guardian;
#[cfg(feature = "combat_jobs")]
pub use self::jobs::Job;
#[cfg(feature = "non_combat_jobs")]
pub use self::jobs::NonCombatJob;
#[cfg(feature = "job_classifications")]
pub use self::jobs::Classification;
#[cfg(feature = "races")]
pub use self::races::Race;
#[cfg(feature = "roles")]
pub use self::roles::Role;
#[cfg(feature = "worlds")]
pub use self::worlds::World;
