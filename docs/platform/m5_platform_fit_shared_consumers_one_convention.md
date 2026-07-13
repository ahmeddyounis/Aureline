# M5 Platform-Fit Shared Consumers: One Convention Across Surfaces

**Status:** Stable · B139 consumer-adoption lane
**Module:** `aureline_ui::m5_platform_fit_shared_consumers_one_convention_across_surfaces`
**Schema:** [`schemas/platform/m5-platform-fit-shared-consumers.schema.json`](../../schemas/platform/m5-platform-fit-shared-consumers.schema.json)
**Proof:** [`artifacts/release/m5-platform-fit-shared-consumers-proof/`](../../artifacts/release/m5-platform-fit-shared-consumers-proof/)
**Fixtures:** [`fixtures/platform/m5-platform-fit-shared-consumers/`](../../fixtures/platform/m5-platform-fit-shared-consumers/)

This lane is the consumer-adoption capstone for the six reusable platform-fit families frozen in the
[platform-fit matrix](m5_platform_fit_contract.md) and implemented by the shortcut-notation,
file-path-reveal / native-window-menu, live theme / contrast, and input-method / credential-store
registries. It binds each shared platform-fit family to the concrete Start Center / shell, settings,
auth / credential, input, docs / help, onboarding, CLI / export, support-export, and general product
consumers that render it, and proves — by fixtures, not screenshots — that the same platform-fit
object presents the **same convention** everywhere it appears.

## Why this exists

The batch already hardens native desktop integration, appearance objects, shell continuity, and
credential / auth surfaces, but it left the platform-fit *conventions* — shortcut notation, window /
menu behavior, file / path / reveal terminology, live theme / contrast response, credential-store
wording, and IME / dead-key / AltGr / dictation / emoji / layout input — too implicit. This lane wires
those rules into the daily-driver surfaces so platform correctness cannot drift between handoff lanes
and ordinary work: every representative desktop-facing surface consumes the shared registry rather
than private wording or presentation logic.

## The three honesty axes

1. **Reuse.** Each of the six platform-fit families is adopted by **at least two distinct consumers**,
   so a family is proven shared infrastructure rather than a one-surface fork of shortcut notation,
   path wording, appearance response, credential copy, or input handling.
2. **One convention / no drift.** For a given platform-fit object every consumer surface presents the
   identical six-word grammar — `platform_fit_role_word`, `family_word`, `registry_reference_word`,
   `host_platform_word`, `surface_context_word`, and `command_identity_word`. The role word must be a
   token from the frozen `M5PlatformFitRole` vocabulary (`shortcut`, `window_menu`, `path_terminology`,
   `appearance`, `credential_wording`, `input_fidelity`, `command_stability`), so no surface rewrites a
   role in its own words. A surface may narrow *how much* it shows across desktop, compact, remote, and
   exported representations, but never reword the grammar per surface — and a role that carries
   shortcut, window-menu, input-fidelity, or command-stability meaning may never let a platform label
   change command or permission meaning, hide a primary action only in OS chrome, silently fall back to
   plaintext credential storage, corrupt input text or trust fidelity, or mislabel a shortcut or
   path / reveal verb.
3. **Map back to one family.** Support and CLI/export consumers point at the canonical per-domain
   schema and the frozen matrix by id, so an exported packet always maps a platform-fit surface back to
   one shared contract family.

## Guardrails (each MUST be false on every binding)

- `platform_wording_changes_command_or_permission_meaning`
- `hides_primary_action_only_in_os_chrome`
- `falls_back_to_plaintext_credential_storage_silently`
- `input_method_corrupts_text_or_trust_fidelity`
- `screenshot_or_docs_mislabels_shortcut_or_path_verb`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason, the
preserved grammar, and the next action; a remote representation names its remote source, and an
exported representation names its export-safe detail boundary rather than collapsing the object out of
view. Stale proof or a missing canonical reference **narrows** the claim via a
`PlatformFitSharedConsumersDowngradeTrigger` rather than hiding the family.

## Seeded coverage

Six platform-fit objects — one per family — fan out to eighteen consumer bindings covering all nine
consumers and all four representations:

| Family | Role | Consumers |
| --- | --- | --- |
| `platform_convention` | `window_menu` | shell, docs/help, CLI export |
| `shortcut_notation` | `shortcut` | shell, settings, support export |
| `file_path_reveal` | `path_terminology` | settings, docs/help, CLI export |
| `theme_contrast_live_change` | `appearance` | shell, settings, product |
| `credential_store_wording` | `credential_wording` | auth, settings, support export |
| `input_method` | `input_fidelity` | input, onboarding, product |

Two checked narrowed fixtures prove the grammar survives compact / remote and exported / redacted
forms without rewording.

## Regenerating the proof

```text
cargo run -p aureline-ui --example dump_m5_platform_fit_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_platform_fit_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_platform_fit_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_platform_fit_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_platform_fit_shared_consumers -- fixture-exported-redaction-narrowed
```

The example is the only mint-from-truth path for the checked support export, matrix CSV, Markdown
summary, and narrowed fixtures; the module tests fail if any drifts from the seed builder.
