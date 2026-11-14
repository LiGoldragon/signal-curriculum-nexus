# signal-curriculum-nexus

The Curriculum Nexus's wire contract: what Flow, the `curriculum` CLI and
`curriculum-deploy` ask the Curriculum Nexus, and what it answers.

`ethos/signal.ethos` is a Signal root. ethos-zero generates
`src/generated/signal.rs` from it: `Input` and `Output`, the types they
carry, the `Answering` capabilities a Nexus implements, one per input, and
`impl Answerable for Input`, the dispatch a Nexus calls as
`input.answer(&nexus)`. `build.rs` regenerates the file at every build and
fails when the committed Rust differs.

On the socket, one big-endian u32 length-prefixed rkyv `Input` frame goes
out and one `Output` frame comes back. The n-th output answers the n-th
input; `Rejected` is the refusal any input may be answered with.

The `datom` feature derives the datom codec on every type, for a client that
reads an input from text or writes an output as text. A Nexus builds without
it.

`tests/published.rs` holds frames recorded from the Curriculum built before
this contract existed (e3ffb31): each reads as its `Input` or `Output` and
archives back to the same bytes.
