# Support bundle contract

This document freezes the support/export bundle contract Aureline uses
for local preview, manual export, field handoff, parity audit, and
future support tooling. It turns support bundles into governed packets
instead of opaque zip files with ad hoc manifests.

Companion artifacts:

- [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — machine-readable boundary for the bundle record and the bundle
  redaction-profile record.
- [`/docs/support/support_center_concept.md`](./support_center_concept.md)
  — product-facing concept note for bundle preview, Project Doctor,
  recovery ladders, repair preview, and issue handoff.
- [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md)
  and
  [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  — object-specific issue/report handoff family that can attach support
  bundles, incident refs, recovery refs, and docs/help descriptor refs
  without inventing surface-local forms.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support packet family index that gives bundle consumers a stable
  packet-family vocabulary.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  and
  [`/docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
  — class-level retention/export posture the bundle must quote rather
  than re-labelling privately.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — secret/redaction defaults the bundle inherits for secret-bearing
  data classes.
- [`/artifacts/governance/evidence_id_conventions.md`](../../artifacts/governance/evidence_id_conventions.md)
  and
  [`/schemas/governance/evidence_packet_header.schema.json`](../../schemas/governance/evidence_packet_header.schema.json)
  — stable evidence-id grammar plus the shared packet header the
  bundle embeds.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  and
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  — release-governance maps that pin which supportability and
  symbolication families are promotion-bearing, which shiproom gate
  reviews them, and how mirror/offline emergency publication preserves
  the same exact-build joins.
- [`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../artifacts/support/deployment_drill_catalog_seed.yaml)
  and
  [`/docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md)
  — continuity/locality/tenant/key posture vocabulary the bundle
  reuses.
- [`/docs/support/exact_build_symbolication_smoke.md`](./exact_build_symbolication_smoke.md)
  and
  [`/artifacts/support/crash_artifact_retention_seed.json`](../../artifacts/support/crash_artifact_retention_seed.json)
  — minimal exact-build crash-symbolication smoke path plus the shared
  crash-artifact retention/redaction seed for envelopes, dump/core
  manifests, symbolication reports, and support-bundle references.
- [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md)
  and
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — shared emergency-action, revocation, mirror/manual-import, and
  local-continuity contract that support packets now preserve by stable
  ref instead of flattening into prose.
- [`/fixtures/support/redaction_profiles/`](../../fixtures/support/redaction_profiles/)
  — seed bundle redaction profiles.
- [`/fixtures/support/support_bundle_examples/`](../../fixtures/support/support_bundle_examples/)
  — worked bundle example demonstrating embedded, by-reference,
  local-only, optional-upload, and intentionally excluded artifacts.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` §10.15 and §10.22.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §8.10,
  §24.2.2, §24.2.3, §24.4, §24.5, and Appendix I.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §22.20, §23.26, and
  Appendix BZ.
- `.t2/docs/Aureline_Milestones_Document.md` §3.20, §3.21, and §7.4.

If this document disagrees with those sources, those sources win and
this document plus the schema update in the same change.

## Why this exists

Support bundles already appear throughout the product spec, but the
repository previously only had surrounding seeds: the support packet
index, exact-build identity, redaction ADRs, record-class governance,
continuity drills, and evidence-id conventions. Without one governed
bundle packet:

- support exports would drift back toward free-form archives;
- exact-build identity, route truth, and continuity posture would be
  bolted on as side metadata;
- attachments would collapse into one generic manifest bucket with no
  distinction between embedded bytes, local-only retention, managed
  references, optional high-friction upload, or intentional exclusion;
- support tooling would have to redesign the packet family before it
  could build previews, audits, or drill automation.

This contract closes that gap by composing the existing vocabularies in
one support-bundle record.

## Scope

Frozen at this revision:

- one `support_bundle_record` that can carry a shared evidence-packet
  header, exact-build/install truth, route/execution truth,
  locality/tenant/key posture, recovery-ladder state, repair and
  checkpoint linkage, fault-domain/restart lineage, storage/cache
  classes, consent markers, waived fields, and a typed artifact
  manifest;
- one `support_bundle_redaction_profile_record` used by preview/export
  tooling to decide what embeds by default, stays local-only, exports
  by reference, needs review, or remains excluded;
- explicit distinction between `storage_mode`,
  `embedding_state`, `support_export_posture`, and `redaction_class`
  on every artifact row;
- direct linkage to the record-class registry, ADR-0007, evidence-id
  conventions, the shared packet header, the continuity drill catalog,
  the emergency-action / revocation contract, and the runtime
  fault-domain contract.

Out of scope:

- hosted ticket submission or automatic upload;
- PII scanning implementation;
- final support-center UI;
- full fault-domain schema outside the support-bundle packet.

## Record overview

Every `support_bundle_record` contains these major blocks:

| Block | Job |
|---|---|
| `bundle_header` | stable packet id, evidence id, ownership, freshness, requirement ids, claim refs, and exact-build joins |
| `governance_bindings` | self-describing links back to the registry, ADRs, evidence-id rules, continuity catalog, and related packet schemas |
| `build_and_install_context` | exact-build identity, install mode/channel, updater owner, docs version-match, and known-limit refs |
| `route_and_execution_context` | route-truth packet refs plus compact origin/target/route/exposure summaries |
| `locality_and_continuity_context` | deployment profile, locality/region/tenant/key posture, continuity drill refs, and local-safe vs blocked capability lists |
| `recovery_context` | current rung, rung history, repair refs, checkpoints, and affected storage classes |
| `fault_domain_context` | fault-domain ids, restart counts, restart-lineage refs, checkpoint refs, and quarantine causes |
| `storage_context` | typed storage/cache class inventory plus reset candidates and pinned evidence refs |
| `consent_markers` / `waived_fields` | visible export consent and typed gaps/waivers instead of hidden missing metadata |
| `artifact_manifest` | explicit artifact rows with section, kind, data class, redaction class, storage mode, and embedding state |

The bundle itself resolves to `record_class_id: support_bundle_archive`
so it inherits the record-class registry's retention/export posture.

## Artifact manifest semantics

The artifact manifest is the core of the contract. Every row names:

- `bundle_section_class` so the bundle stays navigable by support task;
- `artifact_kind_class` so future tooling does not guess what a blob
  is;
- `data_class` and `redaction_class` separately;
- `support_export_posture` so the row preserves the default collection
  rule it came from;
- `storage_mode` so the user/tool can see whether the body is embedded,
  kept locally, represented by a managed ref, available only through a
  later high-friction upload, or intentionally excluded;
- `embedding_state` so references, omissions, and redacted embeddings
  do not look the same;
- `materialization.*_ref` fields that point at the embedded member,
  local retained copy, managed reference, optional upload token, or
  omission reason.

### Storage mode vs embedding state

These fields are intentionally separate:

| Field | Answers |
|---|---|
| `storage_mode` | where the authoritative or available body lives |
| `embedding_state` | whether the exported bundle actually carries the body |

Examples:

- `storage_mode = embedded_export_copy` and `embedding_state = embedded`
  means the bytes are inside the exported bundle.
- `storage_mode = managed_reference_only` and
  `embedding_state = by_reference` means the bundle exports a stable
  ref, not the body.
- `storage_mode = local_only_copy_retained` and
  `embedding_state = omitted` means the body stayed on-device, with a
  local retention ref exported instead.
- `storage_mode = optional_upload_candidate` and
  `embedding_state = omitted` means the bundle does not yet contain the
  bytes, but a later high-friction follow-up could promote them.
- `storage_mode = intentionally_excluded` and
  `embedding_state = omitted` means the bundle carries only the typed
  reason for exclusion.

This is the distinction the PRD and supportability architecture need in
order to keep bundle preview honest.

## Redaction profiles

The sibling `support_bundle_redaction_profile_record` exists so bundle
preview/export tooling can point at one ruleset rather than hard-coding
per-artifact behavior in UI or scripts.

Every rule contains:

- a selector over artifact kinds, bundle sections, data classes, and/or
  record-class ids;
- a `handling_class` such as `export_embedded_metadata_only`,
  `retain_local_only`, `export_by_reference`, `review_before_export`,
  or `exclude_always`;
- the redaction class applied when the rule wins; and
- whether the class remains inspectable without raw content.

The seeded profiles in
[`/fixtures/support/redaction_profiles/`](../../fixtures/support/redaction_profiles/)
show local-first export, operator-reviewed escalation, local-only
retention, review-required code-adjacent captures, and always-excluded
high-risk classes.

## Build, route, locality, and fault truth

The bundle is explicitly allowed to carry support-critical truth without
inventing side metadata:

- build/install truth lives in `build_and_install_context`;
- execution/route truth lives in `route_and_execution_context`;
- locality/tenant/key posture and continuity drill bindings live in
  `locality_and_continuity_context`;
- fault-domain ids, restart counts, restart-lineage refs, and
  checkpoint/quarantine state live in `fault_domain_context`;
- recovery-ladder step, rung history, and repair/checkpoint refs live
  in `recovery_context`.
- emergency-action, revocation, and manual-import receipt refs may live
  in `artifact_manifest` rows with their upstream ids, freshness, and
  continuity state preserved instead of paraphrased.

That makes the packet useful for future support tooling, shiproom, or
field handoff without a redesign of its core fields.

For release-facing support cases, the bundle also follows the release
artifact-family posture map: `support_runbook_bundle`,
`ide_debug_symbols`, `cli_debug_symbols`, `source_map_bundle`, and
`crash_symbols_archive` are promotion-bearing families, not optional
support leftovers. Shiproom and release packets therefore expect the
bundle to preserve the same exact-build, known-limit, mirror/offline,
and rollback joins those families carry in the release maps.

## Example

[`recovery_ladder_remote_connector_loss.json`](../../fixtures/support/support_bundle_examples/recovery_ladder_remote_connector_loss.json)
demonstrates one realistic local-first bundle:

- exact-build identity is embedded;
- a route-truth packet is exported by reference;
- a renderer trace stays local-only with a retained local ref;
- a raw crash dump is marked as an optional upload candidate after the
  user denied high-risk export;
- full shell history is intentionally excluded with an explicit reason;
- the bundle cites continuity drill, fault-domain, restart-lineage,
  repair, and checkpoint refs on the same packet.

That is the minimum shape future support tooling should be able to read
mechanically.

The crash symbolication smoke path under
[`/docs/support/exact_build_symbolication_smoke.md`](./exact_build_symbolication_smoke.md)
reuses the same artifact classes and support-bundle joins on one
minimal crash corpus so exact-build supportability is exercised beyond
manifest publication alone.
