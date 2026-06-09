# Generate the M5 channel/profile/provider rollout matrix for depth lanes

This document is the human-readable companion to the canonical M5 channel/profile/provider rollout matrix checked in at `artifacts/release/m5/generate_the_m5_channel_profile_provider_rollout_matrix_for_depth_lanes.json`.

## Purpose

The rollout matrix locks the M5 depth-lane control surface across three independent dimensions:

- **Channel** — `stable`, `beta`, `preview`, `nightly`, `labs`.
- **Deployment profile** — `desktop`, `browser`, `mobile`, `remote`, `headless`.
- **Provider family** — `aureline_hosted`, `local`, `managed_control_plane`, `third_party`.

Each row in the matrix binds one M5 lane (notebook, data-rich, AI-adjacent, framework, review, companion, managed-depth) to a concrete channel/profile/provider tuple and records whether that tuple is eligible, eligible-degraded, blocked, pending evidence, or not applicable.

## Structure

The matrix contains:

- **Rollout rows** — one per lane/channel/profile/provider tuple that is in scope.
- **Rollout state** — `eligible`, `eligible_degraded`, `blocked`, `pending_evidence`, or `not_applicable`.
- **Effective label** — the lifecycle label the tuple carries after narrowing (`lts`, `stable`, `beta`, `preview`, `withdrawn`).
- **Stop rules** — closed conditions that block rollout. Every gap reason has a corresponding rule.
- **Rollout verdict** — `proceed` or `hold`, computed from firing stop rules.
- **Summary counts** — per-state, per-lane, per-channel, per-profile, per-provider, and per-proof-packet-freshness totals.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The matrix is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the rollout verdict against the stable claim manifest and the M5 feature-train matrix, and fail promotion if the matrix is stale or underqualified.

## Downgrade and rollback posture

Every narrowed row carries:

- the effective label it must publish instead of the canonical claim,
- the active gap reasons that forced narrowing,
- the default action a stop rule prescribes (for example, `refresh_proof_packet`, `define_rollback_path`, or `restrict_provider`).

Rows without a defined rollback path or provider readiness story must stay below the stable cutline until the gap clears.
