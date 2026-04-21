# ADR 0012 — Extension manifest scope, effective permission, publisher continuity, and policy-pack ADR seed

- **Decision id:** D-0018 (see `artifacts/governance/decision_index.yaml#D-0018`)
- **Status:** Proposed — this is an ADR seed. The vocabulary, field set, and invariants named below reserve shape and record fields so the extension-runtime, install-review, registry-mirror, and publisher-continuity lanes at a later milestone cannot invent them ad hoc. Full freeze lands in a successor ADR once the open questions in §Open questions are closed.
- **Decision date:** pending
- **Freeze deadline:** 2026-12-15
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** compatibility_ecosystem_review (co-required with security_trust_review because the permission-projection and policy-pack invariants ride the trust-review remit)
- **Related requirement ids:** none

## Context

Every extension-ecosystem surface the product will eventually expose —
the manifest registry, the install / update review sheet, the
permission inspector, the publisher-continuity packet, the private
registry and mirror adapter, the emergency-disable bundle, and the
host-contract binding for Wasm / WIT / external-host / bridged
runtimes — makes trust claims about code the user has not written.
The .t2 source documents (`Aureline_PRD.md`,
`Aureline_Technical_Architecture_Document.md`,
`Aureline_Technical_Design_Document.md`,
`Aureline_Milestones_Document.md`,
`Aureline_UI_UX_Spec_Document.md`,
`Aureline_UX_Design_System_Style_Guide.md`) already commit the
product to six invariants this seed reserves against:

1. A **manifest** declares permissions, host-contract binding
   (Wasm / WIT world, native bridge, helper binary, remote-side
   component), artifact transport, artifact digest, signature
   fingerprint, and compatibility-bridge notes.
2. Users and admins can see the **effective permission set after
   dependency resolution**, not only the top-level manifest; the
   effective set is the result of projecting the declared set
   through policy, host context, extension-dependency closure,
   and the active identity mode.
3. **Publisher continuity** — key rotation, ownership transfer,
   namespace disputes, orphaning, succession, and fork adoption —
   follows a verified workflow with delay, audit trail, and
   user / admin notification.
4. **Private registries and mirrors** must preserve digests,
   signatures, compatibility metadata, and permission manifests;
   a private mirror may add stricter approval layers but may not
   weaken signed publisher-continuity proof.
5. **Admin policy packs** narrow extensions (allow / deny lists,
   publisher trust tiers, version pinning, permission floors,
   mirror endpoints, emergency-disable bundles); policy never
   silently widens an extension's effective permission beyond
   what the manifest declared.
6. **Install / update review** and the **permission inspector**
   surface the declared-vs-effective permission diff, the
   transitive capability inheritance graph, the publisher lineage
   transfer (orphan / successor / fork markers), and the
   mirror / private-registry continuity state.

The extension runtime itself does not land at this milestone. What
this seed reserves is the **vocabulary and record fields** that
every later lane will have to honour: manifest fields,
effective-permission summary fields, reviewer-visible fields on the
install-review sheet, publisher-continuity packet fields, and the
policy-pack constraint model. The freeze matters now, ahead of the
runtime, because the capability-lifecycle vocabulary frozen in
ADR-0011, the connected-provider vocabulary frozen in ADR-0010, the
secret-broker projection modes frozen in ADR-0007, the settings
resolver frozen in ADR-0008, and the subscription envelope frozen
in ADR-0005 each already expose interfaces that an eventual
extension lane must project into. Without this seed, each of those
interfaces would end up carrying extension-shaped fields invented
per-lane rather than read from one vocabulary.

This ADR rides alongside ADR-0001 (identity modes gate the
`managed_admin_surface` client scope extensions may surface on),
ADR-0004 (manifest and effective-permission records cross RPC as
typed payloads; raw artifact bytes never do), ADR-0005 (permission
and lifecycle views ride the shared subscription envelope with
authority class `derived_knowledge`), ADR-0007 (credential handle
classes and projection modes apply when an extension requests
secret material; raw secret bytes never cross the boundary),
ADR-0008 (admin-policy narrowing is evaluated as an orthogonal
ceiling), ADR-0010 (connected-provider grant resolution supplies
the grant-reason vocabulary any provider-linked extension
dependency quotes), and ADR-0011 (every extension-visible
capability row carries the five orthogonal lifecycle axes and the
dependency-marker record). This ADR does not redefine those
contracts; it reserves the extension-specific fields they will
refer to.

