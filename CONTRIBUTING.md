# Contributing to DPLMT

Thanks for stepping into the conference room.

## Ground rules

- Be a diplomat, not a courier. Disagree with the route, never with the diplomat.
- Open an issue before submitting a substantial change.
- Add tests for new scoring axes and new booths.
- Run `cargo fmt --all` and `cargo clippy --workspace --all-targets` before pushing.

## Adding a new booth

1. Create `crates/router/src/bridges/<bridge>.rs`.
2. Implement the `Booth` trait (`async fn quote`).
3. Add the new variant to `BoothId` and to the `bridges::*` re-export.
4. Update the README "Booths" table with the new seal grade and one-liner.
5. Add a unit test under `crates/router/tests/`.

## Coding style

- Rust 2021, `rustfmt` defaults.
- Avoid the `dyn` Box pattern unless the call site truly cannot know the type.
- Public structs are `pub(crate)` by default; only widen on request.
- Every public function has a real body and a unit test on main branch.
- Document every public function with at least one line.

## License

By contributing, you agree your contribution is MIT-licensed.
