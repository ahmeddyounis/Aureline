# M5 Docs Authoring Certification

This document is the contract for the M5 certification report that qualifies the
docs-authoring stack — the Markdown authoring workspace, the CommonMark safe
preview, docs-maintenance suggestions, docs validation, and docs evidence handoff
— across every claimed docs/browser deployment profile. The report is the
canonical M5 control source for this lane: release gates, support exports,
diagnostics, the release center, and Help/About surfaces ingest the checked-in
report rather than restating docs-authoring behavior. **No profile may stay
greener than this report.**

- Record kind: `m5_docs_authoring_certification_report`
- Schema: [`schemas/docs/m5-docs-authoring-cert-report.schema.json`](../../schemas/docs/m5-docs-authoring-cert-report.schema.json)
- Canonical support export: [`artifacts/m5/docs-authoring/certification-report/support_export.json`](../../artifacts/m5/docs-authoring/certification-report/support_export.json)
- Summary artifact: [`artifacts/m5/docs-authoring/certification-report.md`](../../artifacts/m5/docs-authoring/certification-report.md)
- Waiver-and-downgrade log: [`artifacts/m5/docs-authoring/waiver-and-downgrade-log/waiver_and_downgrade_log.json`](../../artifacts/m5/docs-authoring/waiver-and-downgrade-log/waiver_and_downgrade_log.json)
- Fixtures: [`fixtures/docs/m5/certification-corpus/`](../../fixtures/docs/m5/certification-corpus/)
- Certified against the frozen matrix: [`docs/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md`](../docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md)
- Producer: `aureline_docs::current_stable_docs_authoring_cert_report`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_authoring_certification -- packet`

## Certified profiles

Each profile row certifies the whole docs-authoring stack on one claimed
deployment profile. Every row records which surfaces it covers, the three
certification gates it must pass, an auto-derived qualification class and verdict,
and the proof-freshness state.

| Profile | Claimed | Certified | Notes |
| --- | --- | --- | --- |
| `desktop` | Stable | Stable | Local desktop authoring with first-party packs. |
| `mirrored` | Stable | Stable | Pinned, signed mirror outranks live vendor docs. |
| `cached` | Stable | Stable | Last-known-good with explicit freshness labels. |
| `pinned_pack` | Stable | Stable | Frozen pack revision and signature stay visible. |
| `extension_owned` | Beta | Beta | Less-trusted host; capped at Beta by a standing class cap. |
| `browser_handoff` | Beta | Beta | Narrow companion surface; capped at Beta and never widens authority. |

Every profile covers all five docs-authoring surfaces:
`markdown_authoring_workspace`, `commonmark_preview`,
`docs_maintenance_suggestions`, `docs_validation`, and `docs_evidence_handoff`.
Each coverage entry carries the canonical schema and support-export ref the
producing crate owns, so the certification can never drift from the real surface.

## Certification gates and automatic narrowing

A profile is certified against three gates. The report derives every profile's
qualification and verdict from these gates plus proof freshness, so the report
narrows itself rather than waiting for a human to lower a claim:

1. **Source / version / freshness truth** stays visible on the profile.
2. **Safe rendered-preview boundaries** hold — preview is sanitized, labeled, and
   keeps an escape to source; it is never a privileged execution path.
3. **Export / support parity** holds for the whole authoring stack.

The derivation is fail-closed:

- A profile that loses its **safe rendered-preview boundaries** is **blocked from
  promotion** (`blocked_underqualified`, held) — guardrail first.
- A profile that loses **source/version/freshness truth** or **export/support
  parity**, drops a surface, or whose proof goes **stale** is **narrowed one class
  below its claim** (`narrowed_to_qualified`).
- Otherwise the profile is `certified` at its claimed class.

`proof_age_hours > freshness_window_hours` marks the proof stale. Narrowing
narrows the claim; it never hides the profile. Narrowed and blocked profiles stay
in the report, labeled, and roll up into the certification index.

## Certification index

The `certification_index` is the single roll-up release, support, diagnostics, and
AI surfaces read instead of re-deriving maturity: it lists the certified,
narrowed, and blocked profiles, whether every profile is current and certified,
the covered surfaces, and a one-line summary.

## Compatibility with the frozen matrix

The `compatibility_report` binds the certification to the frozen docs-authoring
matrix by artifact ref, schema ref, and schema version, and asserts that every
profile is present, no profile is greener than the matrix, every covered surface
has a checked-in schema and support export, and the downgrade rules are
auto-enforced.

## Waiver-and-downgrade log

The [`waiver-and-downgrade-log`](../../artifacts/m5/docs-authoring/waiver-and-downgrade-log/waiver_and_downgrade_log.json)
is derived from the report's profile rows and records two kinds of entry:

- **Class caps** (`class_cap`): standing governance decisions that hold a profile
  below Stable — the extension-owned and browser-handoff profiles are capped at
  Beta because they run outside the first-party desktop trust boundary.
- **Auto-downgrades** (`auto_downgrade`): an automatic narrowing currently in
  effect because a certification gate failed or proof went stale.

The stable report carries the two class caps and no auto-downgrades. The corpus
fixtures exercise auto-downgrade entries.

## Downgrade rules

The `downgrade_rules` set is the machine-readable contract release and support
tooling auto-enforce. Each rule binds a trigger to a narrowing action and the
profiles it applies to. The unsafe-preview rule blocks promotion on every
profile; the stale-proof, missing-export-parity, mirror-offline, and
source-version rules narrow below Stable; the scope-expansion rule blocks the
extension-owned and browser-handoff profiles.

## Trust review and consumer projection

`trust_review` records the hard invariants that must hold for the report to
validate: docs stay source-canonical, rendered preview stays safe and is never a
privileged execution path, suggestions stay diff-first, source/version/freshness
truth stays visible, validation state is never silently upgraded, evidence handoff
stays source-linked, browser handoff never hides owner/origin/boundary changes or
silently widens authority, downgrade narrows instead of hiding, and no profile
stays greener than the report. `consumer_projection` records that the release
gate, CLI/headless, support export, diagnostics, Help/About, the release center,
and the evidence index all read this report and label narrowed or blocked
profiles rather than hiding them.

## Known limits

The report publishes its `known_limits` and `migration_refs` so docs-authoring
claims stay aligned with actual proof: the extension-owned and browser-handoff
profiles are capped at Beta; rendered preview never executes diagrams, math, or
custom components as privileged code; cached and mirrored profiles serve
last-known-good docs with explicit freshness labels; and the certification covers
only the desktop/local-first docs-authoring contract — no browser-first docs
product, collaborative rich-text editor, or remote CMS workflow is claimed.