Runtime sandbox details, WIT world layout beyond the identity
field, the extension-bridge compatibility layer, and full SDK
binding are explicitly out of scope at this milestone. They land
with a successor ADR.

## Decision

Aureline reserves four record families — **manifest row**,
**effective-permission summary**, **publisher-continuity row**,
and **policy-pack constraint row** — plus a set of projection
requirements on install / update review, permission inspector,
support export, and mirror adapter surfaces. Every field set
named below is reserved; every vocabulary below is opened as an
enumerable set whose initial members are frozen by this seed and
whose additions are additive-minor with a schema-version bump.

The intent is deliberately narrower than ADR-0011: this ADR
freezes **shape and invariants**, not the full value spaces. Where
this seed names a vocabulary, the values listed are the minimum an
eventual implementation must honour; the successor ADR may add
values (with a schema bump) but may not repurpose the ones below.

### Reserved manifest fields

Every extension manifest row MUST carry, at minimum, the
following fields. Extension metadata tables derived from the
manifest (registry index, review-sheet input, mutation-journal
entry, support-bundle row, claim-manifest entry) MUST preserve
each as a separately addressable field; collapsing any field into
a free-form string is non-conforming.

| Field                               | Shape                                                                                                      | Notes                                                                                                          |
|-------------------------------------|------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| `extension_identity`                | Opaque namespaced id: `publisher_id/extension_id`                                                          | Never reused after retirement; a new identity is minted if lineage transfer would otherwise recycle it.        |
| `extension_version`                 | SemVer string                                                                                              | Pinning and floors are policy-pack concerns; this field carries the declared value only.                       |
| `manifest_schema_version`           | Integer                                                                                                    | Bumped with the extension-manifest schema.                                                                     |
| `host_contract_family`              | One of `wasm_component_model`, `wasm_core_module`, `external_host_process`, `helper_binary`, `remote_side_component`, `compatibility_bridge` | Reserved even though runtime support lands later; establishes the class set the eventual host binder must cover. |
| `host_contract_identity_ref`        | Opaque id / URI of the host contract (WIT world id, process-contract id, helper-binary contract id, bridge-profile id) | Keeps the identity of the ABI the manifest was written against separable from the host family.                 |
| `artifact_transport_family`         | One of `wasm_signed_artifact`, `external_process_signed_artifact`, `helper_binary_signed_artifact`, `remote_component_handle`, `bridge_shim_package` | Names how the artifact is delivered and verified, independent of what runs it.                                 |
| `artifact_digest_ref`               | Stable ref to the digest record (algorithm + value)                                                        | The bytes themselves live in the registry; the manifest carries the ref.                                       |
| `signature_ref`                     | Stable ref to the signature record (publisher signing key id, algorithm, value)                            | Raw signing-key material never crosses the RPC boundary (ADR-0007).                                            |
| `declared_permissions`              | Set of permission-scope entries (see `permission_scope` vocabulary below)                                  | Declared at build time; projection to effective is computed at render / install time.                          |
| `permission_vocabulary_version`     | Integer                                                                                                    | Bumped with the permission-vocabulary schema; additive-minor.                                                  |
| `compatibility_bridge_notes`        | Nullable structured notes (bridge profile id, target compatibility range, known caveats)                   | Populated only for `compatibility_bridge` host family; required in that case.                                  |
| `publisher_identity_ref`            | Ref to the publisher row carrying continuity metadata                                                      | See §Publisher continuity below.                                                                               |
| `registry_source_class`             | One of `public_registry`, `private_registry`, `mirror`, `offline_bundle`, `vendored_local`                 | Establishes registry provenance; the mirror-continuity row quotes this field.                                  |
| `manifest_capability_binding_refs`  | List of capability-lifecycle row refs the extension binds (ADR-0011)                                       | Establishes the lifecycle / dependency-marker projection every surface must read.                              |
| `policy_pack_applicable_refs`       | List of policy-pack constraint row refs whose scope covers this extension                                  | Policy is an orthogonal narrowing ceiling; this field reserves the link, not the result.                       |
| `extension_lifecycle_row_ref`       | Ref to the capability-lifecycle row attached to the extension itself (ADR-0011)                            | So the extension inherits the five frozen axes without minting a parallel vocabulary.                          |

Additional manifest fields (e.g. human-legible display name,
homepage, localisation, icon refs) MAY be present and are not
constrained by this seed; tooling MUST preserve unknown additive
fields across round trips so later additive bumps do not lose
data.

### Reserved permission-scope vocabulary

