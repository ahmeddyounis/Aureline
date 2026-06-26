# M5 debug qualification

This contract certifies every claimed M5 **debugger-facing** row against the shared debug
object model and its evidence corpus, and narrows the published claim automatically when
debugger evidence is stale, partial, or failing. It materializes three families as concrete,
typed, serde-serializable records — [`DebugQualificationRow`], [`DebugClaimPublication`], and
[`DebugDowngradeRule`] — so notebook, profiler/replay, incident/support, and core-runtime
debug rows all read one qualification result, one published maturity, and one narrowing
reason instead of carrying duplicated "stable debugging" prose that outlives its proof.

It does **not** re-express debugger truth. The
[`m5_debug_contracts`](./m5_debug_contracts.md) matrix names the ten governed debugger object
families and one shared vocabulary; the
[session-descriptor](./m5_debug_session_descriptors.md),
[breakpoint-spec](./m5_breakpoint_specs.md),
[frame/variable](./m5_frame_variable_snapshots.md),
[evaluate/REPL](./m5_evaluate_repl_sheets.md),
[chronology/replay/notebook-parity](./m5_chronology_replay_parity.md), and
[dump/mapping/restore](./m5_dump_mapping_restore.md) lanes materialize those families as typed
truth packets. This lane binds each claimed row to (a) the object families it claims and (b)
the proof packets that keep it current, then computes one status and derives the maturity the
product is allowed to publish.

Authoritative product anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.1–§7.6.10.4 and §9.46 on debug
  launch/session, breakpoints, variables/watches, evaluate side-effect governance, chronology
  capture, replay, and notebook-debug parity.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §14.5 on debug session headers, frame mapping,
  variables/watch panes, evaluate/REPL review, and dump/source-map truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` `FR-DEBUG-001` and the debug surface
  rules on stable breakpoints, variables, stack views, chronology cues, and artifact-linked
  evidence.

## The qualification status

Every row computes exactly one [`DebugQualificationStatus`] from its disclosed evidence by
[`DebugQualificationRow::derive_status`], in this precedence (widest narrowing last):

| Status | When | Effect on the published claim |
|---|---|---|
| `policy_blocked` | the claim is blocked by an explicit policy rule | withdrawn |
| `failing` | no current evidence (`missing`) | withdrawn |
| `stale` | evidence aged past the freshness SLO | retest-pending |
| `partial` | evidence is incomplete for the claimed scope | preview |
| `retest_pending` | evidence is aging toward the SLO | retest-pending |
| `certified` | current, complete evidence within the SLO | tempered by disclosed truth (below) |

Freshness is a stored, reviewable input (`fresh`, `aging`, `stale`, `missing`) rather than a
wall-clock read, so the canonical binding stays deterministic and the freeze gate freezes the
freshness each row claims and the status it produces.

## The published maturity

[`DebugQualificationRow::derive_published_maturity`] turns the status into the
[`DebugClaimMaturity`] the product is *allowed* to publish. A degraded status narrows
outright; a `certified` status is tempered by the disclosed support/mapping/parity/replay
truth **within the row's own claimed scope** so an honest-but-limited row never publishes
stable:

- `support_class` below `supported` floors to `preview` (or `inspect_only` when
  `unavailable`).
- A mapping fidelity that is not exact, exact-build floors to `preview`.
- For a notebook row, `divergent` parity floors to `preview` and `unsupported` to
  `inspect_only`.
- For a replay-claiming row, replay support below `supported` floors to `preview` (or
  `inspect_only` when `unavailable`).

Maturities are ranked widest-first by [`DebugClaimMaturity::rank`] — `stable` (0), `preview`
(1), `inspect_only` (2), `retest_pending` (3), `withdrawn` (4) — so an aggregating surface can
republish the *narrowest* maturity across the rows it covers. **Stable is earned, not
asserted:** a row publishes `stable` only when its status is `certified` *and* its disclosed
truth is a supported backend with an exact, exact-build mapping; [`DebugQualificationSet::validate`]
rejects any other stable claim.

When the published maturity is strictly narrower than the historically claimed maturity, the
row is `narrowed` and carries a one-sentence `narrowing_reason`; an un-narrowed row carries
none.

## Claim publications

Each [`DebugClaimPublication`] republishes the **floor** (narrowest maturity) of the rows it
speaks for to one of four channels — the spec's claim publication board, About / help /
service-health, support exports, and release packets. A publication never claims wider than
its narrowest row, always shows each row's qualification status and the evidence refs behind
it, and narrows automatically when any covered row narrows:

| Channel | Rows | Claimed | Published | Narrowed |
|---|---|---|---|---|
| `claim_board` | 3 | stable | retest_pending | yes |
| `about_help_service_health` | 4 | preview | retest_pending | yes |
| `support_export` | 3 | preview | withdrawn | yes |
| `release_packet` | 2 | stable | stable | no |

The release packet covers only the ship-required core rows, both of which hold stable, so it
stays stable — proof the lane does not over-narrow an honest claim. The claim board covers the
core debugger rows and narrows to retest-pending because the variable/evaluate evidence is
aging.

## Downgrade rules

Each [`DebugDowngradeRule`] names one [`DowngradeTrigger`] and the maturity it floors to. The
set guarantees that every row exhibiting a trigger is listed by the active rule and is
narrowed at least that far, so a reviewer reads *why* a claim narrowed without re-deriving it:

| Trigger | Floors to | Rows |
|---|---|---|
| `evidence_aging` | retest_pending | 1 |
| `evidence_stale` | retest_pending | 1 |
| `evidence_partial` | preview | 2 |
| `evidence_missing` | withdrawn | 1 |
| `support_class_degraded` | preview | 7 |
| `mapping_fidelity_degraded` | preview | 4 |
| `notebook_parity_lost` | preview | 2 |
| `replay_evidence_lost` | preview | 4 |
| `policy_blocked` | withdrawn | 1 |

## Freezing and consumption

[`m5_debug_qualification_set`] is the canonical binding: it builds the set deterministically
and computes each [`DebugQualificationInvariant`]'s `holds` flag from the built records, so an
inconsistent edit flips an invariant and fails CI. The published set is
[`fixtures/debug/m5_debug_qualification/canonical_set.json`](../../fixtures/debug/m5_debug_qualification/canonical_set.json),
frozen against `crates/aureline-debug/src/m5_debug_qualification/mod.rs` by the gate at
`crates/aureline-debug/tests/m5_debug_qualification.rs`. The gate also asserts every cited
evidence packet and producing module exists on disk, so a debugger claim cannot harden without
current proof.

The record carries no source bodies, value bodies, raw paths, provider payloads, URLs,
hostnames, or credentials — only opaque object refs, stable tokens, and short reviewable
sentences — so it is safe for support export. The cross-tool boundary schema is at
[`schemas/debug/m5_debug_qualification.schema.json`](../../schemas/debug/m5_debug_qualification.schema.json),
and the human-readable evidence companion is at
[`artifacts/debug/m5_debug_qualification.md`](../../artifacts/debug/m5_debug_qualification.md).
