# AI / recipe / CLI package-mutation governance fixtures

These fixtures exercise the package-mutation governance object
(`aureline-deps`, `automation_governance`) that binds AI, recipe, and
CLI/headless package proposals to the same reviewed mutation contract as a
direct UI operation. Each file is an `automation_governance` packet validated
against the embedded schema with `AutomationGovernance::validate`.

- **`ai_install_proceed_committed.json`** — an AI install proposal whose
  ecosystem delivers every promised capability; it selects its required
  validation, preserves the reviewed contract, and commits with a durable,
  reversible rollback handle.
- **`recipe_capability_gap_inspect_only.json`** — a recipe step against an
  ecosystem that cannot provide a deterministic resolver, durable rollback, or
  in-product validation; it **narrows to inspect-only** rather than executing an
  unsafe fallback.
- **`cli_dry_run_preview_pending.json`** — a CLI/headless dry run that produces
  the same preview-first governed proposal as desktop and AI, awaiting review.
- **`ai_auth_blocked_no_safe_path.json`** — an AI proposal against an offline
  registry with unsatisfied auth; with no safe execution path it **blocks**
  rather than falling back to an unsafe install.

Every fixture binds to the frozen package-state matrix
(`m5-package-state-mutation-matrix:m5:v1`) and reuses the reviewed-mutation
contract (`reviewed-mutation-flows:m5:v1`), so AI, recipe, and CLI proposals can
never become a bypass lane around lockfile-safe review.
