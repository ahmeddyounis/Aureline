# Certify artifact-graph publication, exact-build provenance, rollback/revocation parity, and mirror/offline release truth on all claimed M5 rows

This document is the human-readable companion to the canonical M5
publication-certification register checked in at
`artifacts/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.json`.

## Purpose

The M5 release-control work ships the parts of publication as inspectable product
objects: the release-candidate / publish-target matrix, the promotion ledger, the
scoped rollback/revocation records, the provenance cards, and the clean-room
rebuild proof and rehearsal automation. This register is the **closing
certification layer** over all of them. For every claimed M5 artifact family it
asserts, in one place, that the family is *rebuildable, identifiable,
symbolicated, support-explainable, and revocable as one system* — and narrows the
family below the launch cutline the moment any of that release truth goes stale,
partial, or missing.

A shipped M5 artifact family is not certified just because it builds. It is
certified only when its whole publication artifact graph holds together.

## The seven publication-truth dimensions

Each family row carries exactly one scorecard cell per dimension:

1. **`release_center_parity`** — the release-center object and the headless flow
   render identical artifact-graph truth for the family.
2. **`clean_room_rebuild`** — a fresh clean-room rebuild reproduces the published
   artifact.
3. **`exact_build_symbolication`** — exact-build symbol/source-map linkage supports
   symbolication of the published build.
4. **`publish_target_review`** — the publish target is scoped and reviewed; it never
   inherits ambient credentials.
5. **`rollback_record`** — a scoped rollback record targets the smallest affected
   node set.
6. **`revocation_record`** — a revocation / emergency-disable record reaches every
   channel at parity.
7. **`mirror_offline_parity`** — hosted, mirrored, and offline channels publish the
   family at parity with current drill evidence.

Each cell is graded `pass`, `partial`, `fail`, `waived`, or `missing`. A
non-passing, non-waived cell names its narrowing reason, and the family drops
below the cutline.

## Structure

The register contains:

- **Family rows** — one per claimed M5 artifact family (`notebook_pack`,
  `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`,
  `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`).
- **Scorecard cells** — seven per row, one per publication-truth dimension.
- **`publish_target`** — the scoped-credential posture, with an explicit
  `inherits_ambient_credentials` flag.
- **`mirror_offline`** — hosted/mirrored/offline parity flags and the parity-drill
  freshness state.
- **Proof packet, owner sign-off, downgrade automation, and optional waiver** —
  the same release-control vocabulary used across the M5 batches.
- **Stop rules** — closed conditions that gate promotion; every narrowing reason
  has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Guardrails

- **Publish targets never inherit ambient credentials.** A family whose
  `publish_target.inherits_ambient_credentials` is `true` cannot hold its
  `publish_target_review` dimension; it names `ambient_credential_inherited` and
  narrows below the cutline.
- **No mirror/offline parity claim without current drill evidence.** A family whose
  `mirror_offline` is not at parity across hosted, mirrored, and offline channels —
  or whose parity-drill evidence is breached or missing — cannot hold its
  `mirror_offline_parity` dimension; it names `mirror_offline_drill_stale` and
  narrows.
- **Build-only is not certified.** A family that compiles but cannot be published,
  identified, explained, and revoked as one artifact graph never holds a Stable
  certification.

## Consumption

Downstream release-center, Help/About, service-health, support-export, and docs
surfaces ingest `support_export_projection()` from the typed model rather than
cloning status text. The register reports into the canonical M5 evidence index
named by `evidence_index_ref`, so this publication truth is shiproom-visible
rather than buried in CI.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. The
typed consumer's `validate()` recomputes the summary roll-up and the promotion
verdict against the stable claim manifest; `cargo test -p aureline-release`
enforces the same structural and narrowing invariants the shiproom relies on.
