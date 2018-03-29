# ffxiv_types

These are useful types when working with FFXIV.

Up to date as of *patch 4.25*.

- `DataCenter`
- `World`
- `Role`
- `Job`

All types implement `Debug`, `Display`, `Clone`, `Copy`, and `FromStr`.

They also all have a `fn as_str(&self) -> &'static str`.

They all have a `ALL` const (e.g. `Job::ALL`).

Individual methods:

- `Job`
  - `fn role(&self) -> Role`
- `World`
  - `fn data_center(&self) -> DataCenter`
