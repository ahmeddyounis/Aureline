# Stable qualification matrix — proof packet

Reviewer-facing proof packet for the gated stable qualification matrix and its
mixed-version sections for the desktop, remote/helper, ecosystem, state/schema,
provider, and accessibility lanes.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Matrix: [`/artifacts/release/stable_qualification_matrix.json`](../stable_qualification_matrix.json)
- Schema: [`/schemas/release/stable_qualification_matrix.schema.json`](../../../schemas/release/stable_qualification_matrix.schema.json)
- Companion doc: [`/docs/release/stable_qualification_matrix.md`](../../../docs/release/stable_qualification_matrix.md)
- Validator: `ci/check_stable_qualification_matrix.py`
- Validation capture:
  [`/artifacts/release/captures/stable_qualification_matrix_validation_capture.json`](../captures/stable_qualification_matrix_validation_capture.json)
- Typed consumer: `aureline_release::stable_qualification_matrix`

## What this packet proves

1. **Every lane has exactly one typed qualification row.** Each of the six lanes
   the milestone enumerates — desktop, remote/helper, ecosystem, state/schema,
   provider, accessibility — has a [`QualificationRow`] binding it to the level
   it is put forward as (`claimed_level`), the level it effectively holds after
   narrowing (`effective_level`), its backing stable claim, its proof refs and
   freshness window, and its owner sign-off. The matrix reuses the stable claim
   matrix's level and qualification-state vocabularies rather than re-minting
   lifecycle labels.

2. **Every cross-binary boundary publishes a mixed-version section.** The five
   boundary lanes carry a `mixed_version` section that publishes the negotiated
   fields, the supported skew window, the upgrade order, the rollback order, and
   the unsupported-state behavior for that boundary. The matrix covers all six
   enumerated boundary families:

   | Boundary family | Lane | Claimed → effective posture |
   |---|---|---|
   | `launcher_and_local_sidecars` | desktop | coordinated_upgrade_only → coordinated_upgrade_only |
   | `desktop_cli_and_remote_agent` | remote_helper | bounded_skew_supported → bounded_skew_supported |
   | `desktop_cli_browser_and_managed_control_plane` | remote_helper | rolling_skew_supported → coordinated_upgrade_only |
   | `extension_host_and_abi` | ecosystem | bounded_skew_supported → coordinated_upgrade_only |
   | `saved_artifact_and_schema_readers_writers` | state_schema | rolling_skew_supported → coordinated_upgrade_only |
   | `provider_adapters` | provider | bounded_skew_supported → bounded_skew_supported |

   The accessibility lane is a cross-cutting quality row, not a cross-binary
   boundary, so it carries no mixed-version section.

3. **A boundary that cannot publish complete data is coordinated-upgrade-only.**
   The managed-control-plane boundary leaves its upgrade order and rollback order
   undeclared, and the saved-artifact/schema reader-writer boundary leaves its
   supported skew window undeclared. Both are therefore *coordinated-upgrade-only*
   in `effective_posture` and carry the `mixed_version_data_incomplete` reason —
   they may not inherit a Stable mixed-version claim. A Stable mixed-version claim
   (rolling/bounded effective posture) is allowed only on a complete section whose
   lane itself holds stable; the gate proves a narrowed row cannot inherit one.

4. **Downgrade automation narrows unqualified lanes before publication.** A lane
   that is not qualified, has stale evidence past its freshness window, relied on
   an expired waiver, lost its backing stable claim, or cannot back its
   mixed-version claim narrows below the cutline automatically. Every downgrade
   reason is watched by a downgrade rule, and the firing rules drive the
   promotion `proceed`/`hold` verdict. The CI gate additionally performs the date
   arithmetic the typed model cannot — waiver expiry and evidence staleness
   against the matrix `as_of` date.

## Proof-index registration

Each row registers under one row of the stable proof index
([`/artifacts/milestones/m3/public_proof_index.md`](../../milestones/m3/public_proof_index.md))
via its `evidence.proof_index_ref`, so this lane's proof is anchored to the
public-proof artifact index rather than to ad hoc notes. The cross-binary lanes
register under the version-skew lane (`m3_public_proof:version_skew_truth`); the
accessibility lane registers under the docs-freshness lane.

## Current posture

At this revision four lanes hold a Stable claim (desktop, remote agent/helper,
provider under an active waiver, and accessibility) and three are narrowed below
the cutline: the managed control plane and the state/schema reader-writer
boundary cannot publish complete mixed-version negotiation data, and the
extension host/ABI lane's evidence aged out of its freshness window. Their
reasons fire four blocking downgrade rules, so the stable train **holds**. That
is the honest posture: the repository is pre-implementation and the mixed-version
contracts for several boundaries are not yet ratified.

## Accessibility of this lane

The matrix and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables (no color-only encoding), and the
machine source carries the same truth so Help/About, the release center, support
exports, docs, and shiproom dashboards ingest one record rather than restating
status text. The accessibility lane is itself a first-class qualification row,
backed by the keyboard, screen-reader, IME/grapheme/bidi, zoom, high-contrast,
and reduced-motion fixtures under `fixtures/accessibility/`, so accessibility is
qualified as a lane rather than treated as a post-pass.

## How to refresh

1. Land qualification evidence, declared skew windows, upgrade/rollback orders,
   and waivers first; point each row's `evidence_refs`, `proof_index_ref`, and
   `mixed_version` fields at the canonical packets.
2. Set each row's `qualification_state`, `active_downgrade_reasons`,
   `effective_level`, and `mixed_version.effective_posture` to the honest posture.
   A boundary that has not declared its negotiated fields, skew window, upgrade
   order, rollback order, and unsupported-state behavior stays
   coordinated-upgrade-only and may not claim a Stable mixed-version posture.
3. Recompute the `promotion` block and `summary`, then run
   `python3 ci/check_stable_qualification_matrix.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower stable claim than planned, narrow the claim and
   update the matrix — do not paper over the gap with prose.
