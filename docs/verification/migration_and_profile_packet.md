# Migration import-diff, profile portability, temporary-profile, sync-conflict, and rollback-checkpoint verification seed

This packet freezes one shared verification story for migration /
import / restore claims: which fields an import-diff row MUST project,
how lossy-vs-exact migration labels project onto the
`importer_outcome_row_record` and `restore_provenance_record` vocabulary
without a parallel "best effort" label, which profile-portability
states a profile artifact MAY claim across machines, how a
temporary / ephemeral profile session declares its lifecycle and what
it MUST NOT persist, how a sync-conflict review row names the two
sides and a required reviewer action, and what a rollback-checkpoint
record has to carry so a claim of "restored to prior state" is
inspectable after apply, partial apply, or support export. It exists
so later migration-center, first-run import, post-import review,
profile-library, managed-sync, support-export, and docs/help surfaces
reuse one inspectable object model instead of inventing per-surface
migration copy, silent "approximate" labels, or an ambiguous
"restored" chip that mixes exact round-trip with best-effort reflow.

If this packet, the
[`import_diff_manifest.yaml`](../../fixtures/migration/import_diff_manifest.yaml)
corpus, the
[`portability_and_temp_profile_cases/`](../../fixtures/profiles/portability_and_temp_profile_cases/)
fixtures, the
[`rollback_checkpoint_examples/`](../../artifacts/migration/rollback_checkpoint_examples/)
artifacts, and the frozen migration-center, importer-outcome,
restore-provenance, portable-profile, and state-map contracts
disagree, the frozen contracts win for tooling and this packet must
update in the same change.

Companion artifacts:

- [`/fixtures/migration/import_diff_manifest.yaml`](../../fixtures/migration/import_diff_manifest.yaml)
  — machine-readable case roster covering clean import, partial
  import, skipped objects, alias / remap import, temporary profile
  session, sync conflict requiring review, downgrade migration, and
  rollback to a prior checkpoint.
- [`/fixtures/profiles/portability_and_temp_profile_cases/`](../../fixtures/profiles/portability_and_temp_profile_cases/)
  — reviewer-facing portable-profile and temporary-profile worked
  rows covering the closed `profile_portability_state` and
  `temporary_profile_lifecycle_state` vocabularies this packet
  freezes.
- [`/artifacts/migration/rollback_checkpoint_examples/`](../../artifacts/migration/rollback_checkpoint_examples/)
  — reviewer-facing `migration_restore_record` examples naming
  checkpoint scope, availability, cleanup posture, partial-apply
  disclosure, preserved prior artifacts, and support-export linkage
  for each rollback outcome class.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  — canonical `migration_session_record`,
  `importer_outcome_row_record`, `importer_outcome_packet_record`,
  `migration_shortcut_digest_record`, and `migration_restore_record`
  vocabulary this packet reuses.
- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md)
  — canonical state-plane, fidelity-label, downgrade-reason,
  failure-state, and preserved-prior-artifact vocabulary this packet
  reuses.
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  — canonical portable-profile, state-map row, export-manifest, and
  restore-provenance vocabulary this packet reuses.
- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  — canonical `project_entry_action_record`, `restore_prompt_record`,
  `migration_result_record`, and `restore_level` vocabulary this
  packet cites.
- [`/schemas/migration/migration_session.schema.json`](../../schemas/migration/migration_session.schema.json)
  — boundary schema carrying `migration_session_record`,
  `migration_shortcut_digest_record`, and `migration_restore_record`
  shapes this packet projects.
- [`/schemas/migration/importer_outcome.schema.json`](../../schemas/migration/importer_outcome.schema.json)
  — boundary schema carrying `importer_outcome_row_record` and
  `importer_outcome_packet_record` shapes this packet projects.
- [`/schemas/profile/portable_profile.schema.json`](../../schemas/profile/portable_profile.schema.json)
  — boundary schema for the portable-profile artifact, export
  manifest, state-map rows, and restore-provenance rows this packet
  resolves portability claims against.
- [`/schemas/state/restore_provenance.schema.json`](../../schemas/state/restore_provenance.schema.json)
  — shared restore-provenance schema the packet cites for rollback
  checkpoint preserved-artifact and downgrade-reason projections.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — requirement register, migration posture (RFC 2119 MUST on honest
  import and restore labels), portable-profile portability rules,
  and temporary / ephemeral profile exclusion rules.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — profile library, managed-sync lane, conflict journal, and
  restore-checkpoint architecture this packet's vocabulary composes
  against.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — migration-center, importer outcome, and restore-record field
  set; sync conflict review row shape; rollback-checkpoint scope /
  availability / cleanup posture.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — migration-center review rails, rollback / restore
  disclosure discipline, and the rule that a "restored" claim
  renders its `fidelity_label` and any preserved prior artifact
  inline rather than as a tooltip.
- `.t2/docs/Aureline_Milestones_Document.md`
  — migration, portability, temporary-profile, sync-conflict, and
  rollback-checkpoint claims kept as inspectable packets during the
  foundations phase rather than live product surfaces.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.migration_and_profile.seed
