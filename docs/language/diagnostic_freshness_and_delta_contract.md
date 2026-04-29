# Diagnostic freshness, remap, import, and delta parity contract

This document freezes the language-facing contract that lets live
diagnostics, imported scanner findings, baseline snapshots, suppressions,
review packets, CLI/headless output, and support bundles compare findings
without inventing per-tool packets.

Problems rows, inline markers, review annotations, support exports, and
quality dashboards must preserve the same answer to:

- whether a finding came from current live analysis, imported evidence,
  a baseline, a suppression record, or a support replay;
- whether the current editor range is exact, remapped, stale, generated,
  or not safe to show inline;
- whether a SARIF-like or provider-backed scan is authoritative for a
  target scope or only read-only imported evidence; and
- which delta claims are mechanically comparable across local runs,
  managed CI, provider imports, review packs, and support exports.

Machine-readable companions:

- [`/schemas/language/diagnostic_remap_state.schema.json`](../../schemas/language/diagnostic_remap_state.schema.json)
  - `diagnostic_remap_state_record`, the record every diagnostic anchor
    narrows to before an inline marker, review annotation, CLI row, or
    support export treats a range as exact, remapped, stale, generated,
    or not displayable inline.
- [`/schemas/language/sarif_import_record.schema.json`](../../schemas/language/sarif_import_record.schema.json)
  - `sarif_import_record`, the shared import envelope for SARIF-like
    files and provider-backed structured scan snapshots.
- [`/schemas/language/diagnostic_delta.schema.json`](../../schemas/language/diagnostic_delta.schema.json)
  - `diagnostic_delta_record`, the shared comparison packet for current,
    imported, baseline, stale, suppressed, waived, and support-exported
    findings.
- [`/fixtures/language/diagnostic_delta_cases/`](../../fixtures/language/diagnostic_delta_cases/)
  - worked YAML fixtures covering remapped inline display, remap-needed
    review, SARIF-like import, multi-source deltas, and support-export
    stale deltas.

This contract composes with and does not replace:

- [`/docs/language/diagnostics_and_code_action_contract.md`](./diagnostics_and_code_action_contract.md)
  and its diagnostic cluster, code-action summary, and suppression-review
  schemas. Diagnostic clusters remain the compact issue-row projection;
  this contract supplies the finding-level remap, imported-scan, and
  delta records those rows link to.
- [`/docs/execution/quality_profile_and_on_save_contract.md`](../execution/quality_profile_and_on_save_contract.md)
  and its quality-profile record family. Quality profiles explain which
  tools, rule packs, and baselines are active; this contract explains how
  the resulting findings compare.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  for wider evidence freshness, stale propagation, and rerun-trigger
  discipline.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Architecture_Document.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those sources disagree
  with this contract, those sources win and this contract plus companion
  schemas update in the same change.

## Scope

Frozen at this revision:

- diagnostic anchor-remap states and the rules for when each state may
  still render inline;
- imported scan records for SARIF-like files, provider API snapshots,
  managed CI artifacts, review packs, release packets, and support
  exports;
- scan time, import time, tool identity, rule-pack version, baseline
  family, target scope, severity mapping, profile refs, and imported
  versus live authority fields;
- delta records that compare current, imported, baseline, stale,
  suppressed, waived, and support-exported findings through one schema
  family;
- grouping, collapse, attribution, and compatibility rules for local
  runs, managed CI, provider imports, review packs, release packets, and
  support exports; and
- export/review survival rules so baseline drift, remap needs, imported
  scan deltas, and support replay state remain machine-readable.

Out of scope:

- implementing a SARIF parser or provider scan service;
- defining SARIF itself;
- implementing the Problems UI, scanner UI, review UI, or support bundle
  composer; and
- choosing concrete scanners, rule packs, or default baselines.

## 1. Record family

The three records form one schema family at the finding boundary.

| Record | Purpose | Source of authority |
|---|---|---|
| `diagnostic_remap_state_record` | Names the current anchor state and inline display posture for one diagnostic/finding. | VFS identity, source map or generated lineage, remap service, scanner import, review replay, or support replay. |
| `sarif_import_record` | Preserves one imported scan envelope after normalization without embedding the raw scan body. | SARIF-like file, provider API snapshot, managed CI artifact, review packet, release packet, or support bundle. |
| `diagnostic_delta_record` | Compares result sets and emits counts plus per-finding delta rows. | Current local run, managed CI run, provider import, accepted baseline, suppression set, review packet, or support export. |

