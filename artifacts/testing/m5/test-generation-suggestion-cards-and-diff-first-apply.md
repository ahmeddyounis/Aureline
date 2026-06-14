# M5 Test-Generation Suggestion Cards And Diff-First Apply

- Packet: `test-generation:stable:0001`
- Label: `M5 Test-Generation Suggestion Cards And Diff-First Apply`
- Cards: 5 (1 imported, 1 applied, 1 blocked)
- Source kinds: 5 / 6

## Suggestion cards

- **card:applied:uncovered-line** [locally_generated] source `uncovered_coverage_path`, 1 target(s), 1 file(s)
  - subject `test:auth::login_rejects_expired_token` (concrete_invocation), 1 assumption(s), 1 evidence ref(s)
  - validation `sandbox_validated_pass` → apply `applied` (preview-first true)
- **card:blocked:bug-repro** [locally_generated] source `bug_reproduction`, 1 target(s), 1 file(s)
  - subject `test:checkout::tax_rounding[*]` (parameterized_template), 2 assumption(s), 2 evidence ref(s)
  - validation `not_validated` → apply `blocked_needs_validation` (preview-first true)
- **card:previewed:branch** [locally_generated] source `uncovered_branch_path`, 1 target(s), 2 file(s)
  - subject `test:render::pipeline_partial_branch` (concrete_invocation), 1 assumption(s), 2 evidence ref(s)
  - validation `sandbox_validated_fail` → apply `previewed` (preview-first true)
- **card:imported:smoke** [imported_proposal] source `regression_guard`, 1 target(s), 1 file(s)
  - subject `test:imported::smoke_regression` (concrete_invocation), 1 assumption(s), 1 evidence ref(s)
  - validation `imported_unvalidated` → apply `pending_preview` (preview-first true)
- **card:review:widening** [locally_generated] source `changed_code_gap`, 1 target(s), 1 file(s)
  - subject `test:data::serialize_cases[*]` (parameterized_template), 1 assumption(s), 1 evidence ref(s)
  - validation `not_validated` → apply `rejected` (preview-first true)
