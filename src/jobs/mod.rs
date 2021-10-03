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
pub use self::combat::{Class, Job};
#[cfg(feature = "non_combat_jobs")]
pub use self::non_combat::NonCombatJob;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(feature = "combat_jobs", feature = "non_combat_jobs"))]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
pub enum ClassJob {
    #[cfg(feature = "combat_jobs")]
    Class(Class),
    #[cfg(feature = "combat_jobs")]
    Job(Job),
    #[cfg(feature = "non_combat_jobs")]
    NonCombat(NonCombatJob),
}

#[cfg(any(feature = "combat_jobs", feature = "non_combat_jobs"))]
impl ClassJob {
    #[cfg(feature = "combat_jobs")]
    pub fn as_job(&self) -> Option<Job> {
        match self {
            Self::Job(j) => Some(*j),
            _ => None,
        }
    }

    #[cfg(feature = "combat_jobs")]
    pub fn as_class(&self) -> Option<Class> {
        match self {
            Self::Class(c) => Some(*c),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            #[cfg(feature = "combat_jobs")]
            Self::Job(j) => j.as_str(),
            #[cfg(feature = "combat_jobs")]
            Self::Class(c) => c.as_str(),
            #[cfg(feature = "non_combat_jobs")]
            Self::NonCombat(nc) => nc.as_str(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "combat_jobs")]
            Self::Job(j) => j.name(),
            #[cfg(feature = "combat_jobs")]
            Self::Class(c) => c.name(),
            #[cfg(feature = "non_combat_jobs")]
            Self::NonCombat(nc) => nc.name(),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            #[cfg(feature = "combat_jobs")]
            Self::Job(j) => j.code(),
            #[cfg(feature = "combat_jobs")]
            Self::Class(c) => c.code(),
            #[cfg(feature = "non_combat_jobs")]
            Self::NonCombat(nc) => nc.code(),
        }
    }

    #[cfg(all(feature = "roles", feature = "combat_jobs"))]
    pub fn role(&self) -> Option<crate::Role> {
        match self {
            Self::Job(j) => Some(j.role()),
            Self::Class(c) => Some(c.role()),
            _ => None,
        }
    }

    #[cfg(all(feature = "job_classifications", any(feature = "combat_jobs", feature = "non_combat_jobs")))]
    pub fn classification(&self) -> Classification {
        match self {
            #[cfg(feature = "combat_jobs")]
            Self::Job(j) => j.classification(),
            #[cfg(feature = "combat_jobs")]
            Self::Class(c) => c.classification(),
            #[cfg(feature = "non_combat_jobs")]
            Self::NonCombat(nc) => nc.classification(),
        }
    }
}
