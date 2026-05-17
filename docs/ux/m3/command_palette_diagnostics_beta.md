# Command Palette Diagnostics Beta

This page is the reviewer contract for the beta command-palette
diagnostics projection. The projection lives in
[`crates/aureline-shell/src/palette/diagnostics_beta.rs`](../../../crates/aureline-shell/src/palette/diagnostics_beta.rs)
and is emitted by the headless inspector
[`aureline_shell_command_palette_diagnostics`](../../../crates/aureline-shell/src/bin/aureline_shell_command_palette_diagnostics.rs).

The checked-in JSON is not hand-authored. It is generated from the
same descriptor-backed path that builds palette rows, diagnostics
sheets, invocation preview sheets, deep-link review packets, support
exports, and parity examples.

## Authoritative Artifacts

| Artifact | Purpose |
| --- | --- |
| [`fixtures/ux/m3/command_palette_diagnostics/page.json`](../../../fixtures/ux/m3/command_palette_diagnostics/page.json) | Full diagnostics pack with command rows and deep-link review summaries. |
| [`fixtures/ux/m3/command_palette_diagnostics/support_export.json`](../../../fixtures/ux/m3/command_palette_diagnostics/support_export.json) | Metadata-safe support export that omits raw query text and private argument values. |
| [`artifacts/commands/m3/palette_parity_examples.json`](../../../artifacts/commands/m3/palette_parity_examples.json) | Examples showing the same command truth across palette, menu, keybinding, onboarding hint, CLI, AI, and support surfaces. |
| [`fixtures/commands/m3/command_parity/report.json`](../../../fixtures/commands/m3/command_parity/report.json) | Companion command-parity report consumed by the same surface families. |

## Row Contract

Every command row in the diagnostics pack must expose:

- canonical `command_id`;
- display label and category/path;
- origin/source badge;
- current shortcut, or the literal display value `Unassigned`;
- dominant side-effect class;
- descriptor-owned automation labels and display cues;
- availability class and disabled reason code when material;
- preview pane projected from the descriptor and preflight result;
- action footer with primary run/preview, split or alternate open,
  copy command ID, copy CLI/headless form, add to recipe, and why-not
  automatable actions.

Disabled rows must stay discoverable and must offer a diagnostics
sheet. Preview-required or approval-required rows route through the
invocation preview sheet before apply.

## Covered Cases

The seeded pack covers these cause families:

| Cause family | Representative reason |
| --- | --- |
| `enabled_direct` | Direct run path for an enabled command. |
| `trust` | `workspace_trust_restricted`. |
| `policy` | `policy_blocked_in_context`. |
| `missing_dependency` | `execution_context_unavailable`. |
| `degraded_provider` | `required_provider_unlinked`. |
| `wrong_focus` | `required_argument_unresolved`. |
| `preview_or_approval` | Invocation preview and approval pending posture. |
| `unsupported_surface` | Deep-link review for a surface outside the command descriptor scope. |

The deep-link rows deliberately use the same descriptor, enablement,
preview, approval, trust, and policy review paths as palette dispatch.
A copied command ID, copied CLI form, or external command link can
prefill a review path, but cannot bypass revalidation.

## Privacy And Performance

Palette history remains local-first and privacy-scoped. The support
export quotes command IDs, disabled reason codes, cause families, and
reopen refs, but omits raw query text, raw argument values, and private
workspace paths.

The diagnostics pack carries the protected warm-open budget:
`target_ms: 50` with rows sourced from recent and lexical providers
first while semantic rows stream in later.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- pack \
  > fixtures/ux/m3/command_palette_diagnostics/page.json
cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- support-export \
  > fixtures/ux/m3/command_palette_diagnostics/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- parity-examples \
  > artifacts/commands/m3/palette_parity_examples.json
```

## Verification

```sh
cargo test -p aureline-shell --test command_palette_diagnostics_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_command_palette_diagnostics -- validate
```
