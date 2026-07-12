# M5 Combobox and Checkbox-Radio-Switch Controls

- Packet: `m5-combobox-checkbox-radio-switch-controls:stable:0001`
- Label: `M5 combobox and checkbox-radio-switch controls with filterable selection, selected-value and source-of-value disclosure, explicit immediate-versus-deferred toggle semantics, provenance carried across surfaces, and locked / read-only truth aligned across settings, provider, admin, request, and entry surfaces`
- Consumer surfaces: 6
- Combobox value sources: canonical_option, filtered_subset, free_text_allowed, remote_backed, custom_unverified, source_unknown
- Toggle semantics: checkbox_immediate, checkbox_deferred, radio_exclusive, switch_immediate, tristate_indeterminate, semantics_unknown
- Apply timings: applies_immediately, deferred_until_save, staged_in_review, requires_confirmation, apply_blocked, timing_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **forms_ui**: `stable`
  - Owner: Forms surface owner
  - Scope: The forms surface renders a combobox that discloses its canonical selected value and a checkbox that applies immediately; both degrade honestly when the value source is unresolved or a switch is blurred with a deferred checkbox
  - Combobox examples: 2 / toggle examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface keeps a filterable model combobox filterable and a deferred-until-save checkbox distinct from an immediate switch, and keeps a locked toggle distinct rather than behind generic disabled chrome; both degrade honestly when the filter is missing or a lock hides behind disabled
  - Combobox examples: 2 / toggle examples: 2
- **entry_ui**: `stable`
  - Owner: Start-center entry owner
  - Scope: The start-center entry surface offers a default-provenance region combobox and an exclusive theme radio; both degrade honestly when a policy provenance is undisclosed or a radio group loses its exclusivity
  - Combobox examples: 2 / toggle examples: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface tags a remote-backed catalog combobox with its support class and keeps a switch's immediate semantics explicit; both degrade honestly when an unverified value is presented as canonical without a tag or the toggle semantics are unresolved
  - Combobox examples: 2 / toggle examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved value source, provenance, lock, and apply-timing truth, so an unstable-keyboard combobox or an undisclosed toggle state is visible in evidence rather than hidden behind generic disabled chrome
  - Combobox examples: 2 / toggle examples: 2
- **product_ui**: `stable`
  - Owner: In-product control owner
  - Scope: In-product surfaces reuse the same filterable-set, provenance, and immediate-versus-deferred grammar a user sees in settings and entry, always offering the command-backed detail path and degrading honestly when the trace path is missing or one-of-many versus multi-select is ambiguous
  - Combobox examples: 2 / toggle examples: 2
