# Topology actions and topology-aware review sheets

Repository topology only helps users if they can act on it safely. The topology
descriptors record *why* a surface answer is partial; this contract turns each
partial state into an explicit, reviewed remediation — **widen** a sparse/workset
slice, **deepen** shallow history, **initialize** a submodule child, or
**hydrate** pointer-only or unfetched objects — and surfaces those remediations in
the search, review, blame, and AI lanes instead of a generic empty or error
state.

- Action schema: [`schemas/git/topology_action_review.schema.json`](../../../schemas/git/topology_action_review.schema.json)
- Review schema: [`schemas/git/git_topology_review.schema.json`](../../../schemas/git/git_topology_review.schema.json)
- Canonical action packet: [`artifacts/git/m5/git_topology/topology_action_review.json`](../../../artifacts/git/m5/git_topology/topology_action_review.json)
- Canonical review packet: [`artifacts/git/m5/git_topology/git_topology_review.json`](../../../artifacts/git/m5/git_topology/git_topology_review.json)
- Fixtures: [`fixtures/git/m5/widen-deepen-initialize-hydrate/`](../../../fixtures/git/m5/widen-deepen-initialize-hydrate)
- Code: `crates/aureline-git/src/topology_actions/`, `crates/aureline-review/src/git_topology_review/`

## Reviewed action sheets

Each `topology_action_sheet` is one reviewed remediation for one topology caveat.
The four verbs stay distinct, never collapsing into a generic "resolve":

| Verb | Repairs | Target kind | Network |
|------|---------|-------------|---------|
| `widen` | sparse/workset slice omitting paths | `sparse_slice` | local only |
| `deepen` | shallow/grafted history | `shallow_history` | reviewed fetch |
| `initialize` | uninitialized submodule child | `child_repo` | reviewed fetch |
| `hydrate` | pointer-only LFS or unfetched promisor objects | `pointer_backed_asset` / `promisor_remote` | reviewed fetch |

Before a sheet can mutate anything it discloses, in one place: the object scope it
materializes (and the result truth before/after), the network side effect, the
provider/auth posture, the review/export parity, the recovery path, and the
approval posture. `TopologyActionSheet::for_descriptor` derives the single
remediation a `git_topology_root_descriptor` calls for, so the read surfaces and
the actions share one topology truth.

### Selectors, multi-root preview, and the no-wrong-root guard

- A sheet names exactly one target through its `ActionTargetSelector`, so a user
  can tell whether the action reaches a parent repo, a child repo, a worktree, a
  sparse slice, a promisor remote, a shallow history, or a pointer-backed asset
  before commit.
- A broad action that touches more than one root must carry a `MultiRootPreview`
  that names every additional root, and its safe scope becomes
  `explicit_multi_root_preview_required`. Otherwise the safe scope stays
  `active_root_only`.
- When the caller's active root is not the root that owns the target, the
  `wrong_root_guard` blocks the action (`retarget_required_wrong_root` or
  `blocked_nested_boundary`): the sheet is denied, never pre-approved, and the
  user must retarget or open the child root explicitly.

### Network stays reviewed and attributable

`deepen`, `initialize`, and `hydrate` reach the network. Each carries an approval
posture (`approval_required` / `approved` / `policy_blocked`), an egress
reference, and a recovery reference, and is not executable until approved. A
network-bearing sheet can never become a silent background fetch; a local `widen`
needs no network approval and is never needlessly pre-approved.

## Topology-aware review lanes

Each `git_topology_review_sheet_row` binds one lane — `search`, `review`,
`blame`, or `ai` — to a reviewed sheet and replaces the generic empty/error
fallback with an explicit `scope_limit_label` (`omitted_outside_slice`,
`unfetched`, `uninitialized`, `shallow_bounded`, `pointer_only`,
`wrong_target_root`, or `nested_boundary`). When a remediation exists, the row
advertises the verb it would run.

The rows are advisory. Every row keeps `mutation_applied = false` and
`generic_state_suppressed = true`: a lane may *recommend* widening scope but never
mutates state to do it, and a wrong-root sheet recommends nothing — it surfaces
the limit and asks the user to retarget. The review packet embeds the canonical
action sheets and re-validates them, so a lane can never recommend an action that
would fail the Git contract's no-wrong-root or reviewed-network guards.

## Invariants

- The four remediation verbs are distinct, each pinned to a coherent target kind
  and a matching action class; `hydrate` covers both Git LFS and promisor fetch.
- A sheet only exists to repair a non-complete state; generated/vendor roots get
  no remediation.
- Network-bearing sheets carry approval, egress, and recovery; local widens do
  not pre-approve.
- Cross-root targets without an explicit broadening preview are wrong-root
  guarded, denied, and never approved.
- Review rows never mutate, never fall back to a generic state, and recommend only
  the reviewed verb of an in-scope sheet.
- Both support exports retain their reconstruction fields after redaction and
  assert that raw paths and object bytes are redacted.

Regenerate the canonical packets, summaries, and fixtures with:

```bash
cargo run -p aureline-git --example dump_topology_action_review \
  > artifacts/git/m5/git_topology/topology_action_review.json
cargo run -p aureline-git --example dump_topology_action_review -- --markdown \
  > artifacts/git/m5/git_topology_action_review.md
for v in widen deepen initialize hydrate multi-root wrong-root; do
  cargo run -p aureline-git --example dump_topology_action_review -- "$v" \
    > "fixtures/git/m5/widen-deepen-initialize-hydrate/${v//-/_}.json"
done
cargo run -p aureline-review --example dump_git_topology_review \
  > artifacts/git/m5/git_topology/git_topology_review.json
cargo run -p aureline-review --example dump_git_topology_review -- --markdown \
  > artifacts/git/m5/git_topology_review.md
cargo run -p aureline-review --example dump_git_topology_review -- search \
  > fixtures/git/m5/widen-deepen-initialize-hydrate/review_search_lane.json
```
