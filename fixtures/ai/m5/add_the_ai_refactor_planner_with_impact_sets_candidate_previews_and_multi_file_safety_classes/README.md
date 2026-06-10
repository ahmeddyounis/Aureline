# AI Refactor Planner Fixtures

This directory contains fixture files for the AI refactor planner lane, which
binds a preview-only refactor plan, the impact set of affected sites with their
multi-file safety classes, and the candidate previews it produces.

## Files

- `valid_packet.json` — A fully valid refactor planner packet that passes all
  validation invariants. Mirrors the checked-in support export.
- `unsafe_candidate_not_blocked.json` — A packet whose `behavior_affecting`
  candidate leaves `auto_apply_blocked_for_unsafe_class` false, triggering
  `unsafe_candidate_not_blocked`.
