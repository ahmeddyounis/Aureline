# Package Mutation Alpha Fixtures

This directory protects the runtime package-mutation alpha lane for a TS/JS
workspace. The fixtures use refs, enum values, count buckets, and relative
manifest or lockfile paths only; they do not carry raw registry URLs, tokens,
manifest bodies, lockfile bodies, or lifecycle script bodies.

The paired workspace fixture under `../node_pnpm_workspace` gives the Rust
consumer a real `package.json` plus `pnpm-lock.yaml` to inspect before emitting
review, audit, and support-export packets.
