# Scope propagation alpha — workset and scope truth across remote, provider-linked, and export surfaces

The alpha
[`WorksetArtifactRecord`](../../../crates/aureline-workspace/src/worksets/mod.rs)
named the durable, portable workset / scope artifact. The beta
[`WorksetScopeBetaTruth`](workset_scope_beta.md) hardened the broad-action
admission ladder, lineage chain, and excluded-root accounting that
search, graph, refactor, AI, export, and support consumers read on a
single workspace.

This alpha contract extends that truth across the four crossings the
workflow can leave the local UI for:

- a **remote helper attach** (SSH workspace, container helper,
  devcontainer helper);
- a **provider overlay link** (managed-cloud workspace, provider-locked
  search / index overlay);
- an **export archive write**;
- a **browser-handoff mint** (provider follow-up packet);
- a **support packet bundle** (support triage).

The promise is narrow and frozen: the propagation alpha record carries
the source beta truth's `workset_ref`, `stable_scope_id`, `scope_class`,
`scope_mode`, `included_roots`, `excluded_roots`, patterns, and
`lineage` chain verbatim into every crossing. A crossing that would
flatten a sparse, selected, or policy-limited workset into a
workspace-wide truth — or mask remote / provider degraded state — is
rejected at validation, not silently allowed.

The Rust types live in `aureline_workspace::scope_propagation`. The
canonical fixtures live under
[`/fixtures/workspace/m3/remote_provider_scope/`](../../../fixtures/workspace/m3/remote_provider_scope/).
The integration test
[`crates/aureline-workspace/tests/scope_propagation_alpha.rs`](../../../crates/aureline-workspace/tests/scope_propagation_alpha.rs)
replays every fixture and proves the closed acceptance states.

## 1 Propagation alpha contract

A `ScopePropagationAlphaRecord` is the bounded projection one beta truth
hands to a single crossing. The record quotes:

- the source `workset_ref` and `stable_scope_id` — a destination never
  mints a parallel scope id for the same scope;
- the source `scope_class` / `scope_mode` from the alpha vocabulary
  (`current_repo`, `selected_workset`, `sparse_slice`, `full_workspace`,
  `policy_limited_view`);
- the `BetaConsumerSurface` that minted the source truth (search,
  graph, refactor, AI, export, or support_packet);
- the typed `ScopePropagationCrossingClass`;
- a destination label (e.g. `rh:devhelper:east-1`,
  `po:gh:enterprise:overlay`, `export:disk:bundle:0`,
  `browser:handoff:packet:0`, `support:bundle:0`);
- the closed `ScopePropagationDispositionClass` the crossing resolved
  to;
- a typed `ScopePropagationDegradedReason` when the crossing preserved
  scope labels but disclosed degraded runtime state;
- the preserved `included_roots`, `excluded_roots`,
  `include_patterns`, and `exclude_patterns` from the beta truth;
- the lineage chain (a single entry for narrow / full-workspace scopes,
  two entries for `policy_limited_view`);
- the four `ScopePropagationGuardrail` observations every conforming
  record carries;
- a `remote_attach_disclosed` boolean (always `true` on a conforming
  record — the record itself is the disclosure surface);
- a `hidden_member_count_preserved` boolean and, on `policy_limited_view`
  propagations, the preserved `hidden_member_count` without enumerating
  the hidden list.

## 2 Crossings and dispositions

The closed crossing vocabulary is:

| Crossing | Destination |
| --- | --- |
| `remote_helper_attach` | SSH / container / devcontainer helper. |
| `provider_overlay_link` | Managed-cloud workspace or provider-locked overlay. |
| `export_archive_write` | Export bundle writer (disk or cloud). |
| `browser_handoff_mint` | Provider follow-up browser-handoff packet. |
| `support_packet_bundle` | Support archive bundler. |

Every crossing resolves to exactly one
`ScopePropagationDispositionClass`:

- `scope_labels_preserved_exact` — every scope label survived intact.
- `scope_labels_preserved_degraded` — labels survived, but the crossing
  disclosed a typed degraded reason
  (`remote_helper_unreachable`, `remote_helper_skew`,
  `provider_overlay_stale`, `browser_handoff_expiring_session`,
  `export_target_unavailable`, `support_attribution_only`).
- `blocked_by_outside_scope` — the source row was outside the active
  workset (mirrors the beta `blocked_by_outside_scope` cue).
- `blocked_by_policy` — a policy overlay denies the destination lane.
- `blocked_by_portability` — the portability posture denies the
  destination (e.g. managed-provider-locked export).

Frozen rules:

1. `scope_labels_preserved_degraded` MUST carry a typed
   `degraded_reason` *and* an explain note.
2. `scope_labels_preserved_exact` MUST NOT carry a degraded reason.
3. Every blocked disposition MUST carry an explain note.
4. `remote_helper_attach`, `provider_overlay_link`, and
   `browser_handoff_mint` MUST disclose attach / freshness state — the
   record's `remote_attach_disclosed` boolean is the closed disclosure
   surface.

## 3 Guardrails

Every conforming record observes the closed
`ScopePropagationGuardrail` vocabulary:

- `no_silent_scope_widening` — the crossing did not widen
  `scope_class`, `scope_mode`, or the included-root list relative to the
  source beta truth.
- `hidden_members_not_leaked` — the crossing did not enumerate the
  policy-limited hidden member list; the hidden_member_count is
  preserved as attribution.
- `degraded_state_not_masked` — remote / provider / browser-handoff
  degraded state is disclosed instead of masked as
  attached / fresh.
- `lineage_preserved` — the lineage chain crosses into the destination
  verbatim.

A record missing any guardrail (or carrying it more than once) is
rejected at validation. There is no "best effort" guardrail mode.

## 4 Hidden-member accounting on policy-limited propagations

For `policy_limited_view` scope, the propagation MUST:

- include the underlying workset as `lineage[1]` (the beta truth's
  policy-limited lineage rule applies verbatim);
- carry the `hidden_member_count` from the source artifact's
  `policy_limitation.hidden_member_count` so the support / export
  consumer attributes the gap without leaking the hidden member list;
- pin `hidden_member_count_preserved = true`.

Admin-policy and license-or-export-control narrowing causes never
expose the hidden list — neither at the beta truth boundary nor at the
propagation boundary.

## 5 Support packet — first consumer

The first wired consumer is
[`ScopePropagationAlphaSupportExport`](../../../crates/aureline-workspace/src/scope_propagation/mod.rs).
A support triage flow bundles every crossing for one artifact identity
into a single packet so the reviewer replays scope from the same
artifact rather than re-deriving it from a remote / provider / export
side channel.

The packet enforces:

- every propagation references the same `workset_ref`,
  `stable_scope_id`, `scope_class`, and `scope_mode`;
- every propagation passes
  `ScopePropagationAlphaRecord::validate`;
- the packet's lineage is taken from the first propagation; mixed
  artifacts are rejected with
  `ScopePropagationAlphaError::ScopeLabelsMustMatchBetaTruth`.

The bundled record-kind is
`scope_propagation_alpha_support_export`. The bundled propagation
record-kind is `scope_propagation_alpha_record`.

## 6 Out of scope for this alpha

- Live attach / detach orchestration (owned by the remote-helper crate).
- Provider overlay refresh and cache invalidation (owned by the
  provider crate's overlay machinery).
- Browser-handoff packet *minting* — the runtime / provider crates own
  the actual packet; this alpha owns the scope-preservation contract
  between the active workset and the minted packet.
- Promotion of a propagation into a workspace-level review or approval
  ticket — the existing review / approval crates own that surface.
