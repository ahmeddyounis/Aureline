# Warm-start, prebuild, and live-resume conformance corpus

This corpus is the conformance, interoperability, operations/deployment,
failure / recovery, and design-QA drill harness for the M3 warm-start, prebuild,
and live-resume beta boundary owned by
[`aureline-shell`](../../../../crates/aureline-shell/src/start_center/warm_start_choice/mod.rs)
(`validate_warm_start_choice_card` / `WarmStartChoiceCard`).

It converts the warm-start UX promise into a regression-gated proof system: each
positive drill carries one warm-start choice card and pins the truth the card must
hold — the source class, support class, runtime/host class, the available entry
**lanes** (Resume live, Start from snapshot, Clone fresh, Open minimal, Set up
later, Use template), per-lane availability, the snapshot freshness / age /
invalidation facts, the environment-starter setup location, the local-safe
default, and the honesty marker. Each negative drill applies a typed tamper to a
contract-valid base card and pins the contract finding that must reject it.

Every drill is loaded by the conformance harness at
[`crates/aureline-qe/src/warm_start_live_resume/`](../../../../crates/aureline-qe/src/warm_start_live_resume/)
and replayed by
`cargo test -p aureline-qe --test warm_start_live_resume_conformance`.

## Single source of truth

`manifest.json` is authoritative. Positive drills MUST load a contract-valid card
and match **every** `expect` field in the manifest. Negative drills MUST raise a
finding whose message contains `expected_failure_substring` after the recorded
tamper is applied to the named base card. The fixtures carry only the card record
(no restated expectations), so there is exactly one place to read and audit the
pinned truth.

Boundary schemas, contract, and published evidence:

- Card schema: [`/schemas/workspace/warm_start_choice.schema.json`](../../../../schemas/workspace/warm_start_choice.schema.json)
- Corpus schema: [`/schemas/workspace/warm_start_conformance.schema.json`](../../../../schemas/workspace/warm_start_conformance.schema.json)
- Beta contract: [`docs/workspace/m3/warm_start_and_live_resume_conformance.md`](../../../../docs/workspace/m3/warm_start_and_live_resume_conformance.md)
- Freshness & bypass report: [`artifacts/ops/m3/warm_start_freshness_and_bypass_report.md`](../../../../artifacts/ops/m3/warm_start_freshness_and_bypass_report.md)
- Template / prebuild / resume matrix: [`artifacts/compat/m3/template_prebuild_resume_matrix.json`](../../../../artifacts/compat/m3/template_prebuild_resume_matrix.json)

## Coverage axes

| Axis | Drill id |
| --- | --- |
| Template — first-party certified, local host | `template.first_party_certified.local` |
| Template — local-only, runs local setup without download | `template.local_only.offline` |
| Template — team-managed, setup in a dev container | `template.team_managed.devcontainer` |
| Template — community, experimental support claim | `template.community.experimental` |
| Template — policy-narrowed list blocks the generate lane | `template.policy_narrowed.blocked` |
| Local-only — plain folder, no starter | `local_only.folder_open` |
| Live resume — managed cloud, requires re-authorization | `live_resume.managed_requires_reauth` |
| Live resume — suspended SSH workspace, reconnect | `live_resume.ssh_suspended` |
| Snapshot — fresh local prebuild, takeable resume | `snapshot.fresh_local_resume` |
| Snapshot — stale, capsule-drift downgrade | `snapshot.stale_capsule_drift` |
| Snapshot — invalidated by lockfile change | `snapshot.invalidated_lockfile` |
| Snapshot — mirror-only, unverified freshness | `snapshot.mirror_only.unverified` |
| Clone fresh — remote repository | `clone_fresh.remote_repository` |
| Clone fresh — offline fallback to a cached copy | `clone_fresh.offline_cached_fallback` |
| Negative — stale snapshot offers a takeable live resume | `negative.stale_snapshot_resume_takeable` |
| Negative — stale snapshot omits its invalidation reason | `negative.stale_snapshot_missing_reason` |
| Negative — networked lane masquerades as local | `negative.remote_lane_masquerades_as_local` |
| Negative — open-minimal acquires a network side effect | `negative.escape_hatch_has_side_effect` |
| Negative — default action is not local-safe | `negative.safest_action_not_local_safe` |
| Negative — default action widens trust | `negative.default_widens_trust` |
| Negative — set-up-later loses same weight | `negative.local_first_escape_hatch_not_same_weight` |
| Negative — starter omits a bypass route | `negative.environment_starter_missing_bypass` |
| Negative — starter omits a defer route | `negative.environment_starter_missing_defer` |
| Negative — managed attach undisclosed in summary | `negative.managed_attach_undisclosed` |
| Negative — source-class token drift | `negative.source_class_token_drift` |
| Negative — honesty marker inconsistent with a stale snapshot | `negative.honesty_marker_inconsistent` |

## Transverse invariants

The conformance suite also pins, across the whole positive set, that the corpus
keeps a drill for:

- every source class (`workspace_template`, `prebuild_snapshot`, `live_workspace`,
  `remote_repository`, `local_folder`);
- every support class (`certified`, `supported`, `limited`, `experimental`,
  `community`, `unsupported`);
- every runtime/host model (`local_host`, `devcontainer`,
  `managed_cloud_workspace`, `ssh_workspace`);
- every entry lane (`resume_live_workspace`, `start_from_snapshot`, `clone_fresh`,
  `open_minimal`, `set_up_later`, `use_template`);
- every snapshot freshness state (`fresh`, `cached`, `stale`, `invalidated`,
  `unverified`); and
- every lane availability state (`available`, `available_after_review`,
  `requires_reauth`, `unavailable_stale_snapshot`, `blocked_by_policy`).

On every positive drill the runner additionally re-pins the cross-cutting
warm-start guarantees: the default action resolves to a local-safe lane and never
widens trust or runs networked work; an Open-minimal lane is always present and
local-safe so the user keeps a same-weight path to open without the starter;
local-first cards keep both Open-minimal and Set-up-later at the same weight; any
lane that runs setup, widens trust, fetches over the network, or attaches a
managed/remote runtime is gated behind review (never immediately `available`); a
stale or invalidated snapshot never backs a takeable live resume; and the
serialized card carries no forbidden raw-content token.

## Redaction guarantees

Every fixture is metadata-safe: only typed labels, opaque `sha256:` fingerprint
references, and reviewable sentences cross the boundary. The runner scans each
fixture and the validated card for forbidden raw-content tokens (private keys,
absolute home paths, cloud keys, bearer tokens). Removing any positive or negative
drill without a replacement is a breaking contract change for the
`workspace.warm_start_and_live_resume.beta` corpus.
