# Service-health aggregator and contract-state cards — beta contract

This document is the contract that ships behind the M3 beta service-health
surfaces. It is the shared truth that shell, About, service-health,
diagnostics, CLI/headless inspect, and support exports MUST all read.

## Why one aggregator

Service-health, About, diagnostics, the headless inspector, support
exports, and release-truth packets all need the same answer when a user
or reviewer asks:

1. Which service family is currently impaired?
2. What workflows are affected — i.e. what does the user actually feel?
3. When was the family last checked?
4. Is the rest of the product still safe to use locally?

Without a shared aggregator, each surface invents its own copy. A
service-health chip says "service degraded" while About says
"unavailable" for the same family; the support export quotes a third
phrasing. The chrome ends up painting the whole product as broken when
the only problem is a hosted marketplace that the user wasn't reaching
anyway.

The aggregator at `crates/aureline-shell/src/service_health/aggregator.rs`
fixes this by minting one record (`service_health_aggregator_record`)
that every surface reads. Each member service family is captured as a
single `service_health_card_record` with a closed contract-state token,
the affected launch-critical workflows, the last-checked age bucket, the
boundary class the family sits behind, and the local-continuity posture
the rest of the product can rely on.

## What a card carries

| Field | Meaning |
| ----- | ------- |
| `card_id` | Stable object identity. Support exports and release-truth packets quote this so an incident shows up under one ref across surfaces. |
| `service_family` | Closed vocabulary: `language_services`, `ai_assist`, `sync`, `license_entitlement`, `telemetry`, `marketplace`, `remote_runtime`, `release_channel`, `docs_knowledge`, `status_feed`. |
| `boundary_class` | Closed vocabulary: `local_only`, `local_with_remote_optional`, `local_with_remote_required`, `hosted`, `vendor_provider`. Drives the local-continuity rollup. |
| `contract_state` | Closed vocabulary: `ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`, `policy_blocked`, `unavailable`. Surfaces MUST quote one of these. |
| `local_continuity` | Closed vocabulary: `local_safe`, `local_safe_read_only`, `local_review_only`, `local_unsafe`. |
| `affected_workflows` | Closed set of launch-critical workflow tokens. Empty when `contract_state == ready`. |
| `last_checked` / `last_checked_age` | Probe time and the derived age bucket (`fresh`, `recent`, `stale`, `very_stale`, `never_checked`). |
| `state_explanation` | Short reviewable sentence (<= 240 chars) explaining the state without leaking endpoint trivia. |
| `diagnostics_action` | Stable command-ref the chrome wires into a Run diagnostics affordance. Never a URL. |
| `detail_tokens` | Stable, non-secret tokens (e.g. `provider_class:vendor_chat`). Never URLs / credentials. |

## The contract-state vocabulary

| Token | Meaning |
| ----- | ------- |
| `ready` | All probes are current, the service is honouring its contract. |
| `degraded` | Functioning but at reduced capacity (slow, partial features, retries firing). |
| `local_only` | Service unreachable but the local fallback keeps the workflow usable. |
| `stale` | Cached data is being served because the last fresh probe is past its review window. |
| `contract_mismatch` | Remote responded with a payload outside the agreed contract (schema mismatch, version skew). |
| `policy_blocked` | Policy or governance blocked the service (admin disabled, region gate, license). |
| `unavailable` | Service is unreachable AND no admissible local fallback. |

`stale`, `contract_mismatch`, `policy_blocked`, `local_only`, and
`unavailable` are deliberately distinct — collapsing them into one
generic "service degraded" state would hide the difference between
"your work is safe locally", "the remote schema changed", "your admin
turned this off", and "we cannot reach the service at all".

## The local-continuity invariant

Acceptance criterion: *a single failed service cannot silently flip the
whole product into broken or unavailable messaging when local work
remains safe.*

The aggregator enforces this by computing
`overall_local_continuity` as the *worst* `local_continuity` across cards
whose `boundary_class.can_downgrade_local_continuity()` returns true.

| Boundary | Can downgrade overall local continuity? |
| -------- | -------------------------------------- |
| `local_only` | Yes |
| `local_with_remote_optional` | Yes |
| `local_with_remote_required` | Yes |
| `hosted` | No — outage never drags overall continuity below `local_safe` |
| `vendor_provider` | No — same as hosted, distinguished only in copy |

Concretely:

- A `marketplace` card going `unavailable` (`hosted`) cannot drag
  overall continuity below `local_safe`. The chrome paints the
  marketplace card as unavailable; the editor remains usable.
- A `sync` card going `local_only` with `local_with_remote_required`
  boundary correctly drags overall continuity to `local_safe_read_only`.
  Edits remain safe; only external writes pause.

`overall_contract_state` is the *worst-severity* state across all cards
(no boundary filter); the chrome reads it to title the service-health
section.

## Cross-surface parity

The aggregator record is the cross-tool boundary. The following surfaces
read it verbatim:

- **Desktop shell** — service-health pane, About card, diagnostics
  inspector, status chip.
- **CLI / headless inspect** — `aureline_shell_service_health_inspect`.
  Subcommands `aggregator`, `cards`, `card <id>`, `summary`,
  `plaintext`, and `vocabulary` print the same record the shell paints.
- **Support exports** — `ServiceHealthAggregator::render_plaintext()`
  is the plaintext block the support export embeds. The release-truth
  packet quotes the same record.
- **Diagnostics** — diagnostics drill-down reads the card body and
  pivots into `diagnostics_action`.

If any surface diverges from these tokens or labels it is a contract
violation, not a presentation choice.

## What is out of scope

This contract does not cover:

- **Incident management** workflows. Once a card flips to `degraded`,
  `contract_mismatch`, or `unavailable`, the `diagnostics_action` points
  users at the existing inspector. Responder workflows live in
  `crates/aureline-support/src/incident_workspace_beta` and are out of
  scope here.
- **Live probes.** The runtime probes that mint `ServiceHealthProbeReading`
  inputs live in other crates and are out of scope. Until those are
  wired in, the seeded packet at
  `crates/aureline-shell/src/service_health/seed.rs` is the source of
  truth for the headless inspect and the on-disk fixtures.
- **M6 service-ownership overlays.** Per-service owner / paging
  metadata, runbook owner routing, and escalation chains are explicitly
  out of scope for this beta surface.

## Schemas & fixtures

- `schemas/ops/service_health_card.schema.json` — boundary schema for
  the aggregator and card records.
- `schemas/ops/service_contract_state.schema.json` — closed
  contract-state vocabulary.
- `fixtures/ops/m3/service_health_cards/` — the on-disk fixture corpus
  every surface compares against. See the README in that directory.

## Refreshing the seed

The seeded aggregator is the canonical sample that every surface and
fixture reads. To refresh after editing the seed:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_service_health_inspect -- aggregator \
  > fixtures/ops/m3/service_health_cards/seeded_aggregator.json
```

The fixture-replay test in
`crates/aureline-shell/tests/service_health_card_fixtures.rs` will fail
if the on-disk fixture drifts from the in-code seed, so refreshing the
fixture is mandatory whenever the seed changes.
