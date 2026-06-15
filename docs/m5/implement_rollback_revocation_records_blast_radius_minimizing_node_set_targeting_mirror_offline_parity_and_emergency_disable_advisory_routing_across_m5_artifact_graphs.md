# Implement rollback/revocation records, blast-radius-minimizing node-set targeting, mirror/offline parity, and emergency-disable/advisory routing for M5 artifact graphs

This document is the human-readable companion to the rollback/revocation register checked in at `artifacts/release/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.json`.

## Purpose

Where the per-family release graph (`artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json`) speaks for the *release candidate* each M5 artifact family ships, and the promotion-ledger register (`artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json`) speaks for the *promotion history* every M5 artifact graph accumulates, this register is the **recovery-posture** layer beside them. It materializes one inspectable recovery ledger per M5 artifact family: the rollback, revocation, yank, repin, and emergency-disable records a release-center operator triggers, a headless flow replays, and a security advisory routes to hosted, mirrored, and offline customers alike.

Each ledger joins, into a single record, the recovery truth that must survive after a channel, mirror, registry, or docs feed moves:

- the **affected node set** — the artifact-graph nodes the family's recovery actions target, every node carrying a canonical immutable digest and an `installable_after_action` flag, so a record is anchored to immutable graph material and so an unaffected node stays *explicitly* installable rather than swept up in an over-broad revocation;
- the **recovery records** — canonical rollback/revocation/yank/repin/emergency-disable records, each carrying the affected and explicitly-preserved artifact refs, the blast-radius class, the last-known-good target, the linked advisory and revocation refs, the artifact-graph consistency after the action, and the break-glass disclosure for an emergency-disable;
- the **channel delivery parity** — proof that the hosted, mirrored, and offline channels each received the same recovery record set and advisories, so an offline or mirrored customer is never a second-class citizen for emergency-response evidence.

A family only holds its claimed label when every recovery record targets the smallest affected node set and keeps unaffected nodes installable, when the artifact graph stays consistent, when the hosted/mirrored/offline channels are at parity, when every emergency-disable is advisory-routed and reconciled, when its evidence is fresh, when its proof packet is within SLO, and when it is owner-signed. Any family that fails one of those narrows below the launch cutline before promotion and names every reason that forced it there.

## Structure

The register reuses the canonical release-center object model vocabulary (`crates/aureline-release/src/release_center_model`) rather than inventing a local synonym set. In particular, each recovery record is a `RollbackOrRevocationRecord` carrying the canonical `RollbackOrRevocationKind`, `BlastRadiusClass`, and `ArtifactGraphConsistency`. The register contains:

- **Recovery ledgers** — one per M5 artifact family, keyed by family (`notebook_pack`, `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`, `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`).
- **Affected node set** — `RecoveryGraphNode` rows, each carrying a canonical `ImmutableDigest` and an `installable_after_action` flag. The primary node is the one the action targets (pulled); the sidecar node stays installable — the smallest affected node set with the rest of the graph kept installable.
- **Recovery records** — canonical `RollbackOrRevocationRecord` records. A record's `affected_artifact_refs` name the pulled nodes and its `unaffected_artifact_refs` name the explicitly-preserved nodes; the two together classify every node in the set.
- **Channel delivery parity** — one `ChannelDelivery` row per `DeliveryChannel` (`hosted`, `mirrored`, `offline`), each carrying its delivery state, feed ref, the recovery record ids it received, and the advisories it received. A held family carries every channel `current` and at parity (each channel delivers the full record id set).
- **Proof packet, owner sign-off, waiver** — the remaining release-control fields per family.
- **Stop rules** — closed conditions that gate publication. Every narrowing reason (`blast_radius_unscoped`, `unaffected_nodes_not_preserved`, `graph_consistency_broken`, `last_known_good_missing`, `mirror_parity_missing`, `offline_parity_missing`, `channel_delivery_stale`, `advisory_routing_missing`, `emergency_disable_unreconciled`, `evidence_stale`, `proof_packet_stale`, `proof_packet_missing`, `owner_manifest_unsigned`, `waiver_expired`) has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed only from families whose public claim is still at or above the cutline. A family whose claim is already narrowed inherits that ceiling without blocking the whole train.

## Blast-radius-minimizing node-set targeting

A recovery action targets the *smallest* affected node set. The register records, for every node, whether it remains installable after the action, and binds each record's affected and preserved sets to those nodes. A record is well-scoped when it names at least one affected node, classifies every node as affected or preserved, and lists every installable node in its preserved set. The first guardrail forbids **over-revoke**: a record may never list a node the graph model marks installable in its affected (revoked) set — even on a narrowed family — so a smaller node-set action that preserves unaffected artifacts is always chosen over a blanket revocation.

## Hosted, mirrored, and offline parity

Hosted, mirrored, and offline customers receive the same current rollback/revocation/advisory truth for a claimed family. The register records, per channel, the recovery records and advisories delivered and the delivery state; a held family carries every channel `current` and at parity. The second guardrail forbids making offline or mirrored customers **second-class**: an emergency-bearing family may not withhold the truth from the mirrored or offline channel while the hosted channel already has it. A channel whose recovery truth is stale narrows the family with `channel_delivery_stale`, and a channel with no delivery path narrows it with `mirror_parity_missing` or `offline_parity_missing`.

## Emergency disable and advisory routing

Emergency-disable bundles, security advisories, and extension/provider revocation packets ride the *same* auditable `RollbackOrRevocationRecord` model as an ordinary rollback, with their `BreakGlassDisclosure` naming the actor class, the break-glass event ref, the reason class, the reconciliation state, and the reconcile-by window. A revocation, yank, or emergency-disable record that routes no security advisory narrows the family with `advisory_routing_missing`. An emergency-disable that is captured but left unreconciled past its window narrows the family with `emergency_disable_unreconciled` while remaining fully recorded.

## Audit and advisory replay

The `support_export_projection()` carries a per-record **replay** for each family that reconstructs every recovery action — its kind, blast radius, affected and preserved node counts, last-known-good target, routed advisories, revocation refs, auth source, rollout ring, and emergency state — and the hosted/mirrored/offline delivery states. Update surfaces, advisory exports, release-center history, marketplace/package truth, support, and diagnostics render the projection instead of cloning status text.

## Claim narrowing

A family is narrowed below the launch cutline when a record's blast radius is unscoped, when an installable node is not preserved, when the artifact graph is left broken, when a restore action cites no last-known-good target, when the mirrored or offline channel has no delivery path, when a channel's recovery truth is stale, when a withdrawal record routes no advisory, when an emergency-disable is unreconciled, when a record rides stale or missing blocking evidence, when its proof packet is missing or stale, when a relied-on waiver expired, or when owner sign-off is missing. The register proves that every narrowed family names every reason that forced it below the cutline, and that no family carries a label wider than the public claim it backs.

## Consumption

Release-center surfaces, update flows, advisory/support/audit exports, and diagnostics should ingest `support_export_projection()` from the typed model (`current_m5_artifact_graph_recovery_register()`) rather than cloning status text. The projection exposes per-family ledger state, the per-channel delivery states, channel-parity, proof-packet SLO state, recovery and emergency-disable counts, the active narrowing reasons, and the full per-record replay so operators can reconstruct the recovery posture directly.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact, and is regenerated from the in-code builder (`build_m5_artifact_graph_recovery_register()`); a test proves the embedded JSON never drifts from the builder. CI gates recompute the publication verdict against the stable claim manifest, the M5 publication matrix, the per-family release graph, the promotion-ledger register, and the release-center object model, and narrow any family whose required evidence is missing, stale, or downgraded.
