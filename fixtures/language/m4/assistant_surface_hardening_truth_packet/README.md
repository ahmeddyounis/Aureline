# assistant_surface_hardening_truth_packet fixture corpus

Fixture corpus for the M4 stable assistant-surface hardening truth
packet
(`schemas/language/assistant_surface_hardening_truth.schema.json`).

Each fixture is an `AssistantSurfaceHardeningTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, lane and row-class token sets, support-class,
provider/source class, side-effect class, preview-requirement,
snippet-session field, code-action field, cross-cut condition,
known-limit, downgrade-automation, and evidence-class tokens, and the
support-export safety verdict. Tests in
`crates/aureline-language/tests/assistant_surface_hardening_truth_packet.rs`
load each case and assert that
`AssistantSurfaceHardeningTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Every assistant-surface lane carries an
  `assistant_surface_quality` row at `launch_hardened` plus the five
  required `cross_cut_condition` rows (IME, multi-cursor, large-file,
  restricted-mode, degraded-provider). The completion,
  source-labeling, and AI ghost-text lanes carry
  `provider_source_binding` rows for `deterministic_completion`,
  `cached_local_word_fallback`, `snippet_only_suggestion`, and
  `ai_ghost_text`. The snippet-session lane covers all four
  `snippet_session_truth` field rows. The code-action lane covers all
  four `code_action_truth` field rows. The additional-edit lane
  carries `side_effect_admission` rows for `additional_edits` and
  `protected_surface_write` with bound preview requirements. Every
  row binds support, side-effect, preview-requirement, known limit,
  downgrade automation, and evidence classes; narrowed rows carry
  their disclosure refs, and all eight required consumer projections
  preserve the packet verbatim.
- `launch_hardened_with_unbound_evidence_blocks_stable.json` — The
  completion-lane `assistant_surface_quality` row claims
  `launch_hardened` while its evidence class is `evidence_unbound`;
  the packet blocks the stable claim.
- `missing_cross_cut_condition_for_launch_hardened_blocks_stable.json`
  — The completion lane claims `launch_hardened` but the
  `degraded_provider` `cross_cut_condition` row is missing; the
  packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The
  completion-lane `assistant_surface_quality` row narrows to
  `launch_hardened_below` but drops its disclosure ref; the packet
  blocks the stable claim.
- `projection_collapses_provider_source_vocabulary_blocks_stable.json`
  — The `help_about` consumer projection drops the provider/source
  vocabulary; the packet blocks the stable claim because surfaces
  must preserve the closed provider/source vocabulary that
  distinguishes deterministic completion, cached/local-word fallback,
  snippet-only suggestion, and AI ghost text.
- `raw_source_material_blocks_stable.json` — The completion-lane
  `assistant_surface_quality` row admits raw source bodies past the
  boundary; the packet blocks the stable claim because raw completion
  proposals, snippet bodies, ghost-text output, secrets, and ambient
  editor credentials must never leak through the assistant-surface
  boundary.
