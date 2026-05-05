# Settings row contract

This contract freezes how Aureline settings surfaces render and
interact with one **settings row**. The goal is row-granularity
inspectability: a reviewer can inspect a single row and understand the
active value, the winning source, any override or lock state, and the
reset/diff path without reading hidden configuration files.

Companion artifacts:

- [`/schemas/settings/settings_row_state.schema.json`](../../schemas/settings/settings_row_state.schema.json)
  - boundary schema for one settings row state record (a settings
    specialization of the shared `field_row_record` contract).
- [`/fixtures/settings/setting_rows/`](../../fixtures/settings/setting_rows/)
  - worked settings-row records covering precedence, search landing,
    policy locks, revocable overrides, and high-risk preview-first
    settings.

This contract composes with:

- [`/docs/ux/field_row_and_value_source_contract.md`](../ux/field_row_and_value_source_contract.md)
  and [`/schemas/ux/field_row.schema.json`](../../schemas/ux/field_row.schema.json)
  for the shared row anatomy and the source-pill + effective-value
  inspector structure.
- [`/docs/settings/schema_registry_seed.md`](./schema_registry_seed.md) plus
  the settings schema family (`setting_definition_row`,
  `effective_setting_record`, `write_intent_packet`, and
  `precedence_resolution_packet`) for settings meaning, precedence, lock,
  and preview truth.
- [`/docs/settings/precedence_lock_and_write_scope_contract.md`](./precedence_lock_and_write_scope_contract.md)
  for precedence, shadow-chain, lock reasons, and reset targeting rules.
- [`/docs/admin/policy_explainability_contract.md`](../admin/policy_explainability_contract.md)
  for policy source inspection, deep-link behavior, and export safety.

If this document disagrees with the UX field-row contract, the UX
field-row contract wins and this document must be updated in the same
change. If this document disagrees with the settings ADR and schema
registry seed, those documents win and this contract must be updated in
the same change.

## Core invariants

1. **One row is enough to answer the question.**
   - Every settings row MUST render a value projection and a visible
     source pill.
   - Every settings row MUST offer a one-interaction path to the
     effective-value inspector for that row.
2. **No hidden precedence chains.**
   - The effective-value inspector MUST expose the full contributing
     source chain (shadow chain) in stable precedence order.
   - Settings search MUST NOT invent a separate “search row model”; it
     opens the same exact row projection the settings surface renders
     in-place.
3. **Locks are explainable, not just disabled.**
   - A locked row MUST name the typed lock or constraint reason in its
     row-level state summary and expose an “Explain why / View policy”
     inspection path.
   - A locked row MUST remain inspectable and copy-safe without implying
     editability (no enabled mutating primary action).
4. **Exact-row deep links are stable.**
   - An exact-row deep link MUST land on one row, focus that row, and
     reveal the source pill and lock/override explanation.
   - The canonical `setting_id` is the deep-link anchor; legacy ids
     redirect to the canonical id via the alias table before landing.
5. **Search lands on one row and highlights it.**
   - A settings-search hit MUST open the exact-row deep link and briefly
     highlight the row container and the matching fragment. The highlight
     MUST be accessible (not color-only) and MUST NOT change the meaning
     of the row’s lock/override state.
6. **Reset and diff are row-local and typed.**
   - Reset uses the shadow chain to choose the next-lower non-empty
     contributor; surfaces MUST name the target scope being reset.
   - Diff and preview surfaces MUST use the typed change-preview packet
     output and MUST NOT invent a private diff format for high-risk
     settings.

## Row anatomy (pixel → contract fields)

Every settings row has the same minimum anatomy. The table maps each
row element to the shared row contract fields and to the upstream
settings schema-of-record fields that feed it.

| Row element | Row contract field(s) | Settings schema source |
|---|---|---|
| **Label** | `label.text`, `label.required_class` | `setting_definition_row.summary` (label), plus any required/optional cue declared by the owning surface |
| **Plain-language description** | `label.description` and/or `help.help_text` | `setting_definition_row.description` (long) and `setting_definition_row.summary` (short) |
| **Value control** | `value.*` plus `primary_action` / `secondary_actions` (`edit_value`, `stage_change`, `apply_now`) | `setting_definition_row.value_type` (shape) + `effective_setting_record.value` (projection) + `write_intent_packet` (staging/preview/apply posture) |
| **Source pill** | `source_pill.*` | `effective_setting_record.resolved_scope`, `effective_setting_record.source_label`, `effective_setting_record.control_stack.control_authority` |
| **Lock / status icon** | `state.lock_state`, `state.apply_posture_class`, `state.editability_class`, `state.state_summary` | `effective_setting_record.lock_state`, `effective_setting_record.lock_reason`, `effective_setting_record.write_intent`, `effective_setting_record.write_denial_reason` |
| **Reset / diff affordance** | `secondary_actions[]` (`reset_value`, `open_preview`) + `extension_slots.structured_diff_preview` | `precedence_resolution_packet.shadow_chain` (reset target) + `change_preview_packet` (diff/preview) |
| **Help / docs link** | `help.*` plus optional `secondary_actions[]` (`show_help`, `open_source`) | `setting_definition_row.help_doc_ref` and any lane-specific doc/evidence refs |

## Winning source and effective value visibility

For every settings row:

- The effective (winning) value MUST be visible in-row as a redaction-aware
  value preview, or reachable via the effective-value inspector in one
  interaction.
- The winning source MUST be visible in-row via the source pill, or
  reachable via the effective-value inspector in one interaction.
- The effective-value inspector MUST disclose when a source is present
  but shadowed, constrained, blocked, stale, or uninspectable for the
  current user.

## Policy-locked rows and preserved shadow values

When a policy lock or constraint applies:

- The row MUST render the enforced effective value and a policy-toned
  source pill.
- The row MUST NOT present enabled mutating actions. The primary action
  becomes inspection-oriented (for example “View policy” or “Explain
  why”).
- Any user/workspace/profile “shadow value” remains visible in the
  effective-value inspector as a shadowed source entry. Surfaces MAY
  show shadow values in the inspector or diff preview as long as they are
  explicitly labeled as shadowed and never presented as the effective
  setting.
- If the user can still edit a shadow value (for example, storing a
  personal preference that policy currently caps), the surface MUST make
  it clear that the edit will not change the effective value while the
  policy ceiling remains active.

## Exact-row deep links and search highlight

An exact-row deep link:

- lands on one row (not a section anchor);
- focuses the row and reveals the source pill;
- reveals the lock/override explanation; and
- preserves copy-safe inspection behavior for locked and inspect-only
  rows.

Search uses the same row record:

- A search hit’s `search_projection` cites the matching fragments and
  opens the same `exact_row_deep_link` the settings surface uses.
- On open, the surface briefly highlights the row and the matching
  fragment. Highlighting is an attention aid only; it must not collapse
  distinct states (locked vs read-only vs staged vs disabled).

## Worked examples

See the fixtures in [`/fixtures/settings/setting_rows/`](../../fixtures/settings/setting_rows/).

