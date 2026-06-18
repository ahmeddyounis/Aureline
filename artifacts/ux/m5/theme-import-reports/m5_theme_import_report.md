# M5 imported-theme mapping & rollback report

Generated from the seeded report in
[`crate::theme_import_reports`](../../../../crates/aureline-shell/src/theme_import_reports/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- markdown > \
  artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md
```

- Report id: `shell:m5_theme_import_report:v1:default`
- Rows: 5
- Ecosystems covered: vscode, jetbrains, zed, vim, textmate
- Translated slots: 152/214
- Unresolved slots: 15
- Preview before apply: true
- Every import reversible: true
- No overclaimed parity: true
- Unresolved counts disclosed: true
- No raw theme content: true
- Generated at: `2026-06-17T00:00:00Z`

## Outcome summary

| Outcome | Count |
|---|---:|
| preview_ready | 0 |
| applied | 1 |
| applied_with_warnings | 1 |
| blocked | 1 |
| rolled_back | 1 |
| cancelled | 0 |
| policy_denied | 0 |
| review_required | 1 |
| **total** | **5** |

## Imported themes

| Theme | Source | Outcome | Translated | Unresolved | Syntax | Parity | Reversible |
|---|---|---|---:|---:|---:|---|:---:|
| VS Code dark theme import | `vscode` 1.97.0 | `applied` | 48/48 | 0 | 100% | `claimed_with_report` | yes |
| JetBrains Darcula scheme import | `jetbrains` 2024.3 | `applied_with_warnings` | 40/52 | 2 | 78% | `partial_claim_with_gaps` | yes |
| Zed theme import (rolled back) | `zed` 0.160.0 | `rolled_back` | 36/44 | 1 | 89% | `denied_unresolved_or_blocked` | yes |
| Vim colorscheme import (review required) | `vim` 0.10.2 | `review_required` | 22/40 | 8 | 56% | `partial_claim_with_gaps` | yes |
| TextMate tmTheme import (blocked) | `textmate` 2.0 | `blocked` | 6/30 | 4 | 27% | `denied_unresolved_or_blocked` | yes |

## VS Code dark theme import (`vscode`)

A widely used VS Code dark theme maps one-to-one onto Aureline's semantic and syntax tokens. The apply is checkpointed, so the user can restore the prior appearance if they change their mind.

- Source: Visual Studio Code 1.97.0 (`source:vscode:github_dark`)
- Outcome: `applied`
- Parity: `claimed_with_report` — Every semantic, component, and syntax slot translated to a native token with no unresolved slots; full parity is claimed and backed by this report.
- Translated slots: 48/48
- Unresolved slots: 0
- Syntax coverage: 100% (60/60 scopes)
- Rollback: `restore_appearance_checkpoint` (`rollback:theme-import:github-dark`)

## JetBrains Darcula scheme import (`jetbrains`)

A JetBrains Darcula scheme translates its editor colors cleanly, but a handful of IDE-specific slots have no native target. Those are substituted with disclosed fallbacks or left unresolved, and the warnings ride along with the apply.

- Source: IntelliJ IDEA 2024.3 (`source:jetbrains:darcula`)
- Outcome: `applied_with_warnings`
- Parity: `partial_claim_with_gaps` — Most slots translated; eight fell back to disclosed neutral defaults and two remained unresolved. Parity is claimed only partially, with the gaps listed.
- Translated slots: 40/52
- Unresolved slots: 2
- Syntax coverage: 78% (50/64 scopes)
- Rollback: `restore_appearance_checkpoint` (`rollback:theme-import:darcula`)
- Compatibility note: Editor scheme colors translated, but two IDE-specific scopes and the gutter accent fell back to disclosed neutral defaults; review before relying on them.
- Unresolved slots:
  - `source:jetbrains:darcula:slot:inline_hint` — Inline parameter-hint background has no semantic target and is left unresolved rather than guessed. (fallback disclosed: true)
  - `source:jetbrains:darcula:slot:breadcrumb_bg` — Breadcrumb background tint is unresolved; the shell keeps its native chrome rather than approximate it. (fallback disclosed: true)
- Known deviations:
  - `deviation:jetbrains.gutter_accent` — The gutter change-accent is substituted with the native diff token, not the source accent. (recoverable: true)

## Zed theme import (rolled back) (`zed`)

An applied Zed theme proved semantically misleading: it recolored a protected trust cue with color alone. The honesty check blocked the mapping and the import was rolled back to the checkpoint, demonstrating that imported visual customizations stay reversible.

- Source: Zed 0.160.0 (`source:zed:one_dark_remix`)
- Outcome: `rolled_back`
- Parity: `denied_unresolved_or_blocked` — The applied import recolored a trust/severity cue using color alone; the honesty check blocked it and the import was rolled back. Parity is denied.
- Translated slots: 36/44
- Unresolved slots: 1
- Syntax coverage: 89% (52/58 scopes)
- Rollback: `restore_appearance_checkpoint` (`rollback:theme-import:one-dark-remix`)
- Compatibility note: This theme used color alone to signal a trust state, which would have hidden a protected cue; Aureline rolled the import back rather than ship a misleading appearance.
- Unresolved slots:
  - `source:zed:one_dark_remix:slot:status_accent` — The status accent slot could not be resolved without overriding a protected trust cue, so it is left unresolved. (fallback disclosed: false)
- Known deviations:
  - `deviation:zed.trust_cue_color_only` — The source theme expressed a trust state with color only; Aureline keeps the non-color cue and refuses the override. (recoverable: false)

## Vim colorscheme import (review required) (`vim`)

A Vim colorscheme maps its syntax scopes but leaves the IDE chrome unresolved. The import is held for review with the unresolved slots listed, so the user decides them before anything applies.

- Source: Neovim 0.10.2 (`source:vim:gruvbox`)
- Outcome: `review_required`
- Parity: `partial_claim_with_gaps` — The terminal-oriented colorscheme maps cleanly for syntax but leaves eight UI slots unresolved; review is required before applying.
- Translated slots: 22/40
- Unresolved slots: 8
- Syntax coverage: 56% (28/50 scopes)
- Rollback: `reopen_import_review` (`rollback:theme-import:gruvbox:reopen`)
- Compatibility note: A Vim colorscheme covers syntax scopes but not the full IDE chrome; the unresolved chrome slots are listed so they are not silently defaulted.
- Unresolved slots:
  - `source:vim:gruvbox:slot:statusline` — The statusline palette has no direct chrome target and is left for review. (fallback disclosed: true)
  - `source:vim:gruvbox:slot:tabline` — The tabline palette is unresolved; the shell keeps its native tab chrome pending review. (fallback disclosed: true)
- Known deviations:
  - `deviation:vim.chrome_scope` — Vim colorschemes target terminal syntax, not IDE chrome, so chrome slots need explicit review. (recoverable: true)

## TextMate tmTheme import (blocked) (`textmate`)

A legacy TextMate tmTheme exposes only editor foreground and background. Rather than render a plausible-looking but mostly unmapped theme and imply parity, the migration center blocks the import and the preview is discarded.

- Source: TextMate 2.0 (`source:textmate:monokai_classic`)
- Outcome: `blocked`
- Parity: `denied_unresolved_or_blocked` — The legacy tmTheme format exposes almost no semantic slots; the import is blocked rather than shipped as a plausible-looking but unmapped theme.
- Translated slots: 6/30
- Unresolved slots: 4
- Syntax coverage: 27% (10/36 scopes)
- Rollback: `discard_preview` (`rollback:theme-import:monokai_classic:discard`)
- Compatibility note: Only raw editor foreground and background could be read; the rest of the design system has no source slots, so a parity claim would be misleading.
- Unresolved slots:
  - `source:textmate:monokai_classic:slot:semantic_tokens` — The format carries no semantic token slots, so they cannot be resolved. (fallback disclosed: false)
  - `source:textmate:monokai_classic:slot:diff_tokens` — Diff tokens are absent from the source and are left unresolved rather than defaulted silently. (fallback disclosed: false)
- Known deviations:
  - `deviation:textmate.format_coverage` — The tmTheme format predates semantic theming; most of the design system has no source to map from. (recoverable: false)

