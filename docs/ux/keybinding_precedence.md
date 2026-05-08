# Keybinding precedence and winning-source attribution

This document is the reviewer-facing entrypoint for Aureline’s keybinding
precedence model. It summarizes **which layer wins**, what the resolver must
surface to explain outcomes, and where the machine-readable contract lives.

Canonical contract and schema:

- `docs/ux/keybinding_resolver_contract.md`
- `schemas/commands/keybinding_resolver.schema.json`

Worked resolver packets:

- `fixtures/commands/keybinding_conflict_examples/`

## Precedence model (strict)

The resolver evaluates layers in strict order. Earlier rows **outrank** later
rows. Resolution is deterministic and explainable; imperative “fallthrough” that
is not represented in the resolver packet is non-conforming.

| Rank | Layer | Typical source | Resolver expectation |
|---:|---|---|---|
| 0 | `platform_reserved` | OS / desktop environment | stop dispatch; show host reservation and the shadowed candidate(s) |
| 1 | `emergency_security_hard_block` | incident freeze / security interlock | deny dispatch regardless of binding source |
| 2 | `admin_policy_lock` | managed bundle / fleet policy | deny or pin bindings with policy-owned provenance |
| 3 | `temporary_mode_overlay` | modal state / leader overlay | override durable bindings while active, and remain inspectable |
| 4 | `user_profile_binding` | user overrides + profile truth (including applied imports) | wins over workspace, extension, and defaults |
| 5 | `workspace_recommendation` | workspace manifest / accepted recommendation | applies only when user/profile is silent |
| 6 | `extension_binding` | built-in or third-party extension | applies only when higher layers are silent |
| 7 | `core_default` | shipped Aureline defaults | baseline only |

## What “winning source attribution” means

Every resolved shortcut must be explainable without reverse-engineering code:

- which layer won (`resolver_layer`);
- which binding record won (`candidate_ref` + `source_provenance_ref`);
- which candidates lost and why (`losing_candidates` + typed loss reason); and
- what would change the result (`outcome_change_conditions`).

The canonical packet family for this explanation is
`keybinding_resolution_packet_record` (and, when required,
`keybinding_conflict_review_packet_record`).

## Tie-breaking within a layer

When multiple candidates exist in the same precedence layer, the resolver breaks
ties by declared specificity (surface → mode → focus/context → sequence shape →
scope). If candidates remain equally specific after those axes, the outcome must
be marked as requiring review (no arbitrary winner).

