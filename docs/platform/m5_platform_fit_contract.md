# M5 Platform-Fit Contract

This contract freezes what "native-feeling and truthful" means for Aureline on macOS, Windows, and
Linux. It is the canonical B139 source of truth for platform-convention, shortcut-notation,
file-path-reveal, theme/contrast live-change, credential-store-wording, and input-method behavior. Every
desktop-facing help page, docs page, screenshot generator, and support export consumes this contract
instead of copying platform-specific notes by hand.

- Frozen packet: `crates/aureline-ui/src/m5_platform_fit_matrix` (authoritative Rust validator).
- Combined matrix schema: `schemas/platform/m5-platform-fit-matrix.schema.json`.
- Domain schemas:
  - `schemas/platform/m5-shortcut-notation.schema.json` — platform-convention + shortcut-notation.
  - `schemas/platform/m5-file-path-and-reveal.schema.json` — file-path-reveal + theme/contrast
    live-change + credential-store-wording (terminology, appearance-response, and wording truth).
  - `schemas/platform/m5-input-method-behavior.schema.json` — input-method behavior.
- Checked evidence: `artifacts/release/m5-platform-fit-proof/support_export.json` (+ `matrix.csv`), report
  at `artifacts/platform/m5-platform-fit-matrix.md`, narrowed fixtures under
  `fixtures/platform/m5-desktop-fit/`.
- Emitter (only mint-from-truth path):
  `cargo run -p aureline-ui --example dump_m5_platform_fit_matrix -- <subcommand>`.

## Governed families

| Family | Canonical domain schema | Meaning |
| --- | --- | --- |
| `platform_convention` | `m5-shortcut-notation.schema.json` | Window-control placement, menu-bar behavior, title-bar convention, and system-chrome integration per platform. |
| `shortcut_notation` | `m5-shortcut-notation.schema.json` | Modifier glyphs, accelerator labels, and chord sequences that adapt per platform while the command ID stays stable. |
| `file_path_reveal` | `m5-file-path-and-reveal.schema.json` | File / path / reveal / save terminology matched to the host platform. |
| `theme_contrast_live_change` | `m5-file-path-and-reveal.schema.json` | Live theme / contrast / accent / text-scale response, or an explained fallback. |
| `credential_store_wording` | `m5-file-path-and-reveal.schema.json` | Truthful, non-leaky credential-store wording that names the host store. |
| `input_method` | `m5-input-method-behavior.schema.json` | IME, dead keys, AltGr, dictation, emoji, and layout switching that preserve text and trust fidelity. |

## Platform-fit role vocabulary

The single controlled acceptance-criteria vocabulary consumers bind to, so no surface reinvents a
parallel word: `shortcut`, `window_menu`, `path_terminology`, `appearance`, `credential_wording`,
`input_fidelity`, `command_stability`.

The command-carrying roles — `shortcut`, `window_menu`, `input_fidelity`, `command_stability` — must
preserve command meaning, permission meaning, focus order, text fidelity, and trust semantics as the
platform-specific presentation adapts. The truthful-mapping roles — `path_terminology`, `appearance`,
`credential_wording` — are host-matched presentation rather than command-carrying adaptation.

## Platform-native reference

| Concern | macOS | Windows | Linux |
| --- | --- | --- | --- |
| Modifier glyphs | ⌘ ⌥ ⌃ ⇧ | Ctrl Alt Shift Win | Ctrl Alt Shift Super |
| Reveal verb | Reveal in Finder | Show in Explorer | Open Containing Folder |
| Window controls | Traffic lights (left) | Caption buttons (right) | Varies by DE (right, typical) |
| Menu bar | Global menu bar | In-window menu | In-window / global (DE) |
| Credential store | Keychain | Credential Manager | Secret Service (libsecret) |

These labels are illustrative of the platform-native words the registry adapts to; the packet is the
authoritative source that consumers read at build/screenshot time.

## Hard invariants (all MUST be false on every row)

- `platform_wording_changes_command_or_permission_meaning`
- `hides_primary_action_only_in_os_chrome`
- `falls_back_to_plaintext_secret_storage_silently`
- `input_method_corrupts_text_or_trust_fidelity`
- `screenshot_or_docs_mislabels_shortcut_or_path_verb`

## Automatic claim narrowing

Claim publication and support/export paths narrow desktop-fit claims automatically when the B139 registry
is missing, stale, or not yet qualified: `proof_freshness.auto_narrow_on_stale` is `true`, every family
carries its degraded reasons, and the narrowed fixtures
(`theme_contrast_live_change_beta_narrowed.json`, `input_method_preview_narrowed.json`) show a family
dropping to Beta / Preview while every family stays visible — never hidden behind generic copy.