Shared invariants:

1. Raw source text, raw scanner payloads, raw logs, raw command lines,
   raw absolute paths, and raw secret values do not cross these schema
   boundaries. Records use opaque refs, digests, class labels, counts,
   timestamps, and reviewable summaries.
2. Imported evidence never becomes current live evidence by being opened
   in the editor. Only a compatible local, remote-target, managed CI, or
   provider-authoritative run can upgrade authority, and the record must
   preserve the confirming ref.
3. A visible range is not automatically exact. Every inline marker,
   review annotation, CLI location, and exported report row must carry a
   remap state or a ref to a remap-state record.
4. Baselines and suppressions are interpretation state, not deleted
   findings. Delta packets keep them separate from current, imported,
   and stale evidence.
5. Compatibility-blocked comparisons are still records. They count as
   "not comparable" rather than as improvement, regression, or resolved
   debt.

## 2. Remap states and inline display

`diagnostic_remap_state_class` is the canonical vocabulary for deciding
whether a diagnostic may appear at an inline range and how much warning
the surface must show.

| State | Meaning | Inline rule |
|---|---|---|
| `exact` | The current document/blob/structured object still contains the diagnostic anchor at the same validated range. | May render as a normal inline marker when freshness and authority are current for the target scope. |
| `remapped` | The original range changed, but a compatible remap process found a current range with enough evidence to keep the finding useful. | May render inline only with a visible remap cue and original-context access. It may not be styled as exact. |
| `stale` | The diagnostic belongs to an older epoch, target, branch, generated output, environment, or scan snapshot. | May render inline only as stale/read-only context when the current file still exists and the stale cue is visible. Mutating actions require refresh or review. |
| `source_moved` | The source file, symbol, cell, or structured region moved or was renamed, and the remap can identify the new target. | May render at the moved target only with a moved/remapped cue. If confidence drops below high, it becomes `needs_remap`. |
| `needs_remap` | The prior anchor is known to be invalid or materially suspect and no current safe range has been admitted. | Must not render an inline range. It remains visible in Problems, review, CLI, support export, or a remap queue. |
| `unsupported_anchor` | The producer reported a location kind Aureline cannot project into an editor range, such as a tool-specific region, package coordinate, dependency edge, or target-only finding. | Must not invent a source range. It may render a scope-only row, structured-artifact row, or review/support entry. |
| `generated_anchor_only` | The only safe location is generated, mirrored, minified, compiled, or otherwise derived output without an admitted source counterpart. | May render inline only on the generated/read-only target or paired generated view with generated-source disclosure. Source files are not marked exact. |

Inline display rules:

1. An inline range may be shown only when `inline_range_may_be_shown` is
   true, `current_anchor.anchor_ref` is non-null, and the
   `inline_display_class` is compatible with the remap state.
2. `needs_remap` and `unsupported_anchor` always set
   `inline_range_may_be_shown` to false.
3. `stale`, `remapped`, `source_moved`, and `generated_anchor_only`
   require a visible cue on inline, hover, Problems, review, CLI, and
   support/export surfaces. A color-only distinction is insufficient.
4. Code actions that mutate source may use `exact` anchors directly.
   Other states require either a compatible current confirmation or a
   review flow that names the remap state.
5. Suppressions and baseline accepts may still target stale, remapped,
   or imported findings, but the review record must name the remap state
   and reopen rule.
6. Exporters include both the original anchor ref and current anchor ref
   when redaction allows. When redaction blocks either ref, the omission
   reason remains explicit.

The older diagnostic-cluster `anchor_remap_state_class` vocabulary is a
compact projection. When a cluster links to this contract:

| Cluster projection | Finding-level state |
|---|---|
| `exact` | `exact` |
| `contextual` | `remapped` or `source_moved` |
| `stale` | `stale` or `needs_remap` |
| `unmapped` | `needs_remap` or `unsupported_anchor` |
| `imported_static` | `generated_anchor_only` or imported `stale` |

The finding-level record is authoritative when both are present.

## 3. Imported scan records

`sarif_import_record` is intentionally broader than literal SARIF. It is
the normalized envelope for:

- SARIF 2.1.0 files;
- SARIF-like JSON produced by scanners that do not fully conform;
- provider API snapshots;
- managed CI scan artifacts;
- review-packet or release-packet scan snapshots; and
- support-export replays.