evidence_id: evidence.verification.migration_and_profile.packet
title: Migration import-diff, profile portability, temporary-profile, sync-conflict, and rollback-checkpoint verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
    - GOV-CORPUS-901
    - ARCH-PACK-901
  claim_row_refs:
    - packet_row:migration_and_profile.import_diff_row_contract
    - packet_row:migration_and_profile.lossy_vs_exact_migration_labels
    - packet_row:migration_and_profile.profile_portability_states
    - packet_row:migration_and_profile.temporary_profile_lifecycles
    - packet_row:migration_and_profile.sync_conflict_review_object
    - packet_row:migration_and_profile.rollback_checkpoint_expectations
    - packet_row:migration_and_profile.seed_corpus
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: migration_and_profile_seed@1
  trigger_revision: migration_and_profile_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen migration-center, importer-outcome,
    restore-provenance, portable-profile, and state-map vocabularies
    already landed in the repository. No migration executor,
    importer UI, profile library, managed-sync engine, conflict
    journal, or live restore center is wired to this packet yet.
    Claims are structural: every case in the manifest, every row in
    the portability / temporary-profile fixture set, and every
    rollback-checkpoint example reuses existing frozen tokens
    rather than minting new per-surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.verification.migration_and_profile.import_diff_manifest
    - evidence.verification.migration_and_profile.portability_and_temp_profile_cases
    - evidence.verification.migration_and_profile.rollback_checkpoint_examples
    - evidence.migration.migration_center_object_model
    - evidence.state.migration_and_restore_playbook
    - evidence.state.profile_and_state_map
    - evidence.workspace.entry_restore_object_model
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/migration/import_diff_manifest.yaml
    - fixtures/profiles/portability_and_temp_profile_cases/portable_profile_plain_cross_machine.json
    - fixtures/profiles/portability_and_temp_profile_cases/portable_profile_with_machine_addendum.json
    - fixtures/profiles/portability_and_temp_profile_cases/managed_sync_profile_roaming.json
    - fixtures/profiles/portability_and_temp_profile_cases/temporary_profile_ephemeral_session.json
    - fixtures/profiles/portability_and_temp_profile_cases/support_recovery_profile_snapshot.json
    - fixtures/state/migration_cases/profile_restore_exact.json
    - fixtures/state/migration_cases/profile_restore_compatible_schema_shift.json
    - fixtures/state/migration_cases/layout_restore_layout_only_missing_dependencies.json
    - fixtures/state/migration_cases/support_bundle_manual_review_workspace_conflict.json
  archetype_refs: []
  source_anchor_refs:
    - docs/migration/migration_center_object_model.md
    - docs/state/migration_and_restore_playbook.md
    - docs/state/profile_and_state_map.md
    - docs/workspace/entry_restore_object_model.md
    - schemas/migration/migration_session.schema.json
    - schemas/migration/importer_outcome.schema.json
    - schemas/profile/portable_profile.schema.json
    - schemas/state/restore_provenance.schema.json
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one reviewer-facing `import_diff_row_record` that names the case,
  migration session, outcome state, reason class, domain, source /
  target object refs, mapping basis, preserved-prior-artifact ref,
  and rollback-checkpoint ref every import / restore surface reuses;
- one closed mapping from the frozen `interop_result_state` set
  (`imported`, `mapped`, `skipped`, `manual_review`,
  `bridge_required`, `unsupported`) onto the frozen
  `fidelity_label` set (`exact`, `compatible`, `layout_only`,
  `manual_review`) so a row cannot claim "exact" while a translation
  ran and cannot claim "lossy" as a parallel label;
- one closed `profile_portability_state` vocabulary so portable,
  portable-with-machine-addendum, local-only, managed-sync,
  customer-managed-key sync, self-hosted sync, and support-recovery
  profiles stay separately addressable across a profile-library,
  export-manifest, restore-provenance, and migration surface;
- one closed `temporary_profile_lifecycle_state` vocabulary so
  ephemeral / temporary / short-lived profile sessions are not
  confused with a durable portable profile and MUST NOT promote
  themselves to durable truth without an explicit save step;
- one closed `sync_conflict_review_state` vocabulary plus a required
  `sync_conflict_review_row_record` shape so sync-conflict review
  rows name both sides (local vs remote authority), the last-common
  ancestor ref, the reviewer action required, and the conflict
  journal ref;
- one closed `rollback_checkpoint_outcome_class` vocabulary plus a
  required field set projected onto the existing
  `migration_restore_record` so rollback availability, scope,
  cleanup posture, partial-apply disclosure, and preserved-prior-
  artifact linkage stay inspectable after apply or export; and
- one seed corpus covering every scenario the spec names: clean
  import, partial import, skipped objects, alias / remap import,
  temporary / ephemeral profile session, sync conflict requiring
  review, downgrade migration, and rollback to prior checkpoint.

