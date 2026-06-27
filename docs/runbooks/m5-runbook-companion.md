# Companion-scoped runbook surfaces

A runbook is governed executable guidance, and its authority must hold all the way
out to the smallest client. A browser/mobile **companion** is a legitimate place to
follow an incident, acknowledge a step, or leave a note — but it is also exactly
where a control surface could quietly widen into an off-books mutate path. This
document is the contract for the **companion client scope**: how Aureline lets a
companion follow, acknowledge, comment, request, and hand off a governed runbook
step without ever becoming a hidden privileged mutate channel.

The crate `aureline-runbooks` owns the model. The executable step itself is a
step-library object (`m5_runbook_steps`,
[`RunbookExecutableStep`]); the `m5_runbook_companion` module narrows each governed
step to the companion scope and publishes the register every companion surface
reads. The machine-readable inventory lives at
[`artifacts/runbooks/m5-runbook-companion-register.json`](../../artifacts/runbooks/m5-runbook-companion-register.json)
(human summary:
[`artifacts/runbooks/m5-runbook-companion-register.md`](../../artifacts/runbooks/m5-runbook-companion-register.md)),
and the schema is
[`schemas/runbooks/m5-runbook-companion-register.schema.json`](../../schemas/runbooks/m5-runbook-companion-register.schema.json).
The companion register is **derived** from the same checked-in step objects the
step library publishes
([`schemas/runbooks/m5-runbook-step-library.schema.json`](../../schemas/runbooks/m5-runbook-step-library.schema.json)),
so a companion can never be granted authority a governed step does not declare.

## A companion's authority is narrowed, never widened

Every companion surface is derived from one governed executable step. The companion
never gets a parallel, looser copy of the step — it gets the *same* object, narrowed
to a **scope disposition**:

| Scope disposition | What the companion may do |
|-------------------|---------------------------|
| `follow_in_scope` | A read-only step: follow, acknowledge, and comment within scope. There is nothing to mutate. |
| `act_in_scope` | An in-scope mutation the companion is permitted to run: follow/acknowledge/comment, plus execute and grant the scoped self-approve gate — reusing the same desktop approval/audit objects. |
| `desktop_handoff_required` | The step's approval or mutation is out of companion scope. The companion may follow/acknowledge/comment and surface a request, but the privileged mutate channel is blocked and the step degrades to an explicit desktop handoff. |

### Companion actions

Every companion-visible action is named, so an action a companion may *not* take is
listed as **blocked**, never silently dropped:

- `follow`, `acknowledge`, `comment` — read-only / attributable; available within a
  step's declared scope and never mutate target state.
- `execute_in_scope` — execute the step in-product within the declared companion
  scope. Offered only when the governed step permits a companion to run it.
- `grant_scoped_approval` — grant a scoped self-approve gate from the companion.
  Offered only for a step the companion may both run and self-approve.
- `request_approval` — surface a request (a request, **not** a grant) for an
  approval the companion may not give itself; the grant and execution route to the
  desktop/human authority.
- `handoff_to_desktop` — the clear desktop handoff path a blocked step degrades to.

## Follow and acknowledge are always in scope

For every governed step, a companion may `follow`, `acknowledge`, and `comment`
within the step's declared scope. These are read-only and attributable: they record
who followed or acknowledged a step, but they never change target state. A companion
surface that did not offer them, or that offered an `execute_in_scope` /
`grant_scoped_approval` the underlying step does not permit, is invalid.

## A companion-allowed approval reuses the desktop objects

When a step is an in-scope mutation the companion is permitted to run
(`act_in_scope`), the companion may execute it and grant its scoped self-approve
gate. Crucially, that approval is **not** a companion-only record: the surface
reuses the byte-identical shared **approval-authority ref** and **action-envelope
ref** the desktop path uses (`reused_approval_authority_ref`,
`reused_action_envelope_ref`). An approval taken from a companion client therefore
creates the same durable audit/approval object any other governed mutation creates —
there is no second, weaker approval system hiding behind the companion.

## A blocked privileged action degrades to a clear desktop handoff

When a step needs an approval the companion may not grant, is a privileged mutation,
or is an out-of-plane console/browser pivot, the companion's privileged mutate
channel is **explicitly blocked** (`privileged_mutate_blocked_on_companion`) and the
step degrades to a clear desktop handoff (`handoff_to_desktop` plus a
`desktop_handoff_message_id`). This is never a silent failure and never a misleading
claim of parity: the blocked actions are named in `blocked_actions`, and the
companion may still surface a `request_approval` so a human on the desktop authority
decides. A handoff state with no handoff path, or a block that is not marked, is
invalid.

## No hidden privileged mutate channel

The safety predicate `creates_hidden_mutate_channel` must be false on every surface.
A surface mints a hidden channel if its source step does, or if it would offer an
`execute_in_scope` or `grant_scoped_approval` the step does not actually permit.
This mirrors the step library's own predicate, so a companion can never become a
back door around the shared command/action-envelope and approval systems.

## One register, every surface

The companion app, the desktop incident workspace that receives a handoff, and
support exports all read the same register and render the same narrowing. The
register carries metadata and refs only — no credential bodies and no raw
provider/console payloads.

The headless emitter is the only mint-from-truth path:

```sh
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- register
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- markdown
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- surface <step-id>
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_companion -- validate
```

[`RunbookExecutableStep`]: ../../crates/aureline-runbooks/src/m5_runbook_steps/mod.rs
