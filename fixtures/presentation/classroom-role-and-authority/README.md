# Presentation classroom-role-and-authority fixtures

These fixtures are the literal projection of the seeded classroom-role corpus in
[`aureline-shell::presentation::classroom`](../../../crates/aureline-shell/src/presentation/classroom/corpus.rs).
They prove that teaching / classroom roles (moderator, participant, observer,
approver, scribe) stay separate from product authority, that limited and
low-bandwidth clients join honestly as observers or note-takers rather than broken
controls, and that exercise packets stay command-backed, inspectable, and
authority-bounded.

The role and client-class vocabularies are the canonical teaching-session
vocabularies reused by this M5 presentation lane (see
[`aureline-shell::teaching_session`](../../../crates/aureline-shell/src/teaching_session/session.rs)).

## Files

- `classroom-role-and-authority-corpus.json` — the in-product/inspector truth:
  one case per scenario, each carrying a classroom profile with its members
  (role, client class, the honest capability summary, and product-authority
  attribution) and its exercise packets (with titles, targets, and command-backed
  expected actions). This is the canonical object truth and carries the
  instructional titles and labels the in-product surface renders. Each profile
  validates against
  [`schemas/presentation/classroom-role.schema.json`](../../../schemas/presentation/classroom-role.schema.json)
  and each packet against
  [`schemas/presentation/exercise-packet.schema.json`](../../../schemas/presentation/exercise-packet.schema.json).
- `classroom-role-and-authority-support-export.json` — the support-safe
  projection: one diagnostics row per member and per packet carrying roles, client
  classes, the capability summary, target kinds, counts, and the
  authority-separation booleans only. Instructional prose, action labels, command
  ids, target refs, member display names, and authority-grant refs are excluded by
  construction.

## Cases

- `classroom-case:five-roles-authority-separated` — a session expressing all five
  roles on full clients. The approver's product approval authority comes from a
  separate external grant; no role grants terminal/debug control or implies
  broader authority by itself.
- `classroom-case:constrained-clients-join-as-observer-or-note-taker` — a
  moderator on a low-bandwidth client keeps no drive controls and joins as a
  note-taker, a participant on a limited client joins as a note-taker, and a
  first-class observer stays observing.
- `classroom-case:exercise-packet-authority-bounded` — packets whose expected
  actions all invoke existing commands and stay inside their declared file,
  symbol, diff, and doc targets, opening no hidden mutation path.

## Regenerating

These files are generated, not hand-edited. After changing the classroom model or
the seed corpus, regenerate them so the in-tree test
`checked_in_fixtures_match_the_seed_projection` keeps passing:

```sh
cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- corpus \
  > fixtures/presentation/classroom-role-and-authority/classroom-role-and-authority-corpus.json
cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- support-export \
  > fixtures/presentation/classroom-role-and-authority/classroom-role-and-authority-support-export.json
cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- profile-example \
  > artifacts/presentation/classroom-role.example.json
cargo run -q -p aureline-shell --example dump_presentation_classroom_roles -- packet-example \
  > artifacts/presentation/exercise-packet.example.json
```