It does not claim a migration executor, an importer UI, a shipping
profile library, a managed-sync engine, a conflict-journal runtime,
or a live restore center is wired up. It claims only that the
packet, the import-diff manifest, the portability / temporary-
profile fixture set, and the rollback-checkpoint examples now exist
in one reviewable form and reuse the frozen migration and profile
vocabulary already landed elsewhere.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:migration_and_profile.import_diff_row_contract` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.migration_and_profile.import_diff_manifest` | Freezes one machine-readable `import_diff_row_record` shape every import-diff surface reuses. |
| `packet_row:migration_and_profile.lossy_vs_exact_migration_labels` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.migration_and_profile.import_diff_manifest`, `evidence.state.migration_and_restore_playbook` | Closed mapping from `interop_result_state` onto `fidelity_label`; forbids a parallel "approximate" or "best effort" label. |
| `packet_row:migration_and_profile.profile_portability_states` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.migration_and_profile.portability_and_temp_profile_cases`, `evidence.state.profile_and_state_map` | Closed `profile_portability_state` vocabulary with required state-map posture per row. |
| `packet_row:migration_and_profile.temporary_profile_lifecycles` | `GOV-TRUTH-901`, `GOV-DATA-002` | `seed_only` | `internal` | `evidence.verification.migration_and_profile.portability_and_temp_profile_cases` | Closed `temporary_profile_lifecycle_state` vocabulary plus non-persist rules for ephemeral sessions. |
| `packet_row:migration_and_profile.sync_conflict_review_object` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.migration_and_profile.import_diff_manifest` | `sync_conflict_review_row_record` shape names both sides, the last-common ancestor, and the required reviewer action. |
| `packet_row:migration_and_profile.rollback_checkpoint_expectations` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.migration_and_profile.rollback_checkpoint_examples` | Closed `rollback_checkpoint_outcome_class` vocabulary projected onto the frozen `migration_restore_record`. |
| `packet_row:migration_and_profile.seed_corpus` | `GOV-CORPUS-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.migration_and_profile.import_diff_manifest`, `evidence.verification.migration_and_profile.rollback_checkpoint_examples` | Stable case-id set covers every required scenario named in the spec. |

## What this seed freezes

- One `import_diff_row_record` shape every import-diff surface
  (migration-center diff rail, first-run import review, post-import
  follow-up, issue-template export, support-export row, release-
  evidence packet) reuses for projected migration language.
- One closed mapping from `interop_result_state` onto
  `fidelity_label` so the migration-center and restore-provenance
  surfaces agree on what counts as exact vs compatible vs layout-
  only vs manual-review, and on when `skipped`, `bridge_required`,
  and `unsupported` force a fidelity downgrade.
- One closed `profile_portability_state` vocabulary so later
  profile-library, managed-sync, export-manifest, and migration
  surfaces cannot silently promote a `local_only` row to `portable`
  or collapse `support_recovery_only` into a generic "profile".
- One closed `temporary_profile_lifecycle_state` vocabulary with
  explicit "do not persist" rules so an ephemeral session's settings
  changes do not silently land in a durable profile.
- One closed `sync_conflict_review_state` vocabulary and a required
  row shape so conflict review rows stay reviewable in the conflict
  journal, migration-center review, and support export.
- One closed `rollback_checkpoint_outcome_class` vocabulary plus a
  required field set projected onto the frozen
  `migration_restore_record` so rollback availability, scope,
  cleanup, and preserved-prior-artifact linkage remain honest.

## Import-diff row record

Every case in the machine-readable manifest resolves to one
`import_diff_row_record` with these required fields. The field set
projects the migration-center `importer_outcome_row_record`, the
restore-provenance `state_segments`, and the migration-center
`migration_session_record` fields already frozen; this packet does
not redefine them.

- `case_id`
- `migration_session_ref` — migration-center session id.
- `import_diff_row_id` — opaque, stable id, safe to log.
- `entry_action_ref` — optional `project_entry_action_record` ref
  when the row originated from an import entry surface.
- `migration_result_ref` — optional `migration_result_record` ref
  when the row is the entry-surface projection of a durable
  migration session.
- `domain` — one of the closed `migration_domain` tokens.
- `source_object_ref` — opaque source-side object id.
- `target_object_ref` — opaque target-side object id; null when
  `outcome_state` is `skipped` or `unsupported` without an aliased
  destination.
- `presentation_label` — short human label; never a raw URL or
  redaction-sensitive value.
- `outcome_state` — closed `interop_result_state` token.
- `reason_class` — closed `outcome_reason_class` token.
- `confidence_class` — closed confidence token.
- `mapping_basis` — closed mapping-basis token.
- `fidelity_label_projection` — closed `fidelity_label` token the
  row projects (see §Lossy-vs-exact migration labels).
- `state_plane_projection` — one of `portable_settings`,
  `local_context`, `workspace_shared_manifest`,
  `non_portable_live_authority`.
- `bridge_requirement_ref` — required when `outcome_state` is
  `bridge_required`; null otherwise.
- `alias_remap` — optional block carrying `source_alias`,
  `target_alias`, `collision_class`, and `user_ack_required` when
  the row is an alias / remap. Collisions are never hidden.
- `preserved_prior_artifact_ref` — required when the row touches
  `user_authored_durable_truth` and `outcome_state` in
  `{mapped, manual_review, bridge_required}`; null when no prior
  truth was mutated.
- `rollback_checkpoint_ref` — required when the row's session is in
  `applying`, `applied`, `partially_applied`, `restored`, or
  `failed`; null for preview-only rows.
- `docs_help_refs` — non-empty array.
- `export_inclusion_posture` — `metadata_safe_default`,
  `operator_only_restricted`, or `broadened_capture_opt_in`.
- `redaction_class` — ADR-0007 redaction class.
- `freshness_class` — ADR-0011 freshness class.
- `minted_at` — monotonic timestamp.

Rule: an import-diff row that cannot fill `outcome_state`,
`reason_class`, `fidelity_label_projection`, `state_plane_projection`,
`rollback_checkpoint_ref` (when required), or
`preserved_prior_artifact_ref` (when required) MUST deny render and
route to a `review_disclosure_incomplete` repair hook rather than
fall back to a generic "imported" chip.

Rule: the row's `outcome_state` and `fidelity_label_projection` MUST
agree with the closed mapping in §Lossy-vs-exact migration labels.
Silent disagreement is non-conforming.

