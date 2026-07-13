# M5 Shortcut-Notation and Command-Label Registries

This is the first implement lane over the frozen
[M5 platform-fit matrix](./m5_platform_fit_contract.md). It turns the concrete keyboard-notation grammar of
the **shortcut-notation** platform-fit family into registry resolvers that produce export-safe, honest
projections. The [`M5ShortcutNotationRegistriesPacket`][packet] carried by this lane is canonical for M5
shortcut-notation and command-label truth; later help / docs / screenshots / support exports consume it
directly rather than restating shortcut notation by hand.

The Rust validator in `crates/aureline-ui/src/m5_shortcut_notation_and_command_label_registries` is the
authoritative gate; the [schema](../../schemas/platform/m5-shortcut-notation-and-command-label-registries.schema.json)
documents the shape.

## What the resolvers guarantee

* **Platform-native shortcut notation from one registry.** `resolve_shortcut_notation_entry` refuses to read
  as a clean, registry-bound notation entry unless it names a canonical registry token, a stable command ID,
  a classified host platform, a shortcut-notation role, covers every notation form (visual / spoken /
  searchable), renders notation that matches the host's modifier convention, preserves the stable command ID,
  and explains any OS-reserved fallback.
* **Host-matched modifier convention.** Each host platform carries its canonical primary modifier, and
  `notation_matches_host` rejects a macOS entry rendered with `Ctrl` / `Alt` text or a Windows / Linux entry
  rendered with `⌘` / `⌥` glyphs, so a mislabeled notation degrades to `notation_mislabeled_for_host`.
* **Same command discoverable by ID, label, and shortcut.** `resolve_command_label_mapping_entry` requires the
  command to be discoverable by stable command ID, human label, and platform-appropriate shortcut text, and
  degrades to `discovery_triple_incomplete` when any leg is missing, so a screenshot or tutorial cannot
  reintroduce incorrect notation.

## Platform-native notation reference

| Concept | macOS | Windows | Linux |
| --- | --- | --- | --- |
| Primary modifier | `⌘` (Command) | `Ctrl` | `Ctrl` |
| Option / Alt | `⌥` | `Alt` | `Alt` |
| Control | `⌃` | `Ctrl` | `Ctrl` |
| Shift | `⇧` | `Shift` | `Shift` |
| Save | `⌘S` | `Ctrl+S` | `Ctrl+S` |
| Command palette | `⌘⇧P` | `Ctrl+Shift+P` | `Ctrl+Shift+P` |
| Notation style | glyphs | modifier names | modifier names |

The stable command ID (for example `command.file.save`) never changes across platforms; only the rendered
notation and the human label adapt.

## Acceptance criteria (proven by resolved examples)

1. The same command can be discovered by stable command ID, human label, and platform-appropriate shortcut
   text on every claimed desktop profile.
2. User-visible keyboard notation and help / screenshot content stay consistent with the active platform
   without changing command semantics.
3. Regression suites fail when a platform surface shows the wrong modifier notation, label mapping, or
   reserved-key explanation.

## Hard invariants

* Platform-specific notation never changes command or permission meaning.
* A primary command is never hidden only in OS chrome (menus / title bars).
* Notation is never hand-copied per platform instead of tracing to the registry.
* A screenshot or docs page never mislabels a shortcut.

## Generating docs / help / screenshots from the registry

`M5ShortcutNotationRegistriesPacket::render_platform_help_notation_table` emits a per-platform
command / notation table from the clean, registry-bound notation entries, so docs and tutorials render the
same truth the resolvers produced rather than a hand-copied screenshot.

## Emitter

The headless emitter is the only mint-from-truth path for the checked-in artifacts:

```text
cargo run -p aureline-ui --example dump_m5_shortcut_notation_and_command_label_registries -- support-export
cargo run -p aureline-ui --example dump_m5_shortcut_notation_and_command_label_registries -- report
cargo run -p aureline-ui --example dump_m5_shortcut_notation_and_command_label_registries -- csv
cargo run -p aureline-ui --example dump_m5_shortcut_notation_and_command_label_registries -- help-table
cargo run -p aureline-ui --example dump_m5_shortcut_notation_and_command_label_registries -- fixture-docs-help-beta-narrowed
cargo run -p aureline-ui --example dump_m5_shortcut_notation_and_command_label_registries -- fixture-onboarding-preview-narrowed
cargo run -p aureline-ui --example dump_m5_shortcut_notation_and_command_label_registries -- validate
```

[packet]: ../../crates/aureline-ui/src/m5_shortcut_notation_and_command_label_registries/mod.rs
