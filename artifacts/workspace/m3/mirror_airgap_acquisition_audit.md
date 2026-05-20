# Mirror / air-gap acquisition audit

This artifact is the focused release evidence for the mirror, proxy, and
air-gapped acquisition rows of the M3 repository-acquisition beta lane. It
proves that provenance, freshness, and signer continuity remain explicit —
and that a mirror / proxy / offline source never masquerades as a live
upstream fetch — even when public upstream endpoints are absent.

The audited rows are drills in
[`fixtures/workspace/m3/bootstrap_truth_corpus/`](../../../fixtures/workspace/m3/bootstrap_truth_corpus/),
replayed by
`cargo test -p aureline-qe --test bootstrap_truth_conformance`. The
`mirror_not_masquerading_as_live` guardrail is asserted on every positive
drill; the rows below additionally pin the freshness and signer honesty
labels the surface must render verbatim.

## Audited rows

| Drill | Transport | Declared freshness | Signer continuity | Upstream delta | Honesty labels | Masquerade guardrail |
| --- | --- | --- | --- | --- | --- | --- |
| `clone.mirror_lagged_within_skew` | `mirror` | `mirror_lagged` | `continuous_with_previous_acquisition` | `delta_within_declared_skew` | `mirror_lagged` | holds |
| `mirror.proxy_stale_offline` | `proxy` | `mirror_stale` | `new_signer_first_seen` | `delta_unmeasured` (upstream unreachable) | `mirror_stale`, `signer_first_seen` | holds |
| `interrupted.mirror_clone_resume` | `mirror` | `mirror_lagged` | `continuous_with_previous_acquisition` | `delta_outside_declared_skew` | `mirror_lagged`, `upstream_delta_outside_skew`, `read_only_partial`, `partial_clone` | holds |
| `support.signer_review_export` | `mirror` | `mirror_stale` | `signer_changed_review_required` | `delta_outside_declared_skew` | `mirror_stale`, `upstream_delta_outside_skew`, `signer_changed_review_required` | holds |
| `import.airgap_signed_bundle` | `air_gapped_media` | `signed_offline_bundle` | `signer_rotation_preauthorized` | n/a (offline bundle) | `signed_offline_bundle`, `read_only_partial` | holds |
| `import.handoff_first_signer` | `file_upload` | `offline_snapshot` | `new_signer_first_seen` | n/a (offline handoff) | `offline_snapshot`, `signer_first_seen`, `read_only_partial` | holds |
| `open_archive.snapshot_read_only` | `file_upload` | `offline_snapshot` | `unsigned` | n/a (offline archive) | `offline_snapshot`, `read_only_partial` | holds |

## What the audit proves

1. **No live masquerade.** Every mirror / proxy row declares a non-live
   freshness class and carries mirror-freshness evidence. The projection's
   `mirror_not_masquerading_as_live` guardrail fails any mirror / proxy
   source that declares `live_origin` or omits mirror-freshness evidence,
   and fails any lagged / stale source whose mirror evidence is silently
   rewritten to `live_origin` / `mirror_fresh`.

2. **Freshness stays explicit offline.** The stale proxy row
   (`mirror.proxy_stale_offline`) keeps `mirror_stale` and an
   `delta_unmeasured` upstream delta when the public upstream endpoint is
   unreachable — the source never silently upgrades its freshness to hide
   that it is offline.

3. **Signer continuity stays explicit.** First-seen signers
   (`signer_first_seen`), preauthorized rotations
   (`signer_rotation_preauthorized`, no honesty label, no review block),
   and review-required signer changes (`signer_changed_review_required`)
   are each surfaced distinctly. The `no_hidden_trust_elevation` guardrail
   fails a review-required signer change that lands on an admitted stage
   without a `review_signer_change` hook.

4. **Air-gap parity.** The signed offline bundle and offline handoff /
   archive rows acquire as `local_no_fetch` bytes and stay read-only
   partial roots; they keep their signed-offline-bundle / offline-snapshot
   freshness labels and never present as a network fetch.

5. **Export-safe lineage.** The support row (`support.signer_review_export`)
   reconstructs source identity, mirror freshness, upstream delta, and the
   signer-review ticket through opaque refs and typed labels only — the
   evidence packet stays `export_safe` with no raw paths, credentials, or
   bytes.

## Replay

```
cargo test -p aureline-qe --test bootstrap_truth_conformance
```

A regression in any mirror / air-gap / offline read path appears as a
failure on the corresponding drill — including the
`corpus_covers_mirror_airgap_and_offline_rows` transverse invariant — rather
than silently passing through.