Rule: a row whose `state_plane_projection` is
`non_portable_live_authority` MAY NOT claim `outcome_state =
imported`; live authority never restores as live authority and the
row MUST resolve to `skipped` (intentional), `manual_review`
(reconnect required), or `unsupported` (source never claimed
portability).

## Lossy-vs-exact migration labels

This section freezes the mapping between the migration-center
`interop_result_state` vocabulary and the state-migration /
restore-provenance `fidelity_label` vocabulary. A parallel "lossy",
"approximate", or "best effort" label is non-conforming.

### Closed mapping (frozen)

| `interop_result_state` | Required `fidelity_label_projection` set | Additional conditions |
|---|---|---|
| `imported` | `{exact, compatible, layout_only}` | `exact` only when no canonicalisation or translation ran; `compatible` when canonicalisation changed a byte but meaning stayed; `layout_only` only when the row is scoped to `local_context`. |
| `mapped` | `{compatible, manual_review}` | `compatible` when the mapping basis is `exact_identity`, `semantic_equivalent`, or `capability_based` and no validator returned `failed_*`; `manual_review` when any validator returned `failed_recoverable` or `failed_blocking` or the `mapping_basis` is `name_heuristic`. |
| `skipped` | `{exact, layout_only}` | `exact` when the source object was never requested (the row is informational); `layout_only` when local context retained the pane slot for the skipped object. Never `compatible` — a skipped row did not translate. |
| `manual_review` | `{manual_review}` | Closed: any row at `manual_review` forces `manual_review` on the fidelity projection. |
| `bridge_required` | `{compatible, manual_review}` | `compatible` only when the bridge is `already_present` or `recommended` and the target object renders natively through the bridge; `manual_review` when the bridge is `required_before_use`, `policy_blocked`, or `unavailable`. |
| `unsupported` | `{manual_review, layout_only}` | `manual_review` by default so the user can see the gap; `layout_only` only when the row's plane is `local_context` and the layout slot stayed visible as a placeholder. Never `exact` or `compatible`. |

### Derivation rules (frozen)

1. `fidelity_label_projection` is a producer claim; the packet
   reviewer MAY downgrade it but MAY NOT upgrade it. A surface that
   renders a higher `fidelity_label` than the row's
   `outcome_state` and mapping admit is non-conforming.
2. A row that claims `fidelity_label_projection = exact` while
   `reason_class` is `capability_mapping_available`,
   `semantic_equivalent_available`, `conflict_requires_review`, or
   `bridge_needed_for_parity` is non-conforming.
3. A row that claims `fidelity_label_projection = compatible` MUST
   cite one `equivalence_map_row_ref` on the packet and MUST cite a
   non-null `rollback_checkpoint_ref` on the session.
4. A row that claims `fidelity_label_projection = manual_review`
   MUST pair with at least one preserved prior artifact when the
   row touched `user_authored_durable_truth`; silent overwrite with
   no compare / export ref is non-conforming.
5. A row that claims `fidelity_label_projection = layout_only` MUST
   have `state_plane_projection = local_context`. Claiming
   `layout_only` on a portable-settings or workspace-shared row is
   non-conforming.
6. A session whose rows contain any `manual_review` row MUST set
   the session-level `migration_result_record.result_kind`
   (entry-surface projection) to `needs_review` or `partial` and
   MUST preserve the compare / export refs for every
   `manual_review` row.
7. A session whose rows contain any `bridge_required` or
   `unsupported` row MUST populate the packet's `export_refs` and
   `support_packet_refs` (per the importer-outcome schema) so those
   states do not disappear during support export.

## Profile portability states

Every portable-profile artifact, profile-library entry, and
migration-center source descriptor the packet reviews MUST resolve
to one `profile_portability_state` token below. The set is closed;
additions are additive-minor and require a decision row.

### `profile_portability_state` (frozen)

| Token | Meaning | Required state-map posture |
|---|---|---|
| `portable_plain` | Plain text `*.aureprofile.json`; diffable; no secrets, trust approvals, live authority, or machine-unique anchors. Default for user export. | Body lists only `user_authored_durable_truth` rows whose state-map `portability` is `portable`; secret / live / admin rows listed under `excluded_state_classes` with typed `exclusion_reason_id`. |
| `portable_encrypted` | Same body, encrypted at rest under a user- or enterprise-owned key. | Same contents as `portable_plain`; carries an encryption-key ref; never ships the key. |
| `portable_with_machine_addendum` | Portable body plus a companion machine-local addendum for machine-bound halves (local toolchain paths, machine-specific hints). | Portable body per `portable_plain`; addendum is `local_only` and lists every machine-bound row with a typed `exclusion_reason_id`. |
| `managed_sync_opt_in` | Managed-sync lane carries the body. | Body MAY NOT include any state class whose state-map `sync_posture` is `never_synced`. |
| `customer_managed_key_sync` | Managed sync under a customer-managed encryption key. | Same as `managed_sync_opt_in` plus customer-managed-key ref. |
| `self_hosted_sync` | Self-hosted sync service carries the body. | Same as `managed_sync_opt_in` plus self-hosted endpoint ref. |
| `support_recovery_only` | Not a portable profile; a support / recovery manifest that MAY include redacted excerpts of `user_owned_recovery_state` under opt-in. | MUST NOT include raw auth secrets, long-lived credentials, trust approvals, or admin policy bundles. `redact_to_class_label` is the minimum floor for `ai_memory_metadata`, `deferred_intent_outbox`, and `logs_and_traces`. |
| `local_only_unpublishable` | Profile-library entry that is intentionally local-only; never crosses a machine boundary even under managed sync. | All rows carry state-map `portability = local_only`; any promotion attempt is denied with a typed `portability_denied_local_only` reason. |

