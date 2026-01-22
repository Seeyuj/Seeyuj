# See-Yuj

Persistent sandbox worlds **simulation platform** (not a game).

## Repository layout

- `doc/`: authoritative project documentation (architecture, roadmap, decisions, policy)
- `seeyuj/server/`: Rust workspace (server-side crates, bins, examples)
- `seeyuj/schemas/`: protocol/schema notes (future wire format)
- `seeyuj/clients/`: placeholder for future clients (rendering/UI)

## Core invariants (short)

- **Authoritative server**: the server is the single source of truth.
- **Determinism**: given identical inputs, the core must produce identical outputs.
- **Strict layering**: `sy_core` is pure (no I/O, no std RNG/time; only injected ports).
- **Event sourcing**: state changes are captured as events and can be replayed.

See `doc/ARCHITECTURE.md` for the authoritative version.

## Build prerequisites (Windows)

You need the Rust toolchain available in your shell (`cargo` command).

Recommended:

1. Install Rust via rustup: `https://rustup.rs/`
2. Open a new PowerShell session
3. Verify: `cargo --version`

## Build / test (Rust workspace)

From `seeyuj/server/`:

```bash
cargo build --workspace
cargo test --workspace
```

## Run (current status)

The codebase is currently a **scaffold**: crates/binaries exist and follow the intended dependency boundaries, but most runtime logic is still TODO.
The server binary prints a startup banner; the simulation loop and persistence are not implemented yet.

