# Organization administration, seat lifecycle, and fleet governance contract

This contract freezes the user-visible enterprise administration model
Aureline shows when an install participates in self-hosted, managed,
mirrored, or air-gapped deployments. It exists so a single truth model
backs the organization overview, directory or provider card, seat
lifecycle row, group-to-policy targeting preview, fleet or rollout-ring
dashboard, and device or install enrollment row. Every surface answers
the same questions with the same vocabulary:

- which tenant or organization is in scope and which deployment profile
  applies;
- which administration paths are governing identity and policy
  (standards-based or file-based) and how fresh each path is;
- which seats exist, what entitlement consequences apply when a seat
  changes, and which local artifacts and capabilities survive that
  change;
- which group-to-policy targeting will take effect on commit, what its
  preview reveals, and which seats it will touch;
- which fleet or rollout ring is active, which devices are current,
  stale, offline, drifted, or quarantined; and
- where the user, admin, or support engineer can export reviewable
  evidence without copying raw directory payloads, raw policy bundles,
  or unrelated tenant data.

Managed administration is an additive capability layered on top of
local-core use. An empty or absent control plane is a valid
administration posture; it is not the same as a broken local install.

## Companion artifacts

- [`/schemas/admin/seat_lifecycle_row.schema.json`](../../schemas/admin/seat_lifecycle_row.schema.json)
  — boundary schema for the organization overview, directory or
  provider card, seat lifecycle row, and group-to-policy targeting
  sheet records.
- [`/schemas/admin/fleet_status_row.schema.json`](../../schemas/admin/fleet_status_row.schema.json)
  — boundary schema for the fleet or rollout-ring dashboard record and
  the device or install enrollment row record.
- [`/fixtures/admin/org_admin_cases/`](../../fixtures/admin/org_admin_cases/)
  — worked cases for file-based policy distribution, SCIM drift, seat
  transfer, deprovisioning with local artifacts preserved, and a fleet
  ring containing stale and offline devices.
- [`/schemas/admin/effective_policy_card.schema.json`](../../schemas/admin/effective_policy_card.schema.json)
  and [`/docs/admin/policy_explainability_contract.md`](./policy_explainability_contract.md)
  — sibling vocabulary for effective-policy cards, lock explanations,
  policy diff views, decision-history rows, and admin handoff exports.
- [`/schemas/auth/managed_session_state.schema.json`](../../schemas/auth/managed_session_state.schema.json)
  and [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md)
  — managed-session state, reauth-requirement object, and local-work
  continuity rules consumed by seat lifecycle rows.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json),
  [`/schemas/identity/entitlement_snapshot.schema.json`](../../schemas/identity/entitlement_snapshot.schema.json),
  and [`/schemas/identity/admin_audit_packet.schema.json`](../../schemas/identity/admin_audit_packet.schema.json)
  — upstream signed-bundle, entitlement snapshot, and admin audit
  packet vocabulary that this contract reuses for source class,
  freshness, signer continuity, decision class, and tenant scope.
- [`/schemas/release/install_row.schema.json`](../../schemas/release/install_row.schema.json)
  and [`/docs/release/install_profile_card_contract.md`](../release/install_profile_card_contract.md)
  — install-profile card, side-by-side import sheet, and rollout-ring
  row vocabulary referenced by device or install enrollment rows and
  fleet ring dashboards.
