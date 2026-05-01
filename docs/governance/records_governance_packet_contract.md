# Records-governance packet, retention-hold delta summary, and offboarding-caveat contract

This contract freezes one shared vocabulary for the records-governance
packet. It exists so milestone and release readiness can state, in one
inspectable record, exactly what changed in governed record classes,
what changed in retention and legal-hold posture, what export and
delete contracts are status-tracked, what offboarding evidence is
linked, and what held-data caveats remain open — without scattering
the same information across record-class registry notes, support
handoff appendices, AI-evidence sidecars, claim manifest narrowings,
and release-notes paragraphs.

The records-governance packet is a single object family. Every packet
projects one window (milestone close, release train, weekly governance
review, or ad-hoc review) and renders five typed sections:

1. The **record-class registry diff** — additions, axis-by-axis
   changes, and retirements against
   [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
   since the baseline `evaluated_at`.
2. The **retention and legal-hold policy diff** — typed changes to
   retention/deletion matrix rows and legal-hold posture against
   [`./retention_deletion_matrix_contract.md`](./retention_deletion_matrix_contract.md).
3. The **export and delete contract status** — typed status of every
   `export_path_class` and `delete_path_class` cited in the matrix,
   plus the live status of the shared delete-request-state schema.
4. The **offboarding evidence links** — opaque refs into exit packets,
   destruction receipts, portability matrix rows, support offboarding
   drills, collaboration offboarding evidence, and AI retained evidence
   offboarding packets.
5. The **open held-data caveats** — typed caveats that block clean
   deletion right now (legal hold, policy retention floor, sync /
   provider backlog, support investigation, export-pending hold,
   import-source unreachable, manual local capture required, redaction
   policy, outside platform scope).

A `records_governance_packet_record` also names the typed audience and
redaction profile so the same packet body can serve engineering,
support, enterprise audit, release readiness, and public-proof review
without overexposing raw record contents. The packet carries opaque
refs and bounded reviewable sentences only; raw record payloads, raw
hold justifications, raw policy bundle bodies, raw export bytes, raw
prompts, raw user identifiers, and raw destruction receipt bytes
never appear.

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabularies, the projection rules, the export-
parity floor, the change-significance rules, and the fixture corpus.
It does not implement retention enforcement, deletion backends, legal-
hold tooling, or compliance automation.

## Companion artifacts

- [`/schemas/governance/records_governance_packet.schema.json`](../../schemas/governance/records_governance_packet.schema.json)
  — boundary schema for one `records_governance_packet_record`.
- [`/fixtures/governance/records_governance_cases/`](../../fixtures/governance/records_governance_cases/)
  — worked records covering an informational milestone packet, a
  release-bearing packet that adds a managed copy and tightens
  retention, a claim-narrowing packet that pulls a marketed claim back
  to internal-only because a managed copy moved under a retention
  floor, and a packet with multiple open held-data caveats spanning
  legal hold, sync backlog, and import-source unreachable.
- [`./record_class_governance.md`](./record_class_governance.md) and
  [`/schemas/governance/record_class.schema.json`](../../schemas/governance/record_class.schema.json)
  — class-level record-class registry and row schema. The packet's
  `record_class_registry_diff` cites `record_class_id` verbatim and
  inherits the registry's axis-separation discipline (scope, retention,
  hold, delete, export, offboarding, partial-result cause set stay
  distinct).
- [`./retention_deletion_matrix_contract.md`](./retention_deletion_matrix_contract.md),
  [`/schemas/governance/retention_matrix_row.schema.json`](../../schemas/governance/retention_matrix_row.schema.json),
  and
  [`/schemas/governance/delete_request_state.schema.json`](../../schemas/governance/delete_request_state.schema.json)
  — retention/deletion matrix and delete-request state. The packet's
  `retention_and_legal_hold_policy_diff` cites matrix `row_id`s
  verbatim and reuses the partial-blocker, remaining-location, and
  honesty-invariant vocabulary.
- [`./telemetry_and_support_schema_registry.md`](./telemetry_and_support_schema_registry.md)
  and
  [`/schemas/governance/schema_registry_entry.schema.json`](../../schemas/governance/schema_registry_entry.schema.json)
  — telemetry / diagnostics / support-export schema registry. The
  packet's `linked_artifact_families.telemetry_schema_registry_refs[]`
  cites schema-registry entry ids verbatim.
