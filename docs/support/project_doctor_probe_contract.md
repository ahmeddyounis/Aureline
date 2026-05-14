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

- [`/docs/support/project_doctor_contract_alpha.md`](./project_doctor_contract_alpha.md),
  [`/schemas/project_doctor/probe.schema.json`](../../schemas/project_doctor/probe.schema.json),
  [`/schemas/project_doctor/finding.schema.json`](../../schemas/project_doctor/finding.schema.json),
  and
  [`/artifacts/support/project_doctor_probe_pack_alpha.yaml`](../../artifacts/support/project_doctor_probe_pack_alpha.yaml)
  publish the alpha Project Doctor probe/finding contract and read-only
  probe-pack baseline consumed by the support crate. They reuse the
  vocabularies below rather than replacing them.
- [`/docs/support/probe_family_matrix.md`](./probe_family_matrix.md) and
  [`/artifacts/support/probe_families.yaml`](../../artifacts/support/probe_families.yaml)
  publish the probe-family matrix and non-destructive diagnosis rules
  Doctor must follow before any probe is reviewed.
- [`/schemas/support/probe_catalog_entry.schema.json`](../../schemas/support/probe_catalog_entry.schema.json)
  defines one catalog row with mutability class, prerequisites,
  expected latency, evidence produced, local or remote support class,
  redaction class, failure-mode handling, and side-effect admission.
- [`/schemas/support/doctor_probe.schema.json`](../../schemas/support/doctor_probe.schema.json)
  narrows the catalog row into one probe descriptor that pins the
  closed probe-class taxonomy (read-only inspection, simulation,
  environment check, repair preview, unsafe or unsupported), the
  closed invocation policy (automatic, automatic-inferring-only, with
  user consent, with managed-admin consent, never without explicit
  invocation), and the per-context headless parity rows.
- [`/schemas/support/doctor_finding_card.schema.json`](../../schemas/support/doctor_finding_card.schema.json)
  defines one rendered Project Doctor finding card with finding
  identity, confidence, affected scope, root-cause class, evidence
  source, unsupported-state note, suggested next action, links to
  safe mode, bisect, repair preview, support bundle, and export
  packet, plus exactly four headless parity rows.
- [`/schemas/support/doctor_explanation.schema.json`](../../schemas/support/doctor_explanation.schema.json)
  defines one finding explanation and repair-handoff packet.
- [`/fixtures/support/doctor_probe_cases/`](../../fixtures/support/doctor_probe_cases/)
  contains eight paired probe-descriptor and finding-card seeds covering
  missing toolchain, trust/policy block, watcher health, proxy or CA
  failure, extension regression, schema drift, local-history corruption,
  and remote-target mismatch.
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

## Probe Class And Invocation Policy

The probe descriptor (`schemas/support/doctor_probe.schema.json`)
narrows every probe to one of five classes drawn from a closed set:

| `probe_class` | What the probe does | Allowed mutability classes | Default `diagnosis_posture` |
|---|---|---|---|
| `read_only_inspection` | Reads existing manifests, hashes, logs, or health events. | `non_mutating_read_only`, `metadata_write_local_evidence_only` | `proving_diagnosis` (or `inferring_from_partial_evidence` when typed unknowns remain) |
| `simulation` | Evaluates a hypothetical state (would-this-policy-allow, would-this-target-bind) against existing evidence. | `non_mutating_read_only`, `metadata_write_local_evidence_only` | `simulating_hypothetical` |
| `environment_check` | Reads adjacent device, network, proxy, or runtime state by issuing read-only system or reachability calls. | `non_mutating_read_only`, `metadata_write_local_evidence_only` | `proving_diagnosis` (or `inferring_from_partial_evidence`) |
| `repair_preview` | Materialises a local preview manifest describing what a reviewed repair would do. Never applies. | `metadata_write_local_evidence_only` only | `previewing_repair_only` |
| `unsafe_or_unsupported` | Cannot be run from Doctor at all. The descriptor exists so the catalog can label the failure mode and route through the reviewed repair path or refuse with an unsupported-state finding. | any mutating class | `refusing_unsupported` |

Every descriptor must also pin one `invocation_policy`:

