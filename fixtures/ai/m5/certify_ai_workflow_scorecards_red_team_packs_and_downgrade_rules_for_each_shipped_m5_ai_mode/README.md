# M5 AI Mode Certification Fixtures

## held_branch_agent_certification.json

A certification fixture where the branch-or-worktree-agent mode is held pending
upstream provider graduation. Inline edit, patch review, explain, and debug stay
Stable; test and refactor stay Beta. Demonstrates that a held mode is not a
claimed lane (so it carries no evidence packet refs and skips the always-applicable
red-team coverage gate), while still covering every scorecard dimension and
red-team vector and narrowing to `unavailable` on stale proof.
