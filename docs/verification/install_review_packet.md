# Install-review delta, compatibility-label accuracy, activation-budget, and rollback/quarantine verification seed

This packet freezes one shared verification story for package and
extension install flows: which fields an install-review surface MUST
project, how requested and effective permissions relate (including
transitive permission growth through dependency closure), which
compatibility-label states are admissible against observed runtime
evidence, how activation-budget classes project onto the managed-
workspace budget summary, and how rollback and quarantine posture
render alongside the install review. It exists so later marketplace,
mirror, private-registry, and extension-manager surfaces reuse one
inspectable object model rather than inventing per-surface install
copy, silent transitive-permission growth, or ambiguous "unavailable"
chips that mix unsupported, unclaimed, and degraded truth.

If this packet, the
[`install_review_manifest.yaml`](../../fixtures/ecosystem/install_review_manifest.yaml)
corpus, the
[`compatibility_label_audit.yaml`](../../artifacts/ecosystem/compatibility_label_audit.yaml)
audit, and the frozen ecosystem and runtime taxonomies disagree, the
frozen ADR-0012 reserved fields and ADR-0011 capability-lifecycle axes
win for tooling and this packet must update in the same change.

Companion artifacts:

- [`/fixtures/ecosystem/install_review_manifest.yaml`](../../fixtures/ecosystem/install_review_manifest.yaml)
  — machine-readable case roster covering clean install, transitive-
  permission growth, degraded compatibility label, activation-budget
  overage, mirror/offline catalog row, quarantine/rollback review, and
  unclaimed/unsupported ecosystem rows.
- [`/artifacts/ecosystem/compatibility_label_audit.yaml`](../../artifacts/ecosystem/compatibility_label_audit.yaml)
  — machine-readable accuracy-audit matrix binding observed runtime
  state (target-discovery confidence class, capability-lifecycle axes,
  host-boundary cue stack, managed-workspace lifecycle state, freshness
  class) to admissible compatibility-label states, with the explicit
  downgrade rule applied when evidence is stale or incomplete.
- [`/artifacts/ecosystem/activation_budget_examples/`](../../artifacts/ecosystem/activation_budget_examples/)
  — reviewer-facing activation-budget rows naming budget class, slice
  accounting, degradation markers, host-boundary cue, and the
  rollback/quarantine interaction for each activation-budget class.
- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md)
  — canonical install-review summary slots, compatibility-label packet
  shape, activation-budget-summary shape, and host-boundary cue
  vocabulary this packet reuses.
- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md)
  — canonical manifest-row, effective-permission summary, publisher-
  continuity row, and policy-pack constraint row vocabulary this
  packet reuses.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — canonical five-axis capability-lifecycle vocabulary and
  dependency-marker downgrade rule this packet cites.
