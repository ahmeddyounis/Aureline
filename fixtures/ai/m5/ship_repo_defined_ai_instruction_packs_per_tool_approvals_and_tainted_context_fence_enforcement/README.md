# Repo-Defined AI Instruction Packs, Per-Tool Approvals, and Tainted-Context Fence Fixtures

This directory contains fixture files for the AI governance lane, which binds
repo-defined instruction packs that may narrow but never widen policy, per-tool
approvals that gate every tool side effect behind a disclosed posture with human
review on first use, and tainted-context fences that keep untrusted context from
widening policy or auto-approving a tool — enforced together so tainted context
can never grant a tool approval and a repo instruction can never widen policy.

## Files

- `valid_packet.json` — A fully valid governance packet that passes all validation
  invariants. Mirrors the checked-in support export.
- `tainted_context_granted_tool_approval.json` — A packet whose first fence sets
  `auto_approval_blocked` false, triggering
  `tool_approval_granted_by_tainted_context`.
