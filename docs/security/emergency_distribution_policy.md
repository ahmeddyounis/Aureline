# Mirror-safe emergency distribution, manual-import, supersedence, and expiry policy

This document freezes the pre-implementation policy for how Aureline
distributes trust-affecting emergency artifacts — channel freezes,
capability kill switches, trust-root rotations, capability narrowing,
emergency update pauses, and revocations — through the live authoritative
path, through approved mirrors, through operator-driven manual imports,
and through offline / air-gapped transfers. It exists so emergency
controls stay operationally credible for hosted, mirrored, and air-gapped
deployments without inventing a live-SaaS-only flow, and so a mirrored or
air-gapped emergency import leaves a reviewable receipt rather than an
untracked local state flip.

Companion artifacts:

- [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — boundary schema for the emergency_action_record and
  revocation_record this policy distributes.
- [`/schemas/security/manual_import_receipt.schema.json`](../../schemas/security/manual_import_receipt.schema.json)
  — boundary schema for the manual_import_receipt_record this policy
  requires on every mirrored, manual, or offline import.
- [`/artifacts/security/emergency_artifact_relations.yaml`](../../artifacts/security/emergency_artifact_relations.yaml)
  — machine-readable relation graph covering authoritative distribution
  channels, manual-import admissibility, metadata-chain link rules,
  expiry and supersedence rules, surface projection rules, and
  cross-row invariants.
- [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  — object model for the emergency_action_record and revocation_record.
  This distribution policy layers on top of that model; it does not
  re-define fields.
- [`/docs/security/severity_matrix.md`](./severity_matrix.md) and
  [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  — advisory severity, identity, and affected-install linkage the
  emergency action projects from.
- [`/docs/security/advisory_surface_contract.md`](./advisory_surface_contract.md)
  and
  [`/schemas/security/advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json)
  — surface projection for advisory cards, emergency banners,
  revocation notices, and disclosure links that quote this policy's
  mirror/manual-import freshness and receipt fields.
- [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  — action ids, quorum floors, and the break-glass audit fields
  receipts cite when they record policy-admitted signature skips.
- [`/docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  and
  [`/docs/release/install_topology_plan.md`](../release/install_topology_plan.md)
  — release-artifact graph and mirror / air-gap publication posture this
  policy projects from.
- [`/fixtures/security/emergency_action_examples/`](../../fixtures/security/emergency_action_examples/)
  — worked emergency_action_record and revocation_record fixtures whose
  manual-import `receipt_ref` values this policy realizes.
- [`/fixtures/security/manual_import_receipts/`](../../fixtures/security/manual_import_receipts/)
  — worked manual_import_receipt_record fixtures exercising live,
  mirrored, manual, and air-gapped paths.

Normative sources this policy projects from:

- `.t2/docs/Aureline_PRD.md` §10.15 (security and emergency response)
  and §10.18 (offline and air-gapped distribution).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6.1
  (emergency-control chain), §22.8 (mirror and offline distribution),
  and §26.7 (revocation and continuity).
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13 (advisory
  and emergency-action verification lane).

If this document disagrees with those sources, those sources win and
this document plus the relations YAML and schema update in the same
change.

## Why this exists

The repository already froze:

- one severity vocabulary and advisory identity model;
- one exact-build identity model and install-topology vocabulary;
- one emergency-action and revocation object model;
- one release-artifact graph; and
- one signing-quorum policy for freeze, revocation, disable, and
  trust-root actions.

What it did not yet freeze was the **distribution contract** that turns
those records into credible operational behavior on mirrored, offline,
and air-gapped targets:

- which channels may carry a signed emergency artifact;
- what trigger_source_class / distribution_source_class /
  distribution_path_class tuples are admissible on each channel;
- when a manual import is required, when it is admissible, and when it
  is forbidden;
- how detached-signature verification, signer continuity, freshness, and
  supersedence are recorded on the receiving target;
- how expiry and supersedence propagate so a superseded or expired
  emergency artifact is machine-detectable and does not remain silently
  authoritative on a downstream mirror or offline bundle; and
- how the receiving target records who imported the metadata, from what
  source, under which policy, into which scope, and what follow-up it
  owes.

Without one governed distribution policy, every operator lane would
invent a dialect: an admin pane would quietly flip local state without a
receipt, a mirror import would describe signer continuity differently
from the authoritative origin, and an air-gapped site would lose track
of which emergency action is still in force.

## Scope

Frozen at this revision:

- the closed set of authoritative distribution channels and their
  admissible trigger / source / path tuples;
- the closed set of causes under which a manual import is admitted, and
  the minimum required receipt outcome on each cause;
- the detached-signature verification contract that every manual import
  MUST carry before applying;
- the metadata-chain stub that roots every receipt at an authoritative
  origin and enumerates upstream / supersede / parallel links;
- the expiry and supersedence rules that keep superseded or expired
  emergency artifacts machine-detectable on every downstream mirror or
  offline bundle;
- the surface-projection rules for mirror-import view, admin export,
  support packet, and manual-import banner;
- the follow-up obligation vocabulary a receipt may carry; and
- the change-control rules for adding channels, admissibility rules,
  chain link classes, surface projections, or invariants.

Out of scope at this revision:

- the live emergency-control signing infrastructure;
- the raw bytes of a disable bundle or policy bundle;
- transport protocol implementation for managed push/pull, registry
  publication, mirror sync, or file import;
- a concrete courier / cross-domain transfer tool;
- final banner, update-center, or admin-plane UI implementation; and
- any production deployment of emergency distribution infrastructure.

## Authoritative distribution channels

Every emergency_action_record and revocation_record travels through
exactly one of the channels below. The machine register is
[`emergency_artifact_relations.yaml#authoritative_distribution_channels`](../../artifacts/security/emergency_artifact_relations.yaml).
The schema vocabularies are `trigger_source_class`,
`distribution_source_class`, and `distribution_path_class` on
[`emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json).

| Channel | Trigger sources admitted | Source / path tuples admitted | Receipt posture |
|---|---|---|---|
| `managed_authoritative_origin_channel` | `signed_emergency_bundle`, `release_channel_control`, `runtime_policy` | `authoritative_origin` + `managed_push` or `managed_pull` | Receipt forbidden — live pull does not mint a receipt |
| `approved_mirror_sync_channel` | `signed_emergency_bundle`, `mirror_imported_snapshot` | `approved_private_mirror` or `customer_managed_mirror` + `mirror_sync` | Receipt required on the mirror operator's import |
| `manual_file_import_channel` | `signed_emergency_bundle`, `manual_imported_snapshot` | `manual_import_bundle` + `file_import` | Receipt required |
| `offline_transfer_channel` | `manual_imported_snapshot` | `offline_transfer_snapshot` + `offline_transfer` | Receipt required |
| `runtime_preload_channel` | `runtime_preload` | `runtime_preload` + `runtime_preload`, or `local_cached_copy` + `local_cache_projection` | Not applicable — preload rides on the runtime build identity |

Rules:

1. A distribution row that appears on an emergency_action_record or
   revocation_record MUST resolve to exactly one channel above. A path
   that does not match any channel's admissible tuple is non-conforming.
2. A client whose `distribution_source_class` is
   `manual_import_bundle`, `offline_transfer_snapshot`,
   `approved_private_mirror`, or `customer_managed_mirror` MUST have a
   corresponding manual_import_receipt_record on the operator / mirror
   that performed the import.
3. A client MUST NOT present an imported snapshot as
   `freshness_class = authoritative_live`. Receipts project snapshot,
   mirror, or offline freshness only.
4. A client whose `distribution_source_class` is `runtime_preload`
   MUST declare a freshness class that is honest about the preload
   (typically `unknown`, with `validation_state =
   cached_without_revalidation`).

## When manual import is required, admitted, or forbidden

A manual import is:

- **required** when the target cannot consume the authoritative origin
  live (air-gapped, offline, or cross-domain);
- **admitted** (with typed cause) when a mirror is stale past grace,
  when private triage needs narrow-scope imports, or when a signed
  break-glass posture explicitly admits a signature skip;
- **forbidden** when a live authoritative path remains reachable, or
  when the candidate source artifact carries no detached signature.

The machine register is
[`emergency_artifact_relations.yaml#manual_import_admissibility_rules`](../../artifacts/security/emergency_artifact_relations.yaml)
and
[`emergency_artifact_relations.yaml#forbidden_manual_import_causes`](../../artifacts/security/emergency_artifact_relations.yaml).

| Cause class | Admitted channels | Minimum verified outcome | Minimum applied posture | Notes |
|---|---|---|---|---|
| `target_is_air_gapped_or_offline` | `manual_file_import_channel`, `offline_transfer_channel` | `verified_current_signer`, `verified_prior_signer_with_continuity_statement`, or `verified_cross_signed_transition` | `applied` or `applied_with_narrowed_scope` | Default air-gap rule |
| `mirror_outage_or_lag_above_grace` | `manual_file_import_channel` | `verified_current_signer` or `verified_prior_signer_with_continuity_statement` | `applied` or `applied_with_narrowed_scope` | Operator bypasses a mirror `stale_past_grace` |
| `private_triage_narrow_scope` | `manual_file_import_channel` | `verified_current_signer` or `verification_pending_review` | `applied_with_narrowed_scope` or `pending_operator_confirmation` | Narrow-scope imports during active triage |
| `signed_break_glass_skip_admitted` | `manual_file_import_channel` | `verification_skipped_policy_admitted` | `applied_with_narrowed_scope` | Cites a signing_quorum break-glass profile; requires notify_security_trust_review + schedule_post_incident_review follow-ups |

Forbidden causes:

- `convenience_bypass_of_live_path` — a live authoritative path is
  reachable; manual import would be an ad-hoc escape hatch.
- `unsigned_metadata_import` — manual import MUST chain to a signed
  authoritative origin. An unsigned source artifact MUST result in
  `applied_state = rejected_at_verification` rather than silently
  applying.

## Detached-signature verification contract

Every manual import MUST produce a
`manual_import_receipt_record.verification` envelope with:

- `detached_signature_status` in
  {`detached_signature_present`, `detached_signature_absent`,
   `detached_signature_unreadable`, `detached_signature_optional_by_policy`};
- `verification_outcome` from the closed vocabulary on the schema;
- `verifying_tool_ref` (null only when the outcome is
  `verification_skipped_policy_admitted`);
- `verification_policy_ref` — always required; a skip is not a one-off
  decision;
- `verification_started_at` / `verification_completed_at`; and
- a short, reviewable `verification_note`.

Rules:

1. Any typed verification-failure outcome
   (`verification_failed_signature_mismatch`,
   `verification_failed_revoked_signer`,
   `verification_failed_expired_signer`,
   `verification_failed_unknown_signer`,
   `verification_skipped_policy_rejected`) MUST block the receipt from
   entering any `applied` posture. The receipt is held in
   `rejected_at_verification`, `rejected_at_policy`, or `rolled_back`.
2. `verification_skipped_policy_admitted` is admissible only when
   `verification_policy_ref` resolves to a signing_quorum break-glass
   profile and the receipt records the two required follow-up
   obligations (`notify_security_trust_review`,
   `schedule_post_incident_review`).
3. `verification_pending_review` is a holding state. The receipt
   remains in `pending_operator_confirmation` and MUST NOT claim
   `applied` until a reviewing operator updates the outcome.
4. The verifying tool MUST emit the observed
   `signer_continuity_state_class` (same signer chain, rotated with
   continuity statement, cross-signed transition, continuity review
   required, continuity broken, or unknown offline). A downstream
   receipt MUST NOT claim a stronger continuity state than its
   verifying tool observed.

## Metadata-chain stub

Every receipt carries a `metadata_chain` block with:

- `chain_id` — stable id for the chain of receipts rooted at a single
  authoritative origin;
- `authoritative_origin_ref` — opaque ref to the authoritative
  emergency_action_record or revocation_record;
- `chain_links[]` — typed edges to upstream, downstream, and parallel
  peers; and
- `chain_note` — a short, reviewable summary.

Admissible link classes (full vocabulary in the schema and relations
YAML):

- `from_authoritative_origin`
- `from_approved_mirror_snapshot`
- `from_upstream_manual_import`
- `from_offline_transfer_snapshot`
- `from_removable_media_snapshot`
- `supersedes_prior_receipt_on_target`
- `superseded_by_later_receipt_on_target`
- `parallel_receipt_for_sibling_scope`

Rules:

1. The chain MUST root at an authoritative origin. A chain whose
   `authoritative_origin_ref` is null is non-conforming.
2. Every multi-hop import MUST preserve the full upstream chain; a
   three-hop air-gapped transfer (authoritative → mirror →
   removable-media → air-gapped target) MUST carry three link rows, not
   one.
3. Supersedence is explicit on both the predecessor and the successor
   receipt. The successor MUST carry
   `supersedes_prior_receipt_on_target`; the predecessor MUST carry
   `superseded_by_later_receipt_on_target` and move to
   `receipt_state = superseded`.
4. Parallel sibling links are optional but admitted when one operator
   imports into several sibling mirrors from the same authoritative
   origin.

## Expiry and supersedence

A superseded or expired emergency artifact MUST be machine-detectable
and MUST NOT remain silently authoritative on any downstream surface.

Rules:

1. A receipt's `expiry.effective_expiry_at` MUST NOT be later than the
   imported record's `expires_at`. The receipt inherits or narrows the
   parent expiry.
2. A receipt MAY widen expiry to null only when
   `expiry_hook_class = no_expiry_pending_reconciliation` AND the
   parent record's `reconciliation.state =
   awaiting_post_incident_review`.
3. A receipt whose `freshness_class = offline_snapshot_expired` MUST
   NOT be in `receipt_state = active`. The receipt is moved to
   `expired` or superseded by a new import.
4. A superseded emergency_action_record stays inspectable through
   `history_links` and
   `relationship_links.superseded_by_record_ref`. Every downstream
   receipt for the superseded record MUST move to `receipt_state =
   superseded` and name the successor, or be withdrawn.
5. A revocation is durable. The revocation_record does not enter
   `record_state = expired` automatically; reconciliation names the
   conditions to move to `reconciled`. Superseding a
   manual_import_receipt_record that imported a revocation does not
   change the revocation's own state.
6. A receipt's `target_scope.deployment_profile_scope` MUST be a subset
   of the imported record's `deployment_profile_scope`. A receipt
   MUST NOT widen scope beyond the parent record.

## Follow-up obligations

A receipt MAY carry one or more typed follow-up obligations. Each row
names the obligation class, the owner, the deadline semantics, and the
absolute deadline when one exists.

Closed vocabulary:

- `propagate_to_downstream_mirrors`
- `propagate_to_offline_bundles`
- `notify_release_council`
- `notify_security_trust_review`
- `export_support_packet`
- `re_verify_at_expiry`
- `pin_last_known_good_until_resolve`
- `import_successor_trust_root`
- `schedule_post_incident_review`
- `narrow_channel_until_superseded`
- `rotate_operator_credentials_post_import`

Rules:

1. A receipt whose `verification_outcome =
   verification_skipped_policy_admitted` MUST carry at least
   `notify_security_trust_review` and `schedule_post_incident_review`.
2. A receipt whose imported record is a `revocation_record` with
   `action_kind`-equivalent reason
   `trust_root_compromise_or_rotation` MUST carry
   `import_successor_trust_root` as an obligation somewhere in the
   chain.
3. A receipt whose `freshness_class` is
   `mirrored_stale_past_grace`, `manual_snapshot_stale`, or
   `offline_snapshot_expired` MUST carry `re_verify_at_expiry` or
   `pin_last_known_good_until_resolve`.
4. Surfaces MUST NOT flatten multiple obligation rows into one generic
   banner button. Each row renders with its own owner and deadline.

## Surface-projection rules

The following surfaces project directly from the receipt and the
parent emergency_action_record or revocation_record. They may reformat
or hide rows by policy, but they MUST NOT invent different field names
or omit the shared ids. Full table in
[`emergency_artifact_relations.yaml#surface_projection_rules`](../../artifacts/security/emergency_artifact_relations.yaml).

| Surface | Required receipt fields | Forbidden shortcut |
|---|---|---|
| **Mirror import view** | `receipt_id`, `source_artifact.source_artifact_class`, `verification.verification_outcome`, `observed_signer_continuity_state`, `freshness_class`, `validation_state`, `supersedence_state`, `applied_state`, `relationship_links.imported_emergency_action_ref`, `relationship_links.imported_revocation_ref`, `metadata_chain.chain_id` | Inventing mirror-local freshness or signer-continuity terms |
| **Admin export** | `receipt_id`, `operator`, `import_path_class`, `verification`, `target_scope`, `applied_state`, `expiry`, `follow_up_obligations`, `metadata_chain`, `relationship_links` | Replacing durable refs with prose-only summary |
| **Support packet** | `receipt_id`, `imported_emergency_action_ref`, `imported_revocation_ref`, `observed_signer_continuity_state`, `freshness_class`, `applied_state`, `supersedence_state`, `follow_up_obligations` | Collapsing the receipt into an unstructured support note |
| **Manual-import banner** | `receipt_id`, `receipt_state`, `verification.verification_outcome`, `observed_signer_continuity_state`, `applied_state`, `follow_up_obligations` | Inventing a local `is_verified` boolean or omitting obligations |

## Acceptance mapping

This policy satisfies its acceptance criteria as follows:

- **Emergency controls can be applied and audited without assuming a
  live SaaS control plane.** The authoritative-distribution channel
  table names mirror, manual, offline, and preload paths as first-class
  channels, each carrying typed receipt posture and freshness.
- **Superseded or expired emergency artifacts are machine-detectable
  and do not remain silently authoritative.** The expiry-and-supersedence
  rules in this policy and in
  `emergency_artifact_relations.yaml#expiry_and_supersedence_rules`
  forbid silent expiry, require explicit superseded-by links on both
  ends of a supersedence, and block
  `freshness_class = offline_snapshot_expired` from receipt_state =
  active.
- **Manual import does not become an ad hoc escape hatch with one-off
  metadata or undocumented verification steps.** Manual imports are
  admitted only under the typed cause rows in this policy. The receipt
  schema forces a detached-signature verification envelope, an
  observed-signer-continuity field, and a metadata-chain stub rooted at
  the authoritative origin.
- **A mirrored or air-gapped emergency import leaves a reviewable
  receipt rather than an untracked local state flip.** Every mirrored,
  manual, and offline channel requires a
  manual_import_receipt_record; the receipt's relationship_links bind
  back to the parent emergency_action_record or revocation_record, to
  the advisory record, to the incident workspace packet, and to the
  downstream admin-export and support-packet refs.

## Change control

- Adding a new authoritative distribution channel, manual-import
  admissibility rule, metadata-chain link class, follow-up obligation
  class, surface-projection row, or invariant is additive-minor and
  requires this document, the relations YAML, and the schema (where
  applicable) to update in the same change.
- Repurposing an existing channel id, admissibility rule id, link
  class, surface id, or invariant id is breaking and requires a new
  decision row co-signed by `security_trust_review` and
  `release_council`.
- Loosening `manual_import_receipt_posture` from `required` to
  `not_applicable` on any channel is breaking.
- Loosening any expiry-and-supersedence rule is breaking.

## Current follow-up boundary

This policy intentionally stops at the object boundary. Future work may
still land:

- the live emergency-control signing infrastructure and transparency
  log;
- a concrete offline verifier build and removable-media handoff tool;
- first-class mirror-import and manual-import UI implementations; and
- automated detection of `receipt_state = expired` / `superseded` on
  downstream mirrors and offline bundles.