- [`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml)
  — canonical managed-workspace lifecycle matrix and activation-
  budget slice vocabulary this packet cites without re-deriving.
- [`/schemas/extensions/effective_permission.schema.json`](../../schemas/extensions/effective_permission.schema.json)
  — boundary schema carrying the manifest row, effective-permission
  summary, publisher-continuity row, and policy-pack constraint row
  placeholders every install-review packet quotes.
- [`/fixtures/extensions/manifest_examples/declared_vs_effective_example.yaml`](../../fixtures/extensions/manifest_examples/declared_vs_effective_example.yaml)
  — existing schema-conforming baseline this packet composes over
  instead of re-minting.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — requirement register and evidence-governance posture; explicit
  permission-diff disclosure on install and update review.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — effective permission projected through dependency closure,
  policy-pack narrowing, and host context; mirror and private-
  registry continuity preservation.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — install-review sheet field set, activation-budget summary
  shape, managed-workspace lifecycle and quarantine posture.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — install-review sheet chip and disclosure discipline;
  irreversibility and disclosure-flag rendering.
- `.t2/docs/Aureline_Milestones_Document.md`
  — install-review, compatibility-label, and activation-budget
  claims kept as inspectable packets during the foundations phase.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.install_review.seed
evidence_id: evidence.verification.install_review.packet
title: Install-review delta, compatibility-label accuracy, activation-budget, and rollback/quarantine verification seed
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
    - packet_row:install_review.review_object_contract
    - packet_row:install_review.requested_vs_effective_permissions
    - packet_row:install_review.transitive_permission_visibility
    - packet_row:install_review.compatibility_label_accuracy
    - packet_row:install_review.activation_budget_class_projection
    - packet_row:install_review.rollback_and_quarantine_posture
    - packet_row:install_review.mirror_and_offline_catalog_truth
    - packet_row:install_review.seed_corpus
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
  source_revision: install_review_seed@1
  trigger_revision: install_review_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen extension-manifest, effective-
    permission, publisher-continuity, policy-pack, capability-
    lifecycle, host-boundary, managed-workspace-lifecycle, and
    install-review vocabularies already landed in the repository.
    No marketplace backend, extension runtime, mirror broker, or
    managed control plane is wired to this packet yet. Claims are
    structural: every case in the manifest, every row in the
    compatibility-label audit, and every activation-budget example
    reuses existing frozen tokens rather than minting new per-
    surface language.
artifact_links:
  supporting_evidence_ids:
    - evidence.verification.install_review.manifest
    - evidence.ecosystem.compatibility_label_audit
    - evidence.ecosystem.activation_budget_examples
    - evidence.extensions.declared_vs_effective_example
    - evidence.runtime.target_discovery_taxonomy
    - evidence.runtime.managed_workspace_lifecycle_matrix
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/ecosystem/install_review_manifest.yaml
    - fixtures/extensions/manifest_examples/declared_vs_effective_example.yaml
  archetype_refs: []
  source_anchor_refs:
    - docs/runtime/target_discovery_and_install_review_taxonomy.md
    - docs/adr/0012-extension-manifest-permission-publisher-policy.md
    - docs/adr/0011-capability-lifecycle-and-dependency-markers.md
    - docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md
    - artifacts/runtime/managed_workspace_lifecycle.yaml
    - artifacts/ecosystem/compatibility_label_audit.yaml
    - artifacts/ecosystem/activation_budget_examples/
    - schemas/extensions/effective_permission.schema.json
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one reviewer-facing `install_review_record` object that names
  subject, disposition, manifest and effective-permission refs,
  compatibility-label state, activation-budget summary, rollback and
  quarantine posture, mirror/offline catalog truth, host-boundary
  cue stack, and target-discovery packet ref;
- one explicit `requested_vs_effective_permission_diff` block that
  makes declared, requested, inherited, and effective permission sets
  separately addressable so transitive permission growth cannot hide
  behind a single "effective" list;
- one closed `compatibility_label_state` vocabulary and one accuracy-
  audit matrix binding observed runtime state to admissible labels
  with an explicit downgrade rule when evidence is stale, incomplete,
  or unreachable;
- one closed `activation_budget_class` vocabulary and an example
  corpus naming how each class projects onto the
  `activation_budget_summary_record` slice accounting;
- one closed `rollback_posture_class` and `quarantine_posture_class`
  pair so rollback, emergency-disable, and publisher-quarantine
  events cannot render as a single generic "install failed" chip;
- one `mirror_and_offline_catalog_truth_state` vocabulary and a
  required-field set so mirrored, offline-bundled, vendored-local,
  and live-registry catalog rows stay reviewable in one schema; and
- one seed corpus covering clean install, transitive-permission
  growth, degraded compatibility label, activation-budget overage,
  mirror/offline catalog row, quarantine/rollback review, and
  unclaimed/unsupported ecosystem rows.

It does not claim a marketplace backend, an extension runtime, a
live permission inspector, a live managed-workspace control plane,
a mirror broker, or a publisher-continuity registry is wired up. It
claims only that the packet, the manifest, the compatibility-label
audit, and the activation-budget examples now exist in one
reviewable form and reuse the frozen ecosystem vocabulary already
landed elsewhere.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:install_review.review_object_contract` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.install_review.manifest` | Freezes one machine-readable `install_review_record` shape every review surface reuses. |
| `packet_row:install_review.requested_vs_effective_permissions` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.install_review.manifest`, `evidence.extensions.declared_vs_effective_example` | Declared, requested, inherited, and effective permission sets stay separately addressable. |
| `packet_row:install_review.transitive_permission_visibility` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.install_review.manifest` | Transitive permission growth through `capability_inherit` is required to list each contributing closure member and dependency-marker ref. |
| `packet_row:install_review.compatibility_label_accuracy` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.ecosystem.compatibility_label_audit` | One closed compatibility-label-state vocabulary plus an observed-to-admissible mapping; stale or incomplete evidence forces an explicit downgrade. |
| `packet_row:install_review.activation_budget_class_projection` | `GOV-EVID-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.ecosystem.activation_budget_examples` | Activation-budget class vocabulary projects onto the frozen `activation_budget_summary_record` slice accounting. |
| `packet_row:install_review.rollback_and_quarantine_posture` | `GOV-TRUTH-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.install_review.manifest` | Rollback and quarantine posture classes cover user rollback, admin rollback, emergency-disable, publisher-quarantine, and kill-switch cases. |
| `packet_row:install_review.mirror_and_offline_catalog_truth` | `GOV-TRUTH-901`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.verification.install_review.manifest` | Mirror, offline-bundle, vendored-local, and live-registry rows project onto one catalog-truth vocabulary with signature-reverify state. |
| `packet_row:install_review.seed_corpus` | `GOV-CORPUS-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.install_review.manifest`, `evidence.ecosystem.activation_budget_examples` | One stable case-id set covers the required install-review scenarios named in the task. |

## What this seed freezes

- One `install_review_record` shape every install / update review
  surface (install-review sheet, permission inspector, support-
  export row, claim-manifest entry, release-evidence packet,
  object-handoff packet) reuses for projected install language.
- One `requested_vs_effective_permission_diff` shape so the review
  preserves declared, requested (per-scope), inherited (per-closure-
  member), and effective sets separately and cites each narrowing
  class that produced the effective set.
- One closed `compatibility_label_state` vocabulary plus an
  accuracy-audit matrix binding observed runtime state to
  admissible labels; any surface that projects a label outside the
  admissible set for its observed state is non-conforming.
- One closed `activation_budget_class` vocabulary plus an example
  corpus showing how each class binds the frozen
  `activation_budget_summary_record` slice accounting and which
  rollback / quarantine posture it implies.
- One closed `rollback_posture_class` and `quarantine_posture_class`
  pair so rollback events, emergency-disable applications, and
  publisher quarantines cannot collapse into a single
  "install blocked" chip.
- One `mirror_and_offline_catalog_truth_state` vocabulary and a
  required-field set so mirror, offline-bundle, vendored-local, and
  live-registry rows share a single review schema.

## Install-review record

Every case in the machine-readable manifest resolves to one
`install_review_record` with these required fields. Field set is a
projection of the ADR-0012 reserved install-review summary slots and
the reserved reviewer-visible surface fields; this packet does not
redefine them.

- `case_id`
- `install_review_subject_class` — ADR-0012 subject-class token.
- `install_review_disposition_class` — ADR-0012 disposition token.
- `manifest_row_ref` — ADR-0012 manifest row.
- `effective_permission_summary_ref` — ADR-0012 summary row.
- `declared_permissions_digest` — from the summary row.
- `requested_vs_effective_permission_diff` — see §Requested-vs-
  effective permission diff.
- `transitive_permission_visibility` — see §Transitive permission
  visibility.
- `publisher_continuity_row_ref` — ADR-0012 publisher row.
- `mirror_and_offline_catalog_truth` — see §Mirror and offline
  catalog truth.
- `policy_pack_narrowing_refs` — ordered list of ADR-0012
  `policy_pack_constraint_row` refs that applied.
- `capability_lifecycle_row_refs` — ADR-0011 five-axis rows the
  subject binds (parent + closure members).
- `dependency_marker_refs` — ADR-0011 live dependency markers
  attached to the binding.
- `compatibility_label_state` — from the frozen vocabulary in
  §Compatibility-label accuracy.
- `compatibility_label_packet_ref` — from the frozen install-review
  taxonomy; the declared packet shape.
- `compatibility_label_downgrade_reasons` — required when the
  review's label is a downgrade from the declared packet's label;
  typed list from the audit matrix.
- `activation_budget_class` — from the vocabulary in §Activation-
  budget class projection.
- `activation_budget_summary_ref` — ADR-0009 / managed-workspace
  lifecycle summary when the subject binds a managed workspace.
- `rollback_posture_class` — see §Rollback and quarantine posture.
- `quarantine_posture_class` — see §Rollback and quarantine
  posture.
- `host_boundary_cue_stack` — ordered outermost-to-innermost list
  of `host_boundary_cue_class` tokens.
- `target_discovery_packet_ref` — pointer to the most recent
  target-discovery packet for the install target.
- `notebook_trust_rung` — present iff the subject touches a
  notebook surface; null otherwise.
- `structured_round_trip_risk_packet_ref` — present iff the subject
  performs a structured round-trip; null otherwise.
- `irreversibility_flag_set` — ordered list of ADR-0012
  irreversibility flag tokens.
- `disclosure_flag_set` — ordered list of ADR-0012 disclosure flag
  tokens.
- `review_event_refs` — ordered list of ADR-0012 `install_review`
  audit-event ids.
- `freshness_class` — ADR-0011 token.
- `redaction_class` — ADR-0007 token.
- `export_inclusion_posture` — `metadata_safe_default`,
  `operator_only_restricted`, or `broadened_capture_opt_in`.

Rule: a review surface that cannot fill `compatibility_label_state`,
`activation_budget_class`, `rollback_posture_class`,
`quarantine_posture_class`, `requested_vs_effective_permission_diff`,
or `transitive_permission_visibility` MUST deny commit with the
ADR-0011 `review_disclosure_incomplete` denial reason rather than
fall back to a generic "install?" affordance.

Rule: a review surface MAY NOT downgrade the compatibility label
silently. A downgrade MUST carry a non-empty
`compatibility_label_downgrade_reasons` list from the audit
matrix.

## Requested-vs-effective permission diff

The diff block is a first-class record field, not a tooltip. It
keeps **four** sets separately addressable so transitive growth
cannot hide behind a single "effective" set.

Required sub-fields:

- `declared_permission_set_ref` — the manifest row's declared set.
- `requested_permission_set_ref` — the permission set this install
  attempt is requesting against the effective summary. For an
  install, this equals the declared set; for a publisher-
  continuity migration or a policy-pack re-apply, it may be a
  subset.
- `inherited_permission_set_ref` — the closure members'
  contributions that flow through `capability_inherit`; keyed by
  capability-lifecycle row ref, each entry names the contributing
  permission scopes and the dependency-marker refs that produced
  them.
- `effective_permission_set_ref` — the post-narrowing effective
  set (ADR-0012 effective-permission summary).
- `diff_entries` — one entry per scope; each entry carries
  `scope_kind`, `scope_target`, `declared_present`,
  `requested_present`, `inherited_closure_refs`,
  `effective_state_class`, and `narrowing_reason_refs`.

Reserved `effective_state_class` vocabulary (per scope entry):

- `unchanged` — declared, requested, and effective agree.
- `narrowed` — policy, host context, or dependency marker narrowed
  the scope; effective is strictly narrower than declared.
- `step_up_required` — admin policy floor requires step-up; commit
  is allowed only under the narrowed ceiling.
- `denied` — admin policy denied the scope; effective set is empty
  for this scope.
- `introduced_by_inheritance` — scope not declared by the parent
  but inherited through a closure member; review MUST surface the
  contributing closure ref and the marker ref.
- `inheritance_withheld` — scope declared by the parent but the
  contributing closure member is `disabled_by_policy`, `retired`,
  or `quarantined`; effective set is empty for this scope and the
  contributing member ref MUST render.

Rules:

1. `introduced_by_inheritance` entries MUST list every closure
   member that contributed to the scope and every dependency-
   marker ref that flagged the inheritance. An entry that omits
   either list is non-conforming.
2. `inheritance_withheld` entries MUST preserve the contributing
   closure member ref even after the scope drops to empty; hiding
   it is non-conforming.
3. A review whose `diff_entries` list contains any
   `introduced_by_inheritance` entry MUST set
   `disclosure_flag_set` to include `declared_effective_diff` and
   MUST render the transitive permission visibility section
   inline.

## Transitive permission visibility

The transitive-permission block is the drill-down view backing the
diff entries above. It names each contributing closure member and
the permission scopes it contributed, so reviewers can catch silent
scope growth through dependency closures.

Required sub-fields:

- `transitive_capability_closure_refs` — ADR-0012 closure refs from
  the effective-permission summary.
- `closure_members` — ordered list; each entry carries
  `capability_lifecycle_row_ref`, `contribution_class`
  (`primary_contributor`, `transitive_contributor`,
  `withholding_contributor`), `contributed_scopes`,
  `contribution_freshness_class`, and
  `contribution_dependency_marker_refs`.
- `transitive_growth_flag_set` — typed list drawn from
  `capability_inherit_widened_effective_set`,
  `capability_inherit_introduced_new_scope_kind`,
  `closure_member_freshness_floor_unmet`,
  `closure_member_quarantined_blocks_parent`,
  `closure_member_retired_blocks_parent`,
  `closure_member_disabled_by_policy_blocks_parent`.
- `review_must_render_inline` — always `true` on any case whose
  diff contains `introduced_by_inheritance` or
  `inheritance_withheld`; `false` only when the closure contributes
  zero `effective_state_class = introduced_by_inheritance` entries.

Rules:

1. Any closure member whose contribution would widen the effective
   set beyond the parent's declared set is non-conforming. The
   effective-permission projection denies widening at policy-pack
   load per ADR-0012; this packet only visualises the block.
2. `closure_member_quarantined_blocks_parent`,
   `closure_member_retired_blocks_parent`, and
   `closure_member_disabled_by_policy_blocks_parent` flags MUST
   each pair with at least one `inheritance_withheld` diff entry.
3. A review that omits `contribution_dependency_marker_refs` when
   the contributing member has a live dependency marker is non-
   conforming; silent marker hiding is forbidden under ADR-0011.

## Compatibility-label accuracy

The compatibility label a review renders is the intersection of the
extension's declared compatibility-label packet (see the install-
review taxonomy) with the observed runtime state (target-discovery
packet, capability-lifecycle rows, host-boundary cue stack, managed-
workspace lifecycle state, freshness class). The audit matrix in
[`compatibility_label_audit.yaml`](../../artifacts/ecosystem/compatibility_label_audit.yaml)
binds every admissible combination to the set of allowed labels and
names the downgrade reason tokens the review MUST use when evidence
is stale, incomplete, or unreachable.

### `compatibility_label_state` (frozen)

| Token | Meaning | Typical rendering |
|---|---|---|
| `fully_supported` | Declared compatibility packet intersects observed evidence; capability-lifecycle rows are `generally_available` and `fully_supported`; freshness class is `authoritative_live`. | Green label; no disclosure chip required. |
| `best_effort_supported` | Support class is `best_effort` or evidence is `warm_cached`; label MUST include a best-effort disclosure. | Yellow label with "best effort" disclosure. |
| `beta_supported` | Capability-lifecycle `lifecycle_state` is `beta` or `preview`; label MUST include a beta-channel disclosure. | Yellow label with "beta" disclosure. |
| `degraded_compatible` | Observed evidence disagrees with at least one declared axis (platform-arch mismatch tolerated by a bridge, toolchain skew within warning band, freshness floor unmet with downgrade authorised). | Amber label with typed degraded-reason chip. |
| `compatibility_bridge_required` | Subject runs only through a `compatibility_bridge` host-contract family; bridge profile MUST be named. | Amber label with bridge-profile chip. |
| `unsupported_on_target` | Observed evidence refutes the declared packet on at least one mandatory axis (target class, platform arch, host-contract family, artifact transport, freshness floor). | Red label; install denied with `host_contract_family_unsupported_on_target` or related ADR-0012 denial reason. |
| `unclaimed_ecosystem_row` | No declared compatibility-label packet exists for the subject's target class or host-contract family; ecosystem row has not claimed support. | Gray label with "unclaimed" disclosure; install denied until a claim lands. |
| `compatibility_unknown_pending_probe` | Target-discovery confidence class is `unresolved_requires_user` or `resolver_unavailable`; evidence is insufficient to decide. | Gray label with "pending probe" disclosure; install deferred pending freshness. |

### Downgrade reason vocabulary (frozen)

- `observed_target_class_mismatch`
- `observed_platform_arch_mismatch`
- `observed_host_contract_family_mismatch`
- `observed_artifact_transport_mismatch`
- `observed_toolchain_version_out_of_range`
- `observed_capability_lifecycle_row_not_generally_available`
- `observed_freshness_floor_unmet`
- `observed_freshness_class_below_declared`
- `observed_resolver_unavailable`
- `observed_adapter_unreachable`
- `observed_managed_workspace_not_ready`
- `observed_evidence_stale_beyond_freshness_window`
- `observed_evidence_incomplete_missing_axis`

Rules:

1. A review MAY NOT render `fully_supported` when the freshness
   class of the effective-permission summary, the target-discovery
   packet, or any contributing capability-lifecycle row is below
   `authoritative_live`. The lowest freshness class wins.
2. A downgrade from the declared label MUST carry a non-empty
   `compatibility_label_downgrade_reasons` list drawn from the
   frozen vocabulary; silent downgrade is non-conforming.
3. `compatibility_unknown_pending_probe` MUST set
   `install_review_disposition_class =
   deferred_pending_freshness`; silently installing against an
   unknown probe is non-conforming.
4. `unclaimed_ecosystem_row` MUST deny commit; installing against
   an unclaimed ecosystem row is non-conforming until a compatibility-
   label packet lands.
5. A review whose target-discovery packet reports
   `resolver_unavailable` MUST set the label to
   `compatibility_unknown_pending_probe` regardless of other
   evidence, and MUST NOT collapse into `unsupported_on_target`.

## Activation-budget class projection

The activation-budget class names how the subject's activation-
budget summary (required on every managed-workspace binding install
and on managed-only extension installs) projects onto reviewer-
facing budget posture. The class vocabulary is closed; additions
are additive-minor.

### `activation_budget_class` (frozen)

| Token | Meaning | Required fields |
|---|---|---|
| `not_applicable` | Subject does not bind a managed workspace or any activation-budget slice. Extension installs against `local_host` or `remote_ssh` targets typically carry this. | `managed_workspace_instance_ref = null`, `budget_slices = null`. |
| `healthy_under_budget` | Every slice's `consumed` is strictly less than `budgeted`; no degradation markers. | Full `budget_slices` per slot vocabulary; empty `degradation_markers`. |
| `approaching_slice_ceiling` | At least one slice's `consumed` crosses a warning threshold (default 80% of `budgeted`). | `threshold_breach_markers` required; `degradation_markers` may still be empty. |
| `slice_exceeded_with_degradation_marker` | At least one slice's `consumed` has crossed `budgeted`; control plane has emitted a matching degradation marker with a typed reason. | Non-empty `degradation_markers`; affected slice(s) MUST be named. |
| `multi_slice_exhausted` | Two or more slices have each crossed their budget; review MUST disclose cumulative degradation and admin intervention posture. | At least two entries in `degradation_markers`; admin intervention ref required. |
| `budget_frozen_on_quarantine` | Instance entered `quarantined`; budget slices freeze at last-observed values. | `managed_workspace_lifecycle_state = quarantined`; slices show `frozen_at_utc_instant`. |
| `budget_frozen_on_retiring_or_retired` | Instance entered `retiring` or `retired`; budget slices freeze at last-observed values. | `managed_workspace_lifecycle_state` in `{retiring, retired}`; slices show `frozen_at_utc_instant`; migration hint required. |
| `budget_unknown_pending_refresh` | Control plane unreachable; summary freshness is below `authoritative_live` and slices cannot be quoted. | `freshness_class` in `{warm_cached, stale}`; review MUST defer commit. |

Rules:

1. A managed-workspace binding install MUST carry an activation-
   budget class other than `not_applicable`. `not_applicable` on a
   managed-workspace install is non-conforming.
2. A class of `slice_exceeded_with_degradation_marker` or
   `multi_slice_exhausted` MUST set
   `install_review_disposition_class` to
   `awaiting_admin_confirmation` unless an admin-confirmation
   exemption policy-pack row is applied.
3. A class of `budget_frozen_on_quarantine` MUST render the
   ADR-0012 `publisher_quarantined` denial or the managed-workspace
   `kill_switch_tripped` reason alongside the frozen slices; a
   review that hides the freeze is non-conforming.
4. A class of `budget_unknown_pending_refresh` MUST set
   `install_review_disposition_class =
   deferred_pending_freshness`; silent commit is non-conforming.

## Rollback and quarantine posture

Rollback and quarantine are separate vocabularies. A review may
quote both simultaneously (e.g. a publisher-quarantine event that
triggered a rollback). They do not collapse into a single "install
failed" chip.

### `rollback_posture_class` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `no_rollback_required` | Install is in a steady state; no prior install landed effects to reverse. | No rollback chip. |
| `rollback_pending_user_confirmation` | A prior install attempt must be rolled back; user confirmation required before rollback fires. | Rollback chip with typed confirmation prompt. |
| `rollback_pending_admin_confirmation` | Admin confirmation required because the rollback crosses a policy-narrowing or managed-only boundary. | Rollback chip with admin-confirmation prompt. |
| `rollback_in_flight` | Rollback has started; intermediate state until completion. | Rollback chip with progress disclosure and journal ref. |
| `rollback_completed_verified` | Rollback completed and post-rollback verification passed; effective-permission summary recomputed. | Rollback chip with completion record; install review may proceed. |
| `rollback_failed_manual_repair_required` | Rollback attempt failed; typed repair hook required. | Rollback chip with repair hook ref; commit denied until repair. |

### `quarantine_posture_class` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `no_quarantine_active` | Neither the extension nor the publisher is under quarantine; no emergency-disable bundle applies. | No quarantine chip. |
| `quarantined_by_policy_pack` | An admin policy pack has applied an `emergency_disable` constraint to the subject. | Quarantine chip with policy-pack ref; install denied with `policy_pack_denies_extension`. |
| `quarantined_by_kill_switch` | Kill-switch bundle quarantined the managed-workspace binding or the extension. | Quarantine chip with kill-switch ref; install denied with `kill_switch_tripped`. |
| `quarantined_publisher` | Publisher-continuity row is in `quarantined_publisher` trust tier; every extension under the publisher inherits. | Quarantine chip with publisher-continuity ref; install denied with `publisher_quarantined`. |
| `quarantine_lifted_pending_reverify` | Quarantine has been lifted but post-lift verification has not completed; label MUST still disclose the prior quarantine. | Quarantine chip with "lifted, pending re-verify" disclosure. |

Rules:

1. A review whose rollback posture is any class other than
   `no_rollback_required` MUST preserve the prior install attempt's
   journal ref in `review_event_refs`; dropping the prior audit
   evidence is non-conforming.
2. A review whose quarantine posture is any class other than
   `no_quarantine_active` or `quarantine_lifted_pending_reverify`
   MUST deny commit. A review rendering "install" on an active
   quarantine is non-conforming.
3. `quarantine_lifted_pending_reverify` admits a
   `deferred_pending_freshness` disposition only if the managed-
   workspace or publisher-continuity owner has re-verified after
   the lift; otherwise the review stays denied.
4. `rollback_pending_admin_confirmation` and
   `rollback_failed_manual_repair_required` MUST set
   `export_inclusion_posture = operator_only_restricted`; broadened
   capture is non-conforming.

## Mirror and offline catalog truth

The mirror/offline hook names where the subject's manifest came
from and how its signature chain was verified. It rides the
ADR-0012 `registry_source_class` vocabulary without inventing a
parallel one.

### `catalog_truth_state` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `live_registry_authoritative` | Manifest pulled from `public_registry` or `private_registry`; signature and digest re-verified against the live source. | No catalog-truth chip required. |
| `mirror_replicated_current` | Manifest pulled from a `mirror`; mirror replication is current; signature re-verified against the mirror's snapshot. | Mirror chip with endpoint and snapshot ref. |
| `mirror_replicated_stale` | Manifest pulled from a `mirror`; mirror snapshot is older than the declared freshness floor; review MUST disclose the staleness. | Mirror chip with stale-since timestamp; `freshness_floor_unmet` disclosure flag MUST ride. |
| `mirror_continuity_broken` | Mirror continuity verification failed (signature chain broken, snapshot digest mismatch). | Mirror chip with `mirror_continuity_broken` denial; install denied. |
| `offline_bundle_pinned` | Manifest pulled from `offline_bundle`; bundle digest pinned and re-verified; catalog truth is the bundle. | Offline-bundle chip with bundle digest ref. |
| `vendored_local_pinned` | Manifest pulled from `vendored_local`; ownership inside the workspace; signature is workspace-trusted. | Vendored-local chip with workspace-trust ref. |
| `catalog_truth_unknown_pending_reverify` | Signature re-verify in flight or source unreachable; review MUST defer commit. | Chip with `pending reverify` disclosure; `deferred_pending_freshness` disposition. |

Required fields on every review:

- `registry_source_class` — from ADR-0012.
- `mirror_endpoint_ref` — required when `registry_source_class` is
  `mirror`; null otherwise.
- `offline_bundle_ref` — required when `registry_source_class` is
  `offline_bundle`; null otherwise.
- `vendored_local_path_ref` — required when `registry_source_class`
  is `vendored_local`; null otherwise.
- `catalog_snapshot_revision_ref` — required when
  `registry_source_class` is `mirror` or `offline_bundle`.
- `signature_reverify_state` — one of
  `reverify_passed_authoritative`,
  `reverify_passed_warm_cached`,
  `reverify_pending`,
  `reverify_failed`,
  `reverify_unavailable`.
- `mirror_continuity_row_ref` — ADR-0012 continuity row quoted;
  null only when `registry_source_class` is `vendored_local`.

Rules:

1. `signature_reverify_state = reverify_failed` MUST pair with
   `catalog_truth_state = mirror_continuity_broken` or a denial
   using `extension_signature_verification_failed`; silent fallback
   to a warm-cached state is non-conforming.
2. `mirror_replicated_stale` MUST include the stale-since timestamp
   and MUST raise the `freshness_floor_unmet` disclosure flag.
3. `offline_bundle_pinned` and `vendored_local_pinned` MUST
   preserve the pin evidence across export; a support packet that
   drops the pin ref is non-conforming.
4. `catalog_truth_unknown_pending_reverify` MUST set
   `install_review_disposition_class =
   deferred_pending_freshness`.

## Seed corpus

The machine-readable manifest seeds the following case ids. Every
case carries one `install_review_record` and at least one
conformance-test ref.

| Case id | Subject class | Disposition | Compatibility label | Activation-budget class | Rollback posture | Quarantine posture | Catalog truth | Notes |
|---|---|---|---|---|---|---|---|---|
| `install_review.clean_install.extension_public_registry` | `extension_install` | `approved` | `fully_supported` | `not_applicable` | `no_rollback_required` | `no_quarantine_active` | `live_registry_authoritative` | Clean install of a public-registry extension with declared-equals-effective permissions. |
| `install_review.transitive_permission_growth.capability_inherit` | `extension_install` | `awaiting_user_confirmation` | `best_effort_supported` | `not_applicable` | `no_rollback_required` | `no_quarantine_active` | `live_registry_authoritative` | Transitive growth through `capability_inherit` introduces a new scope via a closure member; review MUST render the transitive block. |
| `install_review.degraded_compatibility_label.platform_arch` | `extension_update` | `awaiting_user_confirmation` | `degraded_compatible` | `not_applicable` | `no_rollback_required` | `no_quarantine_active` | `live_registry_authoritative` | Observed platform arch is outside the declared set but a compatibility bridge absorbs the mismatch; label downgrades with typed reason. |
| `install_review.activation_budget_overage.managed_workspace_binding` | `managed_workspace_binding_install` | `awaiting_admin_confirmation` | `best_effort_supported` | `slice_exceeded_with_degradation_marker` | `no_rollback_required` | `no_quarantine_active` | `live_registry_authoritative` | Managed-workspace binding install lands while the `warming_seconds` slice is in overage; admin confirmation required. |
| `install_review.mirror_offline_catalog_row.offline_bundle_pinned` | `bundle_install` | `approved` | `fully_supported` | `not_applicable` | `no_rollback_required` | `no_quarantine_active` | `offline_bundle_pinned` | Install from an offline-bundle-pinned catalog with re-verified signature and the bundle digest preserved. |
| `install_review.quarantine_rollback_review.publisher_quarantined` | `emergency_disable_apply` | `denied` | `fully_supported` | `not_applicable` | `rollback_pending_admin_confirmation` | `quarantined_publisher` | `live_registry_authoritative` | Emergency-disable apply after a publisher is moved to `quarantined_publisher`; rollback of a prior successful install is pending admin confirmation. |
| `install_review.unclaimed_ecosystem_row.host_contract_family` | `extension_install` | `denied` | `unclaimed_ecosystem_row` | `not_applicable` | `no_rollback_required` | `no_quarantine_active` | `live_registry_authoritative` | Host-contract family on the install target has no compatibility-label packet claim yet; install denied until a claim lands. |

## Surface admissibility

| Surface | May mint `install_review_record` | May claim rollback/quarantine posture | May claim compatibility label | Projection rule |
|---|---|---|---|---|
| `install_review_sheet` | yes | yes | yes | MUST emit one record per install/update attempt; MUST render diff, transitive block, compatibility label, activation-budget class, rollback and quarantine posture inline. |
| `permission_inspector` | no | yes (quoted) | yes (quoted) | Quotes the record minted by the sheet; MUST preserve the transitive block and dependency-marker refs. |
| `support_export` | no | yes (quoted) | yes (quoted) | MUST carry `operator_only_restricted` posture when cue stack contains a non-local boundary or when rollback is in a manual-repair state. |
| `object_handoff_packet` | no | yes (quoted) | yes (quoted) | Preserves failing object's install-review context and any rollback / quarantine posture. |
| `claim_manifest_publisher` | no | yes (quoted) | yes (quoted) | MUST quote `host_contract_family`, `artifact_transport_family`, and `compatibility_label_state` on every claim row. |
| `release_evidence_packet` | no | yes (quoted) | yes (quoted) | MUST quote freshness class; a stale record MAY NOT render as `authoritative_live`. |
| `mirror_adapter` | no | no | no | MAY NOT mint or widen the review; only quotes the catalog-truth fields it published. |

Rule: any surface not named here MAY NOT claim an install-review
record; it quotes one minted by the install-review sheet.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.verification.install_review.manifest` | `verification_corpus` | Defines the case roster every install-review record cites. | current | `fixtures/ecosystem/install_review_manifest.yaml` |
| `evidence.ecosystem.compatibility_label_audit` | `verification_matrix` | Binds observed runtime state to admissible compatibility-label states and names downgrade reasons. | current | `artifacts/ecosystem/compatibility_label_audit.yaml` |
| `evidence.ecosystem.activation_budget_examples` | `verification_corpus` | Supplies reviewer-facing activation-budget example rows and their slice accounting. | current | `artifacts/ecosystem/activation_budget_examples/` |
| `evidence.extensions.declared_vs_effective_example` | `source_anchor` | Canonical baseline manifest, publisher-continuity row, policy-pack constraint row, and effective-permission summary example. | current | `fixtures/extensions/manifest_examples/declared_vs_effective_example.yaml` |
| `evidence.runtime.target_discovery_taxonomy` | `source_anchor` | Canonical install-review summary slots, compatibility-label packet shape, activation-budget summary shape, and host-boundary cue vocabulary. | current | `docs/runtime/target_discovery_and_install_review_taxonomy.md` |
| `evidence.runtime.managed_workspace_lifecycle_matrix` | `source_anchor` | Canonical lifecycle-state matrix and activation-budget slice vocabulary this packet cites. | current | `artifacts/runtime/managed_workspace_lifecycle.yaml` |

## Verification method

- **Verification classes used:** design review, vocabulary-reuse
  review, fixture review, schema-alignment review.
- **Procedure summary:** verified that the packet and its companion
  manifest, audit matrix, and activation-budget examples reuse the
  ADR-0012 install-review summary slots, the ADR-0011 five-axis
  capability-lifecycle vocabulary, the ADR-0007 redaction classes,
  the target-discovery / host-boundary / managed-workspace-lifecycle
  taxonomies, and the ADR-0009 host-context vocabulary without
  minting parallel tokens. Verified that compatibility-label states,
  activation-budget classes, rollback posture classes, quarantine
  posture classes, and catalog-truth states are closed vocabularies
  and that seed cases exercise each required scenario named in the
  spec.
- **Automation refs:** `not_yet_seeded` for a dedicated install-
  review corpus validator; structural parsing is currently the
  available automation.

## Known gaps and waivers

- **Waiver refs:** `none`.
- **Known-limit refs:** `none`.
- **Migration-packet refs:** `none`.
- **Explicit gaps:** no marketplace backend, extension runtime,
  permission inspector, live managed-workspace control plane,
  mirror broker, or publisher-continuity registry is wired to this
  packet yet.
- **Explicit gaps:** no dedicated JSON Schema exists yet for the
  `install_review_record` family, the compatibility-label audit row
  shape, or the activation-budget example row shape. Reserved
  shapes are documented here for later schema landing.
- **Explicit gaps:** the `notebook_trust_rung` and
  `structured_round_trip_risk_packet_ref` slots are reserved but
  the notebook and structured-round-trip lanes have not yet minted
  canonical emitters for install-review contexts.

## Reviewer signoff

- **Reviewer / forum:** `@ahmeddyounis`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:**
  `packet_row:install_review.review_object_contract`,
  `packet_row:install_review.requested_vs_effective_permissions`,
  `packet_row:install_review.transitive_permission_visibility`,
  `packet_row:install_review.compatibility_label_accuracy`,
  `packet_row:install_review.activation_budget_class_projection`,
  `packet_row:install_review.rollback_and_quarantine_posture`,
  `packet_row:install_review.mirror_and_offline_catalog_truth`,
  `packet_row:install_review.seed_corpus`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `corpus_or_audit_matrix_revision_changed`.
- **Expected freshness window:** `P30D`.
- **Next packet family to update with the same evidence ids:**
  support-export packet, release-evidence packet, or extension-
  manager surface packet that starts quoting install-review or
  compatibility-label posture.
