# Alpha Incident Advisory Baseline

This packet records the first alpha advisory rehearsal. It is exercise-only:
no live customer advisory is published, no CVE or GHSA alias is assigned, and
no emergency action is invoked. The purpose is to prove the field-response
path before wider alpha use by rehearsing incident declaration, advisory copy,
affected-build scoping, rollback guidance, and known-limit honesty.

## Canonical Inputs

| Input | Path |
|---|---|
| Advisory template seed | [`docs/security/advisory_template_seed.md`](../../docs/security/advisory_template_seed.md) |
| Affected-build scope example | [`artifacts/release/affected_build_scope_example.yaml`](./affected_build_scope_example.yaml) |
| Exact-build identity | [`artifacts/build/build_identity.json`](../build/build_identity.json) |
| Crash incident trail proof | [`artifacts/support/crash_symbolication_linkage_alpha.md`](../support/crash_symbolication_linkage_alpha.md) |
| Protected fitness packet | [`artifacts/release/protected_fitness_packet_alpha.yaml`](./protected_fitness_packet_alpha.yaml) |
| Release/support crosswalk | [`artifacts/release/release_support_crosswalk.yaml`](./release_support_crosswalk.yaml) |
| Update rollback sequence | [`artifacts/release/update_rollback_sequence.yaml`](./update_rollback_sequence.yaml) |

## Advisory Draft

| Field | Value |
|---|---|
| Advisory ID | `AURELINE-ADV-2026-0201` |
| Severity | `security_severity.high` |
| Status | `exercise_only` |
| Disclosure | `public_on_advisory` rehearsal; no public publication in this packet |
| Affected surface | Preview renderer crash evidence and current local development build publication review |
| Action state | `action_required` for review; no automatic disable or destructive repair |

Summary:

An alpha preview renderer crash and the current local development build are
used to rehearse advisory publication. The affected builds are named by
exact-build identity, current mitigations are scoped to local support review,
rollback routes preserve user-authored state and evidence, and known limits
are kept visible instead of implied away.

## Affected Builds

| Build scope | Exact-build identity | Current mitigation | Rollback route | Known-limit truth |
|---|---|---|---|---|
| `affected_build.preview_renderer_panic` | `build-id:aureline:preview:0.8.0-alpha.1:x86_64-unknown-linux-gnu:release:9f0e7d6c5b4a` | Use the exact-build incident trail and metadata-only support review; raw crash dump bytes and raw stack bodies remain excluded. | `rollback.target.preview.previous_verified_build` through the update rollback review, with `checkpoint.update.pre_restart_checkpoint_created`. | Symbolication linkage is proven on the fixture; diagnosis-latency measurement is still pending. |
| `affected_build.local_dev_current` | `build-id:aureline:dev:0.0.0:unknown:dev:52917c7e2fa0` | Keep the current local development build inside release/support review; do not claim passing protected fitness while the packet reports `evidence_stale`. | `rollback.target.hold_alpha_publication_until_packet_current` or explicit rollback review, with `checkpoint.update.review_emitted`. | Protected fitness is `evidence_stale`; warm-start rows and supportability measurement are incomplete. |

## Current Mitigations

- The preview renderer crash remains in local support review and links to
  exact-build symbolication evidence.
- The current development build remains bounded by the protected fitness
  packet. The packet narrows release confidence to `evidence_stale`.
- Support packets use metadata-safe defaults and carry advisory ID,
  exact-build refs, mitigation state, rollback refs, known-limit refs, and
  support packet refs.
- Raw dumps, raw logs, raw paths, raw command lines, reporter identity,
  private registry URLs, exploit material, and secrets are excluded.

## Rollback Route

Rollback guidance uses the existing update rollback sequence rather than a
support reset shortcut.

- Preview renderer crash: revert to the prior verified preview build through
  the update rollback review. Preserve authored files, support evidence,
  advisory history, crash evidence refs, and local recovery state.
- Current local development build: hold wider alpha publication or route
  through rollback review. Do not describe the build as healthy until the
  protected packet is refreshed or the public claim is narrowed.

## Known Limits

- The protected fitness packet currently reports `evidence_stale`.
- Diagnosis-latency supportability coverage is seeded but not yet measured.
- The advisory template and affected-build scope are checked in; live
  advisory publishing, external vulnerability database automation, hosted
  incident portals, and raw crash upload flows are outside this baseline.
- The baseline proves exact-build, mitigation, rollback, and known-limit
  wording. It does not claim broad health or compatibility beyond the cited
  evidence.

## Support/Export Projection

The first typed consumer is
[`crates/aureline-support/src/advisory_baseline/mod.rs`](../../crates/aureline-support/src/advisory_baseline/mod.rs).
It consumes the affected-build scope YAML and projects a metadata-only support
packet with:

- advisory ID;
- exact-build identity refs;
- current mitigation state;
- rollback target refs;
- known-limit refs;
- linked support packet refs; and
- a raw-private-material exclusion marker.

## Verification

```sh
cargo test -p aureline-support --test incident_advisory_baseline_alpha
```

The protected tests parse the checked-in scope artifact, confirm the template
and baseline docs are present, verify affected builds are covered by the
advisory linkage, and assert that the support/export projection stays
metadata-only and honest about known limits.
