# M5 Result-Scope Counter And Hidden-Narrowing Chip Fixtures

## result_scope_counters_and_hidden_narrowing.json

A coverage fixture for the result-scope counter and hidden-narrowing chip packet.
It wires the first real M5 dense surfaces — pipeline run list, review queue,
incident list, graph list, marketplace results, and provider/admin table — onto
one normalized counter and chip vocabulary across all four view kinds (list,
tree, table, queue).

The bindings exercise the truth states the lane must hold:

- **Complete client** (pipeline run list, graph tree): every count is `exact`
  and `fresh`, and `visible ≤ loaded ≤ matching ≤ total` holds exactly.
- **Hidden narrowing** (review queue): `total` 200 versus `matching` 152 with a
  `hidden_by_scope` count of 48, disclosed as a `policy` chip (30) and a
  `workset` chip (18) rendered next to the active filters. The chip sum and
  `total − matching` both reconcile to 48.
- **Approximate, provider-paginated** (marketplace results table): the matching
  and total counts are `approximate` because the exact totals stay provider-side,
  each carrying a precise basis label.
- **Stale / partial** (incident list): loaded rows are `partial` while more
  stream in, and matching/total are `stale` as of the last refresh — each with a
  precise basis label.
- **Narrow client window** (provider/admin table): a `client` chip discloses the
  10 rows not loaded on this narrow window, reconciling `total − matching`.

Every binding surfaces counters `near_active_filters`, survives reopen and
virtualization, and requires an explicit step before visible rows can stand in
for all matching rows. No binding carries raw row bodies, provider payloads, or
credentials.

The fixture validates against
`schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/support_export.json`.