Every declared permission is one `permission_scope` entry. The
initial frozen members below are the minimum an eventual
implementation MUST recognise; additional members are
additive-minor with a `permission_vocabulary_version` bump.

- `filesystem_read` — read-only access to a declared path set
  under the workspace VFS (ADR-0006 identity rules apply).
- `filesystem_write` — mutating access to a declared path set;
  write narrowing by workspace trust applies.
- `shell_execute` — ability to invoke the shell / command system
  (ADR-0006 shell-command contract).
- `network_egress` — outbound network access, narrowed by admin
  policy packs and identity-mode.
- `ai_provider_access` — ability to call the AI tool-call surface
  through the connected-provider vocabulary (ADR-0010).
- `connected_provider_access` — ability to reference a
  connected-provider handle (non-AI); grant-resolution reasons
  from ADR-0010 apply.
- `secret_handle_use` — ability to request a credential-handle
  projection through the secret broker (ADR-0007); raw secret
  material never crosses the boundary.
- `workspace_settings_read` — read settings through the
  extension-SDK surface (ADR-0008); crawling JSONC directly is
  forbidden.
- `workspace_settings_write` — propose writes through the
  extension-SDK surface; admin-policy narrowing applies.
- `execution_context_bind` — bind an execution context / workset
  scope (ADR-0009); raw command lines and env bodies never cross.
- `subscription_subscribe` — subscribe to a derived-knowledge
  view (ADR-0005) with a declared freshness hint.
- `ui_command_contribute` — register a command palette entry
  or review-sheet contribution under the frozen command graph.
- `capability_inherit` — declare a dependency on another
  extension's declared capability; contributes a dependency
  marker under the ADR-0011 downgrade rule.

Each entry carries, at a minimum, a `scope_kind` (one of the
members above), a `scope_target` (path prefix, provider id,
setting id, capability ref, or host contract ref as appropriate
for the kind), an optional `scope_constraint` (narrowing detail
such as "read-only", "egress to declared hosts only", "requires
step-up"), and a `rationale_label` (human-legible intent; raw
user secrets forbidden).

### Reserved effective-permission summary fields

At render / install / invocation time, the declared permission
set is projected into the **effective permission set** through
four narrowings (orthogonal; each may reduce but never widen):

1. **Admin policy packs** — allow / deny lists, permission
   floors, publisher trust tiers, version pinning, mirror rules,
   emergency-disable bundles. Evaluated as an orthogonal ceiling
   (ADR-0008 semantics extended to extensions).
2. **Host context** — identity mode (ADR-0001), workspace trust
   state, execution-context posture (ADR-0009), freshness floor.
3. **Extension-dependency closure** — transitive capability
   inheritance through the `capability_inherit` scope contributes
   dependency markers under the ADR-0011 downgrade rule; a
   closure member that is `disabled_by_policy` or `retired`
   absorbs the parent.
4. **Dependency markers** for provider-linked, freshness-floor,
   client-scope-restricted, kill-switch, or managed-only
   constraints the manifest inherits from its bound capability
   rows.

Every protected extension surface (install / update review sheet,
permission inspector, support export, mutation-journal entry,
claim manifest, and future SDK binding) MUST carry an
**effective-permission summary record** with, at minimum, the
following fields:

| Field                                          | Notes                                                                                                                           |
|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
| `extension_identity_ref`                       | Ref to the manifest row.                                                                                                        |
| `extension_version`                            | Version the summary was computed against.                                                                                       |
| `declared_permissions_digest`                  | Stable digest of the declared set (so a reviewer can confirm the manifest has not been tampered with post-publish).             |
| `effective_permissions`                        | Set of effective permission-scope entries (same shape as declared; each entry names the narrowings that produced it).           |
| `declared_vs_effective_diff`                   | Typed diff between declared and effective sets: `narrowed`, `denied`, `step_up_required`, `unchanged` per scope entry.          |
| `transitive_capability_closure_refs`           | Refs to every capability-lifecycle row the effective set inherits through `capability_inherit`; one entry per closure member.    |
| `dependency_markers_ref`                       | Refs to the ADR-0011 dependency markers that contributed a narrowing; silent hiding of markers is non-conforming.               |
| `host_context_refs`                            | Refs to the identity-mode envelope, workspace-trust state, execution-context id, and freshness floor that shaped the summary.   |
| `policy_pack_narrowings_ref`                   | Refs to the policy-pack constraint rows that applied; policy-pack content is never embedded, only referenced.                   |
| `publisher_continuity_state_ref`               | Ref to the publisher-continuity row the effective summary was computed against.                                                 |
| `mirror_continuity_state_ref`                  | Ref to the mirror / private-registry continuity row that vouched for the manifest delivery.                                     |
| `summary_freshness_class`                      | One of the ADR-0011 freshness classes; a summary whose canonical owner could not be re-verified renders non-`authoritative_live`. |
| `schema_version`                               | Integer; additive-minor on additions.                                                                                           |

