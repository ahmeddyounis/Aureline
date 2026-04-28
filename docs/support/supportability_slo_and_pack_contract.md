# Supportability SLO and Evidence-Pack Contract

This contract binds support evidence packs, first-actionable-diagnosis
targets, redaction checks, release-candidate drills, and waiver paths
into one measurable supportability model. It complements the support
bundle, Project Doctor, diagnostic artifact matrix, and reconstruction
drill contracts without replacing their schemas.

Companion artifacts:

- [`/artifacts/support/diagnosis_slo_targets.yaml`](../../artifacts/support/diagnosis_slo_targets.yaml)
  - machine-readable diagnosis, accuracy, escalation, and drill targets.
- [`/artifacts/support/redaction_accuracy_checks.yaml`](../../artifacts/support/redaction_accuracy_checks.yaml)
  - seeded secret fixtures and continuous redaction-quality checks.
- [`/fixtures/support/drill_scenarios/`](../../fixtures/support/drill_scenarios/)
  - release-candidate drill packets for extension, toolchain, network,
  and renderer/trace escalation scenarios.
- [`/artifacts/support/support_evidence_pack_matrix.yaml`](../../artifacts/support/support_evidence_pack_matrix.yaml)
  - item-level inclusion, redaction, consent, size, and retention matrix.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  - support-bundle manifest and redaction-profile contract.
- [`/docs/support/project_doctor_packet.md`](./project_doctor_packet.md)
  - Project Doctor scenario, finding, repair, and completeness contract.