Rules (frozen):

1. A migration session whose source descriptor is
   `portable_profile_bundle` MUST name the source artifact's
   `profile_portability_state` on the session record or the entry
   action. Silent `portable_plain` assumption is non-conforming.
2. A profile-library entry that changes its `profile_portability_state`
   (for example, `portable_plain` → `managed_sync_opt_in`) is a
   durable state change and MUST emit an audit event plus a restore
   checkpoint.
3. A surface that renders a profile as "portable" without resolving
   the row against a closed token is non-conforming; the profile-
   library and migration-center MUST project one closed token on
   every entry row.
4. `support_recovery_only` rows render the typed "support recovery"
   chip on the primary surface and MAY NOT imply the profile is
   import-ready as a durable profile; importing from a
   `support_recovery_only` row routes through the migration-center
   review, not a one-click restore.

## Temporary profile lifecycles

A temporary / ephemeral profile session is not a durable profile.
Its settings, layout changes, and AI preset selections MUST NOT
promote into the user's durable profile without an explicit save
step that the user sees.

### `temporary_profile_lifecycle_state` (frozen)

| Token | Meaning | Required non-persist rules |
|---|---|---|
| `ephemeral_in_memory_only` | Session lives in memory; no on-disk profile body; survives only the current process. | MUST NOT write any row to `AURELINE_CONFIG`; MUST clear on process exit; exit code MUST emit an audit event naming the token. |
| `temporary_with_scratch_dir` | Session writes to a named scratch directory carrying a typed lifetime (`until_session_end`, `until_boundary_event`, `until_explicit_cleanup`). | Scratch dir is redacted on every support export; rows are excluded from portable-profile export unless the user invokes `promote_to_durable_profile`. |
| `short_lived_review_session` | Session opened to review or compare a profile artifact; read-only by default. | MUST NOT write any mutation; an attempted mutation forces elevation to `temporary_with_scratch_dir` with an explicit user prompt. |
| `remote_ephemeral_session` | Session bound to a remote runtime (dev-container, managed workspace); the remote owns disposal. | Local machine MUST NOT retain a copy past the remote disposal event; `non_portable_live_authority` handles never cross the boundary. |
| `promotion_required_to_persist` | Session has opted in to durable promotion but the promotion step has not run; current state still behaves as `temporary_with_scratch_dir`. | Promotion runs the durable-profile Save flow with a typed preview and a restore checkpoint before write. |
| `expired_cleanup_pending` | Session's declared lifetime elapsed; cleanup job has not yet run. | Surface MUST deny new writes; cleanup emits an audit event and removes the scratch dir. |
| `cleanup_blocked_manual_review` | Cleanup attempted but blocked (open handle, pinned by policy pack, retained for support). | Surface renders a typed cleanup-blocked chip and routes to a manual-repair hook; MUST NOT silently auto-retry past the declared window. |

Rules (frozen):

1. A temporary-profile session that writes a portable-profile-shape
   body MUST set `profile_portability_state = local_only_unpublishable`
   on the artifact; promoting it to `portable_plain` requires the
   explicit Save flow named in `promotion_required_to_persist`.
2. A temporary-profile session MAY NOT participate in managed sync
   under any token except `promotion_required_to_persist` after a
   completed promotion step; the sync-posture resolver denies the
   write.
3. A support export of a temporary-profile session MUST carry the
   `temporary_profile_lifecycle_state` token and MUST declare the
   declared-lifetime class. Omitting the token is non-conforming.
4. A surface that claims "profile saved" without routing through
   `promotion_required_to_persist` → durable profile Save is
   non-conforming; the token is the only honest way to promote.

## Sync-conflict review object

A sync conflict is reviewable only when the conflict record names
both sides (local vs remote), the last-common ancestor, and the
reviewer action required. A single "conflict" chip without those
fields is non-conforming.

### `sync_conflict_review_state` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `no_conflict_detected` | Sync apply proceeded without divergence. | No review chip. |
| `review_required_user_resolution` | Divergence detected; user must pick a side or merge. | Review chip with "pick local", "pick remote", or "merge" action hooks; the conflict journal ref MUST ride. |
| `review_required_admin_resolution` | Divergence crosses a policy-narrowing or managed-only boundary; admin resolution required. | Review chip with admin-resolution prompt; `export_inclusion_posture = operator_only_restricted`. |
| `auto_resolved_local_wins_policy` | Policy pack resolved to local automatically; no divergence survives. | Resolution chip naming the policy-pack ref; a preserved prior artifact of the remote side MUST ride. |
| `auto_resolved_remote_wins_policy` | Policy pack resolved to remote automatically; no divergence survives. | Resolution chip naming the policy-pack ref; a preserved prior artifact of the local side MUST ride. |
| `deferred_pending_reauth` | Divergence cannot be reviewed until reauthentication to the sync service completes. | Reauth prompt chip; sync row denies write until reauth succeeds. |
| `conflict_journal_corrupt_manual_repair` | Conflict journal itself is corrupt; manual repair required before further sync. | Repair chip with typed repair hook; all sync writes denied until repair completes. |

### `sync_conflict_review_row_record` (required fields)

- `case_id`
- `sync_conflict_review_state` — closed token above.
- `state_class_ref` — state-map row id this conflict is over
  (for example, `user_global_settings`, `keybindings`, `snippets`).