The **declared-vs-effective diff** is a first-class record field,
not a tooltip. A review surface that hides the diff is denied
with the ADR-0011 `review_disclosure_incomplete` denial reason.

### Reserved publisher-continuity fields

Every extension is attached to exactly one
`publisher_continuity_row`. The row records who holds signing
authority, whether the publisher is active, and what lineage
transfers (ownership transfer, key rotation, orphaning,
succession, fork adoption) have occurred.

Reserved fields:

| Field                               | Notes                                                                                                                           |
|-------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
| `publisher_identity`                | Opaque publisher id (never reused across retired publishers).                                                                   |
| `publisher_display_label`           | Human-legible label; never the trust authority.                                                                                 |
| `active_signing_key_refs`           | Refs to currently valid signing-key records; raw key material never carried.                                                    |
| `key_rotation_history`              | Ordered list of past signing-key transitions (old ref, new ref, rotation-timestamp, rotation-reason).                           |
| `lineage_state`                     | One of `active`, `key_rotation_in_progress`, `ownership_transfer_in_progress`, `orphaned`, `succeeded`, `fork_adopted`, `retired`. |
| `predecessor_publisher_ref`         | Ref to the predecessor on a lineage transfer; null otherwise.                                                                   |
| `successor_publisher_ref`           | Ref to the successor on a retirement / succession event; null otherwise.                                                        |
| `orphan_marker`                     | Nullable; set on orphaning with the orphaning-timestamp and the admin / community route that recorded it.                      |
| `fork_adoption_ref`                 | Nullable; set on fork adoption with the adopted-from lineage id.                                                                |
| `trust_tier`                        | One of `verified_publisher`, `community_publisher`, `organisational_publisher`, `unverified_publisher`, `quarantined_publisher`. |
| `delay_window`                      | Lineage transfer delay window (days); transfers fire only after the delay with user / admin notification.                       |
| `audit_event_refs`                  | Ordered list of publisher-continuity audit events (see §Audit events below).                                                    |
| `schema_version`                    | Integer; additive-minor on additions.                                                                                           |

Lineage-transfer rules reserved by this seed:

1. Key rotation, ownership transfer, orphaning, succession, and
   fork adoption each pass through the `delay_window` with a
   user / admin notification and an audit event; no silent
   transfers.
2. A retired publisher's `publisher_identity` is never reused; a
   successor mints a new identity and references the retired row
   via `predecessor_publisher_ref`.
3. Private registries and mirrors MAY apply stricter approval
   layers on top of the default delay window but MAY NOT skip
   the audit event, MAY NOT weaken the signed continuity proof,
   and MAY NOT widen `trust_tier` beyond the source registry's
   declaration.
4. The `quarantined_publisher` tier is a first-class terminal
   state for emergency-disable bundles; every extension under a
   quarantined publisher renders `disabled_by_policy` under the
   ADR-0011 downgrade rule regardless of its own declared state.

### Reserved policy-pack constraint fields

Every admin / managed policy pack that narrows extensions
carries at least one `policy_pack_constraint_row`. The row is a
**typed narrowing record**, not a config blob; tooling reads the
row to compute the effective-permission summary.

Reserved fields:

| Field                               | Notes                                                                                                                         |
|-------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| `policy_pack_id`                    | Opaque policy-pack id.                                                                                                        |
| `policy_epoch`                      | Epoch counter (ADR-0008); constraints are scoped to an epoch so a rolled epoch re-evaluates effective permissions.            |
| `constraint_kind`                   | One of `allow_list`, `deny_list`, `publisher_trust_floor`, `version_pin`, `version_floor`, `permission_floor`, `mirror_rule`, `emergency_disable`, `signed_continuity_required`, `managed_only_narrowing`, `egress_host_narrowing`, `freshness_floor_narrowing`. |
| `constraint_target`                 | Typed target: extension identity set, publisher identity set, permission-scope selector, mirror endpoint set, host-contract family.   |
| `constraint_ceiling`                | Typed ceiling: the narrowed value (e.g. `permission_floor: {filesystem_write: step_up_required}` or `version_pin: "1.4.*"`).   |
| `narrowing_direction`               | Always `narrow_only`; a constraint that would widen beyond the manifest's declared set is denied at policy-pack load.         |
| `source_ref`                        | Ref to the signed policy bundle; raw bundle bytes never carried here.                                                         |
| `audit_event_refs`                  | Ordered list of policy-pack audit events (load, applied, superseded).                                                         |
| `schema_version`                    | Integer; additive-minor on additions.                                                                                         |

