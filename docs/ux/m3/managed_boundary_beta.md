# Managed-boundary, org-switch, seat-quota, grace-window, and offboarding beta surface

Status: seeded
Track: beta managed-truth surface that consumes the published boundary
manifest.

This UX doc is the reviewer-facing entrypoint for the in-product surface
that lets users tell what remains local, what is tenant-owned, what is
entering a grace window, and what is offboarding. It does not mint a
parallel vocabulary; every label, state, and caveat is consumed verbatim
from the published boundary manifest so docs/help, admin, CLI/headless,
support-export, and release-evidence surfaces share one source of truth.

## Canonical artifacts

- Published manifest:
  [`artifacts/milestones/m3/boundary_manifest_beta.yaml`](../../../artifacts/milestones/m3/boundary_manifest_beta.yaml)
- Manifest schema:
  [`schemas/governance/boundary_manifest_beta.schema.json`](../../../schemas/governance/boundary_manifest_beta.schema.json)
- Release-facing narration:
  [`docs/release/m3/managed_boundary_beta.md`](../../release/m3/managed_boundary_beta.md)
- Shell consumer module:
  [`crates/aureline-shell/src/managed_boundary/mod.rs`](../../../crates/aureline-shell/src/managed_boundary/mod.rs)
- Headless inspector binary: `aureline_shell_managed_boundary_beta`
- Fixtures:
  [`fixtures/security/m3/managed_boundary/`](../../../fixtures/security/m3/managed_boundary/)
- Fixture replay test:
  [`crates/aureline-shell/tests/managed_boundary_beta_fixtures.rs`](../../../crates/aureline-shell/tests/managed_boundary_beta_fixtures.rs)

## What the surface shows

Every published boundary row appears on the surface as four aligned
projections, all backed by the same row id and the same vocabulary the
release packet and the support export already use:

1. **Org-switch posture** — the behavior class
   (`preserves_local_state`, `scopes_to_new_org`, `denies_until_resolved`,
   `requires_admin_handoff`, `not_applicable`), the summary, and an
   explicit `local_state_preserved` flag so users see whether switching
   organizations leaves their local buffers, Git state, and on-disk
   profile files untouched.
2. **Seat / quota posture** — the current `quota_state`
   (`seat_active` / `seat_unassigned` / `seat_revoked` /
   `quota_within_window` / `quota_grace` / `quota_exhausted` /
   `not_applicable`), the full set of states the row is allowed to
   observe, and the meter ids the metering surface attributes against.
3. **Grace-window posture** — the window class (`short_lived`,
   `policy_pinned`, `audit_only`, `denied_for_beta`, `not_applicable`),
   the declared `duration_iso8601`, and the summary that names what
   audit/export remain accessible during the window.
4. **Offboarding posture** — the phases the row passes through
   (`announce`, `freeze_writes`, `export_available`, `managed_access_end`,
   `destruction_receipt_issued`, `not_applicable`), the export packet
   class that backs the row's offboarding (`local_support_bundle`,
   `managed_usage_export`, `entitlement_snapshot`, `destruction_receipt`,
   `not_applicable`), and whether a destruction receipt is required when
   managed access ends.

Each projection also carries the row's `local_core_continuity` clause and
its `absence_narrows_to` clause so users can tell what continues to work
locally and how the row narrows when its managed dependencies are absent.

## What the surface refuses to do

- It MUST NOT invent a parallel boundary, seat-state, grace-window, or
  offboarding-phase vocabulary. The closed sets are inherited from the
  published manifest schema; unknown tokens fail closed as typed
  validator defects.
- It MUST NOT widen a `managed` or `paid_seat_bound` row without an
  `absence_narrows_to` clause. The validator (`audit_managed_boundary_beta_rows`)
  emits an `absence_narrowing_missing` defect when the clause is absent.
- It MUST NOT silently fall back to a public endpoint, plaintext secret,
  or implicit managed assumption: the support-export wrapper carries a
  `no_public_endpoint_fallback_invariant`, a
  `local_core_continuity_invariant`, and an `absence_narrowing_invariant`
  that downstream consumers can quote verbatim.
- It MUST NOT block local editing. The published `local_core_continuity`
  clause names the local floor that every other surface narrows against.

## How support and release artifacts link to in-product state

The support-export wrapper
(`ManagedBoundaryBetaSupportExport::from_page`) packages the page, the
per-flow projections, and the invariant flags into a single record with
explicit refs back to:

- the published manifest (`source_manifest_ref`),
- the manifest schema (`source_schema_ref`),
- the release-facing narration (`release_doc_ref`), and
- this UX doc (`ux_doc_ref`).

Release artifacts and support playbooks reference the same row ids and
state tokens shown in product — they do not paraphrase. The fixture
replay test asserts that the in-product projection, the support export,
and the published manifest stay literally in sync.

## How to regenerate the fixtures

```sh
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- page > fixtures/security/m3/managed_boundary/page.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- rows > fixtures/security/m3/managed_boundary/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- org-switch-rows > fixtures/security/m3/managed_boundary/org_switch_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- seat-quota-rows > fixtures/security/m3/managed_boundary/seat_quota_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- grace-window-rows > fixtures/security/m3/managed_boundary/grace_window_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- offboarding-rows > fixtures/security/m3/managed_boundary/offboarding_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- defects > fixtures/security/m3/managed_boundary/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- summary > fixtures/security/m3/managed_boundary/summary.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- render-summary > fixtures/security/m3/managed_boundary/render_summary.json
cargo run -q -p aureline-shell --bin aureline_shell_managed_boundary_beta -- support-export > fixtures/security/m3/managed_boundary/support_export.json
```

The fixture replay test in
`crates/aureline-shell/tests/managed_boundary_beta_fixtures.rs` fails
closed when the checked-in fixtures drift from the published manifest, so
any manifest edit that adds, removes, or changes a row also re-records
the in-product projection.
