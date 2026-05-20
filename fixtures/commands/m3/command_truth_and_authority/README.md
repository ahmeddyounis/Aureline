# Command-truth and palette-authority corpus

Conformance / interoperability corpus for the M3 command-truth and
palette-authority beta boundary owned by
`aureline_commands::CommandAuthorityScenarioRecord`.

`manifest.json` is authoritative. Positive drills MUST parse, validate,
project, and match **every** `expected_*` field in the manifest. Negative
drills MUST FAIL validation with an error whose message contains
`expected_failure_substring`. The fixtures carry only the scenario records
(plus a `$schema`/`__fixture__` prelude) — they do **not** restate the
expectations, so there is exactly one place to read the pinned truth.

Replay: `cargo test -p aureline-qe --test command_truth_authority_conformance`.
Regenerate: `python3 tools/regenerate_command_truth_authority_corpus.py --write`.

## Positive scenarios

- `positive.full_cross_surface_parity` (`positive/full_cross_surface_parity.json`): Reversible command keeps one enablement decision, one preview/approval posture, and one result contract across menu, keybinding, palette, CLI/headless, AI, recipe, voice, and browser-companion surfaces.
- `positive.high_risk_preview_approval_parity` (`positive/high_risk_preview_approval_parity.json`): High-risk durable command preserves its structured-diff preview and explicit-approval requirement on every surface, stays off the AI tool surface, and joins a reversible rollback handle into its lineage.
- `positive.disabled_with_reason_parity` (`positive/disabled_with_reason_parity.json`): When the command is unavailable, every surface reports the same disabled-with-reason decision and disabled-reason code, and the denied attempt still joins a support-reconstructable lineage that needs no rollback handle because nothing was applied.
- `positive.ui_only_narrowed` (`positive/ui_only_narrowed.json`): A UI-only command advertises the ui_only label, stays off every automation surface (the absence is an explicit narrowing, not a gap), and still reconstructs its lineage.
- `positive.deprecated_alias_canonicalization` (`positive/deprecated_alias_canonicalization.json`): A CLI invocation through a deprecated alias resolves to the canonical command id, records the alias warning in the result outcome, and keeps the same authority as the palette invocation.

## Negative scenarios

- `negative.ai_tool_widens_authority` (`negative/ai_tool_widens_authority.json`): rejected with `widens authority`.
- `negative.cli_headless_widens_authority` (`negative/cli_headless_widens_authority.json`): rejected with `widens authority`.
- `negative.surface_suppresses_preview` (`negative/surface_suppresses_preview.json`): rejected with `suppresses the preview requirement`.
- `negative.surface_suppresses_approval` (`negative/surface_suppresses_approval.json`): rejected with `suppresses the approval requirement`.
- `negative.enablement_divergence` (`negative/enablement_divergence.json`): rejected with `diverges from the canonical enablement decision`.
- `negative.approval_label_mismatch` (`negative/approval_label_mismatch.json`): rejected with `approval_required disagrees`.
- `negative.ui_only_exposes_automation` (`negative/ui_only_exposes_automation.json`): rejected with `exposes a non-UI automation surface`.
- `negative.lineage_missing_evidence` (`negative/lineage_missing_evidence.json`): rejected with `without an evidence ref`.
- `negative.alias_noncanonical` (`negative/alias_noncanonical.json`): rejected with `does not resolve to canonical command id`.
- `negative.lineage_missing_rollback` (`negative/lineage_missing_rollback.json`): rejected with `rollback_handle_id`.
- `negative.stable_missing_automation_metadata` (`negative/stable_missing_automation_metadata.json`): rejected with `missing machine-readable automation metadata`.

## Redaction

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. Raw secrets, private keys, credentials, and raw local paths never
appear; the runner scans each fixture for forbidden raw-content tokens before
validation.
