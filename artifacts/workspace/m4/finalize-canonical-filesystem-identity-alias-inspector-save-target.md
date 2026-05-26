# Finalize Canonical Filesystem Identity, Alias Inspector, Save-Target Review, And Wrong-Target Prevention — proof packet

Reviewer-facing proof packet for the finalized canonical-identity lane:
canonical filesystem identity, the alias inspector, save-target review, and
wrong-target prevention composed into one governed, export-safe record per
posture. This packet is the stable-line anchor for this lane: dashboards,
docs, Help/About surfaces, and support exports should ingest the typed
sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/canonical_identity_lineage/`](../../../crates/aureline-workspace/src/canonical_identity_lineage/)
- Filesystem identity (presentation / logical / canonical / aliases):
  [`/crates/aureline-vfs/src/identity/`](../../../crates/aureline-vfs/src/identity/)
- Save-target token (layer 5) and conflict-aware save stub:
  [`/crates/aureline-vfs/src/save.rs`](../../../crates/aureline-vfs/src/save.rs)
- Alias inspector projection:
  [`/crates/aureline-vfs/src/identity/alias_inspection.rs`](../../../crates/aureline-vfs/src/identity/alias_inspection.rs)
- Save-target review projection:
  [`/crates/aureline-vfs/src/identity/save_target_review.rs`](../../../crates/aureline-vfs/src/identity/save_target_review.rs)
- External-change compare (wrong-target prevention runtime):
  [`/crates/aureline-vfs/src/identity/external_change_compare.rs`](../../../crates/aureline-vfs/src/identity/external_change_compare.rs)
- Schema:
  [`/schemas/workspace/canonical_identity_lineage.schema.json`](../../../schemas/workspace/canonical_identity_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_canonical_identity_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_canonical_identity_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/canonical_identity_lineage/`](../../../fixtures/workspace/m4/canonical_identity_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/canonical_identity_lineage_replay.rs`](../../../crates/aureline-workspace/tests/canonical_identity_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/finalize-canonical-filesystem-identity-alias-inspector-save-target.md`](../../../docs/workspace/m4/finalize-canonical-filesystem-identity-alias-inspector-save-target.md)
- Typed consumer: `aureline_workspace::project_from_save_target_token`

## What this packet proves

1. **Canonical-path truth.** The record carries the resolved canonical URI,
   the workspace-relative logical URI, and the presentation URI verbatim. The
   `path_truth_class` mirrors the VFS path-truth chip (`direct`,
   `direct_with_known_aliases`, `via_*` for known alias kinds, or
   `divergent_unknown` for divergent opens with no alias entry).
   `canonical_target_resolved` is true only when the canonical URI is
   well-formed; an unresolved target narrows below Stable with
   `canonical_target_unresolved`. Worked examples:
   [`direct_writable_stable.json`](../../../fixtures/workspace/m4/canonical_identity_lineage/direct_writable_stable.json),
   [`symlink_alias_stable.json`](../../../fixtures/workspace/m4/canonical_identity_lineage/symlink_alias_stable.json).

2. **Alias-inspector honesty.** Every alias entry carries its kind and
   step-by-step resolution chain; the record names the canonical and
   presentation aliases explicitly. A divergent open with no alias entry
   flips `presentation_alias_missing`, which surfaces the degraded state
   instead of guessing. Worked example:
   [`divergent_unknown_alias_narrowed.json`](../../../fixtures/workspace/m4/canonical_identity_lineage/divergent_unknown_alias_narrowed.json).

3. **Save-target review surfaces every blocker.** The record names the
   canonical write URI (`writes_to_canonical_uri`), the atomic-write mode,
   the pinned generation token, the permission snapshot, and every blocker
   the live VFS save-target review collects (read-only, policy-constrained,
   review-required-before-save, review-required-before-rename,
   not-writable-per-snapshot, atomic-write-mode-blocked,
   divergent-unknown-alias, untrusted-workspace). Read-only and
   policy-constrained roots are protective postures — they remain Stable
   when the contract is structurally intact. Worked example:
   [`read_only_blocked_stable.json`](../../../fixtures/workspace/m4/canonical_identity_lineage/read_only_blocked_stable.json).

4. **Wrong-target writes are structurally prevented.** The record proves
   `compare_before_write_pinned` (the generation token captured at open is
   present), `divergent_unknown_alias_guarded` (a divergent unknown open
   adds the matching blocker), and `untrusted_workspace_guarded` (a
   non-trusted workspace adds the matching blocker). Missing one of these
   narrows with `compare_before_write_not_pinned`,
   `divergent_unknown_alias_unguarded`, or
   `untrusted_workspace_save_unguarded`. Worked example:
   [`compare_token_unpinned_narrowed.json`](../../../fixtures/workspace/m4/canonical_identity_lineage/compare_token_unpinned_narrowed.json).

5. **Inspection precedes destructive cleanup.** A destructive action (the
   next save) is always reachable, so the record requires the
   `compare_before_write` inspection hook to be available before it. The
   full hook set is `alias_inspect`, `save_target_review`,
   `compare_before_write`, `export`, and `repair`. A missing
   compare-before-write hook narrows with
   `destructive_action_no_compare_hook`. Worked example:
   [`missing_compare_hook_narrowed.json`](../../../fixtures/workspace/m4/canonical_identity_lineage/missing_compare_hook_narrowed.json).

6. **Lineage and export stay honest.** Every record sets
   `raw_payload_excluded = true`; the shared
   `FilesystemIdentityReferenceSet` is consistent across editor / Git /
   restore / mutation flows (`all_flows_share_identity = true`). An empty
   workspace or root ref narrows with `lineage_export_unsafe`; a
   disagreeing identity reference set narrows with
   `identity_reference_inconsistent`.

7. **The record is replay-gated.** The replay gate re-projects each
   fixture and asserts it equals the checked-in `expected`, so the
   projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                  | Posture                                | Qualification           | Proves                                  |
| ---------------------------------------- | -------------------------------------- | ----------------------- | --------------------------------------- |
| `direct_writable_stable`                 | direct trusted writable                | `stable`                | All pillars proven (direct path)        |
| `symlink_alias_stable`                   | symlink alias to canonical             | `stable`                | Alias kind explained, redirect honest   |
| `read_only_blocked_stable`               | read-only root, save blocked           | `stable`                | Protective posture, contract intact     |
| `divergent_unknown_alias_narrowed`       | presentation differs without alias     | `narrowed_below_stable` | Degraded alias state disclosed          |
| `compare_token_unpinned_narrowed`        | compare-before-write token missing     | `narrowed_below_stable` | Wrong-target prevention not pinned      |
| `missing_compare_hook_narrowed`          | compare inspection hook unavailable    | `narrowed_below_stable` | Destructive action with no compare hook |

## How to verify

```sh
# Unit + replay gate for the canonical filesystem identity lineage projection.
cargo test -p aureline-workspace --lib canonical_identity_lineage
cargo test -p aureline-workspace --test canonical_identity_lineage_replay

# Truth sources (identity, save-target token, alias inspector, save-target review).
cargo test -p aureline-vfs --lib identity
cargo test -p aureline-vfs --lib save

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_canonical_identity_lineage -- --lines \
  fixtures/workspace/m4/canonical_identity_lineage/symlink_alias_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and replay
gate above. The lineage record self-describes its stable qualification:
surfaces that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named reason,
so they never inherit an adjacent green row. No public scope is widened
from this row.
