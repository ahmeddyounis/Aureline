# Classroom and teaching roles

A teaching or classroom presentation needs to say **who is doing what** — someone
drives the session, others attempt exercises, someone signs work off, someone
takes notes, and others watch. The risk this contract exists to remove is that a
classroom metaphor quietly becomes a control plane: a "moderator" gaining
terminal/debug control, an "approver" badge being mistaken for approving real
product changes, or a "participant" getting silent edit rights. Teaching roles
describe the *teaching session* and nothing else.

This M5 presentation lane does not mint a second role vocabulary. It **reuses**
the canonical teaching roles and client classes from
[`aureline-shell::teaching_session`](../../crates/aureline-shell/src/teaching_session/session.rs)
— the same five roles, the same `full` / `limited` / `low_bandwidth` client
classes, and the same role-aware affordance proof — and adds the pieces the
presentation classroom contract still needs: an explicit product-authority
attribution, an honest per-seat capability summary, and command-backed exercise
packets. The machine truth is the classroom profile produced by
[`aureline-shell::presentation::classroom`](../../crates/aureline-shell/src/presentation/classroom/roles.rs),
seeded and validated by its
[corpus](../../crates/aureline-shell/src/presentation/classroom/corpus.rs), and
frozen at
[`schemas/presentation/classroom-role.schema.json`](../../schemas/presentation/classroom-role.schema.json)
and
[`schemas/presentation/exercise-packet.schema.json`](../../schemas/presentation/exercise-packet.schema.json).

## Teaching roles are separate from product authority

A teaching role is a capability *inside the session*. It is never a source of
product authority. Terminal/debug control, approval over real product gates, and
ordinary editing rights all stay in the separate permission system; a member only
ever holds them through an external grant recorded on `product_authority`, never
because of a badge.

| Role          | What it does in the session                                    | Product authority it grants |
| ------------- | -------------------------------------------------------------- | --------------------------- |
| `moderator`   | Drives the session: advances segments, spotlights, narrates    | None                        |
| `participant` | Attempts and submits exercises within packet boundaries        | None                        |
| `observer`    | Watches read-only; is offered no controls                      | None                        |
| `approver`    | Approves a demonstrated mutation through the ordinary fence     | None                        |
| `scribe`      | Records shared session notes                                   | None                        |

Two cases make the separation concrete:

- A **moderator** drives the walkthrough, but driving the session is not control
  over product state — no terminal, no debug, no edits flow from the badge.
- A classroom **approver** routes a demonstrated mutation through the *ordinary*
  approval fence. The authority to actually approve a product gate is a different
  thing: a member who holds it carries a separate, externally recorded grant
  (`product_authority.external_authority_ref`), and
  `product_authority.granted_by_classroom_role` is always `false`.

Every member record fixes `role_grants_terminal_or_debug_control` and
`role_implies_broader_authority` to `false`, and the profile re-states the same
invariants at the top level so a reviewer can prove them rather than trust them.

## Constrained clients join as observers or note-takers — never broken controls

A member's client is not always fully capable. A `limited` or `low_bandwidth`
client must never strand a member in front of buttons that cannot work. Instead
each seat's honest `capability` summary is derived from the role and the client:
drive and mutation capabilities are *omitted* for a constrained client (never
shown then broken), while note-taking — which is low-bandwidth friendly — stays
available.

| Seat                                  | `can_drive_session` | `may_expose_mutation_affordance` | `can_take_notes` | Joins as     |
| ------------------------------------- | ------------------- | -------------------------------- | ---------------- | ------------ |
| moderator on `full`                   | yes                 | yes                              | yes              | driver       |
| moderator on `low_bandwidth`          | no                  | no                               | yes              | note-taker   |
| participant on `limited`              | no                  | no                               | yes              | note-taker   |
| observer on any client                | no                  | no                               | no               | observer     |

Because the summary is computed from the canonical role/client predicates, it
cannot drift from the role-aware affordance projection in `teaching_session`. A
constrained seat is therefore recorded as the observer or note-taker it actually
is — `joins_safely` and `degrades_honestly` are `true` — instead of a seat
holding a capability it cannot use.

## Exercise packets stay command-backed and authority-bounded

An exercise packet constrains a teaching exercise without becoming a hidden
mutation path. Each packet declares its `targets` (files, symbols, diffs, docs, or
graph objects) and its `expected_actions`, and the boundary holds by construction:

- **Command-backed.** Every expected action names an existing `command_id` (a
  `cmd:` id) and a keyboard-reachable `key_binding_ref`; it never carries its own
  mutation path. `authority_bound.all_actions_command_backed` records this.
- **Target-constrained.** Every action operates on one of the packet's declared
  targets; `authority_bound.constrained_to_declared_targets` records this.
- **No hidden plane, no widened rights.** `opens_hidden_mutation_path` and
  `widens_product_authority` are always `false`: a packet runs strictly through
  the command and policy systems and widens no role's product authority.

Because actions only ever invoke commands, an exercise inherits the same policy
and approval checks as any other command — the classroom adds no shortcut around
them.

## Support exports carry posture, never prose

The support-safe projection
([`ClassroomSupportExport`](../../crates/aureline-shell/src/presentation/classroom/roles.rs))
records roles, client classes, the capability summary, target kinds, counts, and
the authority-separation booleans, but never the packet title, an action label, a
command id, a target ref, a member display name, or an authority-grant ref.
Diagnostics, support-export, and telemetry surfaces ingest it rather than cloning
classroom state by hand.

## Fixtures

The checked-in fixtures live under
[`fixtures/presentation/classroom-role-and-authority/`](../../fixtures/presentation/classroom-role-and-authority/)
and are a literal projection of the seed corpus; they prove role/authority
separation, honest observer-or-note-taker degradation, and command-backed,
authority-bounded exercise packets across scenarios.
