# M5 Visual-Designer Component Qualification

- Packet: `m5-visual-designer-component-qualification:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Bundle: `artifacts/release/m5-visual-designer-component-qualification-proof/support_export.json`
- Consolidates 7 certified component packets
- Consumers: 8 qualified across 8 / 8 claimed consumers
- Status: 5 green / 3 yellow / 0 red

## Rows

- **qual:visual-design-surface** (visual_design_surface) — consumer=visual_design_surface source_ownership=certified mapping=certified round_trip=certified token_binding=certified accessibility=certified verdict=qualified
- **qual:preview-runtime** (preview_runtime) — consumer=preview_runtime source_ownership=certified mapping=disclosed_narrowed round_trip=certified token_binding=certified accessibility=certified verdict=qualified_with_narrowing
  - Narrowed: dimension=mapping_quality trigger=unmapped_source — Preview runtime resolves its source mapping only approximately; the mapping-quality claim narrows to disclosed and keeps the source-first anchor visible
- **qual:framework-pack-preview** (framework_pack_preview) — consumer=framework_pack_preview source_ownership=certified mapping=certified round_trip=disclosed_narrowed token_binding=certified accessibility=certified verdict=qualified_with_narrowing
  - Narrowed: dimension=round_trip_state trigger=round_trip_conflict_open — Framework-pack preview has an open round-trip conflict; the round-trip claim narrows to inspect-only until the conflict resolves, never a silent write-back
- **qual:docs-demo-embeds** (docs_demo_embeds) — consumer=docs_demo_embeds source_ownership=certified mapping=certified round_trip=certified token_binding=certified accessibility=certified verdict=qualified
- **qual:handoff-consumer** (handoff_consumer) — consumer=handoff_consumer source_ownership=certified mapping=certified round_trip=certified token_binding=disclosed_narrowed accessibility=certified verdict=qualified_with_narrowing
  - Narrowed: dimension=token_binding_provenance trigger=drifted_from_source — Handoff consumer detected a bound expression drifted from its source binding; the provenance claim narrows to disclosed and keeps the binding distinct from a literal
- **qual:support-packet** (support_packet) — consumer=support_packet source_ownership=certified mapping=certified round_trip=certified token_binding=certified accessibility=certified verdict=qualified
- **qual:help-center** (help_center) — consumer=help_center source_ownership=certified mapping=certified round_trip=certified token_binding=certified accessibility=certified verdict=qualified
- **qual:release-evidence** (release_evidence) — consumer=release_evidence source_ownership=certified mapping=certified round_trip=certified token_binding=certified accessibility=certified verdict=qualified
