# Background branch-agent lifecycle support export

- Packet: `background-branch-agent-lifecycle:stable:0001`
- Stable run ID: `branch-agent-run:stable:request-cache:0001`
- Current state: `ready_for_review`
- Execution locus: `isolated_side_worktree`
- Checkpoints: 2
- Drift drills: base advance, provider/model route drift, policy epoch change, trust narrowing, boundary expansion
- Operator actions: pause, cancel, open review, take over manually, compare to base, cherry-pick, rerun validation, discard branch
- Cleanup posture: preview delete available, evidence and checkpoints retained
