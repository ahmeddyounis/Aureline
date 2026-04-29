# Project Doctor probe catalog, finding explainability, and repair handoff contract

This contract publishes the machine-readable Project Doctor probe
catalog and finding explanation packet used by Doctor, safe mode,
support bundles, help, runbooks, and escalation packets. It narrows the
broader Project Doctor support-scenario packet into two concrete
records:

- a `probe_catalog_entry_record` for what a probe may read, how fast it
  should answer, what evidence it produces, and whether Doctor is
  allowed to run it;
- a `doctor_explanation_packet_record` for why a finding fired, which
  evidence backed it, which contracts are affected, and which governed
  next actions are valid.

The contract is intentionally support-first. It does not implement live
probes or repair automation; it freezes the packet shapes those
systems must emit once implemented.

## Companion Artifacts

- [`/schemas/support/probe_catalog_entry.schema.json`](../../schemas/support/probe_catalog_entry.schema.json)
  defines one catalog row with mutability class, prerequisites,
  expected latency, evidence produced, local or remote support class,
  redaction class, failure-mode handling, and side-effect admission.
- [`/schemas/support/doctor_explanation.schema.json`](../../schemas/support/doctor_explanation.schema.json)
  defines one finding explanation and repair-handoff packet.
- [`/fixtures/support/project_doctor_cases/`](../../fixtures/support/project_doctor_cases/)
  contains seed packets proving read-only probe admission, mutating
  probe promotion, and finding-to-repair handoff.
- [`/docs/support/project_doctor_packet.md`](./project_doctor_packet.md)
  remains the scenario matrix, scoreboard, and finding-code source.
- [`/docs/support/recovery_ladder_packet.md`](./recovery_ladder_packet.md),
  [`/docs/support/repair_transaction_contract.md`](./repair_transaction_contract.md),
  [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md),
  and [`/docs/support/runbook_execution_contract.md`](./runbook_execution_contract.md)
  remain the linked recovery, repair, escalation, and runbook sources.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections
  24.2.1, 24.2.2, 24.2.3, 24.8, and Appendix DK.
- `.t2/docs/Aureline_Technical_Design_Document.md` Appendix AR.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 18.6, 18.8,
  18.21, 22.3, and support handoff patterns.

If this contract disagrees with the schemas, the schemas control packet
shape. If it disagrees with the scenario matrix or repair contracts,
those existing support contracts control vocabulary and this document
must update.

## Probe Catalog Entry

Every probe catalog row must answer these questions before Doctor may
run it:

| Field group | Required answer |
|---|---|
| Identity | `probe_id`, `probe_family_class`, `probe_version`, `rule_ids`, owner, and lifecycle status |
| Mutability | whether the probe is read-only, metadata-only, mutates cache, mutates target/route, mutates files, mutates trust/credentials, has external side effects, or is prohibited |
| Prerequisites | exact-build, support context, policy, trust, redaction, consent, or managed authority dependencies and what happens when each is missing |
| Latency | expected class, p50/p90 budgets, timeout, and first-actionable target ref |
| Evidence | evidence keys, source/signal/data classes, support inclusion posture, redaction class, replayability, and retention |
| Support posture | local-only, local plus remote read-only, managed read-only, managed-admin-required, offline cached, or unsupported |
| Failure handling | emitted finding code, typed unknowns, timeout behavior, and escalation or repair promotion |
| Side effects | explicit allowed and forbidden side effects plus an admission decision |

The catalog row is not merely descriptive. It is the allowlist Doctor
consults before running a probe in desktop, safe-mode, CLI/headless,
managed, or offline contexts.

## No Hidden Probe Side Effects

Doctor diagnosis is read-only by default.

A probe catalog row whose `mutability_class` is
`non_mutating_read_only` may be admitted as `admitted_safe_probe`. A row
whose `mutability_class` is `metadata_write_local_evidence_only` may be
admitted as `admitted_metadata_evidence_only` when its only write is a
local evidence, audit, or preview manifest row.