### Reviewer-visible surface requirements

Every install / update review sheet, permission inspector,
support-export row, claim-manifest entry, and future SDK
binding MUST project, at minimum, the reviewer-visible fields
below. Chip collapsing is a UI freedom; record addressability is
mandatory (this seed inherits the ADR-0011 rule).

| Field                                        | Required on                                                                                                        |
|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------|
| `declared_permission_set`                    | Review sheet, permission inspector, support export, claim manifest.                                                 |
| `effective_permission_set`                   | Review sheet, permission inspector, support export, claim manifest.                                                 |
| `declared_vs_effective_diff`                 | Review sheet (blocks action on `review_disclosure_incomplete` if hidden), permission inspector, claim manifest.     |
| `transitive_capability_inheritance_summary`  | Review sheet, permission inspector; drill-down view required per ADR-0011 projection rules.                          |
| `publisher_lineage_transfer_summary`         | Review sheet, publisher-continuity packet, support export; names predecessor / successor / fork adoption state.     |
| `successor_or_referrer_metadata`             | Review sheet and publisher-continuity packet; names successor publisher, migration recommendation, orphan marker.   |
| `mirror_and_private_registry_continuity`     | Review sheet, support export, claim manifest; names `registry_source_class`, mirror endpoint, signature re-verify.  |
| `policy_pack_narrowing_summary`              | Review sheet, permission inspector, support export; names every `constraint_kind` that applied and its ceiling.     |
| `extension_lifecycle_state_summary`          | Review sheet, permission inspector, support export; quotes the ADR-0011 five axes and live dependency markers.      |
| `host_contract_and_artifact_transport`       | Review sheet, support export, claim manifest; names `host_contract_family`, `host_contract_identity_ref`, `artifact_transport_family`. |

A review surface that is missing any required reviewer-visible
field MUST deny with the ADR-0011
`review_disclosure_incomplete` denial reason rather than fall
back to a generic "install?" affordance.

### Audit events reserved

The eventual extension-registry crate emits a typed audit stream
on `extension_trust`. This seed reserves at minimum the following
audit-event ids; adding additional events is additive-minor.

- `extension_manifest_indexed`
- `extension_manifest_digest_mismatch`
- `extension_signature_verification_failed`
- `effective_permission_computed`
- `effective_permission_narrowed_by_policy_pack`
- `effective_permission_widening_denied`
- `publisher_key_rotation_started`
- `publisher_key_rotation_completed`
- `publisher_ownership_transfer_started`
- `publisher_ownership_transfer_completed`
- `publisher_orphaned`
- `publisher_succession_recorded`
- `publisher_fork_adopted`
- `publisher_quarantined`
- `publisher_quarantine_lifted`
- `policy_pack_constraint_loaded`
- `policy_pack_constraint_applied`
- `policy_pack_constraint_superseded`
- `mirror_continuity_verified`
- `mirror_continuity_broken`
- `extension_install_review_denied`
- `extension_install_review_disclosure_incomplete`
- `extension_manifest_schema_version_bumped`

Raw signing-key material, raw artifact bytes, raw policy-bundle
bytes, and raw publisher-private data never appear on any of
these events.

### Process-boundary constraints

1. Manifest rows, effective-permission summaries,
   publisher-continuity rows, and policy-pack constraint rows
   cross the RPC boundary as typed payloads (ADR-0004). Raw
   artifact bytes, raw signing-key material, raw policy-bundle
   bytes, and raw publisher-private data never cross.
2. The extension registry is authoritative in the host process;
   the eventual extension runtime reads permission records only
   through the shared subscription envelope (ADR-0005) with
   authority class `derived_knowledge` and a declared freshness
   hint.
3. A credential handle an extension requests projects under
   ADR-0007 projection modes only; raw secret bytes never reach
   the extension process.
4. Remote-agent attach surfaces a remote-scoped extension view
   whose `client_scope` (ADR-0011) includes `remote_agent`; a
   host surface renders a `client_scope_restricted_dependency`
   marker when the remote agent does not surface an extension
   the host otherwise would.
