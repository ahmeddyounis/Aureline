# M5 update-center summary objects contract

This contract freezes the typed update-center summary objects the release center, update
center, and Help/About panel inspect before and after a download. It is the concrete
object layer below the [update / support-lifecycle governance
matrix](m5-update-lifecycle-contract.md): the matrix freezes *which* lifecycle states and
proof paths govern an update surface; these objects are the actual per-family update
summaries a user reads.

It does **not** design new package formats or updater backends. It hardens the
user-facing update summaries for already-claimed M5 artifact families.

- Packet schema: [`schemas/release/m5-update-center-summary-object.schema.json`](../../schemas/release/m5-update-center-summary-object.schema.json)
- Delta-row schema: [`schemas/release/m5-artifact-delta-row.schema.json`](../../schemas/release/m5-artifact-delta-row.schema.json)
- Published inventory: [`artifacts/release/m5-update-center-summary.json`](../../artifacts/release/m5-update-center-summary.json)
- Release-grade parity proof: `artifacts/release/m5-update-center-summary-proof/update-center-summary.json` (+ `.md`)
- Machine-readable delta export: [`artifacts/release/m5-update-center-summary-delta.csv`](../../artifacts/release/m5-update-center-summary-delta.csv)
- Per-state fixtures: `fixtures/release/update-center-summary/`
- Producer crate / module: `crates/aureline-release` → `m5_update_summary`
- Headless emitter: `aureline_release_m5_update_summary`

## What the summary covers

Each claimed artifact family gets its own summary entry rather than being flattened into one
generic "an update is available" row. The six families are separated explicitly:

| Family | Primary artifact class | Owner role |
|--------|------------------------|------------|
| `desktop_app` | `core_runtime` | `desktop_release_owner` |
| `extension` | `extension_packs` | `extension_release_owner` |
| `docs_pack` | `docs_help_content` | `docs_release_owner` |
| `policy_bundle` | `configuration` | `policy_governance_owner` |
| `framework_pack` | `schema_contracts` | `framework_pack_owner` |
| `runtime_toolchain` | `language_runtimes` | `toolchain_release_owner` |

Every entry carries:

- the **current** and **target** version and whether an update is available;
- the staged / downloaded / applied **posture** (`up_to_date`, `available`, `downloaded`,
  `staged`, `applied`, `failed`);
- the **rollback** path (`rollback_supported`, `side_by_side_fallback`, `reinstall_only`,
  `no_rollback`) and a derived `rollback_disclosed` flag that is true **only** for a genuine
  version rollback;
- the **release-data state** (`live`, `mirrored`, `offline`, `stale`, `not_provided`) that
  labels how current the backing live-release data actually is; and
- one **artifact-class delta row** per artifact class the update touches.

### Artifact-class delta rows

Each delta row records, for one artifact class, the `change_kind` (`added`, `updated`,
`removed`, `unchanged`), the version delta, the per-class `verification_state` (`verified`,
`pending`, `unverified`, `failed`, `not_provided`), the `restart_impact` (`none`,
`reload_window`, `restart_app`, `restart_host`), and the `release_data_state`.

The entry's disclosed artifact classes, verification state, restart impact, and release-data
state are the **roll-up of its delta rows** — the disclosed set is always the union of the
rows plus the primary class. An update therefore can never hide an artifact class it changes
behind a generic desktop-app update.

## Gate semantics — verification *and* data liveness

An entry's gate is derived, never hand-maintained, from the worst of its rolled-up
verification state and release-data state:

- `verified` + `live` / `mirrored` / `offline` → **governed**. Mirrored and offline data
  stay usable because they are *labeled*, not hidden — they never masquerade as live.
- `pending` / `unverified` verification, or `stale` data → **narrowed** (effective
  qualification at most `beta`).
- `failed` / `not_provided` verification, or `not_provided` data (no live release data) →
  **blocked** from Stable promotion.

`apply_ready` is true only when an update is available, verified, and not blocked.

## Consumers read one summary

The three claimed consumers — `release_center`, `update_center`, `help_about` — bind the
families they read and derive their effective qualification, gate, disclosed artifact
classes, channels, profiles, and coverage gaps from the entries. They read this one packet
rather than cloning version / verification / rollback fields locally. The release center and
update center read every family; Help/About reads the desktop app, extension, and
runtime/toolchain it surfaces.

Every gap names its family, the family's primary artifact class, the gap kind, and a routable
message id, so a drift report says *which* family and *why* rather than collapsing the cause
into one flag. The packet-level release gate aggregates the per-consumer gates and holds
Stable promotion while any consumer is blocked.

## Drills

Two checked-in drills exercise the auto-narrowing deterministically:

- `summary_stale_data_narrowed` ages the `docs_pack` release data to `stale`; exactly the
  release center and update center (which read it) narrow to `beta`, Help/About stays
  certified, and the release gate stays a pass.
- `summary_not_provided_blocked` drops the `framework_pack` release data to `not_provided`;
  exactly the release center and update center block, Help/About stays certified, and the
  release gate blocks Stable promotion.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- registry > artifacts/release/m5-update-center-summary.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- proof    > artifacts/release/m5-update-center-summary-proof/update-center-summary.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- markdown  > artifacts/release/m5-update-center-summary-proof/update-center-summary.md
cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- csv       > artifacts/release/m5-update-center-summary-delta.csv
cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- variant canonical    > fixtures/release/update-center-summary/summary_all_current.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- variant stale        > fixtures/release/update-center-summary/summary_stale_data_narrowed.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_summary -- variant not-provided > fixtures/release/update-center-summary/summary_not_provided_blocked.json
```

The packet carries metadata and refs only: no credential bodies or raw provider payloads.
The export is scanned for forbidden material on every validation.
