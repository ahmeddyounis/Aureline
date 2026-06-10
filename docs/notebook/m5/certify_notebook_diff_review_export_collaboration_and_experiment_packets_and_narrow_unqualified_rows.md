# Certify notebook diff, review, export, collaboration, and experiment packets and narrow unqualified rows

## Overview

This document describes the M5 notebook certification layer that binds the
notebook diff/review, export, collaboration, and experiment lineage lanes to
canonical qualification states, downgrade rules, rollback paths, and automatic
narrowing actions.

## Design principles

- **Certified current is never implied from prose alone.** Every lane must point
to a current sub-packet whose freshness, rollback path, and evidence still
support the claim.
- **Unqualified rows are explicitly narrowed.** When a sub-packet goes stale,
the rollback path is missing, or a gap is detected, the visible label drops
rather than staying green.
- **Downgrade reasons are closed vocabulary.** Surfaces must show the exact
reason (`freshness_expired`, `rollback_path_missing`, `policy_blocked`, etc.)
rather than generic status text.
- **Rollback paths are required for certified_current.** A lane with a missing
or untested rollback path cannot claim certified_current.
- **Automation is default for narrowing.** `automatic_narrowing` is the default
action; `manual_hold` and `emergency_rollback` are used for policy or missing
path cases.

## Certification lanes

| Lane | Sub-packet | Key concern |
|---|---|---|
| **Diff / review** | `notebook_diff_packet_v1` | Cell-aware diff, metadata filters, output visibility, raw JSON fallback |
| **Export** | `notebook_share_and_handoff_packet_v1` | Notebook vs report vs artifact scope, redaction before share |
| **Collaboration** | `notebook_collaboration_follow_presenter_packet_v1` | Follow state, presenter authority, live vs captured runtime |
| **Experiment** | `nb.experiment_lineage.packet.m5.01` | Run identities, environment fingerprints, dataset cards, artifact lineage |
| **Narrowing** | `notebook_certification_packet_v1` | Automatic narrowing automation, claim narrowing rules, staged promotion |

## Records

### `NotebookCertificationRow`

Per-lane certification row that carries the lane kind, sub-packet ref,
certification state, rollback path state, downgrade reasons, narrowing action,
and freshness.

### `NotebookCertificationPacket`

Checked-in certification artifact that downstream docs, help, CI, and support
surfaces ingest instead of cloning status text. Contains exactly one row per
lane, example narrowed rows, downgrade rules, rollback path, and freshness SLO.

## Schema

The boundary schema lives at:
`/schemas/notebook/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.schema.json`

## Fixtures

Worked fixtures live at:
`/fixtures/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows/`

## Integration

The crate `aureline-notebook` exposes these records and validators. Downstream
docs, help, support, and CI surfaces consume the checked-in packet and closed
vocabularies rather than redefining them.
