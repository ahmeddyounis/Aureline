# M5 Coverage Overlays And Snapshot / Golden Review

- Packet: `coverage-review:stable:0001`
- Label: `M5 Coverage Overlays And Snapshot / Golden Review`
- Overlays: 6 (1 imported, 1 stale) across 4 / 4 provenance class(es)
- Merge sheets: 1
- Snapshot cards: 4 (2 open for review)

## Coverage overlays

- **overlay:verified:auth** [verified_current_run] line_coverage `92%` line
  - scope `crates/aureline-auth/src/login.rs` (file), authoritative true
- **overlay:verified:checkout-branch** [verified_current_run] branch_coverage `85%` line, `70%` branch
  - scope `crates/aureline-commerce/src/checkout.rs` (file), authoritative true
- **overlay:changed:diff** [verified_current_run] line_coverage `80%` line
  - scope `changed-set:pr-4821` (changed_set), authoritative true
  - changed since `git:origin/main`: 22/26 covered
- **overlay:imported:nightly** [imported_ci_artifact] line_coverage `62%` line
  - scope `crates/aureline-render/src/pipeline.rs` (file), authoritative false
- **overlay:cached:editor** [cached_local_result] line_coverage `80%` line
  - scope `crates/aureline-editor/src/buffer.rs` (file), authoritative false
- **overlay:stale:legacy** [stale_prior_result] line_coverage `97%` line
  - scope `crates/aureline-legacy/src/migrate.rs` (file), authoritative false

## Coverage merge sheets

- **merge:commerce:full** 2 included / 2 excluded, 2 omitted (complete certainty: false)

## Snapshot / golden review cards

- **card:image:home** [image_snapshot] 3 / 8 changed, scope `platform_specific`
  - fallback `unavailable_binary_only` → decision `needs_raw_inspection`
- **card:text:serialize** [text_snapshot] 2 / 6 changed, scope `per_parameter_case`
  - fallback `text_diff_available` → decision `accepted`
- **card:golden:report** [golden_file] 1 / 1 changed, scope `per_test`
  - fallback `raw_artifact_referenced` → decision `rejected`
- **card:imported:smoke** [serialized_snapshot] 1 / 4 changed, scope `shared_fixture`
  - fallback `raw_artifact_referenced` → decision `pending_review` (imported)
