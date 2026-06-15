# Implement promotion-timeline records, immutable-digest joins, release-center/headless parity, and break-glass event capture for M5 artifact graphs

This document is the human-readable companion to the promotion-ledger register checked in at `artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json`.

## Purpose

Where the per-family release graph (`artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json`) speaks for the *release candidate* each M5 artifact family ships, and the publication review-sheet register (`artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json`) speaks for the *review sheet* every publication lane exposes, this register is the **promotion-history** layer beside them. It materializes one inspectable promotion ledger per M5 artifact family: the record a release-center operator scrolls, a headless automation flow reconstructs, and an audit or postmortem export replays.

Each ledger joins, into a single record, the promotion truth that must survive after a channel, mirror, registry, or docs feed moves:

- the **affected node set** — the artifact-graph nodes the family's promotions touch, every node carrying a canonical immutable digest, so a promotion is anchored to immutable graph material rather than a mutable "latest" pointer;
- the **promotion timeline** — an ordered list of canonical promotion steps, each carrying its source stage, destination stage, approving actors, evidence bundle refs, immutable digest refs, reversible window, and rollback target, **including** break-glass freezes, emergency publications, and out-of-band corrections in the *same* step model as ordinary promotions;
- the **reconstruction parity** — proof that the release-center UI and the headless plan reconstruct the same ordered history under the same history digest, and that an audit/postmortem export can replay it.

A family only holds its claimed label when every promotion — ordinary and break-glass — is captured as a complete timeline step bound to immutable digests, when release-center and headless flows reconstruct the same history, when an audit/postmortem export can replay it, when each step discloses a reversible window, when its evidence is fresh, when its proof packet is within SLO, and when it is owner-signed. Any family that fails one of those narrows below the launch cutline before promotion and names every reason that forced it there.

## Structure

The register reuses the canonical release-center object model vocabulary (`crates/aureline-release/src/release_center_model`) rather than inventing a local synonym set. It contains:

- **Promotion ledgers** — one per M5 artifact family, keyed by family (`notebook_pack`, `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`, `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`).
- **Affected node set** — `ArtifactGraphNode` rows, each carrying a canonical `ImmutableDigest`. A node's id is its digest id; a timeline step joins to the set by citing those digest ids.
- **Promotion timeline** — canonical `PromotionTimelineStep` records carrying source/destination stage, event class, approving actors, evidence refs, immutable digest refs, reversible window, rollback target, sidecar payload refs, and a `BreakGlassDisclosure`. Break-glass events are not a separate object — they are ordinary steps whose disclosure names the freeze/emergency/correction and its reconciliation.
- **History reconstruction parity** — the release-center and headless reconstruction refs, the history digest each flow reconstructs, the audit/postmortem export ref and digest, the ordered step ids both flows agree on, and the parity state. A `matched` state must carry equal release-center and headless digests, and the audit export must replay the same digest and the same ordered steps.
- **Proof packet, owner sign-off, waiver** — the remaining release-control fields per family.
- **Stop rules** — closed conditions that gate publication. Every narrowing reason (`timeline_capture_bypassed`, `digest_binding_missing`, `affected_node_set_incomplete`, `mutable_latest_pointer`, `reconstruction_divergent`, `audit_replay_unavailable`, `break_glass_unreconciled`, `reversible_window_undisclosed`, `evidence_stale`, `proof_packet_stale`, `proof_packet_missing`, `owner_manifest_unsigned`, `waiver_expired`) has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed only from families whose public claim is still at or above the cutline. A family whose claim is already narrowed inherits that ceiling without blocking the whole train.

## Release-center and headless parity

Release-center and headless flows reconstruct the same promotion history for a given M5 artifact graph. The register records the history digest each flow reconstructs and the ordered step ids both agree on; a held family carries `matched` parity with equal digests, and its reconstructed step ids equal the ordered timeline. A divergent or missing reconstruction narrows the family and names `reconstruction_divergent` or `audit_replay_unavailable`.

## Audit and postmortem replay

The `support_export_projection()` carries a per-step **replay** for each family that reconstructs who promoted what, when, on which evidence, and with which reversible window: every replay entry exposes the step's approving actors, auth source, evidence refs, immutable digest refs, source/destination stage, reversible window, rollback target, and break-glass state. Audit and postmortem surfaces render the projection instead of cloning status text.

## Break-glass capture

Break-glass freezes, emergency publications, and out-of-band corrections ride the same timeline step model as ordinary promotions, with their `BreakGlassDisclosure` naming the actor class, the break-glass event ref, the reason class, the reconciliation state, and the reconcile-by window. The register enforces two guardrails directly:

- An emergency step may **not** bypass timeline capture or digest binding: a break-glass step that omits its stages, approving actors, evidence, or immutable digests is a hard violation, not a waivable narrowing.
- A mutable "latest" pointer may **not** stand in for immutable graph history: a family driven by a mutable pointer narrows with `mutable_latest_pointer`.

A break-glass step that is captured and digest-bound but left unreconciled past its window narrows the family with `break_glass_unreconciled` while remaining fully recorded in the timeline.

## Claim narrowing

A family is narrowed below the launch cutline when a promotion bypassed timeline capture, when a step binds no immutable digest, when the affected node set does not back a cited digest, when a mutable pointer stands in for immutable history, when release-center and headless reconstructions diverge, when no audit/postmortem replay is available, when a break-glass action is unreconciled, when a step discloses neither a reversible window nor a rollback target, when a step rides stale or missing blocking evidence, when its proof packet is missing or stale, when a relied-on waiver expired, or when owner sign-off is missing. The register proves that every narrowed family names every reason that forced it below the cutline, and that no family carries a label wider than the public claim it backs.

## Consumption

Release-center surfaces, headless publication flows, and support/audit/export packets should ingest `support_export_projection()` from the typed model (`current_m5_artifact_graph_promotion_ledger()`) rather than cloning status text. The projection exposes per-family ledger state, history-pointer class, parity state, audit-replay availability, proof-packet SLO state, timeline and break-glass step counts, the active narrowing reasons, and the full per-step replay so operators can reconstruct the promotion posture directly.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact, and is regenerated from the in-code builder (`build_m5_artifact_graph_promotion_ledger()`); a test proves the embedded JSON never drifts from the builder. CI gates recompute the publication verdict against the stable claim manifest, the M5 publication matrix, the per-family release graph, and the release-center object model, and narrow any family whose required evidence is missing, stale, or downgraded.
