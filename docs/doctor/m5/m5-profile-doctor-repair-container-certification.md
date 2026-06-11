# M5-profile Doctor, guided-repair, and container/devcontainer certification

This document describes the canonical packet that certifies every **marketed M5
product profile** for combined Project Doctor, guided-repair, and
container/devcontainer maturity, and that automatically narrows or fails promotion
on any underqualified profile before publication. It is the user-facing companion
to the governed artifact at
`artifacts/doctor/m5/m5-profile-doctor-repair-container-certification.json` and the
typed model in the `aureline-doctor` crate
(`certify_doctor_repair_container_maturity_on_all_claimed_m5_profiles`).

Where the
[`doctor-repair-container-maturity-matrix`](doctor-repair-container-maturity-matrix.md)
packet certifies each recovery *capability* against each *deployment profile*, this
packet certifies each *product profile* — the marketed M5 surface a user actually
opens — for the combined recovery story it ships.

## What this packet covers

The packet carries one row for every claimed M5 profile:

1. **`notebook`** — notebook kernels and notebook recovery.
2. **`request_api`** — request/API auth and environment recovery.
3. **`database`** — database target recovery.
4. **`profiler`** — profiler/replay instrumentation recovery.
5. **`remote_preview`** — remote-preview route recovery.
6. **`sync`** — sync/offboarding/device-registry recovery.
7. **`companion`** — companion handoff recovery.
8. **`incident`** — incident-packet recovery.

Each row answers, for its profile:

- **What do the lanes claim?** A `doctor_maturity`, `repair_maturity`, and
  `container_maturity`, each `certified`, `provisional`, `underqualified`, or
  `unsupported`, plus a `declared_qualification` for the profile as a whole.
- **How fresh is the qualification packet?** An `evidence_freshness` of `current`,
  `stale`, `expired`, or `unknown`.
- **Is diagnosis fast enough?** A `diagnosis_latency_state` of `green`, `amber`,
  `red`, or `unmeasured`.
- **Is the engine reachable?** An `engine_reachability` of `reachable`, `degraded`,
  `blocked`, or `not_applicable`.
- **Is the container boundary proven?** A `boundary_proof` of `verified`,
  `partial`, `unverified`, or `not_applicable`.
- **What is backing it?** A `qualification_packet_ref` to the profile's current
  Doctor/repair/container qualification packet, a `latency_corpus_ref`, a
  `rollback_ref`, a `compatibility_ref` (downgrade story), a
  `container_boundary_ref` where a boundary exists, and a `support_export_ref` that
  binds the row into Help/About, support exports, and release surfaces.
- **What does the gate publish?** A `published_qualification`, a
  `certification_decision`, and the headline `narrowing_reasons` that explain it.

## The certification gate narrows automatically

The qualification a profile may publish is **not** copied from
`declared_qualification`. It is recomputed and the `published_qualification`,
`certification_decision`, and `narrowing_reasons` fields must equal that
recomputation or validation fails. The gate lowers the published qualification to
the weakest of:

- the **capability floor** — the minimum of the declared qualification and the
  Doctor, repair, and container maturities;
- the **freshness ceiling** — `current` permits `certified`, `stale`/`unknown` cap
  at `provisional`, and `expired` caps at `underqualified`;
- the **latency ceiling** — `green` permits `certified`, `amber`/`unmeasured` cap
  at `provisional`, and `red` caps at `underqualified`;
- the **engine ceiling** — `reachable`/`not_applicable` permit `certified`,
  `degraded` caps at `provisional`, and `blocked` caps at `underqualified`;
- the **boundary ceiling** — `verified`/`not_applicable` permit `certified`,
  `partial` caps at `provisional`, and `unverified` caps at `underqualified`.

The `certification_decision` then names the result: `promote` for a published
`certified`, `narrow_to_provisional`, `narrow_to_underqualified`, or
`fail_promotion` for a withheld `unsupported` claim.

The `narrowing_reasons` are the five canonical, spec-aligned release-control
triggers, each recomputed from the observed states:

- **`stale`** — freshness is `stale` or `expired`.
- **`diagnosis_latency_red`** — the latency state is `red`.
- **`repair_underqualified`** — the repair maturity is `underqualified` or
  `unsupported`.
- **`engine_blocked`** — engine reachability is `blocked`.
- **`boundary_proof_missing`** — the container/devcontainer boundary proof is
  `unverified`.

This is what lets release/public-truth tooling **prove** that stale or
underqualified profiles narrow before publication: a profile that is stale,
latency-red, repair-underqualified, engine-blocked, or boundary-missing simply
cannot carry a `certified` published claim, because the recomputed gate decision
overrides the stored one.

## Certification stays profile-specific and freshness-bound

A strong notebook lane must never imply container/devcontainer or blocked-user
recovery maturity on an unrelated profile. The packet enforces this several ways:

- Every claimed profile must carry exactly one row (`MissingProfileRow` /
  `DuplicateProfileRow` otherwise), so no profile inherits trust from an adjacent
  one, and a row may not cover a profile outside the claimed set
  (`UnclaimedProfileRow`).
- Every row must carry its own non-empty `qualification_packet_ref`,
  `latency_corpus_ref`, `rollback_ref`, `compatibility_ref`, and
  `support_export_ref`.
- A profile that carries a container/devcontainer boundary must reference its
  boundary proof (`MissingBoundaryProofRef` otherwise), so "missing boundary proof"
  can never hide behind an absent ref.

A promotable profile — one that publishes `certified` — must additionally be
genuinely clean: current freshness, green latency, a reachable/not-applicable
engine, a verified/not-applicable boundary, all-certified capabilities, and no
narrowing reason (`PromotedProfileNotClean` otherwise).

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each profile's
declared and published qualification, its Doctor/repair/container maturities,
freshness, latency, engine, and boundary states, decision, and narrowing-reason
tokens, plus `promotable_count`, `narrowed_count`, and `failed_promotion_count`.
Help/About, `docs/migration`, support exports, and release/public-truth and
shiproom surfaces should ingest this projection directly rather than restating M5
recovery or container status by hand, so public and internal claim surfaces use the
same lifecycle, freshness, downgrade, and reversal vocabulary as the underlying
packet.

## Validation

`M5ProfileCertification::validate()` reports every violation, including an
unsupported schema version or record kind, non-canonical closed vocabularies, empty
required fields, duplicate profile ids, duplicate or missing profile rows,
unclaimed-profile rows, duplicate narrowing reasons, a boundary profile missing its
boundary-proof ref, an overstated published qualification, a decision that
disagrees with the gate, narrowing reasons that disagree with the recomputed set, a
promotable profile that is not clean, and a summary block that disagrees with the
rows.