| `invocation_policy` | When Doctor may run the probe |
|---|---|
| `automatic` | Permitted only for `read_only_inspection`, `environment_check`, or `simulation` whose data class is at most `environment_adjacent` and whose redaction class is `metadata_safe_default`. The descriptor's `automatic_run_gate.permitted` MUST be `true`. |
| `automatic_inferring_only` | Same allowlist as `automatic`, but the probe MUST label its finding with `diagnosis_posture` of `inferring_from_partial_evidence` or `simulating_hypothetical` so a reader cannot mistake it for a proven diagnosis. |
| `with_user_consent` | A single-step user review (or high-friction consent for high-risk captures) gates the probe. `consent_gate.consent_required` MUST be `true`. |
| `with_managed_admin_consent` | Managed-admin authority gates the probe. `consent_gate.managed_admin_required` MUST be `true`. |
| `never_without_explicit_invocation` | The default for `repair_preview` and `unsafe_or_unsupported`. Doctor never runs the probe automatically; the user must invoke it from a deeper surface. |

The schema's `if/then` blocks enforce the matrix above mechanically:
a `repair_preview` descriptor cannot declare `mutability_class` outside
`metadata_write_local_evidence_only`, and an `unsafe_or_unsupported`
descriptor cannot declare an automatic or with-consent invocation
policy. Doctor MUST refuse to honour a descriptor whose probe class
disagrees with the catalog row's mutability class.

## Read-Only-By-Default Finding Card

The finding card (`schemas/support/doctor_finding_card.schema.json`)
is the surface-agnostic render contract every Project Doctor finding
emits before it reaches a desktop UI, a CLI/headless renderer, or a
support packet. The card carries:

- `finding_id`, `finding_code`, `rule_id`, `probe_id`, and a pointer
  to the probe descriptor that produced it;
- `card_posture_class` drawn from a closed six-value set
  (`read_only_diagnosis`, `read_only_evidence_review`,
  `preview_only_no_apply`, `mutating_with_review_and_preview`,
  `handoff_only_no_repair`, `refusing_unsupported`);
- `confidence_class`, `confidence_score`, and `diagnosis_posture` so
  the reader can tell whether Doctor is proving, inferring,
  simulating, previewing, or refusing;
- `affected_scope` (scope class, scope ref, and the support context
  the card was rendered for);
- `root_cause_class` drawn from a closed nineteen-value vocabulary
  spanning the seeded scenario families (missing toolchain, proxy or
  CA failure, schema drift, local-history corruption, remote-target
  mismatch, and so on);
- `evidence_sources[]`, where each source declares its evidence ref,
  source class, redaction class, support-pack inclusion class,
  replayability class, and which card field it supports;
- `unsupported_state_note` with one `unsupported_state_class` (`none`
  for normal cards) and reviewable text keys;
- `remaining_unknowns[]` drawn from the typed-unknown vocabulary;
- `card_text_keys` for title, summary, expected, observed, belief
  basis, confidence, root cause, remaining unknowns, next action, and
  unsupported-state copy (prose may localise; keys may not);
- `suggested_next_action` (action id, action class, approval posture,
  reason text key, rollback ref, checkpoint ref);
- `linked_handoff_refs` for safe mode, bisect, repair preview, repair
  transaction, runbook, help article, support bundle, export packet,
  and escalation packet — every field is required, and an
  inapplicable handoff MUST be `null` rather than omitted; and
- exactly four `headless_parity_rows` (one per support context).

A `read_only_diagnosis` card itself applies no mutation. It MAY
deep-link to a mutating handoff (safe mode, bisect, repair preview,
repair transaction); each of those surfaces carries its own consent
and review gate. A `preview_only_no_apply` card MUST have a non-null
`linked_handoff_refs.repair_preview_ref` and a `suggested_next_action`
of `open_repair_preview`. A `refusing_unsupported` card MUST carry an
`unsupported_state_class` other than `none` and MUST suggest one of
`refuse_unsupported`, `stop_and_escalate`, `open_help_article`,
`open_runbook`, `create_support_bundle`, or `create_escalation_packet`
as its next action.

