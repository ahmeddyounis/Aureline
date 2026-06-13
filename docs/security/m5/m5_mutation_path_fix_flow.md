# M5 Mutation-Path Fix Flows And Auditable Suppressions

This document is the contract for how the new M5 *mutation paths* — save, format,
organize-imports, and AI-apply — treat suspicious bytes when they write back to
the new mutation-bearing artifact families (notebooks, docs pages, AI-evidence
artifacts, structured artifacts, and generated artifacts). Two invariants hold on
every path:

1. **No silent suspicious-byte rewrite.** A fix that would rewrite bidi-control,
   invisible, or mixed-script confusable bytes always routes through a
   *previewable diff* or a *review sheet* before any bytes change.
2. **Auditable suppressions.** When a warning is suppressed instead of fixed, the
   suppression is a scope-aware, auditable object — actor, reason, timestamp,
   scope, optional expiry, and a reachable audit log — never hidden per-pane
   state that silently disappears.

This lane sits on the same shared content-integrity policy library as its
siblings: it runs the shared suspicious-content detector
(`aureline_content_safety::detect_suspicious_content`) over the content each path
touches and derives the shared safe-inspection escape (`escape_for_safe_inspection`)
rather than inventing a parallel detector. The frozen content-integrity matrix
locks the static qualification each surface may claim, and the suspicious-text
detector parity lane keeps the warning vocabulary shared. This lane covers the
orthogonal *mutation-path* gap they leave open.

- Record kind: `m5_mutation_path_fix_flow_packet`
- Schema: [`schemas/security/m5-mutation-path-fix-flow.schema.json`](../../../schemas/security/m5-mutation-path-fix-flow.schema.json)
- Canonical support export: [`artifacts/security/m5/m5_mutation_path_fix_flow/support_export.json`](../../../artifacts/security/m5/m5_mutation_path_fix_flow/support_export.json)
- Summary artifact: [`artifacts/security/m5/m5_mutation_path_fix_flow.md`](../../../artifacts/security/m5/m5_mutation_path_fix_flow.md)
- Fixtures: [`fixtures/security/m5/m5_mutation_path_fix_flow/`](../../../fixtures/security/m5/m5_mutation_path_fix_flow/)
- Producer: `aureline_content_safety::project_m5_mutation_path_fix_flow` /
  `frozen_m5_mutation_path_fix_flow_packet`
- Headless tool: `m5_mutation_path_fix_flow` (`--markdown`, `--clean`, `--validate <packet.json>`)

## Covered mutation paths

| Path | Token | Fix-flow mode | Affordance |
| --- | --- | --- | --- |
| Save | `save` | `previewable_diff` | Preview fix diff |
| Format | `format` | `previewable_diff` | Preview fix diff |
| Organize Imports | `organize_imports` | `previewable_diff` | Preview fix diff |
| AI Apply | `ai_apply` | `review_sheet` | Review proposed change |

The local-edit paths show a previewable diff of raw vs proposed bytes; AI-apply
proposes a change set reviewed in a review sheet. There is deliberately no
silent-rewrite mode: `validate` enforces that the resolved mode matches the
path's declared preview route, that `preview_required` and
`shows_raw_and_proposed` are true, and that `silent_byte_rewrite_blocked` and
`bytes_change_only_after_preview` are true on every path.

## Fix kinds

The shared detector runs over the content each path touches. The fix flow offers
exactly one fix per distinct detected class:

| Fix kind | Detector class | When offered |
| --- | --- | --- |
| `bidi` | `bidi_control` | A bidi-control codepoint is present. |
| `invisible` | `invisible_formatting` | A zero-width/invisible codepoint is present. |
| `confusable` | `mixed_script_confusable`, `whole_script_confusable` | A mixed-script identifier is present. |

A path with no findings offers no fix kind; a path with findings offers a fix for
each detected class. The `suspicious_excerpt_escaped` field is the shared
escaped/safe form (bidi and invisible codepoints become `\u{XXXX}`); it is never
the raw bytes and never masquerades as them — the raw content is reachable only
through `raw_content_ref`.

## Suppression scopes

A suppression is always recorded at one of these scopes, narrowest first, never as
hidden per-pane state:

| Scope | Token | Meaning |
| --- | --- | --- |
| Occurrence | `occurrence` | This single occurrence only. |
| File | `file` | Every occurrence in this file/artifact. |
| Workspace | `workspace` | Every occurrence in the workspace. |
| Admin policy | `admin_policy` | Every occurrence governed by an admin policy. |

Each `recorded_suppression` carries the actor, reason, timestamp, scope, the fix
kinds it covers, an optional `expires_at` that narrows it, a reachable
`audit_log_ref`, and `hidden_per_pane_state: false`. A suppression may only cover
fix kinds the path actually offers, and a clean path may not carry a suppression.

## Invariants

The producer guarantees, and `validate` enforces, that:

- Save, format, organize-imports, and AI-apply are all covered exactly once.
- Every path blocks silent suspicious-byte rewrites and only changes bytes after a
  preview.
- Every fix routes through a previewable diff or a review sheet matching the
  path's declared route.
- A path with findings offers a fix for each detected class; suspicious bytes are
  surfaced, never normalized away, and the escaped excerpt never masquerades as
  raw.
- Every recorded suppression is scope-aware, auditable (actor, reason, timestamp,
  audit log), and never hidden per-pane state; its covered fix kinds are a subset
  of those offered.
- The guard is preserved in product, exported review packets, and support handoff.

The packet is metadata only: no raw artifact bodies, raw provider payloads, or
credentials cross the export boundary — only opaque refs and the escaped
inspection excerpt.

## Consumers

The headless `m5_mutation_path_fix_flow` tool is the first CLI/headless consumer;
it emits the canonical support export, the Markdown summary, the clean fixture,
and validates any packet. Support, diagnostics, review-packet, and release tooling
read the machine-readable packet, fix flows, suppression records, and audit refs
directly rather than cloning prose.
