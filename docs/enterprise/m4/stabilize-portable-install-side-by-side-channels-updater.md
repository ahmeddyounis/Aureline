# Stabilize Portable Install, Side-by-Side Channels, Updater and Handler Ownership

**Scope:** Install-profile stabilization for portable, side-by-side, managed, and air-gapped
install lanes.

**Truth source:**
`fixtures/enterprise/m4/stabilize-portable-install-side-by-side-channels-updater/page.json`

---

## Purpose

This document specifies how install-profile, portable-mode, side-by-side channel, updater
ownership, handler ownership, and rollback-scope truth are made inspectable across every
surface that needs to reason about the running build.

The single canonical object is the `StabilizePortableInstallPage` produced by
`crates/aureline-install/src/stabilize_portable_install_side_by_side_channels_updater`.
Every About, update center, diagnostics, install-review, and support-export surface must
ingest this record rather than inferring topology from launcher behavior or external
deployment notes.

---

## Install-profile rows

Each `InstallProfileStableRow` carries:

| Field | Purpose |
|---|---|
| `install_mode_class` | Distinguishes per-user, managed, portable, side-by-side-preview, and offline-bundle |
| `channel_class` | Names the release channel (stable, preview, beta, lts, portable-stable, portable-preview) |
| `updater_owner_class` | Who owns update decisions: user, admin, external package manager, or managed fleet |
| `binary_root_class` | Where the binary lives: per-user profile area, per-machine area, portable directory, offline-bundle area |
| `handler_ownership` | File-association and protocol-handler registration class, owning channel, and collision disclosure |
| `rollback_owner_class` | Who owns rollback: user, admin, or managed fleet |
| `rollback_scope` | Artifact-graph rollback scope: full graph, binary-only, package-manager-owned, or managed-fleet-owned |
| `isolation_verdict` | Whether channels keep isolated state roots or require import review before sharing |
| `portable_write_guard` | For portable rows: `fully_suppressed`, `disclosed_with_opt_in`, or violation evidence |
| `durable_state_root_refs` | All durable roots belonging to this install |

---

## Side-by-side channel isolation

Stable, Preview, Beta, and LTS channels must keep independent durable-state roots. The
`isolation_verdict` field on each row states this explicitly:

- `isolated` — the channel owns its own state roots with no shared mutable namespace.
- `requires_import_review` — a compare-or-skip checkpoint-backed review is required before
  any state is shared.
- `not_applicable` — the row does not participate in side-by-side.
- `undisclosed` — a structural gap; this verdict must not appear in a stable page.

No channel may silently migrate durable-state roots, collapse namespaces, or inherit
file-association ownership without passing an explicit import review.

---

## Side-by-side import review

`SideBySideImportReviewRow` records prove that every handoff between Stable, Preview, Beta,
portable, and admin-owned installs provides compare-before-apply or skip-preserving-source
semantics backed by a rollback checkpoint. The acceptance criteria are:

1. `review_class` is `compare_or_skip_with_checkpoint` or `skip_preserving_source`.
2. `can_compare_before_apply: true` or `skip_preserves_source: true`.
3. `checkpoint_created_before_apply: true`.
4. `collision_disclosures` is non-empty.

A review row with `review_class: blocked_pending_collision_resolution` blocks apply until
the collision is resolved.

---

## Portable mode

A portable install row must carry:

- `portable_write_guard: fully_suppressed` — all machine-global writes are blocked.
- `portable_shell_integration` — a struct disclosing that shell hooks, PATH mutation,
  credential-store access, and service registration are suppressed.
- `durable_state_root_refs` pointing only to a colocated-bundle root.
- `handler_ownership.file_association_class: portable_no_registration` — no handler
  registration is performed.

The product must not present itself as portable and then write undisclosed machine-global
state. Any row where `portable_write_guard` is `hidden_writes_detected` causes immediate
`Withdrawn` qualification.

---

## Handler ownership

Handler ownership truth is surfaced on every row so About and update surfaces can show who
owns the default-open behavior:

- `user_or_admin_selectable_never_last_writer_wins` — user or admin selects; last-writer-wins
  takeover is blocked.
- `admin_only` — administrator policy owns registration.
- `portable_no_registration` — portable install does not register.
- `not_registered` — install does not register handlers.

The `collision_disclosure` field carries a human-readable statement shown in the import
review and diagnostics surfaces.

---

## Rollback scope

`ArtifactGraphRollbackScope` states what moves together when a rollback occurs:

| Value | Meaning |
|---|---|
| `full_artifact_graph` | Binary, sidecars, symbols, manifests, and update metadata all revert |
| `binary_only_partial_graph` | Only the primary binary reverts; sidecars unchanged |
| `package_manager_owned` | Scope is controlled by the external package manager |
| `managed_fleet_owned` | Scope is controlled by the fleet rollout service |
| `undisclosed` | Not yet evaluated — not permitted in a stable page |
| `unsupported` | Rollback is unsupported for this install |

User-facing recovery copy must name the real blast radius using this field, not a generic
"undo last update" label.

---

## Fleet rollout diagnostics

`FleetRolloutInstallDiagnosticsRow` records confirm that opening logs, exporting diagnostics,
or reverting a managed rollout preserves install-profile identity and channel separation:

- `identity_preserved_in_export: true` — channel, ring, updater owner, binary root, and
  state roots are included in any diagnostic export.
- `channel_separation_maintained_on_revert: true` — a rollout revert does not collapse
  channel namespaces or force a shared-state assumption.

---

## Qualification narrowing

| Condition | Result |
|---|---|
| No install-profile rows | `Preview` |
| Portable row has `hidden_writes_detected` | `Withdrawn` (immediate) |
| Any row has `isolation_verdict: undisclosed` | `Preview` |
| Any row has `rollback_scope: undisclosed` | `Beta` |
| Any import review fails compare-or-skip | `Beta` |
| All conditions met | `Stable` |

---

## Surface projection

Every install-profile row declares which surfaces consume it via `exposed_in_surfaces`:

| Surface | Required for |
|---|---|
| `about` | All rows |
| `update_center` | Rows where updater is user or managed fleet |
| `diagnostics_center` | All rows |
| `import_sheet` | Side-by-side and portable rows |
| `fleet_console` | Managed and air-gapped rows |
| `support_bundle` | All rows |

---

## References

- Rust implementation: `crates/aureline-install/src/stabilize_portable_install_side_by_side_channels_updater/`
- Fixture: `fixtures/enterprise/m4/stabilize-portable-install-side-by-side-channels-updater/`
- Prior install-topology alpha: `crates/aureline-install/src/topology/`
- Prior profile-cards beta: `crates/aureline-install/src/profile_cards/`
- Prior hardened topology audit: `crates/aureline-install/src/harden_installation_topology_state_root_audits_silent_deployment/`
