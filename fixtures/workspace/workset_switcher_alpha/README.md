# Workset Switcher Alpha Fixtures

This folder indexes the protected proof path for portable worksets and sparse
slices. The fixture set deliberately points at the canonical records instead of
copying them:

- durable artifact: `fixtures/workspace/workset_scope_alpha/aureline.workset.jsonc`
- switcher surface: `fixtures/workspace/workset_cross_repo_cases/multi_repo_workset_switcher.yaml`
- scope banner states: `fixtures/workspace/workset_cross_repo_cases/warm_vs_cold_scope_switch.yaml` and `fixtures/workspace/workset_cross_repo_cases/policy_limited_workset_banner.yaml`
- widening review: `fixtures/workspace/scope_widening_cases/widen_current_repo_to_selected_workset.yaml`
- cross-repo identity: `fixtures/workspace/workset_cross_repo_cases/outside_current_scope_result_group.yaml` and `fixtures/workspace/scope_widening_cases/cross_repo_peek_outside_scope.yaml`

The shell test `crates/aureline-shell/tests/workset_switcher_alpha.rs`
re-parses the fixture records and projects a switcher plus scope banner from the
durable workset artifacts.
