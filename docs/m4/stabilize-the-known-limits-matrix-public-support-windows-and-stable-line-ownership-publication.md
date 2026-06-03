# Stabilize the known-limits matrix, public support windows, and stable-line ownership publication

**M04-182** | Generated: 2026-06-03

## Overview

This document defines the M4-stable known-limits matrix, public support window commitments, and stable-line ownership publication register. It is the canonical source for:

- Known-limits entries that document caveats and unsupported states for each surface.
- Public support windows that commit to maintenance timelines for each release line.
- Stable-line ownership records that name the owning team for each published stable claim.

## Register identity

- `register_id`: `stabilize:m4:known_limits_support_windows_ownership`
- `record_kind`: `stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication`
- `schema_version`: `1`
- `as_of`: `2026-06-03`

## Lifecycle labels

The register reuses the closed lifecycle-label vocabulary:

- `lts` — Long-term-support stable
- `stable` — Broad stable
- `beta` — Narrowed to beta
- `preview` — Narrowed to preview
- `withdrawn` — Claim withdrawn

## Row kinds

Three kinds of row are tracked:

1. **Known limit** (`known_limit`) — Documents caveats and unsupported-state behavior.
2. **Public support window** (`public_support_window`) — Commits to a support timeline.
3. **Stable-line ownership** (`stable_line_ownership`) — Publishes the owning team.

## Publication states

- `stabilized` — Backed, current, owner-signed.
- `stabilized_on_waiver` — Held on an active waiver.
- `narrowed_unbacked` — Missing evidence or incomplete.
- `narrowed_claim_narrowed` — Inherited from a narrowed claim.
- `narrowed_stale` — Proof packet breached freshness SLO.
- `narrowed_waiver_expired` — Waiver expired.
- `narrowed_support_expired` — Support window expired (support-window rows).
- `narrowed_ownership_missing` — Ownership record missing (ownership rows).

## Gap reasons

- `claim_label_narrowed`
- `surface_capability_absent`
- `evidence_incomplete`
- `proof_packet_freshness_breached`
- `proof_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`
- `support_window_expired`
- `ownership_unpublished`

## Publication actions

- `hold_publication`
- `narrow_claim_label`
- `refresh_proof_packet`
- `recapture_evidence`
- `renew_support_window`
- `publish_ownership_record`
- `request_owner_signoff`

## Checked-in artifact

The canonical JSON artifact is:

- `artifacts/release/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json`

## Schema

The JSON Schema is:

- `schemas/release/stabilize-the-known-limits-matrix-public-support-windows-and-stable-line-ownership-publication.schema.json`

## Verification

Run the protected tests in `crates/aureline-release/tests/` to validate the checked-in artifact against the typed model.