- `local_side` — opaque ref to the local authority snapshot, plus
  `producer_build`, `redaction_class`, and `emitted_at`.
- `remote_side` — opaque ref to the remote authority snapshot, plus
  `producer_build`, `redaction_class`, and `emitted_at`.
- `last_common_ancestor_ref` — opaque ref to the shared ancestor;
  required for `review_required_*` and `auto_resolved_*` tokens;
  null only for `no_conflict_detected` and
  `conflict_journal_corrupt_manual_repair`.
- `reviewer_action_required` — one of `pick_local`, `pick_remote`,
  `merge`, `escalate_to_admin`, `reauthenticate`,
  `repair_conflict_journal`, `none`.
- `policy_pack_ref` — required when the token is
  `auto_resolved_*`; null otherwise.
- `conflict_journal_ref` — opaque ref to the conflict journal
  entry; required on every token except `no_conflict_detected`.
- `preserved_prior_artifact_refs` — required when the token is
  `auto_resolved_*` (the side that did not win); required for
  `review_required_*` when a reviewer picks a side.
- `export_inclusion_posture` — `metadata_safe_default`,
  `operator_only_restricted`, or `broadened_capture_opt_in`.

Rules (frozen):

1. A sync-conflict row that omits `local_side`, `remote_side`, or
   (when required) `last_common_ancestor_ref` is non-conforming.
   Collapsing both sides into a single "value" blob is forbidden.
2. `auto_resolved_*` rows MUST preserve the losing side's artifact
   under `preserved_prior_artifact_refs`; silent drop of the
   losing side is non-conforming.
3. A `review_required_*` row MUST render the typed reviewer action
   inline on the primary surface; tooltip-only disclosure is
   non-conforming.
4. `deferred_pending_reauth` MUST NOT silently re-attempt the sync
   apply; reauthentication is a user-driven flow, not an automatic
   retry.
5. `conflict_journal_corrupt_manual_repair` MUST deny every sync
   write for the affected state class until repair completes;
   silent fallback to last-known-good is non-conforming.

## Rollback-checkpoint expectations

Every migration apply path that mutates durable truth creates one
`migration_restore_record` whose rollback-checkpoint outcome class
is one of the frozen tokens below. The record survives apply,
partial apply, restore, support export, and downgrade; it is not a
success-only receipt.

### `rollback_checkpoint_outcome_class` (frozen)

| Token | Meaning | Required projection onto `migration_restore_record` |
|---|---|---|
| `checkpoint_created_pre_apply` | Checkpoint created before apply; apply has not yet started. | `availability_state = available`; `cleanup_state = retained`; `restore_action_hints` includes `compare_before_restore`. |
| `checkpoint_available_post_apply_clean` | Apply completed cleanly; checkpoint retained for the declared retention window. | `availability_state = available`; `cleanup_state = retained`; `partial_apply_note = null`. |
| `checkpoint_available_post_apply_partial` | Apply partially landed; checkpoint retained and reviewer MUST see the partial-apply disclosure. | `availability_state = available`; `cleanup_state = retained`; `partial_apply_note` required and non-empty; preserved prior artifact refs required. |
| `checkpoint_restored_to_prior_state` | User / support rolled back; prior state restored. | `availability_state = restored`; `cleanup_state = retained` (compare window) or `cleanup_pending`; preserved prior artifact refs MUST remain reachable. |
| `checkpoint_expired_cleanup_complete` | Retention window elapsed; cleanup ran; checkpoint no longer restorable. | `availability_state = expired`; `cleanup_state = cleaned_up`; restore hooks denied. |
| `checkpoint_cleanup_pending_manual` | Retention window elapsed but cleanup blocked (open handle, pinned by policy pack, retained for support). | `availability_state = cleanup_pending` or `policy_hidden`; `cleanup_state = cleanup_blocked`; repair hook required. |
| `checkpoint_policy_hidden_support_only` | Policy pack restricted visibility to support; the checkpoint exists but user-facing affordance is withheld. | `availability_state = policy_hidden`; `cleanup_state = retained`; `export_inclusion_posture = operator_only_restricted`. |

Rules (frozen):

1. A migration session in `applying`, `applied`, `partially_applied`,
   `restored`, or `failed` state MUST carry a non-null
   `restore_record_ref` on the session; this packet's outcome class
   is the projection onto that record.
2. A session that lands any `manual_review` or `bridge_required`
   row MUST preserve the checkpoint past the apply event (no
   immediate cleanup); a surface that silently cleans up the
   checkpoint is non-conforming.
3. A `checkpoint_available_post_apply_partial` record MUST pair
   with a non-empty `preserved_prior_artifacts` block on the shared
   restore-provenance record when the partial apply touched
   `user_authored_durable_truth` or `workspace_shared_manifest`.
4. A `checkpoint_restored_to_prior_state` record MUST carry the
   post-restore validator outcomes (at least
   `settings_schema_migration` or `layout_restore_sanity`) so the
   restore claim is inspectable rather than just a success badge.
5. A `checkpoint_policy_hidden_support_only` record MUST render the
   typed "support-only" disclosure on any support-export surface
   that quotes it; hiding the restriction during export is
   non-conforming.
6. Every rollback-checkpoint record MUST preserve the migration
   session's `compatibility_row_refs` and
   `compatibility_report_ref`; dropping compatibility linkage at
   restore time is non-conforming.

## Seed corpus

