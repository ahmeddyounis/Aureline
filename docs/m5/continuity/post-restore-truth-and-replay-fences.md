# Post-restore truth, replay fences, and compare/export parity

This contract gives a restored managed or support artifact a truthful
post-restore experience instead of implying full normal operation the moment a
backup or failover restore completes. Every claimed managed, self-hosted, or
sovereign row whose continuity depends on restored state must point to one typed
**restore-from-backup review** that a person — in service-health, support, on a
managed action sheet, or in a partner/public claim — can read directly.

For each restored artifact it produces one **descriptor** that answers the same
questions everywhere:

1. Which artifact family was restored — managed records, policy bundles, sync
   metadata, or support/export records — and was this a *continuity* restore or
   an ordinary workspace/session restore?
2. Did recovery reproduce the artifact **exactly**, or **narrower than normal**,
   and — when narrower — which capability class or data slice is affected, and is
   all replicated data present?
3. Which privileged or externally mutating action lanes depend on the restored
   state, and is each one fenced so it cannot silently auto-replay before an
   explicit reviewed step?
4. Can an operator compare restored-vs-current state and export that comparison
   before assuming full continuity?

The review is produced by
`aureline_continuity::m5_restore_from_backup_reviews`. It reuses the continuity
profile, restore-identity, and qualification vocabulary from the frozen
continuity-claim matrix
(`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`) so there is
exactly one continuity vocabulary across the product, and it complements the
backup/restore/failover packet lane: where that lane proves a restore *can* run,
this lane proves what a restore *actually reproduced* and gates replay afterward.
The descriptor is projected identically onto the service-health, support-center,
managed-action-sheet, release-center, and public claim-manifest surfaces.

## What every surface answers the same way

- Which artifact family was restored, and was it a continuity or ordinary
  restore?
- Was the restore exact or narrower than normal, and which slice is affected?
- Do privileged and externally mutating lanes auto-replay, or are they fenced
  behind an explicit reviewed step?
- Can an operator compare and export restored-vs-current state?

## Stable conditions

A page qualifies `stable` only when all of the following hold at once:

1. Every managed-continuity review discloses whether the restore was exact or
   narrower than normal, and a narrower-than-normal restore names the affected
   capability class or data slice.
2. Every managed-continuity review declares the restore identity recovery
   reproduces.
3. No privileged or externally mutating lane auto-replays: every such lane is
   held for review or cleared only after an explicit reviewed step that it names.
4. Every managed-continuity review lets operators compare *and* export
   restored-vs-current state, and at least one managed artifact family and one
   support/export artifact family carry that parity.
5. Every claimed restored row points to a current clean review.
6. Every review is projected onto all five surfaces, and the restore-identity,
   replay-fence, and compare/export vocabulary is identical across every
   projection.

## Fail-closed guardrails

Three guardrails are load-bearing and withdraw the claim immediately:

- **No green-status overclaim.** A review may not hide narrowed capability or
  missing replicated data behind full, normal status language. A review that
  asserts full normal status while the restore is narrower than normal — or while
  replicated data is incomplete — is withdrawn.
- **No restore-lane conflation.** A managed, self-hosted, or sovereign row may
  not present an ordinary workspace/session restore as a continuity restore (or
  carry a managed-continuity artifact family under an ordinary-restore label).
- **No privileged auto-replay.** A privileged or externally mutating lane may not
  auto-replay after a restore; an unfenced privileged/external lane is withdrawn.

## Automatic claim narrowing

The `RestoreReviewRegistry` is the typed consumer the service-health,
support-center, managed-action-sheet, release-center, and public claim-manifest
surfaces read. It reports, per claimed restored row, whether a current clean
review backs the claim, and whether at least one managed and one support/export
artifact family carries compare/export parity. A row narrows automatically when
its review is missing, narrowed, or withheld:

| Condition | Coverage | Qualification |
|---|---|---|
| A current clean review backs the claim | `current_review` | `stable` |
| The review exists but narrowed (disclosure or parity gap) | `narrowed_review_needs_attention` | `beta` / `preview` |
| The review overclaims status, conflates lanes, or auto-replays | `review_withheld` | `withdrawn` |
| No review backs the claimed restored row | `no_review` | `preview` |

## Export safety

The review is metadata-only. Restore-identity, replay-fence, and compare/export
fields are export-safe by default and remain visible in operator and support
surfaces. It carries closed-vocabulary tokens, export-safe plain-language labels,
UTC timestamps, and opaque refs only. Raw restored bytes, raw provider payloads,
raw hostnames, and secret material never cross this boundary.

## Schema, artifact, and fixtures

- Schema: `schemas/continuity/restore_identity_summary.schema.json`
- Artifact summary: `artifacts/m5/continuity/post_restore_truth_and_replay_fences.md`
- Canonical evidence packets: `artifacts/m5/continuity/restore_reviews/`
- Fixtures: `fixtures/continuity/post_restore_narrowing/`
- Validator: `python3 tools/validate_m5_restore_from_backup_review_fixtures.py`
- CLI inspect: `aureline_restore_review_inspect`
