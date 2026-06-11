# Fixtures: M5 activation budget

This directory contains fixture metadata for the `m5_activation_budget` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-activation-budget.json`

## Coverage

- Eight session records cover the framework-pack, docs-pack, local-model-pack,
  recipe-pack, template, bridge-backed, side-loaded, and mirrored-registry families,
  so activation cost and exercised capability are proven across every marketed M5
  artifact family.
- Every `activation_bucket` (`cold`, `warm`) and every `activation_trigger`
  (`eager_on_startup`, `on_workspace_open`, `on_language_match`, `on_command_invoke`,
  `on_view_open`, `manual`) is exercised, so lazy and eager activation are both
  represented.
- Every `host_class` (`local`, `managed_workspace`, `remote_host`, `container`) and
  every `runtime_origin` is exercised.
- Every `resource_pressure` (`healthy`, `elevated`, `over_budget`, `unknown`,
  `not_applicable`) appears across the cold-start and memory dimensions.
- Every `capability_class` is exercised, and every `capability_exercise_state` is
  represented: `declared_exercised`, the `declared_unused` over-grant candidate
  (`framework_pack_healthy_warm`, `mirrored_pack_healthy_warm`), and the
  `undeclared_exercised` policy violation (`side_loaded_undeclared_quarantined`).
- Each record carries a `governance_family_ref` that resolves to its row in
  `artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`.
- Every `enforcement_action` is exercised: `no_action`, `throttled`, `downgraded`,
  `paused`, and `quarantined`; and every `enforcement_reason` is exercised, each
  routing through an exact reason code with a recovery path rather than a generic
  performance warning.
- Each record's `enforcement_reasons` and `enforcement_action` equal the
  recomputation from its facts; an enforced record names a `recovery_path_ref` and an
  unimpeded record names none.

| corpus id | record id | proves |
| --- | --- | --- |
| framework-pack-healthy-warm | `activation:framework_pack_healthy_warm` | `warm`, `declared_unused` over-grant, `no_action` |
| docs-pack-cold-eager | `activation:docs_pack_cold_eager` | `cold`, `eager_on_startup`, elevated-but-within-budget, `no_action` |
| model-pack-over-budget-throttled | `activation:model_pack_over_budget_throttled` | `activation_budget_exceeded`, `cold_start_budget_exceeded`, `throttled` |
| bridge-pack-memory-downgraded | `activation:bridge_pack_memory_downgraded` | `container`, `memory_budget_exceeded`, `downgraded` |
| recipe-pack-restart-paused | `activation:recipe_pack_restart_paused` | `restart_budget_exhausted`, `paused` |
| template-pack-crashloop-quarantined | `activation:template_pack_crashloop_quarantined` | `crash_loop_detected`, `budget_unknown`, `quarantined` |
| side-loaded-undeclared-quarantined | `activation:side_loaded_undeclared_quarantined` | `undeclared_capability_exercised`, `quarantined` |
| mirrored-pack-healthy-warm | `activation:mirrored_pack_healthy_warm` | `profile`, two `declared_unused` over-grants, `no_action` |
