# M5 workflow-bundle and project-entry governance matrix

This document describes the canonical packet that freezes the **M5 workflow-bundle and
project-entry governance matrix** — the single qualification report that graduates the M5
workflow-bundle, source-acquisition, project-open, project-import, session-resume,
recent-work, and workspace-admission switching lanes. It aggregates the stable-line entry and
bundle packets into one governance gate that automatically narrows or withholds the published
label of any lane whose source is unverified, whose archetype is only probable or mixed, whose
roots did not resolve, whose restore is partial, whose bundle scorecard is stale, or whose
entry topology is unsupported. It is the user-facing companion to the governed artifact at
`artifacts/workspace/m5/m5-entry-and-bundle-governance.json` and the typed model in the
`aureline-workspace` crate (`m5_entry_and_bundle_governance`).

This packet answers the switching/entry depth question for the M5 entry lane as a whole:
**does opening, cloning, importing, resuming, or installing a workspace bundle reach
first-useful-work without silently widening trust over a probable, unverified, or partially
restored entry — or is it automatically downgraded to a bounded or retest-pending label, or
refused, before publication?**

## What this packet covers

The packet carries one governance row for every claimed M5 entry lane, each pinned to one
distinct entry verb so clone, open, import, and resume never blur together, and each pinned to
the canonical entry-truth packet it draws its evidence from:

1. **`workflow_bundle`** (verb `install`) — workflow-bundle composition, install diff, and
   rollback checkpoint.
2. **`source_acquisition`** (verb `clone`) — source-locator resolution and checkout-plan
   acquisition.
3. **`project_open`** (verb `open`) — opening an existing local workspace root.
4. **`project_import`** (verb `import`) — importing or migrating a workspace from another tool.
5. **`session_resume`** (verb `resume`) — resuming a prior session and restoring its state.
6. **`recent_work`** (verb `open`) — recent-work registry truth feeding the start center.
7. **`workspace_admission`** (verb `open`) — workspace-admission routing and first-useful-work
   selection.

The workflow-bundle, source-acquisition, project-import, session-resume, and
workspace-admission lanes are **trust-sensitive**: they can silently widen trust and must
narrow safely rather than inherit a broader stable claim.

Each row answers, for its lane:

- **Who owns it?** An `owner` accountable for the lane's evidence and conformance.
- **What is it?** A `bundle_class` (launch bundle, framework pack, template bundle, imported
  handoff bundle, org-managed bundle, or none) and a `locator_type` (local path, git remote,
  archive import, tool migration, recent handle, or not applicable).
- **How trusted is the source?** A `source_trust` of `first_party`, `trusted_remote`,
  `unverified_remote`, or `untrusted` — the host class and trust stage.
- **How confident is detection?** An `archetype_confidence` of `confirmed`, `probable`,
  `mixed`, or `undetected`.
- **Did the roots resolve?** A `root_resolution` of `resolved`, `single_root_assumed`,
  `probable_multi_root`, or `missing`.
- **How faithful is restore?** A `restore_fidelity` of `exact`, `partial`, `degraded`, or
  `unavailable`.
- **Is the bundle scorecard current?** A `bundle_scorecard` of `current`, `aging`, `stale`, or
  `missing`.
- **Is the entry topology supported?** An `entry_topology_support` of `supported`,
  `degraded_support`, `experimental`, or `unsupported`.
- **How does routing land?** A `setup_queue_class` of `ready`, `setup_later`,
  `blocked_on_setup`, or `missing_root`, with `deferred_setup_count` and `missing_root_count`
  so a ready entry stays distinct from a setup-later, blocked, or root-missing one.

## The governance gate

The gate lowers each lane's declared assurance to the **weakest ceiling** implied by its
observed states. The ceilings:

