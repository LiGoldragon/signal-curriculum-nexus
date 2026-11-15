# Upgrades

## 0.1.0

The contract moves out of the Curriculum crate, and out of Flow's copy of
it, into this repository. Curriculum's `Query` and `Outcome` are `Input` and
`Output`; every variant, payload and position is unchanged, and the frames
are byte for byte those Curriculum already speaks (`tests/published.rs`).
Two outputs are named in the past tense of their inputs: `ResolvedSkills`
is `SkillsResolved` and `ResolvedSources` is `SourcesResolved`. They keep
their positions, so their rkyv bytes are unchanged and only their datom text
changes.

Curriculum's own file formats, its module declarations, roles and workspace
projection, stay in Curriculum and import these types.

## 0.2.0 typed identity wire break

Module identities now cross the wire as `{ Spirit subject Single.0 }` or
`{ Spirit subject Span.{ 0 1 } }`, with typed registry kinds and versions.
The 0.1.0 archive frames are historical and are refused by the 0.2.0 reader.
