# Workspace Serialization and Portable-State Beta

This document freezes the workspace-facing beta contract for remembered
state, portable-state packages, and restore provenance. It composes with
the layout serialization contract, the state portable-package contract,
and the restore-provenance placeholder contract.

Machine-readable boundaries:

- [`/schemas/workspace/portable_state_package.schema.json`](../../../schemas/workspace/portable_state_package.schema.json)
- [`/schemas/workspace/restore_provenance.schema.json`](../../../schemas/workspace/restore_provenance.schema.json)
- [`/schemas/workspace/pane_tree.schema.json`](../../../schemas/workspace/pane_tree.schema.json)

Runtime model:

- [`/crates/aureline-workspace/src/serialization/`](../../../crates/aureline-workspace/src/serialization/)

Fixtures:

- [`/fixtures/workspace/m3/portable_state_and_restore/`](../../../fixtures/workspace/m3/portable_state_and_restore/)

## Boundary

Every package separates these layers:

| Layer | Package posture | Restore posture |
|---|---|---|
| Workspace authority | Opaque refs only; live authority excluded | Rebound only after trust and policy review |
| Window topology | Versioned pane-tree body with stable pane ids | Skeleton first, then hydrate or placeholder |
| Profile defaults | Linked portable-profile artifact refs | Defaults only; never overwrite explicit topology |
| Machine-local hints | Metadata or omitted | Best effort only; safe remap when topology changes |
| Local session context | Evidence or metadata only | Context-only or explicit rerun/reconnect action |

Flattening these layers into one opaque payload is non-conforming.

## Package Rules

- Package records must be diffable JSON with `schema_version`, producer
  ref, workspace ref, layer rows, redaction manifest, exclusions, and a
  restore-provenance card.
- Raw secrets, delegated approvals, approval tickets, delegated
  credentials, live authority handles, machine-unique trust anchors,
  raw command lines, logs, source content, hostnames, and provider
  payload bodies are excluded by default.
- Exclusions must be named in both export review and restore summaries.
- Machine-local hints may be metadata-only or excluded. They must not be
  exported as carried authority.
- Window-topology rows must bind to the pane-tree schema and retain
  stable pane ids for live, context-only, and placeholder panes.

## Inspector And Review

The remembered-state inspection record lists layer rows and pane rows
with schema version, last-write time, persistence class, schema outcome,
restore fidelity, and available actions.

Clear actions are scoped to selected remembered-state artifacts. They
must not delete source files, workspace manifests, credential-store
entries, unrelated caches, or broader workspace content.

The portable-state review sheet is used for both export and import. It
must show selected layers, machine-local and high-risk exclusions,
redaction posture, integrity posture, provenance, and confirm/cancel
actions before bytes leave the machine or before an imported package is
applied.

## Restore Provenance

Restore-provenance cards use the controlled fidelity labels:

- `Exact restore`
- `Compatible restore`
- `Layout only`
- `Recovered drafts`
- `Evidence only`

Schema outcomes use:

- `Exact`
- `Compatible`
- `Layout only`
- `Manual review`

When dependencies are missing, the pane slot remains in the topology
and reopens as a placeholder that names the original role, surface
class, stable pane id, last known provenance, and safe actions.

Restore provenance must stay visible through diagnostics, support
exports, and crash-recovery views. A missing extension, remote target,
provider, or live authority may never disappear silently from the
layout, and live surfaces may not rerun or reacquire authority without
explicit user action.
