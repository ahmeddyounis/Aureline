# Shiproom review packet — execution-context explainability

This packet is the shiproom- and release-center-facing view of the environment-status-strip registry. It
does not maintain its own summary: the claim scope below is read from the canonical packet and narrows
automatically when a strip's context goes stale, blocked, drifted, or conflicting.

## Canonical inputs

- Packet: `artifacts/support/m5/m5-execution-context-explainability.json`
- Reviewer artifact: `artifacts/support/m5/m5-execution-context-explainability.md`
- Schema: `schemas/runtime/m5-environment-status-strip.schema.json`
- Companion doc: `docs/help/support/m5-why-this-execution-context.md`
- Fixtures: `fixtures/runtime/m5/m5-environment-status-strips/`
- Typed model + gate: `aureline-runtime` crate, `m5_environment_status_strips`

- Claim publishable: **yes**
- Resolved strips: `3`
- Flagged strips: `5`
- Blocked strips: `1`

## Claim scope

| Surface | Status | Presentation | Resolution |
| --- | --- | --- | --- |
| `run` | resolved | **resolved** | none |
| `test` | resolved | **resolved** | none |
| `debug` | stale | **flagged** | refresh_target |
| `notebook` | stale | **flagged** | refresh_target |
| `request` | remote_drift | **flagged** | reconnect_remote |
| `database` | blocked | **blocked** | unblock_environment |
| `preview` | conflicting | **flagged** | resolve_conflict |
| `pipeline` | resolved | **resolved** | none |
| `incident` | stale | **flagged** | refresh_target |

## Sign-off gate

Promotion of the status-strip registry holds unless all of the following are true on the current packet
(`M5EnvironmentStatusStrips::validate()` returns no violations):

1. Every run-capable surface carries exactly one strip; none is missing or duplicated.
2. Every strip shows at least one execution-context facet and carries its one-step explain entry and its
   CLI / headless equivalent object — even when blocked — so it can never collapse into a generic
   "current target" chip.
3. Every strip's `presentation`, `downgrade_reasons`, `resolution_path`, and `blocked_before_run` flag
   equal the recomputed fail-closed gate — a stale, blocked, drifted, or conflicting context flags or
   blocks the strip automatically.
4. No flagged or blocked strip is silent: it names its resolution path, a caveat, and the stale-or-blocked
   field; a blocked strip warns before the downstream run failure.
5. The five consumer bindings (desktop-shell, support-center, support-export, issue-report-packet,
   cli-headless) are all present and reuse this packet's status vocabulary and object ids.

A flagged or blocked strip is never silent: a stale target, a blocked environment, a drifted remote, and a
conflicting context each surface as their own downgrade reason and resolution path rather than shipping as
an implied clean chip. A blocked environment is restored before a remote, conflict, or refresh, because it
is the hardest state.

## Reviewer checklist

- [ ] `cargo test -p aureline-runtime m5_environment_status_strips` passes.
- [ ] The artifact validates against the schema (no schema/example drift).
- [ ] Three strips resolve cleanly, proving the gate is not a blanket flag.
- [ ] Each flagged or blocked strip names its downgrade reason, resolution path, and stale/blocked field.
- [ ] No live target handle, secret, or raw private material is embedded in the support export.

## Regenerating this packet

This packet is checked in alongside the registry it reviews. When the status-strip registry changes,
update the packet, schema, reviewer artifact, and fixtures together, then re-run the gate before
re-reviewing:

```sh
cargo test -p aureline-runtime m5_environment_status_strips
python3 - <<'PY'
import json
from jsonschema import Draft202012Validator
schema = json.load(open("schemas/runtime/m5-environment-status-strip.schema.json"))
data = json.load(open("artifacts/support/m5/m5-execution-context-explainability.json"))
errors = list(Draft202012Validator(schema).iter_errors(data))
print("schema OK" if not errors else errors)
PY
```