Any row with one of these classes is not a runnable Doctor probe:

- `mutates_disposable_cache`
- `mutates_target_or_route`
- `mutates_user_files`
- `mutates_trust_policy_or_credentials`
- `external_service_side_effect`
- `prohibited`

Such a row must set `doctor_admission_class` to
`blocked_promote_to_repair` or `prohibited_probe`. If it is repairable,
it must include `repair_promotion` linking to the repair transaction
schema, approval posture, rollback or checkpoint ref, runbook or help
anchor, and escalation template. If it is not repairable, it remains
prohibited and Doctor emits a blocked or insufficient-evidence finding.

This rule prevents probes from hiding cache clears, extension
reactivation, route publication, target retargeting, trust widening,
credential access, file rewrites, or external service writes behind a
diagnosis label.

## Finding Explanation Packet

Every Doctor finding that can reach UI, CLI/headless output, a support
bundle, a help article, or an escalation packet must emit one
`doctor_explanation_packet_record`.

The packet must include:

- `finding_code`, `finding_id`, `rule_id`, and the probe id/version
  that produced the result;
- evidence refs with role, signal class, redaction class, support-pack
  inclusion class, and replayability;
- confidence class, score, confidence reason, and typed remaining
  unknowns;
- affected contract refs such as the Doctor scenario matrix, recovery
  ladder, repair transaction, support bundle, object handoff, runbook,
  or help route contract;
- localization text keys for title, summary, expected state, observed
  state, belief basis, confidence, next action, and remaining unknowns;
- explanation factors that map each part of the explanation back to
  evidence refs; and
- governed next actions with approval posture, rollback/checkpoint
  refs, repair refs, runbook refs, help refs, bundle refs, and
  escalation refs.

Localized prose may change. Finding codes, JSON keys, action ids,
contract refs, and evidence refs may not.

## Repair Handoff

The `repair_handoff` block is the shared deep-link surface for support
and help. It binds a finding to:

- safe-mode session or entry refs;
- extension bisect or quarantine refs;
- recovery rung refs;
- repair transaction refs;
- runbook packet or step refs;
- help article or anchor refs;
- support bundle refs;
- escalation packet refs;
- approval posture;
- rollback refs; and
- checkpoint refs.

Every `governed_next_actions[]` row must point to one of those same
refs or explicitly carry `null` for refs that do not apply. Help,
support, CLI/headless, Support Center, and escalation surfaces must
reuse these anchors instead of minting page-local action labels.

## Failure And Unsupported States

Failure to complete a probe is still a diagnosis outcome. A probe that
times out, lacks prerequisites, enters an unsupported context, cannot
verify redaction, or detects a side-effect boundary must emit one of
the cataloged failure-mode rows. The emitted finding can be a normal
finding, an insufficient-evidence finding, an unsupported-state
finding, or an export/escalation refusal, but it may not disappear
silently.

Partial diagnoses must carry typed unknowns in both the probe catalog
failure handling and the explanation packet confidence block. Free-form
unknown text is for review summaries only.

## Seed Cases

The seed cases cover:

| Case | Schema | Purpose |
|---|---|---|
| `probe_catalog_toolchain_read_only.yaml` | probe catalog | read-only execution-context probe admitted in desktop, headless, managed read-only, and offline cached contexts |
| `probe_catalog_cache_repair_promoted.yaml` | probe catalog | cache mutation blocked in Doctor and promoted to a reviewed repair transaction |
| `finding_explanation_extension_crash_loop.yaml` | explanation | extension crash-loop finding linked to safe mode, bisect, repair transaction, help, support bundle, and escalation packet |
| `finding_explanation_helper_attach_escalation.yaml` | explanation | remote-helper finding linked to managed approval, runbook, support bundle, rollback refs, and escalation packet |

The case manifest lists assertions that reviewers can validate without
a live Project Doctor runtime.
