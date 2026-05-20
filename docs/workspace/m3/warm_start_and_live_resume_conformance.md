# Warm-start, prebuild, and live-resume conformance (M3 beta)

This contract defines the regression-gated proof system for the M3 warm-start,
prebuild, and live-resume beta boundary. It is normative for the
`workspace.warm_start_and_live_resume.beta` corpus and is the document the
warm-start choice surface (`shell:start_center_warm_start_choice_beta:v1`) reads
when it claims a beta starter or resume row.

## Boundary

- **Runtime model:** `aureline_shell::start_center::warm_start_choice::WarmStartChoiceCard`
- **Validator:** `aureline_shell::start_center::warm_start_choice::validate_warm_start_choice_card`
- **Card schema:** [`/schemas/workspace/warm_start_choice.schema.json`](../../../schemas/workspace/warm_start_choice.schema.json)
- **Corpus schema:** [`/schemas/workspace/warm_start_conformance.schema.json`](../../../schemas/workspace/warm_start_conformance.schema.json)
- **Corpus manifest (single source of truth):** [`/fixtures/workspace/m3/warm_start_and_live_resume/manifest.json`](../../../fixtures/workspace/m3/warm_start_and_live_resume/manifest.json)
- **Harness:** [`/crates/aureline-qe/src/warm_start_live_resume/`](../../../crates/aureline-qe/src/warm_start_live_resume/)
- **Replay command:** `cargo test -p aureline-qe --test warm_start_live_resume_conformance`
- **Published evidence:**
  [`/artifacts/ops/m3/warm_start_freshness_and_bypass_report.md`](../../../artifacts/ops/m3/warm_start_freshness_and_bypass_report.md)
  and
  [`/artifacts/compat/m3/template_prebuild_resume_matrix.json`](../../../artifacts/compat/m3/template_prebuild_resume_matrix.json)

## What a warm-start card must disclose

A warm-start choice card describes — **before any networked or trust-widening side
effect occurs** — what each entry path will do. Each card pins:

- **Source class** — `workspace_template`, `prebuild_snapshot`, `live_workspace`,
  `remote_repository`, or `local_folder`.
- **Support class** — `certified`, `supported`, `limited`, `experimental`,
  `community`, or `unsupported`.
- **Runtime / host model** — `local_host`, `devcontainer`,
  `managed_cloud_workspace`, or `ssh_workspace`.
- **Exact setup actions** — the plain-language steps a starter would perform.
- **Snapshot facts** (when a snapshot backs the card) — an opaque fingerprint
  reference, the freshness (`fresh`, `cached`, `stale`, `invalidated`,
  `unverified`), the coarse age bucket, and the invalidation reason whenever the
  snapshot is stale or invalidated.
- **Entry lanes and the next action** — the distinct choices the user picks
  between: Resume live, Start from snapshot, Clone fresh, Open minimal, Set up
  later, or Use template — each with its own availability and side-effect class.
- **Environment starter** — where setup runs and the bypass/defer routes that keep
  a same-weight local path.

## Invariants the corpus pins

On **every** positive drill, in addition to the per-row expectations encoded in
the manifest, the harness re-checks:

1. **Local-safe default.** The `safest_next_action` resolves to a lane that takes
   no network egress, runs no setup, and grants no trust. The default never widens
   trust (`default_widens_trust = false`) and never runs networked work
   (`default_runs_networked_work = false`).
2. **A same-weight path to open without the starter.** Every card keeps a
   local-safe Open-minimal lane. Local-first cards keep both Open-minimal and
   Set-up-later at the same weight as the convenience lanes.
3. **No silent side effects.** A lane that fetches over the network, widens trust,
   runs setup tasks, or attaches a managed/remote runtime is gated behind review
   — it is never immediately `available`.
4. **A stale snapshot is never a live resume.** When a snapshot is stale or
   invalidated, the card surfaces the invalidation reason and the live-resume lane
   is not takeable.
5. **No remote masquerade.** A lane that reaches the network, attaches
   managed/remote runtime, or widens trust must not advertise a local-safe
   side-effect class and must not be one of the local escape hatches.
6. **Redaction.** Only typed labels, opaque `sha256:` fingerprint references, and
   reviewable sentences cross the boundary; the validated card carries no raw
   secret, credential, key, or absolute home path.

## Negative drills

Each negative drill applies a typed tamper to a contract-valid base card and
requires `validate_warm_start_choice_card` to reject it with a finding containing
the recorded substring. The tampers cover: a stale snapshot offering a takeable
live resume; a stale snapshot that omits its reason; a networked lane masquerading
as local; an Open-minimal lane that acquires a side effect; a default that is not
local-safe; a default that widens trust; Set-up-later losing same weight; a
starter that runs setup without a bypass or defer route; a managed attach hidden
from the side-effect summary; a source-class token drift; and an honesty marker
that contradicts a stale snapshot.

## Coverage and degradation

The corpus keeps a drill for every source class, support class, runtime/host
model, entry lane, snapshot freshness state, and lane availability state. Stale,
invalidated, mirror-unverified, offline, and policy-narrowed starters degrade to
precise states — a disabled lane with a named reason, a withheld resume pending
verification, an offline-usable cached copy, or a `blocked_by_policy` lane — never
a generic setup failure. The published freshness/bypass report and
template/prebuild/resume matrix are regression-gated to cover every drill and to
agree with the corpus on source class and snapshot freshness, so the claim
manifest, docs/help, and Start Center / workspace-switcher badges stay aligned.

## Change control

`manifest.json` is authoritative. Removing any positive or negative drill without
a replacement, or adding a warm-start row to the beta claim without a corresponding
drill, is a breaking contract change for the
`workspace.warm_start_and_live_resume.beta` corpus.