5. Crash dumps and core files MUST NOT inherit unresolved
   effective-permission projections; a crash discards the
   projection rather than persisting a partial set.
6. Mutation-journal entries, save manifests, claim manifests,
   and support bundles carry extension identity, declared-
   permissions digest, effective-permission summary id,
   publisher-continuity row id, and policy-pack constraint row
   ids only; they MUST NOT embed raw artifact bytes, raw
   signing-key material, or raw policy-bundle bytes.

### Denial posture

Failures in extension trust computation fail closed. Denial is
typed, visible, auditable, and repairable. The following denial
reasons are reserved by this seed; additional reasons are
additive-minor.

- `extension_manifest_schema_unknown`
- `extension_signature_verification_failed`
- `extension_digest_mismatch`
- `publisher_quarantined`
- `publisher_lineage_transfer_pending`
- `publisher_trust_tier_below_policy_floor`
- `policy_pack_denies_extension`
- `policy_pack_denies_permission_scope`
- `effective_permission_widening_attempted`
- `host_contract_family_unsupported_on_target`
- `artifact_transport_family_unsupported_on_target`
- `compatibility_bridge_required_not_present`
- `mirror_continuity_broken`
- `mirror_narrowing_attempted_widening`
- `review_disclosure_incomplete`
- `freshness_floor_unmet_for_effective_permission`

Silent downgrade to a generic "not available" or "install
blocked" chip is forbidden; every denial emits the corresponding
audit event with the typed reason.

### Schema-of-record posture

Rust types in an eventual extension-manifest crate will be the
source of truth. The JSON Schema export at
`schemas/extensions/effective_permission.schema.json` is the
cross-tool boundary every non-owning surface reads at this
milestone. The schema carries **placeholder record kinds** for
`extension_manifest_row`, `effective_permission_summary_record`,
`publisher_continuity_row`, and `policy_pack_constraint_row`,
with the vocabulary reserved above pinned as enumerated sets.
Adding a new member to any enumerated set, or an additive field,
is additive-minor with a schema bump; repurposing a member is
breaking and requires a new decision row.

No external IDL or code-generator toolchain at this milestone;
this mirrors ADR 0004 through ADR 0011.

## Consequences

- **Reserved:** the extension-manifest field set, the
  permission-scope vocabulary, the effective-permission summary
  field set, the publisher-continuity field set (with lineage
  states `active`, `key_rotation_in_progress`,
  `ownership_transfer_in_progress`, `orphaned`, `succeeded`,
  `fork_adopted`, `retired`, and trust tiers `verified_publisher`
  through `quarantined_publisher`), and the policy-pack
  constraint field set (with `constraint_kind` members enumerated
  above). Every later lane reads these rather than invent its own.
- **Reserved:** the reviewer-visible field set for install /
  update review, permission inspector, support export, and claim
  manifest surfaces. A surface that hides a required field MUST
  deny with `review_disclosure_incomplete`.
- **Reserved:** the audit-event id set on the `extension_trust`
  audit stream and the denial-reason set for extension-trust
  failures.
- **Reserved:** process-boundary constraints. Raw artifact bytes,
  raw signing-key material, raw policy-bundle bytes, and raw
  publisher-private data never cross RPC. Extension records cross
  as typed payloads.
- **Reserved:** the schema-of-record posture. The eventual
  extension-manifest crate is the source of truth; the boundary
  schema lives at
  `schemas/extensions/effective_permission.schema.json`; no
  external IDL at this milestone.
- **Permitted:** later additive-minor additions to any
  enumerated set (new permission scopes, new host-contract
  families, new artifact transports, new policy-pack constraint
  kinds, new lineage states, new trust tiers, new audit events,
  new denial reasons) with a schema-version bump.
- **Permitted:** admin policy MAY narrow extension trust further
  (permission floors, version pins, mirror rules, emergency
  disable). Policy MUST NOT silently widen.
- **Follow-up:** the successor ADR closes the open questions in
  §Open questions (runtime sandbox model, full WIT world layout,
  compatibility-bridge contract, full SDK binding, per-surface
  reveal-on-demand rules for raw artifact bytes, cross-registry
  federation rules) and promotes this seed's `Proposed` status
  to `Accepted`.
- **Follow-up:** the install-review sheet, the permission
  inspector, the publisher-continuity packet, the policy-pack
  narrowing record, and the mirror-continuity row instrument
  against this vocabulary before the extension runtime lands.