- [`/docs/support/reconstruction_drill.md`](./reconstruction_drill.md)
  - post-export reconstruction drill.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` section 10.22 and Appendix Z.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections
  24.4, 24.8, Appendix I, and Appendix DN.
- `.t2/docs/Aureline_Milestones_Document.md` sections 12.1.2,
  12.1.4, and release-readiness supportability thresholds.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 18.10, 18.21,
  23.35, and Support Center patterns.

If this document conflicts with a schema, the schema controls packet
shape. If it conflicts with `diagnosis_slo_targets.yaml` or
`redaction_accuracy_checks.yaml`, those artifacts control target values
and check rows.

## Contract

Supportability is promotion-grade only when a reviewer can answer all
of the following from current artifacts:

1. Which evidence-pack class was assembled.
2. Which support item ids were included, omitted, redacted, retained
   locally, or left as optional uploads.
3. Which exact-build, docs-pack, route, profile, and policy refs join
   the pack to the failing product state.
4. Which user or admin consent steps allowed any code-adjacent,
   trace-bearing, crash, or high-risk artifact to leave the machine.
5. Whether Project Doctor produced the first actionable diagnosis within
   the published target for the scenario family and claimed profile.
6. Whether redaction fixtures prove secret absence, manifest honesty,
   and preview/reopen parity.
7. Which owner lane, release gate, or waiver path applies when a target
   misses.

An evidence pack is not promotion-grade if it is only a zip file, if it
cannot be reopened locally with the same redaction manifest, or if it
requires raw source upload before support can identify the first
actionable next step.

## First Actionable Diagnosis

First actionable diagnosis is the first Project Doctor finding,
support-center finding, or headless diagnosis record that contains all
of these fields:

| Required field | Minimum meaning |
|---|---|
| `finding_code` | stable finding or typed insufficient-evidence code |
| `confidence_class` | observed, inferred, or unknown with remaining-unknowns |
| `scenario_family` | stable scenario or drill family |
| `next_action_class` | safe repair, no-touch observation, or escalation packet |
| `evidence_refs` | exact-build plus relevant support item or packet refs |
| `redaction_state` | visible included, omitted, and redacted classes |

The primary target is p90 under 600 seconds for common extension,
toolchain, and network/proxy/certificate failures. The timer starts at
`probe_run_started` or the support-scenario start event and stops when
the first diagnosis record above is emitted on desktop, safe mode, or
headless surfaces.

Release review treats diagnosis latency misses as follows:

| State | Rule |
|---|---|
| Green | p90 is at or below the target and required scenario coverage is current |
| Yellow | p90 is more than 20 percent over target, or one non-blocking drill is aging |
| Red | p90 is more than 50 percent over target, any protected repair/export drill is red, or a claimed profile has no current drill |

The target values and scenario bindings live in
[`diagnosis_slo_targets.yaml`](../../artifacts/support/diagnosis_slo_targets.yaml).

## Evidence-Pack Classes

Pack classes are named inclusion contracts over support item ids. They
do not replace `support_pack_item_id`; every included or omitted item
still cites the item matrix.

| Pack class | Default purpose | Required item ids | Consent posture |
|---|---|---|---|
| `baseline_metadata_pack` | every manually generated support bundle and issue handoff | `support.item.build_identity`, `support.item.policy_trust_state`, `support.item.docs_inventory` when docs truth is in scope | metadata export only; no extra consent |
| `diagnosis_context_pack` | Project Doctor and support-center diagnosis | baseline plus `support.item.execution_context_summary`, relevant `support.item.extension_inventory` or `support.item.connection_profiles`, and Doctor finding refs | bundle preview reviewed |
| `trace_escalation_pack` | renderer, IPC, LSP, DAP, or timing escalation | diagnosis context plus `support.item.runtime_traces` manifest and local retention ref | trace bodies remain local unless explicit trace opt-in is recorded |
| `crash_symbolication_pack` | crash, core, or exact-build symbolication handoff | baseline plus crash envelope, dump manifest, symbolication refs, and optional `support.item.crash_dump_or_core` ticket | raw dump/core bytes require high-friction opt-in or policy |
| `user_selected_context_pack` | bounded code, notebook, or mutation context selected by the user | diagnosis context plus `support.item.user_selected_code` selection refs | exact selected range or cell opt-in only |
| `absence_proof_manifest` | proof that prohibited classes were not exported | `support.item.raw_secrets` and `support.item.full_shell_history` omission markers | no consent path for raw values or full shell history |

Pack assembly must use the narrowest class that explains the failure.
Adding a broader class requires a visible reason in the preview
manifest and a consent or policy row where the item matrix requires one.

## Preview and Reopen Manifests

Supportability quality is measured before export and after export.
Every pack class above reserves both manifests.

The preview manifest is created before export and must include:

- pack class and scenario family;
- exact-build identity refs and docs-version match state;
- selected, omitted, redacted, local-only, by-reference, and optional
  upload item rows;
- redaction profile id, redaction summary id, and seeded check refs;
- user/admin consent steps, including denied or deferred choices;
- destination/share boundary and retention class.

The reopen manifest is created with the exported packet and must include:

- preview manifest digest and producer build;
- support-bundle manifest digest;
- item ids and redaction summary refs sufficient to reconstruct the
  original preview decision;
- retained-local refs or typed expiration notes for local-only artifacts;
- optional-upload ticket refs that were not fulfilled;
- reconstruction steps and typed gaps if a reviewer cannot follow the
  exact-build, route, docs, or claim joins.

Local review, redaction review, export, and post-export reconstruction
therefore remain one measurable contract family instead of separate
support promises.

## Redaction Quality

Redaction is a supportability SLO, not a privacy side note. Continuous
checks seed fake secret, path, host, token, certificate, and shell-history
markers into supported artifact classes and require:

- zero raw seeded secret values in exported packets;
- 100 percent redaction-marker coverage for seeded secret fixtures;
- 100 percent omission-manifest coverage for prohibited item ids;
- preview/reopen manifest parity for included, redacted, omitted, and
  local-only rows;
- no policy-driven expansion beyond documented data-class defaults.

The concrete checks, fixture labels, and artifact class bindings live in
[`redaction_accuracy_checks.yaml`](../../artifacts/support/redaction_accuracy_checks.yaml).

## Release-Candidate Drills

Every claimed release-candidate profile must run at least one current
supportability drill. Across the active candidate set, drills must cover:

- extension regression and quarantine or bisect;
- stale toolchain or execution-context drift;
- proxy, certificate, or blocked egress failure;
- renderer, IPC, or trace-capture escalation.

At least one drill per claimed operating-system profile must run through
safe mode or headless mode so supportability does not depend on a fully
healthy desktop shell. A profile drill is current only while the
support-scenario quality proof is within its freshness SLO.

## Owners, Gates, and Waivers

The default owner lane for this contract is `support_export`.
Co-review lanes are determined by scenario:

| Scenario area | Co-review lane |
|---|---|
| Extension regression | `compatibility_ecosystem_review` |
| Toolchain and execution context | `product_scope_review` and owning runtime lane |
| Proxy, certificate, transport, or trust | `security_trust_review` |
| Renderer, IPC, trace capture, or crash | owning runtime lane plus `release_evidence` |
| Redaction checks | `security_trust_review` plus `support_export` |

Release gates consume the current artifacts through the support-scenario
quality proof class and support handoff output. A release candidate is
blocked when:

- a claimed profile lacks a current supportability drill;
- redaction checks leak a seeded secret or omit an omission marker;
- first-actionable diagnosis misses the red threshold;
- an escalation packet is incomplete for a scenario that requires human
  handoff;
- a waiver is missing, expired, ownerless, or broader than the affected
  scenario/profile row.

A waiver must name the affected SLO/check/drill row, owner lane,
expiration, narrowed support claim, compensating evidence, and rerun
trigger. It must not silently widen data collection, bypass redaction,
or relabel an unsafe repair as safe.
