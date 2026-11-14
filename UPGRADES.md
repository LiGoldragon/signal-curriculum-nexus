# Upgrades

## 0.1.0

The contract moves out of the Curriculum crate, and out of Flow's copy of
it, into this repository. Curriculum's `Query` and `Outcome` are `Input` and
`Output`; every variant, payload and position is unchanged, and the frames
are byte for byte those Curriculum already speaks (`tests/published.rs`).
Curriculum's own file formats, its module declarations, roles and workspace
projection, stay in Curriculum and import these types.
