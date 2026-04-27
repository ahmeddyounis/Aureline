# Supportability Subsystem ADR Seed

- **Decision id:** pending formal register row
- **Status:** Accepted
- **Decision date:** 2026-04-27
- **Owner:** `@ahmeddyounis`
- **Forum:** architecture_council
- **Related requirement ids:** `REL-SUPPORT-001`, `REL-SUPPORT-002`, `REL-SUPPORT-003`, `REL-REPAIR-015`, `OPS-SUP-005`

## Context

Aureline already has separate seeds for Project Doctor findings,
support bundles, object-specific issue handoffs, recovery actions, and
repair transactions. The product requirements and architecture also
require safe mode, extension bisect, exact-build trace capture, and
headless support flows to stay trustworthy when the full desktop UI is
unhealthy.

Without one governing supportability boundary, these capabilities would
drift into surface-local terms: Doctor could emit one finding code,
safe mode could name a different state, support bundles could apply a
third redaction rule, and escalation packets could lose the exact build
or evidence quality needed for first diagnosis. This ADR makes
supportability one subsystem with shared IDs, state vocabulary,
redaction defaults, and action handoff rules.

Companion artifacts:

- [`schemas/support/doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json)
  defines the shared finding, probe-catalog, evidence, redaction,
  latency, replayability, and governed-action hooks.
- [`schemas/support/trace_capture_request.schema.json`](../../schemas/support/trace_capture_request.schema.json)
  defines exact-build trace-capture requests and their consent,
  side-effect, redaction, and support-bundle inclusion rules.
- [`fixtures/support/supportability_cases/`](../../fixtures/support/supportability_cases/)
  contains worked desktop, headless, managed, and offline cases.
- [`docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md),
  [`docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md),
  [`docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md),
  and
  [`docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  remain the detailed packet contracts this ADR composes.

If this ADR disagrees with the PRD, technical architecture, technical
design, UI/UX spec, or the linked support contracts, those sources win
and this ADR plus its companion schemas update in the same change.

## Decision

Aureline will treat Project Doctor, safe mode, extension bisect,
recovery-ladder state, support-bundle preview, escalation packets,
exact-build trace capture, and repair entry points as one
**supportability subsystem**. The subsystem owns the shared vocabulary
and evidence quality bar; repair executors, extension hosts, transport
adapters, crash handlers, and UI surfaces own their local mechanics but
must enter supportability through the governed record shapes.

The subsystem has five hard rules:

1. Diagnosis is read-only by default. A safe probe may collect
   metadata, hashes, manifests, counters, and existing local evidence,
   but it may not mutate trust, credentials, files, extension state,
   routes, remote helpers, policy, or external services.
2. Any mutation suggested by Doctor, safe mode, bisect, or a support
   flow is a reviewed repair transaction with preview, impacted and
   preserved state classes, checkpoint or no-checkpoint honesty,
   reversal class, and postcondition checks.
3. Every diagnosis, recovery rung, repair preview, support bundle,
   trace capture, and escalation packet uses the same finding codes,
   probe versions, redaction classes, support-pack inclusion classes,
   recovery rungs, and repair classes.
4. Exact-build identity, redaction policy, evidence source,
   replayability, and consent travel with support evidence. A trace or
   bundle without exact-build joins is not eligible to support a strong
   diagnosis claim.
5. Desktop, CLI/headless, managed support, and offline support contexts
   expose the same capability map, even when a capability is read-only
   or unavailable in a given context.

## Subsystem Boundary

| Area | Owned by supportability | Not owned by supportability |
|---|---|---|
| Project Doctor | probe catalog, finding schema, confidence, latency, evidence, unsupported-state labels, next governed action | domain-specific probe implementations |
| Safe mode and bisect | recovery-rung vocabulary, disabled/preserved capability summaries, attributable rung history | extension host scheduling and process launch mechanics |
| Support bundle | item-level inclusion class, redaction class, high-risk gates, exact-build joins, omitted-class reasons | archive compression and final storage transport |
| Escalation packet | finding IDs, evidence minimum, owner class, destination class, object refs, support-bundle linkage | hosted ticket provider behavior |
| Trace capture | exact-build request record, consent, duration, side-effect budget, redaction ceiling, output inclusion class | renderer, IPC, LSP, DAP, runtime, and OS trace collection APIs |
| Repair entry | repair class, source findings, preview requirement, recovery rung, no-touch boundaries | actual mutating repair executor |

Supportability may write local evidence records, manifests, preview
records, and audit rows. It may not change product state except through
a reviewed repair transaction or a clearly audit-only export action.

## Shared Vocabulary

The following tokens are canonical hooks. Later support features extend
these matrices instead of minting parallel support lexicons.

| Matrix | Canonical hook | Source of truth |
|---|---|---|
| Probe family | `probe_family_class` | Doctor finding schema and Project Doctor packet |
| Recovery rung | `recovery_rung_class` | support-bundle and recovery-action schemas |
| Repair class | `repair_class` | repair-transaction schema and Doctor finding schema |
| Signal class | `signal_class` | Doctor finding and trace-capture schemas |
| Secret class | `secret_class` | Doctor finding and trace-capture schemas, composed with the secret-broker redaction ADR |
| Support-pack inclusion | `support_pack_inclusion_class` | Doctor finding, trace-capture, and support-bundle artifact rows |
| Safety | `safety_class` | Doctor finding schema |
| Replayability | `replayability_class` | Doctor finding schema |
| Context availability | `support_context_class` plus `availability_class` | Doctor finding and trace-capture schemas |

The subsystem reuses these state classes:

| Class | Meaning |
|---|---|
| `safe_probe_read_only` | Reads existing local metadata or evidence only. |
| `safe_probe_metadata_only` | Collects bounded metadata, hashes, counters, or manifests and writes only a local evidence row. |
| `reviewed_repair_required` | Diagnosis found a possible fix, but any mutation must enter the repair transaction path. |
| `high_risk_capture_requires_consent` | Evidence may include code-adjacent or high-risk material and needs explicit high-friction review. |
| `prohibited_probe` | The requested probe would cross policy, trust, secret, or external side-effect boundaries and must refuse. |

Unsupported states are first-class. A probe that cannot establish truth
emits a finding with `unsupported_state_class` or
`insufficient_evidence` instead of silently omitting the row.

## Doctor Findings

Every Doctor finding must name:

- stable `finding_code` and `finding_id`;
- `probe_family_class`, `probe_version`, and `rule_id`;
- severity, confidence, affected scope, and latency class;
- evidence refs with source class, signal class, redaction class,
  support-pack inclusion class, and replayability;
- first actionable explanation covering expected state, observed state,
  belief basis, confidence, and safest next governed action;
- no-touch boundaries and safety class;
- redaction defaults, secret class, and high-risk gate;
- support context availability for desktop, CLI/headless, managed, and
  offline flows; and
- escalation owner plus minimum packet refs when local repair is unsafe
  or diagnosis remains incomplete.

Finding codes are additive-only. Repurposing a finding code, probe
family, repair class, or recovery rung is breaking and requires a
formal decision update.

## Probe Catalog And Explanation Hooks

Each later probe-catalog row must include:

- `probe_id`, `probe_family_class`, `probe_version`, `rule_id`, and
  owner;
- latency class and first-actionable-diagnosis target;
- execution posture and allowed side effects;
- signal classes read and secret-class ceiling;
- support context availability;
- evidence locator contract and redaction profile;
- redaction-accuracy test ref for any output that may enter a bundle;
- unsupported-state labels and remaining-unknown classes; and
- next governed actions allowed by the probe.

Each finding explanation must answer: what was expected, what was
observed, why Aureline believes it, how confident it is, what remains
unknown, and what the safest next governed action is. Localized prose
may vary, but IDs, JSON keys, exit semantics, and evidence refs are not
localized.

## Recovery And Repair

Safe mode is a published runtime profile. Extension bisect is a
versioned recovery profile. Quarantine, cache-reset candidates,
restricted reopen, and rollback/reinstall candidates are separate
recovery rungs, not synonyms for "reset".

Every rung transition records:

- entry reason;
- disabled or narrowed capability classes;
- preserved state classes;
- support context availability;
- related finding IDs and probe versions;
- support-bundle inclusion rows; and
- exit artifact or escalation trigger.

Every repair entry records source findings, repair class, preconditions,
impact scope, preserved state classes, checkpoint behavior, reversal
class, idempotency posture, and verification plan. If no safe local
repair exists, the governed action is `create_escalation_packet` or
`create_support_bundle`, not a generic destructive reset.

## Support Bundles And Escalation Packets

Support bundles remain manifest-first and redacted by default. The
supportability subsystem owns the item-level inclusion vocabulary:

| Inclusion class | Rule |
|---|---|
| `included_by_default_metadata` | Metadata-only evidence included when a user or admin requests a bundle. |
| `included_by_reference` | Stable refs, hashes, and manifests travel; payload bodies stay outside the bundle. |
| `local_only_retained` | Evidence remains on the device or managed workspace; the bundle carries a local retention ref and omission reason. |
| `review_required_before_export` | The preview must show data class, redaction state, owner, and destination before inclusion. |
| `opt_in_high_risk` | Code-adjacent or high-risk payloads require explicit high-friction consent or policy. |
| `excluded_always` | Raw secrets, forbidden policy payloads, and prohibited classes are not exported. |

High-risk artifacts are gated before collection and again before export.
Raw crash dumps, full terminal history, clipboard history, raw
credential material, raw environment variables, and code bodies are
never included by default. Redaction accuracy is a release-gated quality
bar: bundle and trace rows that claim metadata-safe output must be
tested against seeded secret fixtures and carry the redaction profile or
test ref used.

Escalation packets must preserve finding IDs, probe versions,
support-bundle refs, exact-build identity, recovery rung, repair
history, redaction choices, first-actionable-diagnosis target, and the
current owner class. The packet may carry typed unknowns, but it may not
hide missing reconstruction fields.

## Exact-Build Trace Capture

Trace capture is not a generic "collect logs" button. A conforming
trace-capture request includes:

- exact-build identity ref, product version, channel, platform, and
  component build refs where applicable;
- capture goal, signal classes, target refs, and support context;
- duration and size bounds;
- consent class or policy authority;
- allowed and forbidden probe side effects;
- redaction profile, secret-class ceiling, data-class ceiling, and
  redaction-accuracy check requirement;
- support-pack inclusion class and storage/embedding state; and
- bundle or escalation packet binding.

Trace capture may create bounded local trace artifacts when consent or
policy admits it. It may not run repo-owned hooks, widen trust, publish
routes, rotate credentials, reattach remote helpers, or mutate user
files as a side effect of capture.

## Capability Map

The Support Center is the primary desktop entry surface for this map.
Crash-loop recovery banners, command-palette diagnostics, object issue
handoffs, bundle previews, CLI/headless commands, managed admin support
views, and offline local export flows all read the same capability
state. A surface may filter or render the map differently, but it may
not rename a capability, finding, recovery rung, redaction class, or
next action into a surface-local term.

| Capability | Desktop | CLI/headless | Remote-managed support | Offline support |
|---|---|---|---|---|
| Project Doctor | `available` | `available` | `available_read_only` unless admin policy admits repair | `available_read_only` with cached/local probes |
| Safe mode | `available` | `read_only` for inspection and entry packet generation | `read_only` unless managed supervisor admits entry | `available` for local desktop entry |
| Extension bisect | `available` | `available` for installed extension sets | `read_only` unless admin policy admits cohort activation | `available` without registry fetch |
| Support-bundle preview | `available` | `available` | `available` with managed policy redaction floor | `available_local_only` |
| Exact-build trace capture | `available` with consent | `available` with consent | `available_read_only` or policy-authorized bounded capture | `available_local_only` for local signals |
| Escalation packet | `available` | `available` | `available` through approved destination class | `available_local_only` until later handoff |
| Repair entry | `available` through reviewed repair transaction | `available` through preview/apply contract | `read_only` unless admin consent is present | `limited_available` for local-only repairs |

No release claim may count a supportability capability that works only
in a healthy desktop UI. Headless and safe-mode paths are first-class
verification paths.

## Evidence Quality

Supportability owns the evidence quality expectations attached to
support capture mechanics:

- first actionable diagnosis target per scenario family;
- finding accuracy and unsupported-state reporting;
- false-safe-repair rate;
- escalation-packet completeness;
- redaction accuracy against seeded secret fixtures;
- support-pack item-level inclusion and omission correctness; and
- exact-build trace and crash symbolication joins.

Common extension, toolchain, proxy, and cache failures target first
actionable diagnosis within the published supportability target on each
claimed profile. The current architecture target for common cases is
p90 under 10 minutes, with metadata-only warm local probes expected to
stream much earlier according to the UI/UX latency rules.

## Consequences

- Doctor, safe mode, bisect, support bundles, trace capture, repairs,
  and escalation packets can cite the same finding IDs and redaction
  rules.
- Safe probes remain non-mutating unless the user or policy enters the
  governed repair path.
- Support bundles can describe item-level inclusion and redaction
  classes instead of only saying "bundle exported".
- Later crash, repair, recovery-ladder, support-center, and managed
  support work extends one vocabulary instead of creating parallel
  support models.
- Capability availability is explicit across desktop, CLI/headless,
  managed, and offline contexts.

## Alternatives Considered

- **Separate per-surface support vocabularies.** Rejected because it
  makes support bundles and escalation packets translate between Doctor,
  safe mode, repair, and trace terminology after the fact.
- **Trace capture as an ungoverned attachment flow.** Rejected because
  traces frequently contain code-adjacent, path, argument, timing, or
  secret-adjacent evidence and must carry exact-build, consent, and
  redaction state.
- **Repair-first diagnosis.** Rejected because read-only diagnosis and
  unsupported-state labeling are required before any mutation is
  trustworthy.

## Source Anchors

- `.t2/docs/Aureline_PRD.md:2971` -- incident severity, support
  bundles, safe mode, and forensic readiness.
- `.t2/docs/Aureline_PRD.md:3069` -- diagnostic data classes and
  support-bundle redaction defaults.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4598` --
  observability, diagnostics, and supportability architecture.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4664` --
  Project Doctor probe, finding, and explainability architecture.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4687` -- safe
  mode, extension bisect, and recovery-ladder orchestration.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4739` --
  supportability and field-diagnostics evidence pack.
- `.t2/docs/Aureline_Technical_Design_Document.md:2246` -- Project
  Doctor supportability subsystem contract.
- `.t2/docs/Aureline_Technical_Design_Document.md:4631` -- support
  bundle, repair, and field-readiness design.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:7408` -- Project Doctor
  finding contract and guided repair.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:7779` -- probe families,
  diagnosis latency, and explainability bars.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:8087` -- support intake,
  escalation packets, and field-readiness UX.

## Supersession History

None.
