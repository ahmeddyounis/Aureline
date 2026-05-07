# Warm-start chooser contract: resume live, start from snapshot, clone fresh, open without starter (freshness + revalidation truth)

This artifact freezes the cross-surface contract for the **warm-start chooser** shown anywhere Aureline offers an accelerative “warm start” path.

Its goal is supportable honesty: warm starts MUST be explicit about **liveness**, **age**, and **what must be revalidated** before the workspace can be treated as live/write-capable again. Those disclosures MUST be visible **before commit**, and MUST remain recoverable through **open**, **restore**, and **support exports**.

This file does **not** define final UI composition. It freezes the **record fields, lane separation, and failure posture** so later surfaces cannot hide materially different recovery implications behind a single ambiguous “Open”.

## 1. Canonical sources (quoted by reference)

Warm-start path separation and invalidation truth:

- `docs/workspace/prebuild_fingerprint_contract.md` (path separation: resume-live vs snapshot vs clone fresh)
- `schemas/workspace/prebuild_fingerprint.schema.json` (required revalidation axes; cache classes; host/platform classes)

Entry routes, restore/resume vocabulary, and safe fallback hooks:

- `docs/workspace/entry_restore_object_model.md`
- `docs/ux/project_entry_contract.md`
- `docs/ux/workspace_entry_route_matrix.md`

Warm-start, template/prebuild, and open-without-starter disclosure invariants:

- `docs/ux/template_and_prebuild_contract.md` (four-lane separation + equal-weight bypass)
- `artifacts/entry/environment_starter_summary_contract.md` (starter side-effect truth; bypass parity)

Supportable post-choice truth and failure posture:

- `docs/workspace/bootstrap_packet_contract.md`
- `schemas/workspace/bootstrap_packet.schema.json` (post-open prerequisites; resumability actions; freshness classes)

Authoritative design anchors:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` (templates/warm-start disclosure; session restore truth)
- `.t2/docs/Aureline_Technical_Design_Document.md` (environment/prebuild semantics; managed-workspace boundaries)

Machine-readable boundary schema + fixtures for this artifact:

- `schemas/entry/freshness_revalidation.schema.json`
- `fixtures/entry/warm_start_cases/`

## 2. Warm-start lanes (non-negotiable separation)

Every warm-start chooser instance MUST render these four lanes as **distinct choices** (never a single “Open”):

1. **Resume live workspace** — reattach to a still-live session (`lane_class = resume_live_workspace`).
2. **Start from snapshot** — materialize a prepared snapshot/prebuild (`lane_class = start_from_snapshot`).
3. **Clone fresh** — acquire source bytes from origin/mirror without relying on prior warm state (`lane_class = clone_fresh_repository`).
4. **Open without starter** — the safe bypass lane (`lane_class = open_without_starter`).

If a lane is not currently safe/available, it MUST remain **visible-but-disabled** with an actionable reason; it MUST NOT disappear and force users into a less honest lane.

## 3. Required disclosure axes (every lane row)

Each lane row MUST disclose the following axes before commit:

1. **Identity anchor**
   - Live: `live_session_handle_ref` plus `attach_authority_class`.
   - Snapshot: `snapshot_ref` and (when known) `commit_identity_ref`.
   - Clone fresh: (when known) `commit_identity_ref` or revision intent refs.
2. **Freshness truth**
   - `declared_freshness_class` and `freshness_age_class` with an `as_of` anchor.
   - A snapshot MUST NOT be described as “live” even if it is recent.
3. **Host/platform truth**
   - `host_class` and `platform_arch` (and incompatibility, if known).
4. **Pending updates**
   - Any cache classes expected to rebuild/refresh (`pending_cache_update_classes`).
   - Any typed prerequisites expected/required (`pending_prerequisite_classes`).
5. **Revalidation requirements**
   - A lane MUST enumerate `revalidation_items[]` with:
     - the item class (credentials/ports/indexes/policy/trust/environment/toolchain/etc plus post-open prerequisites like package restore, extension restore, devcontainer attach),
     - the posture (required before open / before mutation / before trusted bootstrap / optional deferred),
     - and whether it blocks the lane’s “live again” claim.
6. **Restore safety**
   - `write_safety_badge` and `restore_availability` so the user can tell whether writes are allowed, blocked, or require revalidation.
7. **Failure posture (safe fallback)**
   - `failure_posture` MUST be explicit: if revalidation fails, the surface must keep at least one safe action available and recommend at least one safe lane fallback (with `open_without_starter` always remaining reachable).

## 4. Lane-specific rules

### 4.1 Resume live workspace (`resume_live_workspace`)

Required truth:

- `attach_authority_class` MUST be present and MUST fail closed:
  - `authority_expired` / `authority_pending_reauth` MUST surface as revalidation-required (not silently refreshed).
- Resume-live rows MUST NOT imply that:
  - cached credentials are valid,
  - cached policy/trust is current,
  - ports or tunnels remain live,
  - terminals/tasks/debug sessions are “running again”.

Failure posture:

- If resume cannot proceed, the chooser MUST keep safe fallbacks available (start from snapshot, clone fresh, open without starter; plus resumability actions such as `continue_in_restricted_mode`, `open_read_only_partial`, or `set_up_later` where applicable).

### 4.2 Start from snapshot (`start_from_snapshot`)

Required truth:

- Snapshot rows MUST disclose `snapshot_ref`, `commit_identity_ref` when known, and `freshness_age_class`.
- Snapshot rows MUST enumerate what will revalidate or rebuild (dependencies, indexes, extensions, toolchains/runtime images) and MUST be explicit about write safety.

Failure posture:

- If the snapshot is invalidated, incompatible, or missing required artifacts, the row MUST be disabled with a reason and MUST recommend clone-fresh/open-without-starter fallbacks.

### 4.3 Clone fresh (`clone_fresh_repository`)

Required truth:

- Clone-fresh rows MUST disclose `declared_freshness_class` (live origin vs mirror vs offline bundle) and any prerequisites that block clone/open (credentials, mirror refresh, policy review).
- Clone-fresh MUST NOT imply trust; trust remains routed through the trust/admission flows.

### 4.4 Open without starter (`open_without_starter`)

Required truth:

- This lane is the bypass path and MUST remain present at equal weight.
- If the environment/starter lanes are narrowed or blocked, this lane MUST remain actionable (even when it leads to a narrower capability posture such as restricted or minimal open).

## 5. Exportability (open/restore/support continuity)

The warm-start chooser emits:

- a chooser-set record before selection,
- a decision record at selection,
- and an outcome record after the lane resolves.

Support and troubleshooting exports MUST be able to recover:

- which lane was chosen,
- the chosen lane’s identity anchors (snapshot/commit/live session),
- freshness age class at decision time,
- the declared revalidation items and the safe failure posture,
- and (when available) the linked `bootstrap_packet_record` that reconstructs the post-choice path.

