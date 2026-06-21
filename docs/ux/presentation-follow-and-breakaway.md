# Presentation follow and breakaway across clients

A presentation can be watched from more than one place at once — the desktop
shell, the browser surface, and the companion app. Wherever it is watched, the
viewer must be able to answer one question without guessing: **am I watching the
presenter live, browsing on my own, or looking at a cached snapshot?** This doc
is the human-readable face of that contract; the machine truth is the
cross-client follow-state packet produced by
[`aureline-shell::presentation::follow_state`](../../crates/aureline-shell/src/presentation/follow_state/state.rs),
seeded and validated by its
[corpus](../../crates/aureline-shell/src/presentation/follow_state/corpus.rs),
frozen at
[`schemas/presentation/follow-state-truth.schema.json`](../../schemas/presentation/follow-state-truth.schema.json),
and covered by the
[cross-client follow matrix](../../artifacts/presentation/cross-client-follow-matrix.md).

The canonical session object model (the presentation session, follow waypoints,
speaker notes, and the reversible overlay) lives in
[`aureline-shell::presentation_mode`](../../crates/aureline-shell/src/presentation_mode/session.rs)
and is documented in
[presentation-and-walkthrough-truth.md](presentation-and-walkthrough-truth.md).
This lane adds the **cross-client follow-state vocabulary**: the explicit states
a client can be in, the liveness each one reads as, and the recovery actions that
get a viewer back to the truth — spoken identically on every claimed client.

## Follow states are explicit, never inferred

Follow state is a deliberate, attributable signal. The product never decides a
viewer has broken away because their cursor drifted, never decides they are stale
because a packet was late, and never leaves the state riding on a toast that has
already faded. Every client carries a durable follow-state record drawn from one
vocabulary:

| Follow mode             | Liveness        | What the viewer is seeing                                  |
| ----------------------- | --------------- | ---------------------------------------------------------- |
| `presenting`            | Live            | This client is driving the walkthrough.                    |
| `following_live`        | Live            | Tracking the presenter's live anchor in real time.         |
| `requesting_take_over`  | Live            | Still on the live route, with a take-over request pending.  |
| `broken_away`           | Independent     | Browsing independently; the live route may have moved on.   |
| `requesting_follow`     | Independent     | Asked to rejoin; not yet resynced to the live route.        |
| `cached_snapshot`       | Cached snapshot | A stale picture that is **not** the live route.             |

The three liveness classes — **live**, **independent**, and **cached
snapshot** — are the one honesty axis a viewer reads. A mode and its liveness are
bound together: a `cached_snapshot` view can never present itself as `live`.

## Break away keeps a durable banner and a way back

When a client breaks away to browse independently, it raises a **durable
breakaway banner** — not a toast. The banner says "You are browsing
independently", names the presenter's current anchor, and carries a
keyboard-reachable **Return to presenter** action. It persists for as long as the
viewer is broken away, so the return path is never a moment they can miss. The
same banner, with the same return action, appears on desktop, browser, and
companion.

## A cached snapshot says so

If a client loses the live route — the connection dropped, the provider went
offline, a reconnect is still in flight, or the viewer paused updates — it does
not keep pretending to be live. It falls back to a **cached snapshot** that
identifies itself: it is labeled as a snapshot, it records why it is stale and
when it was captured, and it **never claims to be a live shared route**. From a
snapshot, the viewer can **Refresh live** to rejoin the presenter's live route or
**Return to presenter** to jump to the current anchor.

This is the guardrail this lane exists to hold: a client only ever showing a
cached picture must not imply live shared state.

## The same vocabulary and recovery on every client

Desktop, browser, and companion use the **same follow-state vocabulary and the
same recovery actions**. The recovery actions come from one canonical table, so a
given action — its command id, key binding, visible label, and accessible label —
is identical no matter which device a viewer is on:

| Recovery action       | Offered from                                  |
| --------------------- | --------------------------------------------- |
| Return to presenter   | broken away, requesting follow, requesting take over, cached snapshot |
| Refresh live          | cached snapshot                               |

A viewer who learns the follow controls on one client already knows them on the
others. The cross-client truth packet records each client's follow mode,
liveness, banner, snapshot identity, and recovery actions, and asserts the
parity, durability, and non-inference guardrails so a reviewer can prove them
rather than trust them.

## Scope

Following is **not** control. None of these states grants a mutation shortcut or
widens collaboration authority: `grants_mutation_authority` and
`grants_control_authority` are always false. A take-over **request** is a distinct
state, not a control grab — granting it flows through the ordinary authority path,
not through the act of following. This lane governs follow-state truth and its
cross-client parity only; it does not open browser editing or broaden
collaboration scope.