Every imported scan record preserves:

- import format and import source class;
- source artifact ref and preserved raw payload ref;
- scan start, scan completion, and import timestamps;
- source tool id/version plus adapter id/version;
- rule-pack ref, version, and digest;
- effective/local/imported/provider profile refs when known;
- baseline family ref and baseline compatibility state;
- target scope, target revision, execution context, and environment refs;
- severity mapping from producer labels to normalized diagnostic
  severity;
- imported-versus-live authority; and
- normalized diagnostic, remap-state, and delta refs.

Authority rules:

1. `imported_read_only_evidence` may populate Problems, review cards,
   release packets, CLI output, and support bundles, but it may not
   unlock silent fix-all or current-exact wording.
2. `provider_authoritative_snapshot` means the provider is authoritative
   for its declared scan snapshot, not for the user's current buffer.
3. `managed_ci_authoritative_snapshot` may be release-relevant when the
   target scope and baseline family match the release claim, but it is
   still a snapshot with scan time and target identity.
4. `locally_confirmed_current` requires a compatible local or
   remote-target run ref and keeps the imported origin visible.
5. `live_current_authoritative` is reserved for active analysis against
   the admitted current epoch and target scope. Importing a file cannot
   create this authority by itself.
6. `support_replay_read_only` is replay evidence. It is useful for
   diagnosis and comparison but never a current local proof.

Severity mapping rules:

1. Producer severity labels remain exportable as reviewed summaries or
   opaque labels.
2. Normalized severity uses the diagnostic severity vocabulary:
   `error`, `warning`, `information`, and `hint`.
3. Policy may override severity, but the mapping row must state that the
   normalized value came from policy rather than the scanner.
4. Unknown or provider-custom severities map through a reviewed mapping
   row or fail to an explicit review-required state.

## 4. Diagnostic delta records

`diagnostic_delta_record` compares result sets, not raw logs. Each
result set names its role, authority, freshness, profile/tool/rule-pack
refs, and produced time.

Allowed result-set roles:

| Role | Meaning |
|---|---|
| `current_live` | Current local or remote-target diagnostic run for the admitted scope. |
| `imported_snapshot` | Imported SARIF-like or provider-backed evidence. |
| `baseline_snapshot` | Accepted baseline family or baseline export. |
| `suppressed_findings` | Governed suppression set that narrows visibility or enforcement. |
| `stale_findings` | Findings retained for lineage after their epoch fell below the current floor. |
| `managed_ci_current` | Managed CI run for the declared target. |
| `provider_live` | Provider-backed live result where the provider owns the current authority. |
| `support_export_snapshot` | Replayed support bundle or support packet evidence. |
| `review_pack_snapshot` | Review-pack findings bound to a diff/base/head identity. |

Compatibility is checked before a delta claim is admitted. The required
axes are:

- effective profile;
- tool identity and tool version;
- rule pack;
- baseline family;
- target scope;
- anchor mapping family;
- environment or execution context; and
- suppression policy.

If any required axis is blocked, the comparison emits
`compatibility_blocked` rows and counts. It does not report new,
resolved, or persisting movement for the blocked portion.

### 4.1 Delta states

| State | Meaning |
|---|---|
| `new` | Candidate finding is absent from the compatible baseline or imported comparison set. |
| `persisting` | Finding remains present across compatible sets. |
| `resolved` | Baseline/imported finding is absent from the compatible current set. |
| `suppressed` | Finding is present but visibility/enforcement is governed by a suppression. |
| `waived` | A waiver or release-visible exception changes release interpretation. |
| `stale` | Finding is retained only as older evidence. |
| `needs_remap` | Finding cannot be compared until its anchor is remapped or reviewed. |
| `imported_only` | Finding exists only in imported evidence. |
| `locally_confirmed` | Imported or baseline finding was reproduced or validated by a compatible current run. |
| `remapped` | Finding remains comparable after a disclosed remap. |
| `source_moved` | Finding remains comparable after source move/rename remap. |
| `unsupported_anchor` | Finding is scope-only or structured-location-only and cannot compare by editor range. |
| `generated_anchor_only` | Finding compares only on generated output lineage. |
| `compatibility_blocked` | Tool/profile/rule/mapping/target mismatch prevents a truth-bearing delta. |

### 4.2 Grouping and collapse

Grouping is allowed only for mechanical comparison. Display collapse is
not a license to erase provenance.