- **Follow-up:** the ADR-0011 capability-lifecycle schema at
  `schemas/governance/capability_lifecycle.schema.json` will
  eventually reference the extension-manifest schema through the
  `manifest_capability_binding_refs` field; this seed reserves
  the link direction without re-freezing ADR-0011.
- **Ratifies:** the ADR-0001 identity modes gate the
  `managed_admin_surface` client scope extensions may surface
  on. ADR-0007 projection modes gate `secret_handle_use`
  permissions. ADR-0008 admin-policy narrowing is the orthogonal
  ceiling that extension policy packs ride. ADR-0010 grant-
  resolution reasons are quoted by `ai_provider_access` and
  `connected_provider_access` permissions. ADR-0011 capability-
  lifecycle and dependency-marker vocabulary is the readiness
  / support / channel / freshness / client-scope projection
  extensions inherit.

## Alternatives considered

- **Defer extension trust vocabulary until the runtime lands.**
  Rejected: the default-if-unresolved narrowing on `D-0018`
  (no manifest field set, no reviewer field set, no publisher-
  continuity vocabulary, no policy-pack constraint vocabulary,
  no declared-vs-effective diff) would force the install-
  review, permission-inspector, support-export, and claim-
  manifest lanes to re-invent extension-shaped fields at
  runtime-landing time; ADR-0011, ADR-0010, ADR-0008, ADR-0007,
  and ADR-0005 would each expose extension-ignorant interfaces
  that later runtime work could not reconcile without a
  compatibility bump.
- **One big "trust" enum (`trusted / untrusted / disabled`).**
  Rejected: collapses publisher continuity, effective
  permission, mirror continuity, and policy-pack narrowing into
  one label; reviewers cannot see why an extension fell to
  "untrusted" or what repair is available.
- **Free-form permission strings rather than an enumerated
  vocabulary.** Rejected: not machine-readable; install review,
  support export, policy packs, and the declared-vs-effective
  diff cannot be computed against free-form strings.
- **Single publisher identity with no lineage record.**
  Rejected: hides key rotation, ownership transfer, orphaning,
  succession, fork adoption, and quarantining. PRD and
  architecture docs require verified workflows with delay and
  audit trails for each of those events.
- **Let each registry invent its own manifest schema.**
  Rejected: the public registry, private registries, mirrors,
  and offline bundles need to preserve digests, signatures,
  compatibility metadata, and permission manifests identically;
  per-registry schemas break that invariant.
- **Embed raw policy-bundle bytes, raw artifact bytes, or raw
  signing-key material in the records.** Rejected: violates
  ADR-0004 RPC rules and ADR-0007 broker rules; this seed
  instead carries typed refs and keeps the bytes in their
  respective registries / stores.
- **External IDL + codegen for manifest / permission /
  publisher payloads.** Rejected: same reasoning as ADR 0004
  through ADR 0011 — no second-language consumer yet; the JSON
  Schema export reserves a clean integration point.

The `D-0018` `freeze_lane` default-if-unresolved posture would
block the install-review, permission-inspector, publisher-
continuity, and mirror-continuity lanes from landing at the
first-beta milestone until a successor ADR lands. Accepting the
seed's `Proposed` status now — with its reserved vocabulary and
invariants — avoids that freeze by giving those lanes the
record fields they need while still leaving the runtime ADR
(sandbox model, full WIT world layout, compatibility bridge,
SDK binding) to close in a successor decision.

## Open questions

These questions MUST be answered by the successor ADR before
this seed is promoted to `Accepted`. They are listed so no later
lane assumes a resolution silently.

1. **Runtime sandbox model.** What are the concrete sandbox
   guarantees for `wasm_component_model`, `wasm_core_module`,
   `external_host_process`, and `helper_binary` host families
   (memory isolation, time-bounded execution, network egress
   enforcement, filesystem path scoping, syscall surface)?
2. **Full WIT world layout.** What is the canonical WIT world
   identity scheme, how do world versions align with manifest
   versions, and how are world additions projected into the
   capability-lifecycle row (ADR-0011)?
3. **Compatibility-bridge contract.** What is the exact profile
   set for `compatibility_bridge` host families, and which
   declared permissions does the bridge honour vs translate vs
   refuse?
4. **Full SDK binding.** How does the stable-surface SDK bind to
   the manifest and the effective-permission summary (binding
   kind per ADR-0011 `sdk_or_api` client scope), and what
   stability-window labels apply?
