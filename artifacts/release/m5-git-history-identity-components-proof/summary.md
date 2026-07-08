# Git-History Identity Components: Working-Context and Topology Truth

- Packet: `m5-git-history-identity-component:stable:0001`
- Surface: `Git-history identity components: working-context and topology truth`
- Components: 4 (1 on a linked worktree, 1 on a partial/shallow checkout, 3 carrying recovery/reflog availability)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Components

- **commit_graph_header** [`current_repo_worktree`]: repo `aureline (/work/aureline)`, ref `main`, worktree `/work/aureline` — divergence `ahead`, dirty `clean`, topology `complete`
- **history_graph_row** [`partial_or_shallow_checkout`]: repo `aureline (shallow mirror)`, ref `release/24.10`, worktree `/work/aureline-shallow` — divergence `unknown`, dirty `clean`, topology `shallow`
- **branch_comparison_chip** [`detached_or_bare_root`]: repo `aureline (detached HEAD)`, ref `detached @ a1b2c3d`, worktree `/work/aureline` — divergence `detached_no_upstream`, dirty `clean`, topology `complete`
- **worktree_row** [`linked_worktree`]: repo `aureline (linked worktree)`, ref `feature/import`, worktree `/work/aureline-import` — divergence `behind`, dirty `dirty_uncommitted`, topology `complete`