- [`./privacy_history_and_lifecycle_contract.md`](./privacy_history_and_lifecycle_contract.md)
  and
  [`/schemas/governance/privacy_history_event.schema.json`](../../schemas/governance/privacy_history_event.schema.json)
  — privacy-history event family. Held-data caveats cite the
  privacy-history event ids that recorded the hold or retention floor.
- [`./claim_manifest_contract.md`](./claim_manifest_contract.md) and
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  — claim manifest. A packet whose
  `change_significance_summary.overall_significance_class =
  claim_narrowing` cites the affected claim_row ids and one or more
  public-proof rows.
- [`./waiver_register_contract.md`](./waiver_register_contract.md) and
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)
  — waiver register. A retention or hold change held by an active
  waiver cites the waiver register entry id.
- [`./data_portability_and_exit_matrix.md`](./data_portability_and_exit_matrix.md)
  and
  [`/schemas/governance/portability_row.schema.json`](../../schemas/governance/portability_row.schema.json)
  — per-domain export, deletion, and exit contract. The packet's
  `offboarding_evidence_links.portability_matrix_refs[]` cites
  portability row ids verbatim.

If this contract disagrees with those companion sources, the schemas
win and this contract, the schema, and the fixtures update in the same
change.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` — record-class, retention, deletion,
  legal-hold, export, offboarding, support-handoff, and claim-
  narrowing requirements (RFC 2119 MUST / SHOULD language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — managed-
  copy, support-export, AI-retained-evidence, collaboration-evidence,
  and offboarding-packet record shapes.
- `.t2/docs/Aureline_Technical_Design_Document.md` — release-evidence
  and supportability-evidence record shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — privacy, retention,
  deletion, legal-hold, and offboarding disclosure rules.

## 1. Why this contract exists

1. **Scattered retention notes fail review.** Without one shared
   packet, "what changed in record governance this milestone" lives as
   a paragraph in a record-class registry README, a comment on a
   retention matrix row, an appendix to a support handoff, an
   afterthought on the AI-evidence sidecar, and a sentence in the
   release-notes appendix. The reviewer cannot assess whether a record
   class moved from local-authoritative to managed-authoritative,
   whether a retention floor was added, whether a legal hold was
   extended, or whether the marketed claim manifest narrowed — without
   hunting across documents and chat history. The packet exists so
   every consuming surface renders the **same** typed sections with
   the **same** stable IDs.
2. **Legal hold, retention, delete/export, and offboarding posture
   must be visible together.** A packet that names retention without
   legal hold, or hold without offboarding, or offboarding without
   delete, hides operational risk. The five typed sections sit on the
   same record so a reviewer reads class-level posture, matrix row
   posture, contract status, offboarding evidence, and open caveats
   side by side.
3. **Audience and redaction are explicit.** Engineering, support,
   enterprise audit, release readiness, and public-proof reviewers all
   consume the same packet body. The packet pins the typed audience
   and redaction profile so a public-proof export does not include
   support-only payload refs, and a support handoff does not include
   enterprise-only contract status. The schema rejects packets whose
   redaction profile would widen scope past the named audience.
4. **Change significance must be typed.** A records-governance delta
   is one of: informational (no marketed claim or release floor
   moves), release-bearing (a release train requires the change to
   land or be waived), or claim-narrowing (a marketed public claim is
   narrowed because a record-class or retention axis shifted). Surfaces
   that render every delta as "informational" or every delta as
   "release-bearing" hide the difference.
5. **Held-data caveats cannot hide.** Open legal holds, active policy
   retention floors, sync / provider backlogs that prevent clean
   delete, support investigations, import-source-unreachable cases,
   and outside-platform-scope copies stay visible on the packet until
   they clear or a waiver register entry is recorded. A packet that
   filters out a caveat under a "needs review" affordance is non-
   conforming.
6. **One packet drives every consuming surface.** The ship-room
   dashboard, the milestone scorecard, the release-evidence packet,
   the support handoff bundle, the AI evidence packet, the claim
   manifest, the public-proof index, and the weekly governance review
   read the same `records_governance_packet_record`. A surface that
   reformats the packet, takes a screenshot of the dashboard, or
   reconstructs a parallel narrative status note is non-conforming.

## 2. Packet shape

A `records_governance_packet_record` carries:

- `records_governance_packet_schema_version` — integer. Current `1`.
- `record_kind` — constant `records_governance_packet_record`.
- `packet_id` — stable, machine-readable id quoted by every consuming
  surface.
- `evaluated_at` — RFC 3339 UTC timestamp at which the packet was
  projected. Distinct from underlying registry, matrix-row, and
  evidence chronology; the packet is reprojected when registry rows
  change, matrix rows change, holds advance, evidence freshness flips,
  or a waiver register entry transitions.
- `packet_window` — typed `window_kind_class`, `window_ref`,
  `baseline_evaluated_at`, and a bounded reviewable
  `window_summary` (§3).
- `audience_class` — typed audience (§4).
- `redaction_profile` — typed `redaction_profile_class`,
  `redaction_profile_ref`, the `raw_record_payloads_excluded` honesty
  invariant (constant `true`), and a bounded reviewable
  `redaction_profile_summary` (§5).
- `record_class_registry_diff` — typed additions[], changes[],
  retirements[], plus the `registry_schema_version_advanced_class`
  and a bounded reviewable `registry_diff_summary` (§6).
- `retention_and_legal_hold_policy_diff` — typed
  `retention_matrix_changes[]`, `legal_hold_posture_changes[]`, and a
  bounded reviewable `policy_diff_summary` (§7).
- `export_delete_contract_status` — typed `export_path_rows[]`,
  `delete_path_rows[]`, the `delete_request_state_status_class`, and
  a bounded reviewable `contract_status_summary` (§8).
- `offboarding_evidence_links` — typed
  `exit_packet_refs[]`, `destruction_receipt_refs[]`,
  `portability_matrix_refs[]`, `support_offboarding_drill_refs[]`,
  `collaboration_offboarding_evidence_refs[]`,
  `ai_offboarding_evidence_refs[]`, and a bounded reviewable
  `offboarding_links_summary` (§9).
- `open_held_data_caveats[]` — typed caveat entries (§10).
- `linked_artifact_families` — typed refs into the artifact families
  the packet composes over (§11).
- `change_significance_summary` — typed
  `overall_significance_class`, optional `release_bearing_rationale`,
  optional `claim_narrowing_links[]`, and a bounded reviewable
  `significance_summary` (§12).
- `consuming_surface_parity` — typed booleans for the dashboard,
  milestone scorecard, release packet, support export, governance
  packet, claim manifest, public-proof index, and weekly governance
  review (§13).
- `linked_waiver_register_refs[]` — refs into waiver register entry
  ids that hold any change in the packet.
- `linked_release_truth_summary_refs[]` — refs into release-truth
  summaries that quote this packet.
- `linked_requirement_status_row_refs[]` — refs into requirement-status
  snapshot rows whose state is gated by changes in this packet.
- `headline_label` and `packet_summary` — bounded reviewable label and
  one-sentence summary.
- `contract_doc_ref` — constant
  `docs/governance/records_governance_packet_contract.md`.
- `record_class_registry_ref` — constant
  `artifacts/governance/record_class_registry.yaml`.
- `retention_matrix_contract_doc_ref` — constant
  `docs/governance/retention_deletion_matrix_contract.md`.
- `notes` — optional bounded reviewable sentence.

## 3. Packet-window vocabulary

Closed four-class `window_kind_class`:

| Class | Meaning |
| --- | --- |
| `milestone_close_window` | Packet projects a milestone-close window; `window_ref` cites the milestone slug. The packet pairs with the milestone scorecard and the requirement-status snapshot. |
| `release_train_window` | Packet projects a release train window (stable, LTS, preview); `window_ref` cites the release train id. The packet pairs with the release-truth summary. |
| `weekly_governance_review_window` | Packet projects the weekly governance review cadence; `window_ref` cites the review row. |
| `ad_hoc_review_window` | Packet projects a one-off review (incident close, audit deep-dive, claim narrowing); `window_ref` cites the ad-hoc review id. |

`baseline_evaluated_at` is the prior packet's `evaluated_at` (or the
registry / matrix's seed timestamp on the first projection). Every
diff section is computed against that baseline.

## 4. Audience vocabulary

Closed five-class `audience_class`:

| Class | Required scope |
| --- | --- |
| `engineering_internal` | Internal engineering review. Carries the full diff sections and contract status. Raw payloads still excluded. |
| `support_handoff` | Support operator review. Carries open caveats, retention/hold changes, and offboarding refs; suppresses release-train rationale and unrelated public-proof narrowings. |
| `enterprise_audit` | Enterprise / tenant compliance review. Carries the full retention/hold diff, the export/delete contract status, and the offboarding refs; suppresses internal-only contract-status churn. |
| `release_readiness` | Ship-room and release-evidence review. Carries the full registry diff, the retention/hold diff, the contract status, and the change-significance summary; suppresses unresolved support-only payload refs. |
| `public_proof_safe` | Public-proof / claim manifest review. Carries only marketed-claim narrowings, the public-proof refs, the typed change significance, and audience-safe summaries. Raw matrix row ids are admissible; opaque schema refs are admissible; raw record contents are not. |

A packet whose audience cannot be typed denies with
`audience_class_unresolved` rather than collapsing to
`engineering_internal`.

## 5. Redaction-profile vocabulary

Closed six-class `redaction_profile_class`:

| Class | Pairs with audience | Required posture |
| --- | --- | --- |
| `engineering_internal_only` | `engineering_internal` | Full diff sections; no support-only or enterprise-only suppression. |
| `support_handoff_redacted` | `support_handoff` | Suppresses release-train rationale, unrelated public-proof narrowings, and enterprise-only contract status. |
| `enterprise_audit_redacted` | `enterprise_audit` | Suppresses internal-only contract-status churn and support-only handoff refs. |
| `release_readiness_summary` | `release_readiness` | Suppresses support-only payload refs; preserves contract status and change significance. |
| `public_proof_safe_zero_payload` | `public_proof_safe` | Suppresses internal-only diff axes that do not narrow a marketed claim; raw record contents excluded; only marketed-claim narrowings, public-proof refs, and audience-safe summaries appear. |
| `redaction_profile_class_unresolved` | (none) | Forbidden on a published packet; a packet whose redaction profile is unresolved denies. |

The schema enforces that `redaction_profile_class` matches the
`audience_class` per the table above. A packet whose redaction profile
widens scope past the named audience denies with
`redaction_profile_widens_scope`.

The honesty invariants on the redaction profile:

- `raw_record_payloads_excluded` — constant `true`.
- `raw_hold_justifications_excluded` — constant `true`.
- `raw_policy_bundle_bytes_excluded` — constant `true`.
- `raw_user_identifiers_excluded` — constant `true`.

## 6. Record-class registry diff

Every packet projects one diff against the record-class registry seed
since the baseline.

### 6.1 Addition entry shape

Each entry on `record_class_registry_diff.additions[]`:

- `record_class_id` — verbatim from the registry.
- `addition_summary` — bounded reviewable sentence.
- `change_significance_class` — typed (§12).
- `linked_redaction_artifact_refs[]` and
  `linked_retention_artifact_refs[]` — at least one each per the
  registry's change-discipline floor.

### 6.2 Change entry shape

Each entry on `record_class_registry_diff.changes[]`:

- `record_class_id` — verbatim from the registry.
- `changed_axis_class[]` — typed change-axis vocabulary (§6.4).
- `before_summary` — bounded reviewable sentence quoting the prior
  posture.
- `after_summary` — bounded reviewable sentence quoting the new
  posture.
- `change_significance_class` — typed (§12).
- `linked_redaction_artifact_refs[]` and
  `linked_retention_artifact_refs[]`.

### 6.3 Retirement entry shape

Each entry on `record_class_registry_diff.retirements[]`:

- `record_class_id` — verbatim from the registry.
- `retirement_summary` — bounded reviewable sentence.
- `superseding_record_class_id` — opaque ref into a row that supersedes
  the retired class, or `null` when the class is retired without
  replacement.
- `change_significance_class` — typed (§12).

### 6.4 Changed-axis vocabulary

Closed seven-class `record_class_changed_axis_class` mirroring the
record-class row block names verbatim:

- `scope_posture`
- `retention_posture`
- `hold_posture`
- `delete_posture`
- `export_posture`
- `offboarding_posture`
- `partial_result_cause_set`

Surfaces MUST NOT collapse two axes into one chip. A change that moves
a class from local-authoritative to managed-authoritative AND adds a
retention floor is two changed axes, not one.

### 6.5 Schema-version-advanced vocabulary

Closed three-class `registry_schema_version_advanced_class`:

| Class | Meaning |
| --- | --- |
| `not_advanced` | Registry rows changed but `record_class_schema_version` did not advance. |
| `additive_minor` | Registry schema added a new enum value or new optional field; version advanced additively. |
| `breaking` | Registry schema repurposed an existing enum value or required a new decision row; version advanced as a breaking change. The packet's `change_significance_class` MUST be `release_bearing` or `claim_narrowing`. |

## 7. Retention and legal-hold policy diff

### 7.1 Retention-matrix change entry shape

Each entry on
`retention_and_legal_hold_policy_diff.retention_matrix_changes[]`:

- `matrix_row_ref` — verbatim from the retention/deletion matrix.
- `changed_matrix_axis_class[]` — typed (§7.3).
- `before_summary` and `after_summary` — bounded reviewable sentences.
- `change_significance_class` — typed (§12).
- `linked_record_class_refs[]` — opaque refs into the registry rows
  the matrix row narrows.

### 7.2 Legal-hold posture change entry shape

Each entry on
`retention_and_legal_hold_policy_diff.legal_hold_posture_changes[]`:

- `hold_change_kind_class` — typed (§7.4).
- `hold_class` — typed `legal_hold_class` mirroring the
  retention/deletion matrix's hold vocabulary (`tenant_legal_hold`,
  `support_investigation_hold`, `regulatory_hold`,
  `customer_managed_hold`, `cross_tenant_hold`).
- `affected_matrix_row_refs[]` — non-empty.
- `affected_record_class_refs[]` — non-empty.
- `expected_clear_at` — RFC 3339 UTC timestamp or `null` (when no
  clear time can be promised; surfaces MUST render this as "requires
  hold review" rather than implying a clock).
- `change_significance_class` — typed (§12).
- `hold_change_summary` — bounded reviewable sentence; raw hold
  justifications and raw legal text MUST NOT appear.

### 7.3 Retention-matrix changed-axis vocabulary

Closed seven-class `retention_matrix_changed_axis_class`:

- `location_class`
- `default_retention`
- `export_path`
- `delete_path`
- `policy_owner`
- `partial_blockers`
- `remaining_location_classes`

### 7.4 Legal-hold change-kind vocabulary

Closed five-class `legal_hold_change_kind_class`:

| Class | Meaning |
| --- | --- |
| `hold_added` | A hold was placed on at least one matrix row / record class. |
| `hold_extended` | An existing hold was extended (new expected-clear-at, expanded scope). |
| `hold_narrowed` | An existing hold was narrowed (some affected rows / classes cleared). |
| `hold_cleared` | A hold was released; affected rows resume their default delete path. |
| `hold_review_pending` | A hold is in review and the next state-change cannot be promised; pairs with `expected_clear_at = null`. |

A hold change whose kind cannot be typed denies with
`legal_hold_change_kind_class_unresolved` rather than defaulting.

## 8. Export and delete contract status

### 8.1 Export-path row shape

Each entry on `export_delete_contract_status.export_path_rows[]`:

- `export_path_class` — typed; mirrors the retention/deletion
  matrix's `export_path_class` enum.
- `contract_doc_ref` — opaque doc ref.
- `contract_status_class` — typed (§8.3).
- `manifest_required` — boolean.
- `raw_secret_excluded_required` — constant `true`.
- `gating_blocker_refs[]` — refs into open blockers (waiver register
  entries, retention/deletion matrix `partial_blockers`).
- `export_path_summary` — bounded reviewable sentence.

### 8.2 Delete-path row shape

Each entry on `export_delete_contract_status.delete_path_rows[]`:

- `delete_path_class` — typed; mirrors the matrix's `delete_path_class`.
- `contract_doc_ref` — opaque doc ref.
- `contract_status_class` — typed (§8.3).
- `honors_legal_hold` — constant `true`.
- `gating_blocker_refs[]` — refs into open blockers.
- `delete_path_summary` — bounded reviewable sentence.

### 8.3 Contract-status vocabulary

Closed five-class `contract_status_class`:

| Class | Meaning |
| --- | --- |
| `contract_frozen` | Contract is frozen; no open changes. |
| `contract_in_review` | Contract is in review for a forthcoming change; `gating_blocker_refs[]` MUST cite the review packet. |
| `contract_breaking_change` | A breaking change has been recorded; the packet's `change_significance_class` MUST be `release_bearing` or `claim_narrowing`. |
| `contract_needs_renewal` | A renewal is owed (e.g. waiver expired, evidence stale); pairs with at least one waiver register entry ref or stale evidence ref. |
| `contract_not_yet_seeded` | Contract is named but not yet seeded; admissible only when the affected matrix row's status admits a sentinel. |

### 8.4 Delete-request-state status

Closed four-class `delete_request_state_status_class`:

| Class | Meaning |
| --- | --- |
| `delete_request_state_live_in_use` | The shared delete-request-state schema is live and consumed by every surface that resolves a delete request. |
| `delete_request_state_in_review` | A schema change is in review; gating refs MUST cite the review packet. |
| `delete_request_state_needs_renewal` | The schema needs renewal (additive minor or breaking version bump). |
| `delete_request_state_not_yet_seeded` | Schema is not yet seeded; admissible only when no surface resolves a delete request through it yet. |

## 9. Offboarding evidence links

Refs cite opaque ids only; raw exit-packet bytes, raw destruction
receipt bodies, raw collaboration session payloads, and raw AI
retained prompts MUST NOT appear.

- `exit_packet_refs[]` — refs into offboarding exit packet ids
  resolved through
  [`./record_class_governance.md`](./record_class_governance.md)'s
  `offboarding_exit_packet` and
  [`./data_portability_and_exit_matrix.md`](./data_portability_and_exit_matrix.md).
- `destruction_receipt_refs[]` — refs into destruction receipt ids
  resolved through `destruction_receipt_record`.
- `portability_matrix_refs[]` — refs into per-domain portability rows.
- `support_offboarding_drill_refs[]` — refs into support drill ids
  that exercise the offboarding path.
- `collaboration_offboarding_evidence_refs[]` — refs into collaboration
  evidence packets carrying offboarding posture.
- `ai_offboarding_evidence_refs[]` — refs into AI retained evidence
  packets carrying offboarding posture.

The packet's `offboarding_links_summary` carries one bounded reviewable
sentence pinning what offboarding evidence is in scope this window.

## 10. Open held-data caveats

Each entry on `open_held_data_caveats[]`:

- `caveat_id` — stable opaque id.
- `caveat_class` — typed (§10.1).
- `affected_matrix_row_refs[]` — non-empty.
- `affected_record_class_refs[]` — non-empty.
- `linked_waiver_register_entry_ref` — opaque ref or `null`. Required
  non-null when the caveat is held under an active waiver.
- `linked_privacy_history_event_refs[]` — refs into the privacy-
  history events that recorded the hold or retention floor.
- `expected_clear_at` — RFC 3339 UTC timestamp or `null`.
- `caveat_summary` — bounded reviewable sentence.

### 10.1 Caveat-class vocabulary

Closed eleven-class `held_data_caveat_class`:

| Class | Meaning |
| --- | --- |
| `legal_hold_active` | A legal hold is active; pairs with at least one matrix row and at least one record class. |
| `policy_retention_floor` | A retention floor blocks destructive lifecycle steps for a managed or audit subset. |
| `support_investigation` | A support investigation holds the data; pairs with the support case ref. |
| `export_pending_hold` | An export window blocks delete completion. |
| `sync_backlog` | A sync backlog blocks managed-archive purge. |
| `provider_backlog` | A downstream provider backlog blocks managed-archive purge. |
| `managed_service_unavailable` | The managed service is unavailable; the hold lifts when service is restored. |
| `manual_local_capture_required` | The data is held pending a manual local capture (e.g. crash diagnostic that requires user re-run). |
| `redaction_policy` | Redaction policy holds the data behind a transformation gate. |
| `outside_platform_scope` | The data lives outside Aureline platform scope (e.g. third-party export remains under user control). |
| `import_source_unreachable` | The import origin is unreachable; deletion at the origin cannot be promised. |

A caveat whose class cannot be typed denies with
`held_data_caveat_class_unresolved` rather than defaulting.

## 11. Linked artifact families

Refs cite stable artifact-family ids resolved through the named
schemas / docs. Empty arrays are admissible.

- `telemetry_schema_registry_refs[]` — schema-registry entry ids from
  [`/schemas/governance/schema_registry_entry.schema.json`](../../schemas/governance/schema_registry_entry.schema.json).
- `support_export_packet_refs[]` — support export packet ids.
- `collaboration_evidence_refs[]` — collaboration evidence packet ids.
- `ai_retained_evidence_refs[]` — AI retained evidence packet ids.
- `offboarding_export_artifact_refs[]` — offboarding / exit packet
  artifact ids.
- `claim_manifest_row_refs[]` — claim manifest row ids the packet
  composes over.
- `public_proof_row_refs[]` — public-proof / claim manifest publication
  row ids.

Linkage is systematic: a packet whose record-class registry diff
touches a class in {`telemetry_contract_schema`,
`crash_diagnostic_payload`, `performance_capture_evidence`} MUST cite
at least one telemetry schema-registry entry. A packet that touches
`support_bundle_archive` MUST cite at least one support-export packet
ref. A packet that touches `collaboration_evidence_packet` MUST cite
at least one collaboration evidence ref. A packet that touches
`ai_retained_evidence_packet` MUST cite at least one AI retained
evidence ref. A packet that touches `offboarding_exit_packet` or
`destruction_receipt_record` MUST cite at least one offboarding /
exit packet artifact ref. The schema enforces these pairings.

## 12. Change-significance rules

Closed four-class `overall_significance_class`:

| Class | Meaning |
| --- | --- |
| `informational` | The packet records changes that do not move a release floor, do not change a marketed claim, and do not block an active milestone-close lane. The release packet, the claim manifest, and the public-proof index render this packet as a routine update. |
| `release_bearing` | At least one change in the packet must land or be waived before the affected release train, milestone, or compatibility window can close. The release-truth summary MUST cite this packet. |
| `claim_narrowing` | At least one marketed public claim row narrows because of a change in this packet (e.g. a managed copy moved under a retention floor that the public claim previously asserted as user-controlled). The claim manifest, the public-proof index, the About / Help disclosure, and the release notes render the narrowing. |
| `claim_widening_blocked` | A previously narrowed claim is awaiting a widening decision (e.g. correction landed and the marketed claim could be widened back); the widening is blocked pending decision. The packet MUST cite the gating waiver register entry and the gating decision register row. |

Schema-enforced pairings:

- `release_bearing` requires a non-null `release_bearing_rationale`
  reviewable sentence and at least one
  `linked_release_truth_summary_refs` entry.
- `claim_narrowing` requires a non-empty `claim_narrowing_links[]`
  array; each entry carries `claim_row_ref` and at least one
  `public_proof_row_ref`. The packet's
  `linked_artifact_families.claim_manifest_row_refs[]` and
  `linked_artifact_families.public_proof_row_refs[]` MUST contain the
  same ids.
- `claim_widening_blocked` requires a non-empty
  `linked_waiver_register_refs[]` and a `claim_widening_blocked_decision_ref`.
- `record_class_registry_diff.changes[]` of class
  `change_significance_class = claim_narrowing` requires that the
  packet's `overall_significance_class` is also `claim_narrowing` (or
  `claim_widening_blocked`).
- `retention_and_legal_hold_policy_diff.legal_hold_posture_changes[]`
  with `change_significance_class = release_bearing` requires
  `overall_significance_class` in
  `{ release_bearing, claim_narrowing }`.

The packet's `significance_summary` carries one bounded reviewable
sentence pinning the overall posture; raw policy bodies, raw waiver
justifications, and raw user identifiers MUST NOT appear.

## 13. Consuming-surface parity

Every consuming surface that renders the packet MUST render the same
typed sections. The parity floor is enforced by the schema's
`consuming_surface_parity` block.

Required on every consuming surface:

- `packet_id`, `packet_window.window_kind_class`,
  `packet_window.window_ref`;
- `audience_class`, `redaction_profile.redaction_profile_class`;
- `record_class_registry_diff` section counts (additions, changes,
  retirements) and every entry's `record_class_id` plus
  `change_significance_class`;
- `retention_and_legal_hold_policy_diff` section counts and every
  entry's `matrix_row_ref` / `hold_change_kind_class` plus
  `change_significance_class`;
- `export_delete_contract_status.delete_request_state_status_class`
  and every contract row's `contract_status_class`;
- `offboarding_evidence_links` array sizes and the
  `offboarding_links_summary`;
- `open_held_data_caveats[]` (every entry's `caveat_class`,
  `affected_matrix_row_refs`, `affected_record_class_refs`,
  `expected_clear_at`, `caveat_summary`);
- `change_significance_summary.overall_significance_class` and the
  `significance_summary`;
- `headline_label`, `packet_summary`.

Forbidden collapses on every consuming surface:

- Rendering a packet whose `overall_significance_class =
  claim_narrowing` as `informational` to keep the public claim
  manifest clean.
- Dropping `open_held_data_caveats[]` to keep the dashboard quiet.
- Reformatting the registry diff into a screenshot or a parallel
  narrative status note.
- Rendering a `release_bearing` packet on a `public_proof_safe`
  surface without the audience-safe summary projection.
- Filtering out a `legal_hold_active` caveat under a "review needed"
  affordance.
- Substituting "should be quick" or "still working on it" for a typed
  `expected_clear_at` value.

## 14. Authoring rules

When the record-class registry, retention/deletion matrix, legal-hold
posture, export/delete contract, or offboarding evidence inventory
changes:

1. Mint or update a `records_governance_packet_record` projecting the
   change against the named window.
2. Resolve every `record_class_id` against
   [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml).
3. Resolve every `matrix_row_ref` against the retention/deletion
   matrix fixtures and contract.
4. Resolve every `linked_waiver_register_entry_ref` against the waiver
   register seed.
5. Resolve every `claim_row_ref` and `public_proof_row_ref` against
   the claim manifest.
6. Recompute `change_significance_summary.overall_significance_class`
   per §12 and the schema's `allOf` block.
7. Recompute `consuming_surface_parity` so the dashboard, milestone
   scorecard, release packet, support export, governance packet,
   claim manifest, public-proof index, and weekly governance review
   render the same record.

A reprojection is required when:

- A registry row is added, changed, or retired.
- A retention/deletion matrix row's posture changes.
- A legal-hold posture transitions (added, extended, narrowed, cleared,
  review pending).
- A delete-path or export-path contract status flips.
- An offboarding artifact family's evidence rotation completes.
- A waiver register entry holding a caveat transitions through
  `register_active_within_expiry`,
  `register_active_pending_renewal`,
  `register_renewed_under_new_decision`,
  `register_closed_correction_landed`,
  `register_narrowed_claim_published`,
  `register_escalated_pending_resolution`,
  `register_rejected_no_protection`, or
  `register_expired_no_decision`.

The reprojection MUST advance `evaluated_at` and recompute the diff
sections, the open-caveat list, and the change-significance block.

## 15. Out of scope

This contract does not implement:

- Retention enforcement, deletion backends, legal-hold tooling, or
  compliance automation.
- The record-class registry itself (that lives on
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  and
  [`./record_class_governance.md`](./record_class_governance.md)).
- The retention/deletion matrix or the per-request delete-state
  records (those live on
  [`./retention_deletion_matrix_contract.md`](./retention_deletion_matrix_contract.md),
  [`/schemas/governance/retention_matrix_row.schema.json`](../../schemas/governance/retention_matrix_row.schema.json),
  and
  [`/schemas/governance/delete_request_state.schema.json`](../../schemas/governance/delete_request_state.schema.json)).
- The waiver register or the renewal-or-close decision objects (those
  live on
  [`./waiver_register_contract.md`](./waiver_register_contract.md) and
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)).
- The claim manifest itself (that lives on
  [`./claim_manifest_contract.md`](./claim_manifest_contract.md) and
  [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)).

This contract is the projection vocabulary that the underlying record-
class, retention, hold, export, delete, and offboarding records flow
through when they reach the milestone scorecard, the release packet,
the support handoff bundle, the claim manifest, the public-proof
index, and the weekly governance review — so a reviewer can read
governance posture in one packet rather than hunting across documents.