5. **Reveal-on-demand for raw artifact bytes.** When MAY a
   reviewer inspect raw artifact bytes (e.g. to diff a signed
   artifact against a previously approved version), and under
   which ADR-0007 projection mode?
6. **Cross-registry federation.** What are the rules for
   federating a private registry with a public one (trust-tier
   inheritance, signature re-verification, mirror rewrite
   rules)?
7. **Policy-pack epoch transitions.** How are in-flight installs
   handled when a policy epoch rolls mid-install (ADR-0008
   epoch semantics extended to extensions)?
8. **Orphan-adoption workflow.** What is the exact verified
   workflow for adopting an orphaned publisher (community
   maintainership, corporate adoption, successor minting) and
   the delay-window default?
9. **Mirror-authority relationship.** When a mirror and the
   source registry disagree (e.g. a mirror carries a newer
   signed continuity row), which one wins and how is the
   divergence audited?
10. **Transitive closure depth limits.** Is there a depth limit
    on `capability_inherit` transitive closure, and what
    happens to the effective-permission summary when the
    closure exceeds it?

Each question blocks the `Proposed` -> `Accepted` transition and
is tracked in the `decision_history` of `D-0018`.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — "Users and admins can see the
  effective permission set after dependency resolution, not only
  the top-level extension manifest."
- `.t2/docs/Aureline_PRD.md` — "Publisher key rotation, ownership
  transfer, namespace disputes, and maintainer removal follow a
  verified workflow with delay, audit trail, and user/admin
  notification."
- `.t2/docs/Aureline_PRD.md` — "private registries and offline
  bundles must preserve digests, signatures, compatibility
  metadata, and permission manifests."
- `.t2/docs/Aureline_PRD.md` — "Extensions | Allow/deny lists,
  publisher trust tiers, private registry mirrors, version
  pinning, permission floors."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "canonical extension ID, version, artifact digest, signature
  fingerprint, declared-permissions digest, compatible host ABI,
  and source-registry class."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "Extension host <-> platform SDK | manifest SDK range,
  permission vocabulary version, WIT ABI version."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "Wasm-first contracts map to WIT/component-model worlds."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "Orphaning / succession / fork adoption | abandoned-state
  marker, successor/fork linking, mirror continuity workflow."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "private registries may add stricter approval layers, but they
  must not weaken signed publisher continuity proof."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "Admin policy bundle | enforced settings, network rules, AI
  provider allow/deny, extension policy, mirror endpoints,
  feature posture."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "Emergency disable bundle | extension/provider disable list,
  channel freeze, required minimum version, emergency policy
  narrowing."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  "AD-008 | Extension runtime | Wasm capability sandbox +
  isolated external hosts."
- `.t2/docs/Aureline_Technical_Design_Document.md` — "Install /
  update review sheet | runtime origin, bridge/native state,
  permission diff, helper/executable disclosures, trust-mode
  behavior, rollback path."
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — "transitive
  capability widening from dependencies should be visible in a
  compact summary and in a drill-down view. Users should not
  have to inspect nested manifests manually."
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` — "right
  inspector: publisher continuity, transparency events,
  quarantine/revocation history, managed approval state, and
  install/rollback actions."
- `.t2/docs/Aureline_Milestones_Document.md` — "Capability
  manifest and effective permission view ... effective
  permission set after dependency resolution, runtime host/budget
  class, lifecycle state."

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0018`
- RFC: none (this seed is the first narrative; a successor RFC
  may run down the open-question option space before the
  successor ADR lands).
- Effective-permission boundary schema (placeholder):
  `schemas/extensions/effective_permission.schema.json`
- Worked example manifest row:
  `fixtures/extensions/manifest_examples/declared_vs_effective_example.yaml`
- Capability-lifecycle vocabulary extensions inherit:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
- Connected-provider vocabulary quoted by provider-linked
  permissions:
  `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`
- Settings resolver whose admin-policy narrowing posture
  extensions extend:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
- Secret-broker projection modes `secret_handle_use`
  permissions cite:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
- Subscription envelope the permission views ride:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
- Identity-mode envelope that gates `managed_admin_surface`
  client scope:
  `docs/adr/0001-identity-modes.md`
- Affected lanes: `governance_lane:compatibility_ecosystem_review`,
  `governance_lane:security_trust_review`,
  `governance_lane:docs_public_truth`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance (as a seed at `Status: Proposed`). A successor
ADR promotes this seed to `Accepted` once the open questions are
closed, and records the supersession in this section without
rewriting the body above.
