//! Job types

#[cfg(feature = "job_classifications")]
pub mod classification;
#[cfg(feature = "combat_jobs")]
pub mod combat;
#[cfg(feature = "non_combat_jobs")]
pub mod non_combat;

#[cfg(feature = "job_classifications")]
pub use self::classification::Classification;
#[cfg(feature = "combat_jobs")]
pub use self::combat::Job;
#[cfg(feature = "non_combat_jobs")]
pub use self::non_combat::NonCombatJob;