Allowed grouping keys:

- rule and anchor family;
- rule and target scope;
- baseline family;
- source kind;
- owner or policy scope;
- suppression scope;
- severity and category; and
- provider run.

Collapse classes:

| Collapse class | Rule |
|---|---|
| `no_collapse` | Keep entries separate. |
| `collapse_display_only` | One compact row may render, but every contributing record remains expandable and exportable. |
| `collapse_mechanical_equivalent` | Records may compare as one unit because rule, target, profile, baseline, remap family, and authority are compatible. |
| `collapse_under_suppression` | Suppressed entries may be grouped under the governing suppression, but counts and refs remain visible. |
| `collapse_under_baseline` | Baseline-matched entries may group under the accepted baseline family. |
| `collapse_blocked_requires_review` | A requested collapse would hide incompatible authority, remap, target, or suppression truth and must stay separated. |

### 4.3 Attribution

Every per-finding delta entry names an attribution class:

- `local_run`
- `managed_ci`
- `provider_import`
- `provider_live`
- `baseline`
- `suppression`
- `support_export`
- `review_pack`
- `mixed_attribution_requires_detail`

Rules:

1. A row with mixed attribution must keep per-source refs and detail
   expansion. It may not show a single producer badge.
2. A suppression or waiver changes interpretation but does not become
   the source of the finding.
3. A support export preserves what was captured. It does not become the
   current source of truth after replay.
4. Provider imports and managed CI may be authoritative for their
   declared target, but local surfaces must still show that authority
   boundary before offering current-buffer actions.

## 5. Surface, export, and review rules

### 5.1 Problems and editor markers

- Current exact live findings may render normally.
- Remapped, moved, stale, imported, generated-only, and support-replayed
  findings require visible state cues in the row, inline marker, hover,
  and detail surface.
- Findings with `needs_remap` or `unsupported_anchor` render as
  Problems/review/support rows, not invented source ranges.
- Filters may hide rows only as a user-visible filter decision. They may
  not remove baseline, suppression, remap-needed, or imported-only state
  from exports.

### 5.2 Review packs

Review packs include:

- current/live result-set refs;
- imported scan refs;
- baseline family refs;
- suppression and waiver refs;
- remap-state refs for every finding whose current range is not exact;
- compatibility status and blocked axes; and
- per-finding attribution.

Review comments or findings bound to stale base/head identity use delta
state `stale`, `needs_remap`, or `compatibility_blocked`; they do not
silently retarget to the user's current diff.

### 5.3 CLI and headless output

CLI/headless output uses the same record ids and schema refs as the UI.
Text output may summarize, but JSON output preserves:

- result-set role;
- authority class;
- freshness;
- remap state;
- baseline/suppression/waiver refs;
- delta state;
- compatibility class; and
- counts.

### 5.4 Support bundles and support packets

Support exports preserve enough structured state to explain why a user
saw a marker without requiring raw source code or raw SARIF bodies:

- scan/import refs and timestamps;
- tool and rule-pack refs;
- baseline family and drift state;
- remap state and original/current anchor refs where redaction permits;
- omitted-anchor reason where redaction blocks location refs;
- delta counts and compatibility-blocked counts;
- active suppression/waiver refs; and
- redaction class.

Support replay uses `support_replay_read_only`. Replaying a bundle may
reconstruct rows and deltas, but it may not upgrade authority to current
live evidence.

### 5.5 Release and quality reports

Any report that claims quality improved, regressed, or stayed flat must
include a diagnostic delta record or name why compatibility was blocked.
Minimum release/report content:

- effective quality-profile ref;
- tool and rule-pack versions;
- baseline family ref;
- delta counts;
- active suppression/waiver counts;
- remap-needed and unsupported-anchor counts;
- imported-only and locally-confirmed counts; and
- parity note for local, managed CI, provider import, and support export
  sources.

## 6. Fixture expectations

The fixture corpus demonstrates the contract in pre-implementation form.
Each fixture is one concrete record:

- a remapped finding that may still render inline with disclosure;
- a finding that needs remap after the target changed and therefore has
  no inline range;
- a SARIF-like provider scan import with rule-pack, severity mapping,
  baseline family, target scope, and imported authority;
- a delta packet comparing current, imported, baseline, and suppressed
  findings; and
- a support-export replay delta where stale/remap-needed rows survive
  export without becoming current live truth.
