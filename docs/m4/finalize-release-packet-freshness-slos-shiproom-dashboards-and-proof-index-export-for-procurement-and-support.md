# Finalize release-packet freshness SLOs, shiproom dashboards, and proof-index export for procurement and support

**M04-183** | Generated: 2026-06-03

## Overview

This document defines the M4-stable release-packet freshness SLO register, shiproom dashboard alarm integration, and proof-index export format. It is the canonical source for:

- Freshness SLOs covering claim-publication manifests, reference-workspace reports, compatibility reports, evaluation evidence packs, pilot evidence packs, Help/About consumers, support-export consumers, build packets, benchmark packets, shiproom dashboard panels, and proof-index export objects.
- Shiproom dashboard stale-claim and stale-report alarms that reference exact object IDs before any widening action.
- Proof-index export fields that preserve validity window, stale reason, downgrade-propagation status, and consuming-surface set for every public-proof object.

## Register identity

- `register_id`: `finalize:m4:freshness_slos_shiproom_dashboards_proof_index_export`
- `record_kind`: `finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support`
- `schema_version`: `1`
- `as_of`: `2026-06-03`

## Lifecycle labels

The register reuses the closed lifecycle-label vocabulary:

- `lts` — Long-term-support stable
- `stable` — Broad stable
- `beta` — Narrowed to beta
- `preview` — Narrowed to preview
- `withdrawn` — Claim withdrawn

## Object kinds

Eleven kinds of object are tracked:

1. **Claim publication manifest** (`claim_publication_manifest`)
2. **Reference workspace report** (`reference_workspace_report`)
3. **Compatibility report** (`compatibility_report`)
4. **Evaluation evidence pack** (`evaluation_evidence_pack`)
5. **Pilot evidence pack** (`pilot_evidence_pack`)
6. **Help/About consumer** (`help_about_consumer`)
7. **Support export consumer** (`support_export_consumer`)
8. **Build packet** (`build_packet`)
9. **Benchmark packet** (`benchmark_packet`)
10. **Shiproom dashboard panel** (`shiproom_dashboard_panel`)
11. **Proof index export object** (`proof_index_export_object`)

## Object states

- `current` — Backed, current, owner-signed.
- `current_on_waiver` — Held on an active waiver.
- `narrowed_unbacked` — Missing evidence or incomplete.
- `narrowed_claim_narrowed` — Inherited from a narrowed claim.
- `narrowed_stale` — Proof packet breached freshness SLO.
- `narrowed_waiver_expired` — Waiver expired.
- `narrowed_downgrade_pending` — Downgrade propagation is pending.

## Gap reasons

- `claim_label_narrowed`
- `object_capability_absent`
- `evidence_incomplete`
- `proof_packet_freshness_breached`
- `proof_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`
- `downgrade_propagation_pending`
- `stale_claim_alarm`
- `stale_report_alarm`

## Publication actions

- `hold_publication`
- `narrow_object_label`
- `refresh_proof_packet`
- `recapture_evidence`
- `request_owner_signoff`
- `propagate_downgrade`

## Downgrade propagation statuses

- `not_required` — No downgrade needed.
- `propagated` — Downgrade has been pushed to all consuming surfaces.
- `pending` — Downgrade is pending propagation.
- `blocked` — Downgrade propagation is blocked.

## Consuming surfaces

- `docs_site`
- `help_about`
- `service_health`
- `support_export`
- `release_packet`
- `shiproom_dashboard`
- `procurement_portal`

## Checked-in artifact

The canonical JSON artifact is:

- `artifacts/release/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support.json`

## Schema

The JSON Schema is:

- `schemas/release/finalize-release-packet-freshness-slos-shiproom-dashboards-and-proof-index-export-for-procurement-and-support.schema.json`

## Verification

Run the protected tests in `crates/aureline-release/tests/` to validate the checked-in artifact against the typed model.
