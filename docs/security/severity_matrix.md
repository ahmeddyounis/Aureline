# Security-response severity matrix

This document is the pre-implementation severity matrix for Aureline's
security-response posture. It exists so every incident report,
advisory artifact, emergency-action record, revocation record, and
support-export entry has one frozen vocabulary to resolve against
before any incident-response tooling, live on-call lane, or
emergency-action UI lands.

Companion artifacts:

- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  — boundary schema for the `advisory_record` every public advisory,
  release-note, About / Help surface, support-bundle export, and
  revocation / disable-bundle record resolves against field-for-field.
- [`/docs/security/advisory_surface_contract.md`](./advisory_surface_contract.md)
  and
  [`/schemas/security/advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json)
  — surface contract for advisory cards, emergency banners, revocation
  notices, and disclosure links. This is the rendering / export projection
  of the advisory, emergency-action, revocation, and manual-import records;
  it does not replace those source records.
- [`/schemas/security/incident_workspace_packet.schema.json`](../../schemas/security/incident_workspace_packet.schema.json)
  — boundary schema for the `incident_workspace_packet_record` the
  private-triage workspace exports for evidence capture, timeline,
  continuity notes, and handoff / export routing.
- [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  and
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  — emergency-action and revocation model the advisory and incident
  packet now link into for freeze, kill-switch, trust-root, revocation,
  mirror/manual-import, and local-continuity truth.
- [`/fixtures/security/advisory_examples/`](../../fixtures/security/advisory_examples/)
  — worked fixtures covering at least one alias-ready advisory (CVE +
  GHSA + Aureline advisory id) with an affected-install assessment
  stub tied to exact-build identity refs, and one incident packet
  that exercises omitted / embedded / redacted / by-reference
  evidence items while preserving exact-build linkage.
- [`/fixtures/security/advisory_cases/`](../../fixtures/security/advisory_cases/)
  — worked advisory-surface fixtures covering staged disclosure, active
  emergency disable, mirror-only advisory behavior, and superseded
  advisory history chains.

Normative sources this matrix projects from:

- `.t2/docs/Aureline_PRD.md` (security-response obligations, advisory
  discipline, and revocation language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §3.2
  (trustworthiness and safety architecture driver) and §4.1
  (protected principles including signed artifacts and least
  privilege).
- `.t2/docs/Aureline_Milestones_Document.md` (release publication,
  provenance, attestation, advisory, and revocation discipline).
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  for redaction defaults every evidence export inherits.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  for the exact-build identity an advisory names when it says which
  build is affected.
- [`/docs/release/install_topology_plan.md`](../release/install_topology_plan.md)
  for the install-profile card, channel, and publication-posture
  vocabulary an affected-install assessment resolves against.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  for the `security_issue` class (private security channel, private-
  with-public-advisory privacy, `public_on_advisory` disclosure,
  `security_trust_review` forum).
- [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  for the default quorum and break-glass action ids that govern
  channel-freeze, revocation, disable-bundle, kill-switch, and
  high-severity publication actions until the emergency-action schema
  lands.

## Why publish this now

Before any incident response is written, Aureline needs to freeze:

- **Severity levels.** One closed vocabulary every incident, advisory,
  release-note, About / Help notice, and support-export row resolves
  against, so no surface silently invents a "medium-critical" label.
- **Owner and response clock per severity.** So a reporter receiving a
  monitored acknowledgement can tell, without reading a runbook, how
  long triage, mitigation, and public advisory take.
- **Minimum review forum per severity.** So the forum quorum for
  closing an advisory is explicit and mechanical — there is never a
  single-person close on a critical advisory, and the release council
  is always co-required for any publication that affects a signed
  artifact.
- **Documentation / update obligation per severity.** So an advisory
  without a public summary cannot silently ship, and a revocation or
  disable-bundle is pinned to a severity tier rather than left to
  case-by-case judgment.
- **One advisory identity model.** So CVE, GHSA, and Aureline advisory
  aliases resolve into a single record, not three parallel tables, and
  later disclosure, revocation, and disable-bundle artifacts do not
  invent incompatible IDs.
- **One affected-install assessment linkage.** So incident, advisory,
  support, and update surfaces all resolve "which install lanes are
  affected" from the same install-profile card, exact-build identity,
  channel, publication posture, and mirror-freshness class.
- **One evidence-embedding vocabulary.** So an incident packet can
  distinguish omitted, embedded, redacted, and by-reference evidence
  items without the support-export or release-evidence lane having
  to guess.

Left implicit, every surface would re-invent a dialect: support
bundles would carry a different severity label than release notes,
revocation records would embed a different affected-build handle than
the advisory, and incident packets would quietly include raw secrets
because the evidence-embedding state was not typed. Freezing the
vocabulary now — **before** any incident-response tooling or
emergency-action UI / state model lands — ends those failure modes.

This is a **pre-implementation plan**. No incident-response
automation, live on-call lane, or emergency-action state model is
implemented at this milestone. Every row, vocabulary value, and
reserved field is tagged in the companion schemas; rows are not
deleted, they are superseded.

## Scope

In scope for this revision:

- A closed severity vocabulary (`security_severity_class`) with owner,
  response-clock, minimum-review-forum, and documentation / update
  obligation per level.
- The `advisory_record` shape every public advisory, release-note, and
  support export reads: advisory identity (Aureline advisory id plus
  CVE and GHSA aliases), subject-kind, affected subject refs, signer /
  source state, affected-install linkage, disclosure class,
  deployment-profile scope, evidence refs with embedded / by-reference
  / redacted / omitted flags, and reserved hooks into emergency-action,
  revocation, and disable-bundle artifacts.
- The `incident_workspace_packet_record` shape the private-triage
  workspace exports: evidence-capture items with typed embedding state,
  redaction class, timeline fields, artifact refs, continuity notes,
  handoff / export routing, deployment-profile scope, and a monitored
  SECURITY.md contact path anchor.
- One monitored SECURITY.md contact-path ref (`security_md_monitored_contact`)
  that both the advisory and the incident packet resolve against so
  reports never land in an untracked inbox.
- Reserved alias slots for CVE, GHSA, and Aureline advisory ids plus a
  closed `advisory_subject_kind` enum so later disclosure, revocation,
  and disable-bundle artifacts inherit the identity without inventing
  parallel keys.
- Reserved `affected_install_assessment` linkage fields naming install-
  profile card refs, exact-build identity refs, channel classes, mirror
  freshness class, and local-continuity notes so incident, advisory,
  support, and update surfaces resolve against one artifact-identity
  model.

Out of scope until a superseding decision row opens:

- Production incident-response automation, paging, or on-call tooling.
- Live on-call rotation assignments or escalation trees.
- Final emergency-action UI behaviour beyond the object model, shared
  field set, and linkage refs frozen here and in
  [`/docs/security/emergency_action_model.md`](./emergency_action_model.md).
- Disable-bundle artefact bytes. The advisory record reserves the
  linkage ref (`disable_bundle_refs`); the disable-bundle schema is a
  later lane co-owned by security / trust review and release council.
- Revocation-record bodies. The advisory record reserves the linkage
  ref (`revocation_refs`); the revocation schema is a later lane.
- CVSS or other quantitative scoring systems. The severity class is a
  closed label every downstream surface uses; attaching a CVSS vector
  is additive-minor and requires a companion schema bump when it
  lands.
- Coordinated-disclosure timelines with outside parties. The incident
  packet reserves the `private_triage_workspace_scope` enum with a
  `coordinated_disclosure_group` value; the scheduling and party list
  are runbook content.

## Severity vocabulary

Closed set. Adding a new severity class is additive-minor and requires
a new decision row in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
co-signed by `security_trust_review` and `release_council`. Every
advisory record and incident packet MUST carry exactly one severity
class from this set.

| Severity class                           | Summary                                                                                                                                      | Owner DRI lane         | Minimum co-review forum | Acknowledge target                       | Triage target                           | Fix / mitigation target                                  | Publication / update obligation                                                                                                                                                      |
|------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|------------------------|-------------------------|-------------------------------------------|------------------------------------------|-----------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `security_severity.operational_emergency`| Signing-key compromise, malicious official update, verified remote-code-execution worm, trust-root rotation, or a forced-disable / channel-freeze event that changes what users can safely run. | `security_trust_review`| `release_council`       | immediate; no business-window delay       | within 24 hours of monitored acknowledgement | immediate containment, channel freeze / pause, or trust-root action; trusted recovery path published before channel resume | Out-of-band emergency notice required; signed advisory required; recovery guide required; emergency-action, revocation, or disable-bundle hook required; About / Help and update-center notice required. |
| `security_severity.critical`             | Confirmed code exfiltration, signed-artifact-chain compromise bounded below the emergency tier, or a flaw that compromises a shipped signed artifact family on an active release lane. | `security_trust_review`| `release_council`       | within 24 hours                           | within 72 hours                          | within 7 days or a compensating control                  | Signed advisory draft required; public advisory required on fix or mitigation; CVE / GHSA preparation required; release note required; About / Help notice required; revocation or disable-bundle hook required when the signed install lane is affected. |
| `security_severity.high`                 | High-severity privilege escalation, workspace-trust bypass, credential-broker projection bypass, or a flaw affecting a signed artifact family on at least one channel with bounded exploitability. | `security_trust_review`| `release_council`       | within 24 hours                           | within 5 days                            | within 30 days                                           | Advisory record and operator guidance required; public advisory expected at disclosure; compatibility / risk note and backport decision required; release note required; About / Help notice required for installed signed artifacts. |
| `security_severity.medium`               | Sandbox narrowing, partial redaction bypass with no raw-secret egress, preview-only exploit, or a flaw that requires user interaction and does not reach the signed-artifact chain. | `security_trust_review`| none                    | within 3 business days                    | within 10 days                           | within 90 days                                           | Tracked advisory item, release-note linkage, and mitigation guidance required; public advisory at disclosure when the `security_issue` route remains active; revocation and disable-bundle hooks optional. |
| `security_severity.low`                  | Defense-in-depth fix, hardening improvement, or an issue whose mitigation is already default-on and whose blast radius does not justify an advisory-center interrupt. | `security_trust_review`| none                    | within 5 business days                    | within 20 days                           | next appropriate release                                 | Issue classification and fix tracking required; release note or public guidance as needed; revocation, disable-bundle, and emergency-action hooks forbidden at this level. |

`security_severity.operational_emergency` maps to the PRD's
`S0 / emergency` lane; `security_severity.critical` maps to the PRD's
`S1 / critical` lane.
Design-system telemetry that uses `advisory_severity` maps
`security_severity.medium` to `moderate` and keeps the other labels
verbatim.

Rules:

- **Never single-forum on operational-emergency, critical, or high.**
  An advisory at `security_severity.operational_emergency`,
  `security_severity.critical`, or `security_severity.high` MUST have
  at least one co-signing forum action from the release council
  recorded on the advisory's decision trail. Single-forum close is
  non-conforming at these levels because a signed artifact, a trust
  root, or the signing chain may be involved.
- **Emergency never waits for the next forum slot.** A
  `security_severity.operational_emergency` record MAY open with an
  audited break-glass action, but release-council co-sign and
  reconciliation MUST land on the decision trail within the emergency
  triage target. The action still cites the applicable break-glass row
  from
  [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml);
  "single responder" is a bounded exception path, not a separate
  approval system.
- **Never silent publication on medium or higher.** An advisory at
  `security_severity.medium`, `security_severity.high`,
  `security_severity.critical`, or
  `security_severity.operational_emergency` MUST carry a non-null
  `public_summary_ref` at publication time. Releasing a fix for a
  medium-or-higher finding with no public advisory is non-conforming
  and blocks the release-evidence gate.
- **Never emergency hooks on low.** An advisory at
  `security_severity.low` MUST NOT populate `emergency_action_refs`,
  `revocation_refs`, or `disable_bundle_refs`. If a finding is
  reclassified upward, the advisory id is preserved and a new decision
  row records the severity change.
- **Clocks start at monitored acknowledgement.** Every clock above
  starts when the monitored SECURITY.md contact path acknowledges the
  report (see "Monitored contact path" below). An incident that never
  reaches the monitored contact is a governance failure logged against
  the `security_trust_review` lane, not a reason to stop the clock.
- **No backdating.** Severity downgrades and clock extensions are
  recorded as decision-history rows on the advisory, never by
  mutating prior timestamps.

## Monitored contact path

Every advisory record and incident workspace packet MUST reference the
monitored SECURITY.md contact path (`security_md_monitored_contact`).

The planned contact path is `/SECURITY.md`. That file has not landed
yet at this milestone, so the schemas carry the contact ref as the
reserved opaque id `security_md_monitored_contact`; no advisory is
publishable without resolving that ref to a monitored path.

Rules:

- The monitored contact is a private route by default: reports land in
  the private security channel defined by the
  `security_issue` row in
  [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  (`default_route_class: private_security_channel`,
  `privacy_class: private_with_public_advisory`,
  `disclosure_class: public_on_advisory`,
  `owning_forum_placeholder: security_trust_review`).
- The monitored contact is monitored. Unmonitored drop-boxes,
  unattended shared mailboxes, or chat channels that nobody is
  on-call for are non-conforming contact refs.
- The monitored contact path sets the acknowledge clock. When a report
  is received, the timestamp of the acknowledgement event is the one
  the severity-clock rules above project from.

## Private-triage workspace

Every incident workspace packet carries a `private_triage_workspace_scope`
value drawn from a closed enum:

- `private_security_channel` — default private triage workspace for
  Aureline's own reports. Raw evidence may circulate inside this scope.
- `private_partner_channel` — triage workspace shared with a partner
  under a separate agreement. Raw evidence may circulate only under
  the partner's written consent per the `private_partner_case` row in
  `issue_routing.yaml`.
- `coordinated_disclosure_group` — triage workspace shared with an
  outside party (upstream maintainer, embargo coordinator, reporter-
  side coordinator) under a coordinated-disclosure agreement.
- `vendor_only` — triage workspace scoped to Aureline maintainers
  only, with no outside participant.

A packet that cannot resolve its scope to one of the above is
non-conforming; `redaction_class: private_triage_only` is admissible
only when a packet is inside one of these scopes.

## Advisory identity and subject kind

Every advisory record carries an `advisory_identity` envelope with
three alias fields reserved at the schema level:

- The machine boundary for the shared advisory-ID family is
  [`/schemas/security/advisory_identity.schema.json`](../../schemas/security/advisory_identity.schema.json).

- `aureline_advisory_id` — the stable Aureline advisory id. Required.
  Minted locally and stable for the life of the advisory even when
  the finding is reclassified.
- `cve_id` — the CVE id (`CVE-YYYY-NNNN+`). Nullable; populated when a
  CVE is assigned.
- `ghsa_id` — the GHSA id (`GHSA-xxxx-yyyy-zzzz`). Nullable; populated
  when a GHSA is minted.
- `additional_alias_refs` — optional list of opaque alias refs for
  future disclosure schemes. Reserved so later schemes do not add a
  new top-level field; they land under this array.

Every advisory record also carries a closed `advisory_subject_kind`
enum so the identity resolver knows which artefact family is the
subject:

- `build_binary` — an `ide_binary` / `cli_binary` / `sdk_library`
  artifact family from the exact-build identity model.
- `build_symbol` — a `*_debug_symbols` / `source_map_bundle` / symbol
  archive artifact family.
- `docs_pack` — a `docs_pack_manifest_record` pack revision.
- `extension` — an installed extension (third-party or first-party).
- `capability_gate` — a runtime capability gate or workspace-trust
  decision point.
- `settings_definition` — a setting definition affecting security
  behaviour.
- `secret_class_posture` — a redaction or secret-class rule set.
- `signing_material` — signing material (HSM-backed, ephemeral
  developer, CI transparent, release production).
- `install_profile_card` — an install-profile card row from the
  install-topology matrix.
- `runtime_component` — an in-process runtime component (renderer,
  buffer, VFS, RPC, secret broker, search readiness, context
  assembly).
- `managed_cloud_surface` — a managed-cloud surface on the boundary
  manifest.
- `third_party_dependency` — an external dependency surfaced by a
  transitive finding.
- `workspace_trust_policy` — workspace-trust rules or identity-mode
  posture.
- `ai_context_assembly` — an AI context-assembly rule or a tainted-
  content fence posture.

A record whose subject cannot be typed fails closed on
`subject_kind_unresolved` rather than defaulting.

## Affected-install assessment linkage

The advisory record reserves an `affected_install_linkage` envelope so
the same vocabulary that install, support, and update surfaces read
applies to the advisory's "which install lanes are affected?" claim:

- The per-install local assessment record (build/channel/install-mode +
  mitigation state + mirror freshness) is defined in
  [`/schemas/security/affected_install_assessment.schema.json`](../../schemas/security/affected_install_assessment.schema.json)
  and described in
  [`/docs/security/advisory_identity_and_install_assessment_contract.md`](./advisory_identity_and_install_assessment_contract.md).

- `install_profile_card_refs` — opaque refs into
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml).
  An advisory that affects one install mode on one channel lists one
  ref; one that affects every shipped card lists every ref.
- `exact_build_identity_refs` — opaque refs into
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json).
  The advisory names the precise builds it applies to; free-text
  "version 1.x" is non-conforming.
- `channel_classes` — the channels the advisory affects, drawn from
  the exact-build identity model's `release_channel_class` enum
  (`dev_local`, `nightly`, `preview`, `beta`, `stable`, `lts`,
  `hotfix`).
- `publication_posture_classes` — drawn from the install-topology
  plan's `publication_posture_class` enum (`online_vendor`,
  `offline_signed_bundle`, `customer_managed_mirror`,
  `third_party_package_index`).
- `mirror_freshness_class` — one of `up_to_date`, `stale_within_grace`,
  `stale_past_grace`, `offline_expired`, `unknown`. Used so mirror
  and air-gap lanes know whether their local mirror carries the fix.
- `local_continuity_note` — a short, reviewable sentence describing
  what still works locally if the admin disables the affected
  install-profile card before the fix is available. Pairs with the
  `absence_narrows_to` field on the boundary manifest.

A `security_severity.operational_emergency`,
`security_severity.critical`, or `security_severity.high` advisory
with an empty
`affected_install_linkage.install_profile_card_refs` or empty
`affected_install_linkage.exact_build_identity_refs` is non-conforming.
A medium or low advisory MAY carry an empty install-profile card
list when the finding is channel-agnostic (for example a docs-pack
hardening), but MUST still carry at least one exact-build identity
ref so a verifier can tell which cut observed the finding.

## Evidence embedding vocabulary

Every `incident_workspace_packet_record.evidence_items[]` entry carries
exactly one `embedding_state` value:

- `omitted` — the evidence item is named but no body is included. The
  packet records the class, the rationale, and a pointer back to the
  private-triage workspace record that retains it.
- `embedded` — the evidence body is embedded in the packet. Only
  admissible under `redaction_class: private_triage_only` or
  `redaction_class: evidence_packet_only` and only for non-secret
  classes.
- `redacted` — the evidence body is embedded after the broker-owned
  redaction pass frozen in ADR-0007. The packet carries the
  redaction-pass class and a placeholder (e.g. `<redacted:
  ai_provider_token>`) in place of every secret-bearing span.
- `by_reference` — the evidence body lives in another artifact
  (support bundle, release-evidence packet, private-triage workspace
  note). The packet carries an opaque reference ref and names the
  resolver lane; no body is embedded.

A packet that asserts `embedded` but does not carry a body, or
asserts `redacted` but does not name a redaction-pass class, is
non-conforming. A packet that asserts `by_reference` without a
non-null `reference_ref` is non-conforming.

## Handoff and export routing

Every incident packet carries a typed `handoff_routing` class and a
typed `export_routing` class so a downstream consumer knows where the
packet is authoritative:

`handoff_routing` closed set:

- `within_private_triage` — the packet stays in the private-triage
  workspace. No outside export.
- `to_release_council` — the packet hands off to the release council
  for advisory publication or release gating.
- `to_support_export` — the packet hands off to the support-export
  lane, inheriting that lane's redaction defaults.
- `to_coordinated_disclosure_group` — the packet hands off to the
  coordinated-disclosure group. Pairs with
  `private_triage_workspace_scope: coordinated_disclosure_group`.
- `to_partner_channel` — the packet hands off to a partner channel
  under the partner's agreement.

`export_routing` closed set:

- `no_export` — the packet is not exported. Raw evidence stays inside
  the workspace.
- `export_embedded_in_advisory` — the packet exports under the
  advisory record's `evidence` array with `embedding_state:
  embedded` or `embedding_state: redacted` only. Admissible only at
  `redaction_class: evidence_packet_only`.
- `export_by_reference_from_advisory` — the packet exports under the
  advisory record's `evidence` array with `embedding_state:
  by_reference` only. No embedded body.
- `export_to_support_bundle` — the packet's non-raw fields export
  into the support-bundle family. Raw evidence is never included.
- `export_to_release_evidence_packet` — the packet's non-raw fields
  export into a release-evidence packet for advisory publication.

A packet with `handoff_routing: within_private_triage` and
`export_routing != no_export` is non-conforming.

## Deployment-profile scope

Every advisory record and incident packet carries a
`deployment_profile_scope` array drawn from the boundary-manifest
deployment-profile enum (`individual_local`, `self_hosted`,
`enterprise_online`, `air_gapped`, `managed_cloud`). The scope names
which deployment profiles the finding applies to. A finding that
affects every profile lists every value; a finding that is
profile-specific lists the subset.

A `security_severity.operational_emergency`,
`security_severity.critical`, or `security_severity.high` advisory
with an empty
`deployment_profile_scope` is non-conforming. The scope composes with
`affected_install_linkage`: the scope is the "who feels this" view,
the install-profile cards are the "which install lanes observe it"
view.

## Linkage into other control artifacts

- **Exact-build identity.** The advisory record's
  `affected_install_linkage.exact_build_identity_refs` resolves into
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json).
  An advisory that cannot name the exact build it applies to is
  non-conforming.
- **Install topology.** The advisory record's
  `affected_install_linkage.install_profile_card_refs` resolves into
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml).
  An advisory that names an install-profile card outside the matrix
  fails closed.
- **Secret broker.** Redaction defaults on advisory evidence and
  incident packets inherit ADR-0007 vocabulary through two linked
  layers: record-level redaction classes
  (`support_export_only`, `evidence_packet_only`, `release_public`,
  `private_triage_only`) and broker-owned redaction-pass classes
  (`broker_default_support_bundle`,
  `broker_default_evidence_packet`,
  `broker_default_logs_local`). `private_triage_only` is admissible
  only inside a named private-triage workspace scope.
- **Issue routing.** The advisory record's disclosure posture is
  governed by the `security_issue` row in
  [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml).
- **Decision register.** Severity reclassification and advisory
  publication are decision-register events recorded in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
  A severity upgrade without a decision-history row is non-conforming.
- **Emergency action.** The advisory record's `emergency_action_refs`
  now resolve into
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  `emergency_action_record` ids. The shared object model lives in
  [`/docs/security/emergency_action_model.md`](./emergency_action_model.md)
  and reuses the same affected-install linkage, deployment-profile
  scope, and continuity vocabulary this document freezes.
- **Approval quorum.** The action classes, quorum floor, and
  break-glass rules for `emergency_action_refs`, `revocation_refs`, and
  disable/kill-switch publication are those published in
  [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml).
- **Revocation.** The advisory record's `revocation_refs` now resolve
  into
  [`/schemas/security/emergency_action_record.schema.json`](../../schemas/security/emergency_action_record.schema.json)
  `revocation_record` ids. Revocation stays on the same artifact
  identity graph as advisories and emergency actions; it no longer
  relies on a future placeholder schema.
- **Disable bundle.** The advisory record reserves
  `disable_bundle_refs` for opaque disable-bundle ids. The disable-
  bundle artefact is a later lane.

## Change control

- Adding a `security_severity_class`, `advisory_subject_kind`,
  `private_triage_workspace_scope`, `handoff_routing` class,
  `export_routing` class, `mirror_freshness_class`, or
  `embedding_state` value is additive-minor. Adding an entry requires:
  bumping the matching schema version, extending the schema and this
  document in the same change, and co-signing by `security_trust_review`
  and `release_council`.
- Repurposing an existing vocabulary value is breaking. Repurposing
  requires a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and the concurrence of both councils.
- Changing the response-clock window on any severity level is
  security / trust review authority with release-council co-sign;
  changes are recorded as decision-history rows on the linked
  decision.

## Next-milestone expectations

- Seed a real `/SECURITY.md` monitored-contact file that every
  `security_md_monitored_contact` ref resolves into, with a published
  PGP key / signed-contact anchor and a documented acknowledgement
  clock.
- Open decision rows that bind the severity clocks above to named
  release-council SLAs once the release-cadence decision (D-0010)
  closes.
- Land the disable-bundle transport/payload schema under
  `/schemas/security/` so `disable_bundle_refs` stops being the only
  remaining reserved slot in the security-response object family.
- Open the coordinated-disclosure runbook referenced by the
  `coordinated_disclosure_group` scope value. The schema reserves the
  scope; the runbook content is support / security authority.