The machine-readable manifest seeds the following case ids. Every
case carries one `import_diff_row_record` (or a matching
sync-conflict / rollback-checkpoint projection) plus at least one
conformance-test ref.

### Import-diff cases (see `import_diff_manifest.yaml`)

| Case id | Outcome state | Fidelity label projection | State plane | Notes |
|---|---|---|---|---|
| `migration_and_profile.clean_import.settings_round_trip` | `imported` | `exact` | `portable_settings` | Clean import of a `user_global_settings` row with no canonicalisation beyond documented whitespace normalisation. |
| `migration_and_profile.partial_import.extension_missing` | `manual_review` | `manual_review` | `local_context` | Partial import: a pane slot is retained as a placeholder because its extension dependency is missing. |
| `migration_and_profile.skipped_object.user_declined_setting` | `skipped` | `exact` | `portable_settings` | User declined to import a telemetry setting; the source object is informational. |
| `migration_and_profile.alias_remap.keybinding_reserved_chord` | `mapped` | `compatible` | `portable_settings` | Alias / remap: a source chord collides with a reserved target chord; user ack recorded; equivalence-map row ridden. |
| `migration_and_profile.temporary_profile_session.ephemeral_scratch` | `skipped` | `layout_only` | `local_context` | Temporary / ephemeral profile session; layout pane slots survive but no portable-settings row persists. |
| `migration_and_profile.sync_conflict_review.keybindings_diverged` | `manual_review` | `manual_review` | `portable_settings` | Sync conflict review: local and remote keybindings diverged; `review_required_user_resolution`; conflict journal ref ridden. |
| `migration_and_profile.downgrade_migration.schema_meaning_changed` | `mapped` | `compatible` | `portable_settings` | Downgrade migration: source profile emitted by a newer build; theme-density token meaning changed; prior artifact preserved. |
| `migration_and_profile.rollback_to_prior_checkpoint.partial_apply_reverted` | `manual_review` | `manual_review` | `workspace_shared_manifest` | Rollback: user reverts a partial apply; checkpoint restored; preserved prior artifacts remain reachable. |

### Portability / temporary-profile cases (see `fixtures/profiles/portability_and_temp_profile_cases/`)

| Fixture | Portability state | Lifecycle state | Notes |
|---|---|---|---|
| `portable_profile_plain_cross_machine.json` | `portable_plain` | n/a | Baseline portable profile export; no secrets, no trust approvals, no admin bundle; imports across machines. |
| `portable_profile_with_machine_addendum.json` | `portable_with_machine_addendum` | n/a | Portable body plus machine-local addendum for `machine_specific_settings`; addendum excluded from any further portable copy. |
| `managed_sync_profile_roaming.json` | `managed_sync_opt_in` | n/a | Managed-sync lane carries the body; rows whose state-map `sync_posture` is `never_synced` stay excluded. |
| `temporary_profile_ephemeral_session.json` | `local_only_unpublishable` | `ephemeral_in_memory_only` | Temporary / ephemeral session; writes scratch-only; no durable promotion without explicit save. |
| `support_recovery_profile_snapshot.json` | `support_recovery_only` | n/a | Support-recovery manifest with redacted excerpts and typed exclusion reasons. |

### Rollback-checkpoint cases (see `artifacts/migration/rollback_checkpoint_examples/`)

| Example | Outcome class | Notes |
|---|---|---|
| `checkpoint_created_pre_apply.yaml` | `checkpoint_created_pre_apply` | Session is at `diff_ready`; checkpoint retained; restore hints include `compare_before_restore`. |
| `checkpoint_available_post_apply_clean.yaml` | `checkpoint_available_post_apply_clean` | Clean apply; checkpoint retained for the declared retention window. |
| `checkpoint_available_post_apply_partial.yaml` | `checkpoint_available_post_apply_partial` | Partial apply; preserved prior artifacts ridden; partial-apply note required. |
| `checkpoint_restored_to_prior_state.yaml` | `checkpoint_restored_to_prior_state` | User rolled back after apply; post-restore validators ridden. |
| `checkpoint_expired_cleanup_complete.yaml` | `checkpoint_expired_cleanup_complete` | Retention window elapsed; cleanup ran; restore hooks denied. |
| `checkpoint_cleanup_pending_manual.yaml` | `checkpoint_cleanup_pending_manual` | Cleanup blocked by an open handle; repair hook rides. |
| `checkpoint_policy_hidden_support_only.yaml` | `checkpoint_policy_hidden_support_only` | Policy pack restricted visibility to support; export posture `operator_only_restricted`. |

## Surface admissibility

| Surface | May mint `import_diff_row_record` | May claim rollback-checkpoint outcome | May claim profile-portability state | Projection rule |
|---|---|---|---|---|
| `migration_center_diff_rail` | yes | yes | yes | MUST emit one record per diff row; MUST project `outcome_state`, `fidelity_label_projection`, `state_plane_projection`, and any rollback / sync-conflict posture inline. |
| `first_run_import_review` | yes (entry-surface projection only) | yes (quoted) | yes (source-descriptor projection) | MUST quote the migration-center record ids once the durable session exists; MUST NOT re-mint diff rows. |
| `post_import_follow_up` | no | yes (quoted) | yes (quoted) | Quotes the session's packet; MUST render preserved prior artifacts and manual-review rows inline. |
| `profile_library_entry_row` | no | no | yes | Projects one `profile_portability_state` token per entry; MAY NOT silently promote or demote a row. |
| `remembered_state_inspector` | no | yes (quoted) | yes (quoted) | Reads state-map rows and restore-provenance refs; MUST NOT mint a parallel portability or rollback vocabulary. |
| `sync_conflict_review_row` | yes (sync-conflict projection) | no | yes (the state class being synced) | MUST carry `local_side`, `remote_side`, `last_common_ancestor_ref` (when required), and `reviewer_action_required`. |
| `support_export_row` | no | yes (quoted) | yes (quoted) | Preserves the record under the support-export redaction envelope; MUST preserve compare / export refs. |
| `release_evidence_packet` | no | yes (quoted) | yes (quoted) | MUST quote freshness class; a stale record MAY NOT render as `authoritative_live`. |

