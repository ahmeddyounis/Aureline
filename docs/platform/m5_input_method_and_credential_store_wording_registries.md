# M5 input-method and credential-store-wording registries

This lane is the input-and-credential implement lane over the frozen
[M5 platform-fit matrix](./m5_platform_fit_contract.md). It turns the concrete *IME / dead-key / AltGr /
dictation / emoji / layout-switch* grammar of the `input_method` family and the *truthful, non-leaky
credential-store wording* grammar of the `credential_store_wording` family into registry resolvers that
produce export-safe, honest projections, so editor, terminal, settings, dialogs, prompts, auth, docs, CLI,
and support surfaces resolve one canonical text-entry and credential-wording truth instead of a per-surface,
hand-copied assumption.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_input_method_and_credential_store_wording_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/platform/m5-input-method-and-credential-store-wording-registries.schema.json`.
- **Domain schemas:** input-composition rows point at
  [`schemas/platform/m5-input-method-behavior.schema.json`](../../schemas/platform/m5-input-method-behavior.schema.json)
  and credential-wording rows point at
  [`schemas/platform/m5-file-path-and-reveal.schema.json`](../../schemas/platform/m5-file-path-and-reveal.schema.json)
  as their canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-input-method-and-credential-store-wording-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/platform/m5-input-method-and-credential-store-wording-registries/`
  (`composition_beta_narrowed.json`, `credential_preview_narrowed.json`).

## Two registries

1. **Input composition** (`resolve_input_composition_entry`) — validates IME composition, dead keys, AltGr,
   dictation, emoji input, and layout switching across editor, terminal, settings, dialogs, prompts, and
   support forms so text arrives intact and correctly segmented. A clean entry names a canonical registry
   token, a classified input-method stack, and an input-method role, covers the literal / canonical /
   accessible presentation forms, delivers committed text that matches the expected text for its stack,
   preserves command interpretation, shortcut routing, and trust / approval copy, and explains any
   unsupported-composition fallback. Otherwise it degrades honestly.
2. **Credential-store wording** (`resolve_credential_store_wording_entry`) — keeps credential-store copy
   truthful and generic by default, surfacing platform-specific detail only when it materially helps recovery,
   repair, or admin diagnosis. A clean entry names a classified credential-wording surface and provides the
   generic-wording / disclosure-route / truthful-and-non-leaky disclosure triple; wording that hides a
   plaintext downgrade, leaks a secret, or asserts false certainty degrades to
   `storage_claim_untruthful_or_leaky`.

## Input-method stack and composition-model reference

The input-method stack carries its canonical composition model, so the registry — never a hand-copied
per-surface assumption — is the single source of truth. `input_composition_matches_stack` rejects a committed
text that drifts from its expected text.

| input-method stack | composition model |
| --- | --- |
| macOS input methods | marked-text composition |
| Windows IME / TSF | tsf composition |
| Linux IBus / fcitx | preedit composition |

A committed text that drifts from the expected text degrades to `composed_text_corrupted_for_stack`, and a
composition that fights shortcut routing or rewrites trust copy degrades to
`command_or_trust_fidelity_not_preserved`, so a corrupted composition or a shortcut-composition fight can
never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Text entered through supported platform input methods arrives intact and correctly segmented across all
  claimed desktop profiles.** Clean input entries cover the `input_fidelity` / `command_stability`
  semantic-role families and the first editor / terminal / settings / dialog / prompt surfaces, a
  text-corrupted example degrades, and no clean input entry delivered corrupted text.
- **Shortcut handling and text composition do not fight each other under IME, dead-key, AltGr, or
  dictation-heavy workflows.** A command-or-trust-fidelity example and a behavior-not-bound example degrade, a
  clean bound input entry is present, and no clean entry lost command / trust fidelity.
- **Credential-store copy remains truthful, privacy-safe, and platform-correct without leaking false certainty
  or hidden storage downgrades.** Clean credential entries cover the settings / auth / support surfaces with
  full presentation-form coverage while providing the disclosure triple, and a wording that hides a plaintext
  downgrade degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_input_method_and_credential_store_wording_registries -- support-export
cargo run -p aureline-ui --example dump_m5_input_method_and_credential_store_wording_registries -- csv
cargo run -p aureline-ui --example dump_m5_input_method_and_credential_store_wording_registries -- report
cargo run -p aureline-ui --example dump_m5_input_method_and_credential_store_wording_registries -- input-composition-table
cargo run -p aureline-ui --example dump_m5_input_method_and_credential_store_wording_registries -- fixture-composition-beta-narrowed
cargo run -p aureline-ui --example dump_m5_input_method_and_credential_store_wording_registries -- fixture-credential-preview-narrowed
```
