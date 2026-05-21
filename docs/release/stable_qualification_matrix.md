# Stable qualification matrix and mixed-version sections

This document is the reviewer-facing companion for the gated stable
qualification matrix:

- [`/artifacts/release/stable_qualification_matrix.json`](../../artifacts/release/stable_qualification_matrix.json)
- schema: [`/schemas/release/stable_qualification_matrix.schema.json`](../../schemas/release/stable_qualification_matrix.schema.json)
- proof packet: [`/artifacts/release/m4/stable_qualification_matrix_proof_packet.md`](../../artifacts/release/m4/stable_qualification_matrix_proof_packet.md)

The matrix is the **canonical truth** for whether each launch lane has *earned*
its stable claim, and — for every lane that spans two binaries or two services —
what its mixed-version story is. It finalizes the per-lane qualification rows
that ground the stable claim matrix
([`/docs/release/stable_claim_matrix.md`](./stable_claim_matrix.md)): where the
claim matrix decides which *subjects* may publish as Stable, this matrix decides,
lane by lane, whether the qualification behind those claims holds. Downstream
dashboards, docs, Help/About surfaces, the release center, and support exports
MUST ingest this matrix by `row_id` rather than cloning its status text.

It does not re-mint the lifecycle vocabulary: the level vocabulary
(`lts`/`stable`/`beta`/`preview`/`withdrawn`), the qualification states, and the
launch cutline are reused from the stable claim matrix.

## The six lanes

Every row declares exactly one `row_scope` from the closed lane set:

| Lane (`row_scope`) | Covers |
|---|---|
| `desktop` | Desktop launcher and the local helper/sidecar processes it launches. |
| `remote_helper` | Desktop/CLI clients and the remote agent/helper or the managed control plane. |
| `ecosystem` | The extension host and the extension ABI surface. |
| `state_schema` | Saved-artifact and schema readers and writers. |
| `provider` | Provider adapters. |
| `accessibility` | Accessibility behavior across the touched surfaces. |

## The launch cutline

The cutline fixes the boundary between "claimed Stable" and "narrowed below
Stable", reusing the stable claim matrix's level ranking. A lane may hold a
Stable (or LTS) claim only when its qualification state holds the claim with
current, owner-signed proof. Any lane that is not qualified, has stale evidence,
relied on an expired waiver, lost its backing stable claim, or — for a
cross-binary boundary — cannot publish complete mixed-version negotiation data,
drops below the cutline and never inherits an adjacent green row.

## Mixed-version sections

Every cross-binary or cross-service lane (all but `accessibility`) carries a
`mixed_version` section. The section publishes, for that boundary:

- **`boundary_family`** — one of the six enumerated families: launcher and local
  sidecars, desktop/CLI and remote agent, desktop/CLI/browser and the managed
  control plane, the extension host and ABI, saved-artifact/schema readers and
  writers, and provider adapters.
- **`negotiated_fields`** — the fields the two sides negotiate.
- **`supported_skew_window`** — the window class, a human summary, and a ref into
  the version-skew register.
- **`upgrade_order`** and **`rollback_order`** — the declared step order, each
  with a reviewable note.
- **`unsupported_state_behavior`** — the state class, the out-of-window posture
  (`fail_closed`/`read_only`/`degraded`/`explicitly_unsupported`), and the
  contract rule.
- **`claimed_posture`** and **`effective_posture`** — the mixed-version posture
  the boundary is put forward as and the one it effectively holds.

### Completeness rule: coordinated-upgrade-only is the floor

A boundary publishes a **Stable mixed-version claim** only when its
`effective_posture` is `rolling_skew_supported` or `bounded_skew_supported`. A
section that does not publish complete negotiation data — every one of negotiated
fields, supported skew window, upgrade order, rollback order, and
unsupported-state behavior — is **coordinated-upgrade-only**: its
`effective_posture` is forced to `coordinated_upgrade_only`, it carries the
`mixed_version_data_incomplete` reason, and it may not inherit a Stable
mixed-version claim. A Stable mixed-version claim is, additionally, allowed only
on a lane that itself holds stable; a narrowed lane cannot inherit one.

`coordinated_upgrade_only` is also a legitimate *declared* posture for a boundary
that genuinely ships as one artifact set (the desktop launcher and its local
sidecars), as long as its data is complete.

## Downgrade rules and the promotion verdict

Each `downgrade_reason` is watched by a `downgrade_rule`. A rule fires when a row
in its watch set carries the rule's trigger reason; a firing rule that
`blocks_promotion` holds the stable train. The `promotion` block records the
recomputed verdict, the blocking rule ids, and the blocking row ids. The CI gate
(`ci/check_stable_qualification_matrix.py`) recomputes the verdict and summary,
performs the waiver-expiry and evidence-staleness date arithmetic the typed model
cannot, and runs negative drills proving the narrowing, mixed-version,
waiver-expiry, and promotion rejections all fire. Run with `--require-proceed`,
the gate fails when the verdict is `hold`, so shiproom and release tooling block
promotion directly from this artifact.

## Consuming surfaces

Help/About, the release center, support exports, docs, and shiproom dashboards
consume the same record. The typed consumer exposes
`StableQualificationMatrix::support_export_projection`, a redaction-safe
projection carrying each row's lane, claimed and effective level, qualification
state, active downgrade reasons, boundary family, and effective mixed-version
posture — so unsupported-skew downgrades surface as product truth instead of
support-only lore.

## How to refresh

See the proof packet's "How to refresh" section. In short: land the evidence and
the mixed-version declarations first, set each row's honest posture, recompute the
`promotion` and `summary` blocks, run the gate, and commit the regenerated
capture in the same change set. If delivery proves a narrower claim than planned,
narrow it and update the matrix — never paper over a gap with prose.