A finding whose `diagnosis_posture` is `inferring_from_partial_evidence`
or `simulating_hypothetical` MAY NOT declare `confidence_class` of
`observed_authoritative`. The schema enforces this so a card cannot
present an inferred or simulated outcome with the same confidence
language as a proven one.

## Headless Parity Contract

Both probe descriptors and finding cards carry exactly four
`headless_parity_rows`, one per `support_context_class`: `desktop`,
`cli_headless`, `remote_managed`, and `offline_local`. Every row
declares:

- `parity_class` drawn from a closed six-value vocabulary
  (`full_parity`, `machine_readable_only_no_ui`,
  `ui_suppressed_consent_required`, `ui_suppressed_unsupported`,
  `ui_suppressed_managed_authority_required`, `unavailable_in_context`);
- `machine_readable_result_fields[]` drawn from a closed twenty-one
  value field vocabulary. Every row's set MUST include `finding_id`,
  `finding_code`, `probe_id`, `probe_class`, `diagnosis_posture`, and
  `exit_code_class` so the same finding can be replayed across
  surfaces without semantic drift;
- `suppressed_ui_affordances[]` drawn from the desktop UI affordance
  vocabulary (primary action button, secondary action button, evidence
  drawer, preview-diff pane, consent dialog, managed-admin dialog,
  help link, runbook link, export link). Empty array means no
  affordance is suppressed in this context;
- `unimplemented_capability_class` drawn from a closed five-value
  lifecycle vocabulary (`implemented`, `not_yet_implemented_planned`,
  `not_yet_implemented_descoped`, `deprecated_will_remove`,
  `permanently_unsupported`) so a row that does not yet exist is
  labelled rather than dropped silently;
- `headless_exit_code_class` drawn from a closed six-value vocabulary
  (`exit_clean_no_findings`, `exit_findings_advisory_only`,
  `exit_findings_actionable`, `exit_unsupported_context`,
  `exit_blocked_consent_required`, `exit_probe_runtime_error`); and
- a reviewable `notes` sentence describing the parity decision.

This four-row block is the parity audit. A reviewer can read all four
rows side-by-side and see which UI affordances are hidden, which JSON
fields are still emitted, which capability is not yet implemented, and
which exit class the headless renderer returns. A parity gap is
visible per row instead of implied by omission, so the same finding
can be rendered credibly in desktop UI, CLI/headless, and support
packets without semantic drift.

## Seed Cases

The seed cases cover:

| Case | Schema | Purpose |
|---|---|---|
| `probe_catalog_toolchain_read_only.yaml` | probe catalog | read-only execution-context probe admitted in desktop, headless, managed read-only, and offline cached contexts |
| `probe_catalog_cache_repair_promoted.yaml` | probe catalog | cache mutation blocked in Doctor and promoted to a reviewed repair transaction |
| `finding_explanation_extension_crash_loop.yaml` | explanation | extension crash-loop finding linked to safe mode, bisect, repair transaction, help, support bundle, and escalation packet |
| `finding_explanation_helper_attach_escalation.yaml` | explanation | remote-helper finding linked to managed approval, runbook, support bundle, rollback refs, and escalation packet |

Six new paired cases live under
[`/fixtures/support/doctor_probe_cases/`](../../fixtures/support/doctor_probe_cases/),
one probe descriptor and one finding card per scenario:

| Scenario | Probe class | Invocation policy | Card posture |
|---|---|---|---|
| Missing toolchain | `read_only_inspection` | `automatic` | `read_only_diagnosis` |
| Proxy or CA failure | `environment_check` | `with_user_consent` | `read_only_diagnosis` |
| Extension regression | `read_only_inspection` | `automatic` | `read_only_diagnosis` |
| Schema drift (cache rebuild preview) | `repair_preview` | `with_user_consent` | `preview_only_no_apply` |
| Local-history corruption | `unsafe_or_unsupported` | `never_without_explicit_invocation` | `refusing_unsupported` |
| Remote-target mismatch | `simulation` | `automatic_inferring_only` | `read_only_diagnosis` |

The case manifest lists assertions that reviewers can validate without
a live Project Doctor runtime.
