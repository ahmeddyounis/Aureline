# Artifact: Stabilize Portable Install, Side-by-Side Channels, Updater and Handler Ownership

**Status:** Stable  
**Truth source:** `fixtures/enterprise/m4/stabilize-portable-install-side-by-side-channels-updater/page.json`  
**Rust module:** `crates/aureline-install/src/stabilize_portable_install_side_by_side_channels_updater/`

---

## Delivery summary

This artifact closes the install-profile stabilization gap by materializing one
`StabilizePortableInstallPage` record that About, update center, diagnostics,
install-review, and support-export surfaces can all consume without independently
inferring topology from launcher behavior.

Five install-profile rows are seeded and pass the `Stable` qualification gate:

| Row | Mode | Updater owner | Rollback scope | Portable guard |
|---|---|---|---|---|
| Windows Stable per-user | Per-user installed | User | Full artifact graph | n/a |
| Windows Preview per-user | Side-by-side preview | User | Full artifact graph | n/a |
| Windows Portable Stable | Portable | User | Full artifact graph | Fully suppressed |
| Windows Managed per-machine Stable | Managed deployed | Managed fleet | Managed fleet owned | n/a |
| Air-gapped Bundle Stable | Offline bundle | Admin | Full artifact graph | n/a |

Two side-by-side import-review rows prove compare-or-skip with checkpoint:

- Stable → Preview: `compare_or_skip_with_checkpoint`, checkpoint created before apply.
- Portable → per-user installed: `compare_or_skip_with_checkpoint`, checkpoint created before apply.

Two fleet rollout diagnostics rows confirm identity preservation and channel
separation through export and revert for managed and air-gapped lanes.

---

## Acceptance criteria status

| Criterion | Status |
|---|---|
| About, update, diagnostics, and fleet surfaces report identical install mode, channel, updater owner, and state roots | Met — all five rows carry `exposed_in_surfaces` covering required surfaces |
| Stable and preview side-by-side prove state-root isolation and import-review gating | Met — `isolation_verdict: isolated` on both rows; import-review row requires compare-or-skip |
| Portable row proves no hidden machine-global writes | Met — `portable_write_guard: fully_suppressed`; all integrations suppressed and disclosed |
| Rollback drills name real blast radius | Met — `rollback_scope: full_artifact_graph` for user rows; `managed_fleet_owned` for managed row |
| Managed and air-gapped diagnostics preserve install-profile identity | Met — `identity_preserved_in_export: true` and `channel_separation_maintained_on_revert: true` |
| Install-profile truth resolves identically across UI, CLI, diagnostics, and support export | Met — single seeded page record consumed by all surfaces |

---

## New Rust public API

`crates/aureline-install` re-exports from the new module:

- `StabilizePortableInstallPage` — top-level page record
- `InstallProfileStableRow` — one install-profile stable row
- `SideBySideImportReviewRow` — import-review row with compare-or-skip proof
- `FleetRolloutInstallDiagnosticsRow` — fleet diagnostics preservation row
- `StabilizePortableInstallSupportExport` — metadata-safe support export
- `ArtifactGraphRollbackScope` — rollback scope discriminant
- `PortableWriteGuardClass` — portable write-guard discriminant
- `SideBySideIsolationVerdict` — isolation verdict discriminant
- `HandlerRegistrationClass` — handler registration discriminant
- `ImportReviewClass` — import review class discriminant
- `StabilizeQualificationToken` — qualification token
- `StabilizeNarrowReasonToken` — narrow reason token
- `audit_stabilize_portable_install_page` — returns defects for a page
- `validate_stabilize_portable_install_page` — returns a structured validation report
- `seeded_stabilize_portable_install_page` — returns the canonical seeded page

---

## Guardrails verified

- **No hidden portable writes:** Any row with `portable_write_guard: hidden_writes_detected`
  causes immediate `Withdrawn` qualification; the audit function short-circuits.
- **No silent state-root collapse:** Import review rows with `blocked_pending_collision_resolution`
  block apply until the collision is resolved.
- **No last-writer-wins handler takeover:** `last_writer_wins_blocked: true` is required on
  all rows that register handlers.
- **No undisclosed isolation:** `isolation_verdict: undisclosed` on any row narrows the page
  to `Preview`; the seeded page carries only `isolated` or `not_applicable` verdicts.
- **No undisclosed rollback blast radius:** `rollback_scope: undisclosed` on any row narrows
  the page to `Beta`; the seeded page carries only named scope values.

---

## Downstream ingest

Surfaces and tools that need install-profile truth should reference:

1. `fixtures/enterprise/m4/stabilize-portable-install-side-by-side-channels-updater/page.json`
   for the seeded stable baseline.
2. `crates/aureline-install::seeded_stabilize_portable_install_page()` for the Rust-facing
   equivalent at test or diagnostic-generation time.

Do not clone status text from this artifact into help copy, About strings, or deployment
notes. Use the typed fields from `InstallProfileStableRow` instead.
