# ffxiv_types

These are useful types when working with FFXIV.

Up to date as of *patch 4.25*.

- `DataCenter` (feature `data_centers`)
- `World` (feature `worlds`)
- `Role` (feature `roles`)
- `Job` (feature `combat_jobs`)
- `NonCombatJob` (feature `non_combat_jobs`)
- `Classification` (feature `job_classifications`)

`Job`, `NonCombatJob`, and `Classification` can all be simultaneously enabled with feature `jobs`.

All types implement `Debug`, `Display`, `Clone`, `Copy`, and `FromStr`.

With feature `with_serde`, all types also implement `Serialize` and `Deserialize`.

They also all have a `fn as_str(&self) -> &'static str`.

With feature `all_const`, they all have an `ALL` const (e.g. `Job::ALL`).