| State (best → worst) | `verified` | `bounded` | `retest_pending` | `withheld` |
| --- | --- | --- | --- | --- |
| `source_trust` | first_party | trusted_remote | unverified_remote | untrusted |
| `archetype_confidence` | confirmed | probable | mixed | undetected |
| `root_resolution` | resolved | single_root_assumed | probable_multi_root | missing |
| `restore_fidelity` | exact | partial | degraded | unavailable |
| `bundle_scorecard` | current | aging | stale | missing |
| `entry_topology_support` | supported | degraded_support | experimental | unsupported |

The **published assurance** is the minimum of the declared floor and every ceiling above, so
**probable or mixed workspace detection never silently widens trust** — a probable archetype
caps the lane at `bounded`, a mixed one at `retest_pending`, and an undetected one at
`withheld`, regardless of how strong the other states are.

The **admission outcome** mirrors the published label one-to-one: `admit_full` for `verified`,
`admit_bounded` for `bounded`, `admit_retest` for `retest_pending`, and `refuse` for
`withheld`. The recorded published label, admission outcome, and downgrade reasons must equal
the recomputed gate decision, so a narrowed lane cannot stay stable by inertia.

### Downgrade reasons and recovery

The six headline downgrade reasons are recomputed from the observed states, not asserted by
hand:

- `unverified_source` — the source is not first-party.
- `probable_or_mixed_detection` — the archetype is only probable or mixed.
- `missing_roots` — expected workspace roots are missing or ambiguous.
- `partial_restore` — the restore is partial, degraded, or unavailable.
- `stale_bundle_scorecard` — the bundle scorecard is aging, stale, or missing.
- `unsupported_entry_topology` — the entry topology is degraded, experimental, or unsupported.

A narrowed or refused lane must offer a real recovery path (`verify_source`,
`confirm_archetype`, `resolve_roots`, `repair_restore`, `refresh_bundle_scorecard`,
`request_topology_support`, or `withhold_claim`), list at least one caveat, and name what is
stale or narrowing. A `verified` lane must be genuinely whole-trust — first-party, confirmed,
fully resolved, exact, current, supported, with a `ready` setup queue and nothing deferred or
missing — so a lane never widens trust over a probable or unverified entry.

## Distinct verbs and first-useful-work routing

Clone, open, import, and resume remain distinct verbs, validated against each lane's pinned
verb, and bundle install is its own verb. First-useful-work routing stays explicit: the
setup-queue class keeps a `ready` entry distinct from a `setup_later`, `blocked_on_setup`, or
`missing_root` one, so an entry that opens minimally with setup deferred is never conflated
with one blocked on required setup or missing its root. The setup-later and open-minimal paths
are preserved as bounded labels rather than refused.

<a id="release-evidence"></a><a id="help-surface"></a><a id="docs-badges"></a><a id="support-export"></a>

## Consumer surfaces

Release evidence, help/start-center, docs badges, and support export each bind to this one
packet via the `release_evidence_ref`, `help_surface_ref`, `docs_badge_ref`, and
`support_export_ref` on every row, ingest it, preserve its labels and recovery paths, and
narrow with it, so a row narrowed here cannot read as stable downstream. The
`export_projection` is the redaction-safe index those surfaces render instead of restating each
lane's posture by hand; it carries typed states, counts, and opaque refs only — no credential
bodies, raw provider payloads, or workspace contents.

## Conformance anchors

<a id="workflow-bundle"></a><a id="source-acquisition"></a><a id="project-open"></a><a id="project-import"></a><a id="session-resume"></a><a id="recent-work"></a><a id="workspace-admission"></a>

Each lane's row references this document for its conformance, release-evidence, help-surface,
docs-badge, and support-export anchors. The typed model and its validation gate live in
`crates/aureline-workspace/src/m5_entry_and_bundle_governance/`, the JSON Schema at
`schemas/workspace/m5-entry-and-bundle-governance.schema.json`, and the fixture corpus at
`fixtures/workspace/m5/m5-entry-and-bundle-governance/`.
