# M5 file-path-presentation and native-window / menu registries

This lane is the file-and-window implement lane over the frozen
[M5 platform-fit matrix](./m5_platform_fit_contract.md). It turns the concrete *file / path / reveal /
open-save terminology* grammar of the `file_path_reveal` family and the *native window-chrome and menu
availability* grammar of the `platform_convention` family into registry resolvers that produce export-safe,
honest projections, so shell, settings, docs, onboarding, CLI, and support surfaces resolve one canonical
path and window / menu truth instead of a per-surface, hand-copied string.

- **Canonical Rust module:** `crates/aureline-ui/src/m5_file_path_reveal_and_native_window_menu_registries`
  (the authoritative validator).
- **Combined schema:**
  `schemas/platform/m5-file-path-reveal-and-native-window-menu-registries.schema.json`.
- **Domain schema:** every row points at
  [`schemas/platform/m5-file-path-and-reveal.schema.json`](../../schemas/platform/m5-file-path-and-reveal.schema.json)
  as its single canonical terminology domain contract.
- **Checked proof:** `artifacts/release/m5-file-path-reveal-and-native-window-menu-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/platform/m5-file-path-reveal-and-native-window-menu-registries/`
  (`docs_help_beta_narrowed.json`, `reveal_preview_narrowed.json`).

## Two registries

1. **File-path presentation** (`resolve_file_path_presentation_entry`) — adopts host-appropriate file / path
   separators, drive or mount vocabulary, reveal verbs, and open / save terminology while keeping the
   literal-versus-canonical path truth explicit. A clean entry names a canonical registry token, a classified
   host platform, and a file-path-reveal role, covers the host-styled / canonical / accessible presentation
   forms, renders a separator and reveal verb that match the host convention, preserves the canonical path
   truth, and explains any unavailable-reveal fallback. Otherwise it degrades honestly.
2. **Native window / menu action** (`resolve_window_menu_action_entry`) — ensures high-frequency actions stay
   reachable from product surfaces and commands rather than hidden only in OS menus or title-bar affordances,
   and preserves native window-chrome and menu phrasing. A clean entry names a classified product action
   surface and provides the stable-ID / in-product-surface / command reachability triple; an action reachable
   only through OS chrome degrades to `reachable_only_in_os_chrome`.

## Platform-native path and reveal reference

The host platform carries its canonical separator and reveal verb, so the registry — never a hand-copied
per-surface string — is the single source of truth. `path_presentation_matches_host` rejects a drifted entry.

| host platform | path separator | reveal verb |
| --- | --- | --- |
| macOS | `/` | Reveal in Finder |
| Windows | `\` | Show in Explorer |
| Linux | `/` | Open Containing Folder |

A Windows entry rendered with a forward-slash path or a `Reveal in Finder` verb, and a macOS or Linux entry
rendered with a backslash path or a `Show in Explorer` verb, degrade to `path_or_reveal_mislabeled_for_host`
so a mislabeled screenshot or docs page can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **File, folder, workspace, and reveal flows use host-correct terms and separators across UI, docs, logs,
  and support exports.** Clean file-path entries cover the `path_terminology` / `command_stability`
  semantic-role families and the first file-open / save / reveal / breadcrumb / help surfaces, a hand-copied
  example degrades, and no clean entry is unbound.
- **No critical action is reachable only through OS chrome or menu-bar affordances on any claimed desktop
  profile.** Clean window / menu entries cover the command-palette / toolbar / command-list product surfaces
  with full presentation-form coverage while providing the reachability triple, and a menu-only action
  degrades.
- **Cross-platform review fixtures prove path language, reveal verbs, and window / menu behavior are correct
  and stable.** A mislabeled-path example and a menu-only-action example both degrade, clean entries trace to
  the registry, and no clean entry is mislabeled for its host.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_file_path_reveal_and_native_window_menu_registries -- support-export
cargo run -p aureline-ui --example dump_m5_file_path_reveal_and_native_window_menu_registries -- csv
cargo run -p aureline-ui --example dump_m5_file_path_reveal_and_native_window_menu_registries -- report
cargo run -p aureline-ui --example dump_m5_file_path_reveal_and_native_window_menu_registries -- path-reveal-table
cargo run -p aureline-ui --example dump_m5_file_path_reveal_and_native_window_menu_registries -- fixture-docs-help-beta-narrowed
cargo run -p aureline-ui --example dump_m5_file_path_reveal_and_native_window_menu_registries -- fixture-reveal-preview-narrowed
```
