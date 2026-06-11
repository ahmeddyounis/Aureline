# M5 mirror, private-registry, and side-load review

This document describes the canonical packet that freezes the **M5 mirror,
private-registry, and side-load review** lane: export-safe install/update review
packets that keep the same disclosure for mirrored, private-registry, manual-import,
and air-gapped artifacts as for public-registry installs, plus the enterprise policy
filtering that gates the new M5 artifact families. It is the user-facing companion to
the governed artifact at `artifacts/ecosystem/m5/m5-mirror-and-sideload.json` and the
typed model in the `aureline-ecosystem` crate (`m5_mirror_and_sideload`).

Where the
[`M5 ecosystem install-governance matrix`](m5-ecosystem-install-governance-matrix.md)
freezes one governance row per marketed artifact family, the
[`M5 marketplace fact-views`](m5-marketplace-fact-views.md) project that truth into the
storefront, and the [`M5 install review`](m5-install-review.md) turn install into one
reviewed change, this packet proves that the review a user sees does **not** get
thinner when an artifact arrives through a mirror, a private registry, a manual import,
or an air-gapped transfer. Sovereign and offline users get the same review surface, not
a second-class one.

## What this packet covers

The packet carries three record families:

- a **review packet** is one export-safe install/update review for a single
  acquisition;
- a **policy filter** is one enterprise gate over a stable package dimension; and
- a **policy evaluation** applies the filter set to a review packet and recomputes a
  gate decision.

Each review packet reproduces the full public-registry install-review fact set —
package identity (`package_kind`, `source_class`, `governance_family_ref`,
`publisher_ref`, `signing_root_ref`, `namespace_ref`), `compatibility_label`,
`permission_manifest_state`, `runtime_origin`, `bridge_native_state`,
`lifecycle_state`, `support_class`, `evidence_freshness`, `activation_budget_band`,
and `rollback_posture` — and adds the continuity facts these lanes make visible:

- **How did it arrive?** An `acquisition_channel` of `public_registry`,
  `enterprise_mirror`, `private_registry`, `manual_import`, or `air_gapped_import`, and
  a `mirror_posture` of `direct_first_party`, `enterprise_mirrored`, `private_registry`,
  or `manually_imported`.
- **Did the publisher change?** A `publisher_transfer_state`, a
  `signing_root_continuity`, and a `namespace_state`.
- **Is it still maintained?** A `maintenance_state` of `actively_maintained`,
  `maintenance_reduced`, `unmaintained`, or `orphaned`.
- **How fresh and trustworthy is the copy?** A `mirror_freshness` and a
  `provenance_level` of `full_attested`, `signed_no_attestation`, `checksum_only`, or
  `unverifiable`.
- **What does it require of the network?** A `network_class` of `offline`,
  `local_only`, `scoped_network`, or `open_network`.

Every packet also carries the full backing-ref set — `provenance_ref`,
`permission_manifest_ref`, `compatibility_ref`, `activation_budget_ref`,
`rollback_ref`, `publisher_continuity_ref`, and `support_export_ref` — so a
manual-import or air-gapped review produces an export-safe packet equivalent to a
public-registry install.

## Disposition is recomputed, not stored by hand

The `review_disposition` a packet publishes — `proceed`, `review_required`, or
`blocked` — is **not** hand-entered. It is recomputed as the strongest minimum
disposition over every `continuity_signal` detected from the packet's facts, and the
stored disposition and signal set must equal that recomputation or validation fails.

Review-class signals force at least `review_required`: `evidence_not_current`,
`mirror_stale`, `publisher_transferred_verified`, `maintenance_reduced`,
`provenance_reduced`, `non_native_runtime`, `support_narrowed`, and
`activation_budget_exceeded`.

Guardrail-class signals force `blocked`: `publisher_discontinuous`,
`signing_root_changed`, `namespace_discontinuous`, `unmaintained`,
`provenance_unverifiable`, `permission_expansion_unreviewed`,
`compatibility_unsupported`, `rollback_incomplete`, and `quarantined`.

## A mirror or side-load can never bypass a guardrail

This is the lane guardrail. A mirrored, private-registry, manually-imported, or
air-gapped artifact must run the same checks as a first-party install. The packet
enforces this several ways:

- the recomputed disposition routes an unreviewed permission expansion, an unsupported
  compatibility target, an incomplete or irreversible rollback, a quarantine, a changed
  signing root, a discontinuous namespace, an unmaintained package, an unverified
  publisher transfer, or unverifiable provenance straight to `blocked` regardless of
  the acquisition channel;
- every packet must be export-safe — it must carry every backing ref a first-party
  install carries (`NotExportSafe` otherwise) — so reduced provenance widens the review
  but never drops evidence; and
- publisher transfer, signing-root and namespace continuity, mirror freshness, and
  provenance reduction are surfaced as continuity signals **before** install or update
  proceeds, and preserved into the export projection for support and audit.

## Enterprise policy filtering

A `policy_filter` gates the new M5 artifact families by a stable `policy_dimension` —
`publisher`, `signing_root`, `runtime_origin`, `capability_class`, `network_class`,
`support_class`, `bridge_state`, or `activation_budget_band` — with a `policy_effect`
of `allow`, `require_approval`, or `block`. A filter matches a packet when any of the
packet's values for that dimension is in the filter's `match_values`.

A `policy_evaluation` applies the filter set to a review packet and recomputes its
`gate_decision` (`allowed`, `approval_required`, or `blocked`) as the stronger of the
matched filter effect and the packet's own review disposition. Policy can therefore
**tighten** admission — an over-budget block can stop a package the review only flagged
for review — but it can never **loosen** it below the guardrail the review already
requires: a blocked review can never be admitted, and an approval-required review can
never drop to allowed.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each packet's package
kind, source class, acquisition channel, mirror posture, runtime origin, network
class, mirror freshness, provenance level, publisher transfer state, review
disposition, and continuity-signal tokens, plus `blocked_count` and
`side_loaded_count`. Support bundles, docs/help, and release/audit packets should
ingest this projection directly rather than re-describing mirror, side-load,
provenance, and continuity status by hand, so the product, support exports, and
release evidence all cite the same records.

## Validation

`M5MirrorAndSideload::validate()` reports every violation, including an unsupported
schema version or record kind, non-canonical closed vocabularies, empty required
fields, duplicate review/filter/evaluation ids, duplicate continuity signals or
capability classes, a disposition or signal set that disagrees with the recomputation,
a packet that drops a backing ref, a filter with no match values, an evaluation that
dangles off a missing packet or filter, an evaluation whose matched filters, strongest
effect, or gate decision disagrees with the recomputation, and a summary block that
disagrees with the records.
