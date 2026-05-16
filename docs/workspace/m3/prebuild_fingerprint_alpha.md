# Prebuild fingerprint alpha

The alpha prebuild fingerprint, reuse-decision, and disclosure records are
the durable, reviewable records that Start Center, CLI / headless entry,
docs, and support packets read **before any prebuild is reused, rebuilt,
or labelled as a live resume**. A prebuild is an accelerator, not an
authority: reuse requires matching source, environment capsule,
toolchain, host, policy, feature, trust, credential-handle, port, index,
cache-artifact, freshness, and redaction dimensions. A stale snapshot or
invalidated prebuild cannot be represented as a successful live resume.

This alpha document is intentionally narrower than the broader
[`/docs/workspace/prebuild_fingerprint_contract.md`](../prebuild_fingerprint_contract.md):

- the **contract** owns the full fingerprint vocabulary, the reuse rules,
  and the path-separation matrix;
- the **alpha** record family freezes the Rust-projectable record shape,
  the closed-vocabulary cross-rules, the fixture set the validators
  consume, and the first Start Center consumer surface that proves the
  lane is inspectable end-to-end.

The companion schemas live at:

- [`/schemas/workspace/prebuild_fingerprint.schema.json`](../../../schemas/workspace/prebuild_fingerprint.schema.json)
- [`/schemas/workspace/prebuild_invalidation_reason.schema.json`](../../../schemas/workspace/prebuild_invalidation_reason.schema.json)

The canonical alpha fixtures live under:

- [`/fixtures/workspace/m3/prebuild_fingerprint/`](../../../fixtures/workspace/m3/prebuild_fingerprint/)

The headless validator that gates every alpha fixture lives at:

- [`/ci/check_prebuild_fingerprint_alpha.py`](../../../ci/check_prebuild_fingerprint_alpha.py)

The Rust types are exported from `aureline_workspace::prebuilds` at
[`/crates/aureline-workspace/src/prebuilds/mod.rs`](../../../crates/aureline-workspace/src/prebuilds/mod.rs).
The integration test
[`/crates/aureline-workspace/tests/prebuild_fingerprint_alpha.rs`](../../../crates/aureline-workspace/tests/prebuild_fingerprint_alpha.rs)
replays every fixture and proves the closed acceptance states. The first
shell consumer is
[`/crates/aureline-shell/src/start_center/prebuild_fingerprints/mod.rs`](../../../crates/aureline-shell/src/start_center/prebuild_fingerprints/mod.rs),
which renders deterministic prebuild rows directly from the checked-in
alpha fixtures.

## 1 Why freeze this now

Warm starts have to stop being convenient lies. Recent-work rows, prebuild
pickers, and Start Center continue boxes all promise speed, but they have
no shared vocabulary for the questions that decide whether a warm start is
honest:

- which source identity, environment capsule, toolchain, policy, trust,
  credential-handle, and port posture did this prebuild capture;
- how old is the prebuild, who produced it, and is it still signed;
- which invalidation reasons (dependency drift, policy drift, trust drift,
  secret-handle change, missing artifact, stale index) currently apply;
- whether the user is being shown a live resume, a snapshot start, a
  fresh clone, or a cached prebuild — and whether those choices remain
  distinct.

The alpha freezes those answers as three durable record kinds:
`prebuild_fingerprint_record`, `prebuild_reuse_decision_record`, and
`prebuild_disclosure_record`. Each one is JSON-shaped, schema-validated,
Rust-projectable, and consumed by the first Start Center surface.

## 2 Record shape

Every alpha payload is one of three record kinds, discriminated on
`record_kind`. All three share the same schema version.

| Record kind | Required content |
| --- | --- |
| `prebuild_fingerprint_record` | Source, environment, toolchain, policy / feature, secret-handle, port / route identity blocks; cache artifacts; freshness with producer and signer posture; closed redaction posture. |
| `prebuild_reuse_decision_record` | Requested entry path, source materialization class, candidate / current fingerprint refs, reuse outcome, invalidation bundle refs, required revalidations, disclosure ref, support-packet inclusion posture. |
| `prebuild_disclosure_record` | Disclosure state, requested path, source materialization class, freshness age class, host / platform, required revalidations, rebuild / fresh-clone flags, local-override disclosure, the resume-live invariant, alternative lane refs, excluded residue classes, and a reviewable summary. |

## 3 Frozen rules

The validator and the integration test both enforce:

1. **Stale snapshots cannot masquerade as live resume.** Every disclosure
   record asserts
   `stale_snapshot_must_not_be_labeled_live_resume = true`. A decision
   that requests `resume_live_workspace` against a `prebuilt_snapshot` or
   `stale_prebuild_snapshot` materialization MUST resolve to
   `resume_live_denied`.
2. **`reuse_allowed` is unconditional.** A decision with
   `reuse_outcome = reuse_allowed` MUST list no invalidation bundle refs
   and no required revalidations. If anything must change before reuse,
   the outcome is not `reuse_allowed`.
3. **Closed export.** Every fingerprint and disclosure excludes raw
   secrets, raw credential bodies, raw environment values, machine-unique
   trust anchors, and uncommitted workspace edits. The fingerprint also
   keeps `broadened_capture_approved = false`.
4. **Distinct paths.** The closed
   `resume_live_workspace` / `start_from_snapshot` / `clone_fresh` /
   `reuse_cached_prebuild` vocabulary keeps the four user choices visible
   and separate on every disclosure surface. Fresh-clone disclosures set
   `fresh_clone_required = true`. Local-override and stale-prebuild
   rebuild disclosures set `rebuild_required = true`.
5. **Pre-execution review.** A disclosure record is inspectable before
   any networked setup or rebuild runs. Routing through it never silently
   widens authority, masks a degraded warm start, or hides a missing
   credential handle.

## 4 Reusable consumers

The alpha record family is the single record set every disclosure
consumer reads:

- **Start Center** projects the alpha records into compact rows that
  surface fingerprint id, freshness, host class, requested path, reuse
  outcome or disclosure state, required revalidations, and the local
  override or fresh-clone flag before the user commits.
- **CLI / headless entry** prints the projection as deterministic
  plaintext so scripted and remote workflows see the same disclosure
  surface.
- **Docs / workspace help** lifts the disclosure summary and alternative
  lane refs verbatim, keeping the surface in lockstep with the product.
- **Support exports** quote fingerprint ids, decision ids, invalidation
  bundle refs, and disclosure ids — never raw secrets or raw host
  residue.

## 5 Boundary placement

The alpha record family stays narrow on purpose. It owns:

- the **review record vocabulary** every prebuild surface reads before
  use;
- the closed cross-rules that keep stale snapshots out of the live-resume
  lane and keep credentials, ports, indexes, and policy explicit when
  they need revalidation;
- the seeded fixtures and the first consumer that prove the lane is
  inspectable end-to-end without a managed control plane.

It does not own:

- the runtime materializer or the cache manager that produces and prunes
  prebuilds;
- the cloud production / OCI distribution pipeline;
- managed-fleet scheduling or remote prebuild services.

Those records exist (or will exist) elsewhere. The alpha bundle is the
disclosure truth that must remain honest **before** any of them are
invoked.