Rule: any surface not named here MAY NOT mint an import-diff row, a
rollback-checkpoint record, or a portability-state claim; it quotes
one minted by the authoritative surface above.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.verification.migration_and_profile.import_diff_manifest` | `verification_corpus` | Defines the case roster every import-diff row cites. | current | `fixtures/migration/import_diff_manifest.yaml` |
| `evidence.verification.migration_and_profile.portability_and_temp_profile_cases` | `verification_corpus` | Defines the portability / temporary-profile fixtures every profile-library and migration surface cites. | current | `fixtures/profiles/portability_and_temp_profile_cases/` |
| `evidence.verification.migration_and_profile.rollback_checkpoint_examples` | `verification_corpus` | Defines the rollback-checkpoint examples every migration-restore-record projection cites. | current | `artifacts/migration/rollback_checkpoint_examples/` |
| `evidence.migration.migration_center_object_model` | `source_anchor` | Canonical migration-center / importer-outcome / restore-record vocabulary this packet projects. | current | `docs/migration/migration_center_object_model.md` |
| `evidence.state.migration_and_restore_playbook` | `source_anchor` | Canonical state-plane / fidelity-label / downgrade-reason vocabulary this packet projects. | current | `docs/state/migration_and_restore_playbook.md` |
| `evidence.state.profile_and_state_map` | `source_anchor` | Canonical portable-profile, state-map row, export-manifest, and restore-provenance vocabulary this packet reuses. | current | `docs/state/profile_and_state_map.md` |
| `evidence.workspace.entry_restore_object_model` | `source_anchor` | Canonical entry-surface `project_entry_action_record` and `migration_result_record` vocabulary this packet cites. | current | `docs/workspace/entry_restore_object_model.md` |

## Verification method

- **Verification classes used:** design review, vocabulary-reuse
  review, fixture review, schema-alignment review.
- **Procedure summary:** verified that the packet and its companion
  manifest, portability fixtures, and rollback-checkpoint examples
  reuse the migration-center `migration_session_record` and
  `importer_outcome_row_record` shapes, the state-migration
  playbook's `fidelity_label` and `state_plane` vocabularies, the
  portable-profile and state-map `portability`, `sync_posture`,
  `clear_posture`, and `redaction` vocabularies, and the restore-
  provenance `preserved_prior_artifacts` and `downgrade_reasons`
  shapes without minting parallel tokens. Verified that the
  lossy-vs-exact mapping is closed, that portability and temporary-
  profile tokens are closed vocabularies, that sync-conflict review
  records name both sides plus the last-common ancestor, and that
  every seed case exercises one required scenario named in the
  spec (clean import, partial import, skipped objects, alias /
  remap, temporary / ephemeral session, sync conflict, downgrade
  migration, rollback to prior checkpoint).
- **Automation refs:** `not_yet_seeded` for a dedicated migration-
  corpus validator; structural parsing is currently the available
  automation. The `state/migration_cases/` fixtures are separately
  validated against `schemas/state/restore_provenance.schema.json`.

## Known gaps and waivers

- **Waiver refs:** `none`.
- **Known-limit refs:** `none`.
- **Migration-packet refs:** `none`.
- **Explicit gaps:** no migration executor, importer UI, profile
  library, managed-sync engine, conflict-journal runtime, or live
  restore center is wired to this packet yet.
- **Explicit gaps:** no dedicated JSON Schema exists yet for the
  `import_diff_row_record`, the `sync_conflict_review_row_record`,
  the `profile_portability_state` enum, the
  `temporary_profile_lifecycle_state` enum, or the
  `rollback_checkpoint_outcome_class` enum. Reserved shapes are
  documented here for later schema landing.
- **Explicit gaps:** the conflict-journal row body shape, the
  managed-sync apply pipeline, and the customer-managed-key broker
  interactions are out of scope here; they will land with later
  packets that quote this seed.

## Reviewer signoff

- **Reviewer / forum:** `@ahmeddyounis`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:**
  `packet_row:migration_and_profile.import_diff_row_contract`,
  `packet_row:migration_and_profile.lossy_vs_exact_migration_labels`,
  `packet_row:migration_and_profile.profile_portability_states`,
  `packet_row:migration_and_profile.temporary_profile_lifecycles`,
  `packet_row:migration_and_profile.sync_conflict_review_object`,
  `packet_row:migration_and_profile.rollback_checkpoint_expectations`,
  `packet_row:migration_and_profile.seed_corpus`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `corpus_or_portability_vocabulary_revision_changed`.
- **Expected freshness window:** `P30D`.
- **Next packet family to update with the same evidence ids:**
  support-export packet, managed-sync apply packet, conflict-
  journal packet, or profile-library surface packet that starts
  quoting import-diff rows, portability states, temporary-profile
  lifecycle tokens, or rollback-checkpoint outcome classes.