- [`/schemas/policy/admin_policy.schema.json`](../../schemas/policy/admin_policy.schema.json)
  and [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — local `aureline.policy.json` artifact and signed-bundle cache
  semantics that anchor file-based and mirrored administration.

Normative product sources for this contract are the enterprise admin
plane requirements, the seat lifecycle and entitlement design, the
group-to-policy targeting UX, the fleet rollout dashboard layout, and
the offboarding-preserves-local-work guarantees in `.t2/docs/`. If this
document disagrees with those sources, the `.t2/docs/` source wins and
this contract must be updated in the same change.

## Scope

Frozen at this revision:

- one shared vocabulary for tenant identity, deployment profile,
  administration model class, directory or provider class, freshness,
  failure, rollback, auditability, seat state, entitlement consequence,
  group-to-policy targeting preview, fleet ring scope, device
  enrollment state, mirror posture, trust-root state, local-continuity
  assertion, and exportable-evidence reference;
- two strict JSON Schema files that together cover:
  - `organization_overview_record`;
  - `directory_provider_card_record`;
  - `seat_lifecycle_row_record`;
  - `group_policy_targeting_sheet_record`;
  - `fleet_ring_dashboard_record`;
  - `device_enrollment_row_record`;
- worked YAML fixtures that show standards-based and file-based admin
  as peers, seat-state transitions with explicit consequences, and
  fleet-ring dashboards with stale and offline devices preserved as
  first-class state; and
- export rules for paired human-readable summary and machine-readable
  packet handoff, mirroring the policy-explainability contract.

Out of scope:

- implementing an admin backend service, directory connector, or fleet
  agent;
- authoring or signing policy bundles, entitlement snapshots, or audit
  packets;
- live OIDC, SCIM, or provider integrations; and
- any flow that requires a vendor-only hosted control plane to be
  reachable for the model to render.

## Core principles

1. Local-core use is sovereign. Edit, save, undo or redo, local Git,
   diagnostics, and user-owned export remain explicit on every record
   that describes a seat, lifecycle event, or fleet posture.
2. Standards-based and file-based administration are peers. OIDC, SCIM,
   signed file or bundle policy, manual paths, and mirrored
   distribution share the same freshness, failure, rollback, and
   auditability vocabulary. No surface implies "real" administration
   only happens through one of them.
3. Deprovisioning is visibly distinct from local artifact deletion.
   Removing a seat, downgrading a plan, or revoking access never
   implies that local files, unsaved edits, or user-owned exports are
   gone or unsafe.
4. Group-to-policy targeting is reviewable before commit. The
   targeting sheet is a preview record that names affected seats,
   impacted policies, and proposed effective state without committing
   change.
5. Fleet posture is honest about distance. Stale, offline, drifted, or
   quarantined devices are distinct first-class states and are not
   collapsed into "out of date".
6. No vendor-only control plane is assumed. Every record can be
   produced by a self-hosted deployment, a signed file or bundle path,
   or a mirrored or air-gapped distribution. Records cite the path that
   produced them.
7. Exports stay clean. Machine-readable packets carry refs, schema
   refs, typed vocabulary, and reviewable summaries. Raw directory
   payloads, raw provider responses, raw bundle bytes, raw signing
   material, raw user identifiers, raw email or display names, raw
   group display names, raw hostnames, raw URLs, raw paths, raw
   tokens, and raw secret material never cross this boundary.

## Shared terms

Every administration surface MUST preserve these field families by name
when it renders, logs, exports, or emits CLI output.

| Field family | Meaning |
|---|---|
| `tenant_identity` | Tenant or organization scope summary, including `tenant_or_org_ref`, export-safe label, identity mode, deployment profile, and policy epoch. |
| `administration_model` | Which administration paths are governing this scope: standards-based, file-based signed bundle, mirrored, manual, or none. Each path carries a freshness class, validation state, last successful sync timestamp, and reviewable note. |
| `directory_or_provider` | Class of directory or identity provider, standards class, freshness, failure family, rollback state, and auditability summary. |
| `seat_lifecycle` | Previous and effective seat state, group memberships, entitlement consequences, local continuity rules, and offboarding or export path. |
| `group_policy_targeting` | Group selector refs, preview class, policy diff refs, affected seat counts, projected effective state, and commit gate. |
| `fleet_ring` | Ring class and scope, device counts by enrollment state, stale, offline, drifted and quarantined cohorts, last successful sync, and preserved evidence links. |
| `device_enrollment` | Device or install ref, enrollment state, install-profile card ref, mirror posture, trust-root state, bundle freshness, compliance state, and local-continuity assertion. |
| `local_continuity` | Reusable assertion that local edit, save, undo or redo, and user-owned export remain available; identifies retained local capabilities and paused or revoked managed capabilities. |
| `redaction_summary` | The included and omitted data classes plus the redaction class applied to the record. |
| `export_pair` | Paired human-readable summary ref and machine-readable packet ref with format set, schema ref, redaction summary, and compatible consumer classes. |

## Organization overview

`organization_overview_record` is the product-facing summary an admin or
support surface renders for a tenant or organization scope. It MUST
include:

- tenant identity and deployment profile;
- the active administration model with freshness for each governing
  path (standards-based, file-based, mirrored, manual);
- a seat summary by lifecycle class with counts that never embed user
  identifiers;
- a rollout-ring summary that names the lane class, exposure scope,
  and last successful promotion;
- a policy-source summary citing the effective bundle, signer
  continuity state, and last successful refresh;
- last-successful-sync evidence for each governing path;
- exportable-evidence refs to the related decision-history rows,
  policy diff views, audit events, and admin handoff exports;
- a redaction summary; and
- a local-continuity assertion that names retained local capabilities
  even if every managed path is offline.

The overview MAY be rendered as a desktop dashboard card, a CLI table,
or a support-bundle section. A compact projection MAY hide details
behind disclosure controls but MUST NOT drop the deployment profile,
administration model freshness, seat lifecycle counts, last successful
sync, or local-continuity assertion.

## Directory or provider card

`directory_provider_card_record` describes one configured administration
path. The same record represents an OIDC issuer, a SCIM provisioning
provider, a signed file-based policy distribution path, a mirrored
bundle source, or a manual file import path. The record MUST preserve:

- `provider_class` and `standards_class` from the closed vocabulary
  (OIDC v1, SCIM v2, signed file or bundle, mirrored bundle, manual
  CSV, manual JSON, air-gapped transfer, runtime preload);
- the configured `administration_path_class` (identity_only,
  provisioning_only, identity_and_provisioning, file_based_policy,
  mirrored_policy, manual_path);
- `freshness_class` from the shared distribution-freshness vocabulary;
- `validation_state_class` covering verification, cached without
  revalidation, pending review, expired, or revoked;
- `last_successful_sync_at` and `next_refresh_due_at`;
- a `failure_family_class` value that names whichever family currently
  applies (none, network, tenant policy denied, missing credential,
  authenticator incompatibility, bundle expired, signer rotated,
  signature failed, schema mismatch, mirror unreachable, manual
  import pending review);
- a `rollback_state_class` value (not applicable, last known good
  available, last known good in use, rollback in progress, blocked);
- an `auditability_summary` that says how a decision routed through
  this path can be reviewed locally, including audit event refs and
  decision-history row refs; and
- a redaction summary.

The card MUST NOT include raw issuer URLs, raw SCIM endpoint URLs, raw
client identifiers, raw bearer tokens, raw mirror hostnames, raw bundle
bytes, raw signing material, raw directory attribute values, or raw
group display names. Stable refs and reviewable labels carry the
information instead.

## Seat lifecycle row

`seat_lifecycle_row_record` is the durable answer to "what changed for
this seat and what survives the change?". Every row MUST distinguish:

- `seat_id_ref`, an opaque stable id;
- `previous_state` and `effective_state` from the closed seat lifecycle
  vocabulary (`active`, `pending`, `suspended`, `reclaimed`,
  `transferred`, `downgraded`, `deprovisioned`,
  `restored_from_offline`);
- `actor_class` for the change (signed lifecycle update, admin action,
  emergency policy, system resolver, support operator, scheduled task);
- `entitlement_consequences` over the controlled feature areas:
  managed AI, settings or workspace sync, real-time collaboration,
  review and approvals, marketplace publishing or installation, and
  any additional managed capability area in scope;
- `local_continuity` rules that fix the four local artifact operations
  (edit, save, undo or redo, export) at `true` and explicitly name
  retained capabilities, paused managed capabilities, and any
  capability that becomes unavailable;
- `group_membership_refs` that the seat held at change time;
- `offboarding_path_class` and `export_path_class` covering local
  archive only, admin transfer, seat transfer, account
  deprovisioning, and offboarding export available without a live
  managed seat;
- `audit_event_ref` and `policy_decision_history_row_refs` that link
  to the corresponding decision-history rows and admin audit packet;
- a `reauth_requirement_ref` when the change creates a new reauth
  requirement; and
- a redaction summary plus a reviewable consequence sentence.

Rows MUST NOT embed display names, email addresses, raw subject claims,
raw provider attributes, or seat-directory payloads.

A seat lifecycle row that names `deprovisioned` or `downgraded` MUST
also assert that local edit, save, undo or redo, and user-owned export
remain available, and MUST point at an offboarding or export path that
does not require a live managed seat.

## Group-to-policy targeting sheet

`group_policy_targeting_sheet_record` is the preview record an admin
sees before committing a group-to-policy change. It MUST distinguish
preview from committed state and MUST include:

- `targeting_state_class` from `proposed`, `previewed_dry_run`,
  `ready_for_commit`, `blocked`, `committed`, or `rolled_back`;
- one or more `group_selector_refs` carrying group class (security
  group, dynamic group, all-seats, role-based, manual list, or
  imported file);
- one or more `affected_policy_refs` and `policy_diff_refs` with
  baseline and effective source projections from the policy
  explainability contract;
- `affected_seat_summary` with seat counts grouped by current
  lifecycle state (no per-seat identity is embedded);
- `commit_gate` listing remaining checks (signer continuity, seat
  count thresholds, conflicting policy, mirror freshness, manual
  approval needed) and whether the gate is currently passing,
  blocked, or pending;
- `preview_continuity_assertion` that names which seats keep their
  current managed posture and which would change if committed;
- `audit_event_refs` and `policy_decision_history_row_refs` for the
  preview attempt and any prior commits; and
- a redaction summary.

A `committed` sheet MUST link to the audit event for the commit, and
MUST NOT mutate `affected_policy_refs` baselines after commit; instead
the rolled-forward state appears as a new sheet record.

## Fleet ring dashboard

`fleet_ring_dashboard_record` is the device or install posture
dashboard for one rollout ring or fleet lane. It MUST preserve:

- `ring_class` (`canary`, `pilot`, `broad`, `stable`, `preview`,
  `beta`, `lts`, or `custom_named_lane`) and `lane_scope_class`
  (`deployment_exposure`, `release_channel_population`,
  `long_term_support_population`, `air_gapped_population`,
  `mirrored_population`);
- `device_population_summary` with counts by enrollment state class:
  `current`, `pending_enrollment`, `stale_within_grace`, `stale_past_grace`,
  `offline_last_known_good`, `offline_unverified`, `drifted`,
  `quarantined`, `deprovisioned`, and `unknown`;
- `last_successful_sync_at` for the ring as a whole;
- a `staleness_summary` and `offline_summary` with the deepest stale
  device age, the count of devices past grace, and a reviewable note;
- `mirror_posture_summary` if any device in the ring uses a customer
  managed or vendor managed mirror;
- `preserved_evidence_links` to the rollout-ring row, install-profile
  cards, ring-history packet, and rollout decision the dashboard
  references;
- `export_pair` for the human-readable summary and machine-readable
  packet; and
- a redaction summary.

The dashboard MUST NOT embed device hostnames, raw IP addresses, raw
serial numbers, raw user identifiers, or raw geolocation. Each cohort
references devices by opaque id and class.

## Device or install enrollment row

`device_enrollment_row_record` is the row a fleet console, project
doctor, support packet, or admin handoff renders for one enrolled
device or install. It MUST preserve:

- `device_id_ref` (opaque), `enrollment_state_class` from the same
  closed vocabulary as the dashboard, and a reviewable enrollment
  summary;
- a reference to the matching install-profile card record;
- `mirror_posture_class` reused from the policy explainability
  vocabulary;
- `trust_root_state_class` reused from the policy explainability
  vocabulary;
- `bundle_freshness_class` and `last_successful_sync_at`;
- `compliance_state_class` (`compliant`, `compliant_with_grace`,
  `non_compliant_remediation_pending`, `non_compliant_blocked`,
  `not_evaluated_offline`);
- `local_continuity_assertion` reaffirming that local edit, save,
  undo or redo, and user-owned export remain available even if the
  device is stale, offline, or quarantined;
- `linked_seat_refs` that bind one or more seat lifecycle rows;
- `audit_event_refs` and `policy_decision_history_row_refs` for the
  most recent state-changing decisions; and
- a redaction summary.

`stale_within_grace`, `stale_past_grace`, `offline_last_known_good`,
and `offline_unverified` are distinct states. A row MUST NOT collapse
them into a generic "out of date".

## Standards-based and file-based parity rules

The administration model intentionally treats OIDC plus SCIM, signed
file or bundle policy, manual paths, and mirrored or air-gapped
distribution as peers:

| Capability | Standards-based path | File-based path | Manual path |
|---|---|---|---|
| Identity authority | OIDC issuer evidence with signer continuity. | Not applicable; identity is local-only or managed elsewhere. | Locally configured account, no upstream issuer. |
| Provisioning | SCIM v2 provider with last successful sync and drift detection. | Signed seat-roster bundle with version, epoch, and last successful import. | Imported CSV or JSON file with manual review state. |
| Policy distribution | Vendor or self-hosted bundle source. | Signed local admin bundle in `aureline.policy.json` or attached signed bundle file. | Manual policy import with reviewer note. |
| Freshness vocabulary | `authoritative_live`, `mirrored_current`, `mirrored_stale_within_grace`, `mirrored_stale_past_grace`. | `manual_snapshot_current`, `manual_snapshot_stale`, `offline_snapshot_unexpired`, `offline_snapshot_expired`. | `manual_snapshot_current` or `manual_snapshot_stale`. |
| Failure family | Network, tenant policy, missing credential, authenticator. | Bundle expired, signer rotated, signature failed, schema mismatch, mirror unreachable. | Manual import pending review or rejected. |
| Rollback | Last-known-good signed snapshot, signer continuity, narrowing-only changes. | Last-known-good cached entry from the bundle cache. | Reapply previous file with reviewer attestation. |
| Auditability | Admin audit packet plus decision-history rows. | Admin audit packet plus decision-history rows. | Admin audit packet plus reviewer note. |

A surface that lists administration paths MUST list the file-based and
manual paths next to standards-based paths. Listing one and burying the
other is a parity defect.

## Seat lifecycle consequences

Every seat lifecycle transition declares its consequence over the
controlled feature areas. Consequences are classes, not free text.

| Seat state | Managed AI | Sync | Collaboration | Review and approvals | Marketplace | Local artifacts |
|---|---|---|---|---|---|---|
| `active` | Available within plan and policy. | Available. | Available. | Available. | Available within plan and policy. | Edit, save, undo or redo, export available. |
| `pending` | Not yet provisioned. | Not yet provisioned. | Not yet provisioned. | Not yet provisioned. | Not yet provisioned. | Edit, save, undo or redo, export available. |
| `suspended` | Paused with reason. | Paused with reason. | Paused with reason. | Paused with reason. | Paused with reason. | Edit, save, undo or redo, export available. |
| `reclaimed` | Revoked from this seat; transferable to another seat per policy. | Revoked from this seat. | Revoked from this seat. | Revoked from this seat. | Revoked from this seat. | Edit, save, undo or redo, export available. |
| `transferred` | Pauses on the source seat, resumes on the target seat after reauth. | Pauses on the source seat. | Pauses on the source seat. | Pauses on the source seat. | Pauses on the source seat. | Edit, save, undo or redo, export available on both seats. |
| `downgraded` | Narrowed to plan ceiling. | Narrowed to plan ceiling. | Narrowed to plan ceiling. | Narrowed to plan ceiling. | Narrowed to plan ceiling. | Edit, save, undo or redo, export available. |
| `deprovisioned` | Revoked. | Revoked. | Revoked. | Revoked except retained legal or audit reads. | Revoked. | Edit, save, undo or redo, export available. Offboarding export does not require a live managed seat. |
| `restored_from_offline` | Resumes within plan and policy after signer-continuity check. | Resumes after sync. | Resumes after sync. | Resumes after sync. | Resumes within plan and policy. | Edit, save, undo or redo, export available throughout. |

Every seat lifecycle row MUST cite the matching managed-session state
record (where applicable), the entitlement snapshot record, and the
admin audit packet record, so the same change is queryable from the
auth, entitlement, and audit lanes.

## Group-to-policy targeting preview rules

1. A targeting sheet is `proposed` when no preview has been computed,
   `previewed_dry_run` when an evaluation has produced diff rows
   without committing, `ready_for_commit` when every commit gate
   passes, `blocked` when at least one gate fails, `committed` when
   the change is in force, and `rolled_back` when a previously
   committed change is reverted.
2. A `previewed_dry_run` sheet MUST cite the policy diff view records
   it is previewing, MUST include affected seat counts grouped by
   current seat state, and MUST NOT mutate any managed posture.
3. A `ready_for_commit` sheet MUST list the commit gates that pass and
   any that remain pending. It is admissible to require an additional
   admin approval, an offline mirror refresh, or an audit-quorum
   confirmation as gates.
4. A `committed` sheet MUST point at the admin audit event that
   committed the change and the resulting decision-history rows. It
   MUST NOT silently rewrite its own baseline references.
5. A `blocked` sheet MUST identify the blocker class (signer
   continuity broken, mirror unreachable, bundle past grace, manual
   approval pending, conflicting policy detected) and the next safe
   action.

## Fleet ring and stale or offline devices

The fleet ring dashboard presents stale and offline devices as
first-class state, not as exceptions:

- `current` devices have completed their most recent expected sync.
- `pending_enrollment` devices have begun enrollment but have not
  reached `current` for the first time.
- `stale_within_grace` devices have a freshness window that has
  expired but remains within the configured grace period.
- `stale_past_grace` devices have a freshness window past grace; new
  managed privilege MUST NOT appear from a stale source.
- `offline_last_known_good` devices are using a verified last-known-
  good cache; the dashboard cites which evidence is preserved.
- `offline_unverified` devices have no verified path forward; the
  dashboard cites the next safe action and notes that local artifacts
  remain safe.
- `drifted` devices show provisioning drift relative to the last
  observed signed seat roster (for example, SCIM no longer reports a
  seat the local install believes it has).
- `quarantined` devices are held outside the rollout ring by an
  emergency or admin action; the dashboard preserves the reason class
  and audit event ref.
- `deprovisioned` devices have a deprovisioned seat row but local
  artifacts remain visibly safe.
- `unknown` devices are reachable in the inventory but have not yet
  produced a signed posture report.

A dashboard or device row MUST NOT promote `stale_past_grace` or
`offline_unverified` into `current` solely on the basis of a recent
heartbeat or unverified report.

## Deep-link and reuse rules

1. An organization overview MUST link to the directory or provider
   cards, the latest fleet ring dashboard, the seat lifecycle row
   set, and the active group-to-policy targeting sheets.
2. A directory or provider card MUST link to its decision-history
   rows, audit events, and admin handoff exports.
3. A seat lifecycle row MUST link to the matching managed-session
   state record, entitlement snapshot record, admin audit packet
   record, and any policy decision-history rows that explain the
   transition.
4. A group-to-policy targeting sheet MUST link to the policy diff
   view records it is previewing, the audit events for prior commits,
   and the admin handoff export.
5. A fleet ring dashboard MUST link to its rollout-ring row record,
   ring-history packet, and the install-profile cards covered by the
   ring.
6. Every device enrollment row MUST link to its install-profile card,
   the seat lifecycle rows it is bound to, and the most recent admin
   audit event affecting its enrollment.
7. Deep links are stable route refs, not browser-only console URLs.
   They MUST work in desktop, CLI projection, self-hosted, mirrored,
   air-gapped, or partially managed contexts whenever the referenced
   record is locally available.
8. If a referenced record is unavailable offline, the link still
   renders with an unavailable reason and the next safe action.

## Export and handoff rules

Every export path produces a paired summary and machine-readable
packet, exactly as the policy explainability contract requires:

- a human-readable summary suitable for a user, admin, or support
  engineer; and
- a machine-readable packet with schema ref, source build or channel
  context where available, tenant scope, administration model
  posture, seat lifecycle rows, fleet ring dashboard refs, redaction
  summary, and compatible consumer classes.

Machine-readable output MUST stay clean. CLI or headless JSON cannot
be polluted by progress text, instructional copy, screenshots, or
rendered Markdown. Human-readable context belongs in the paired
summary.

Exports MUST NOT include whole policy bundles, raw policy rule
bodies, raw signatures, raw tenant directory payloads, raw provider
payloads, raw issuer URLs, raw SCIM endpoint URLs, raw mirror
hostnames, raw user identifiers, raw email addresses, raw display
names, raw group display names, raw device hostnames, raw IP
addresses, raw serial numbers, raw command lines, raw paths, raw
tokens, or raw secret material. They MAY include opaque refs,
fingerprints, source labels, schema refs, redaction notes, and
reviewable summaries.

## Offline, mirrored, and partially managed states

The admin surfaces remain useful without a live hosted console:

- `current_mirrored` and `mirrored_current` render as current but
  cite the mirror source and last successful sync.
- `stale_within_grace` may continue existing narrowed administration
  behavior, but new managed privilege MUST NOT appear from the stale
  source.
- `stale_past_grace`, `offline_unverified`, `verification_failed`,
  and `revoked` states fail closed for new managed privilege with
  visible explanations and repair or escalation actions.
- `offline_last_known_good` MUST cite the bundle, snapshot, or seat
  roster used as the fallback and MUST show which managed actions
  are paused.
- `partially_managed` MUST identify which administration paths are
  governing and which remain local or user-owned. It MUST NOT imply
  fleet control over unmanaged areas.

## Fixture coverage

The seeded organization-administration cases cover:

- a self-hosted deployment whose policy and seat roster are
  distributed by a signed file-based bundle (no SCIM, no OIDC
  discovery) with full freshness and audit evidence;
- a managed deployment where the SCIM provisioning provider has
  drifted from the local seat roster and the targeting preview
  surfaces the drift before commit;
- a seat transfer that pauses managed capabilities on the source
  seat, resumes them on the target seat after reauth, and preserves
  local artifacts on both seats;
- a deprovisioning case where seat removal revokes managed
  capabilities while local edit, save, undo or redo, and offboarding
  export remain available without a live managed seat; and
- a fleet ring dashboard where some devices are current, some are
  stale within grace, some are stale past grace, one is offline with
  a verified last-known-good cache, and one is offline unverified.

Each fixture carries previous and effective state, source provenance,
audit and decision-history links, redaction summaries, exportable
evidence refs, and the local-continuity assertion. Fixtures are
examples of the product contract; they do not describe a directory
service, provisioning service, or fleet agent implementation.

## Change management

- Adding a new enum value, optional property, or additive record kind
  is additive-minor and bumps
  `org_admin_seat_and_fleet_schema_version` in each affected schema.
- Renaming or repurposing an existing value is breaking and requires
  a governance decision because it would change the meaning of
  existing support packets, CLI output, decision-history rows,
  fixture cases, and admin handoff exports.
- Any change that weakens local edit, save, undo or redo, or
  user-owned export continuity, hides standards-based or file-based
  paths from peer parity, or collapses stale and offline device
  states into a single bucket MUST update this contract, both
  schemas, and the fixtures together.
