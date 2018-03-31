//! Types for use in FFXIV-related projects.

#[cfg(feature = "with_serde")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "data_centers")]
pub mod data_centers;
pub mod errors;
pub mod jobs;
#[cfg(feature = "roles")]
pub mod roles;
#[cfg(feature = "worlds")]
pub mod worlds;

#[cfg(feature = "data_centers")]
pub use self::data_centers::DataCenter;
#[cfg(feature = "combat_jobs")]
pub use self::jobs::Job;
#[cfg(feature = "non_combat_jobs")]
pub use self::jobs::NonCombatJob;
#[cfg(feature = "job_classifications")]
pub use self::jobs::Classification;
#[cfg(feature = "roles")]
pub use self::roles::Role;
#[cfg(feature = "worlds")]
pub use self::worlds::World;
