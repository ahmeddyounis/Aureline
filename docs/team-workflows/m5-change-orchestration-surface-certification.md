# M5 change-orchestration surface certification (M05-1303)

This contract is the **closing B154 surface-certification capstone** over the frozen M5 change-object, patch-stack,
and landing matrix (`m5_change_object_patch_stack_and_landing_matrix`). Where the freeze matrix defines the six
governed change-orchestration object classes — **change object, patch-stack / queue, stack-edit / review sheet,
landing-candidate sheet, portable shelf / bundle, and worktree cleanup preview** — the M05-1295–1300
implementation lanes resolve their change-object record / selected-change binding, patch-stack / member landing,
stack-edit review / disposition, landing-candidate sheet / authorization, portable shelf / reopen parity, and
worktree-manager / cleanup-preview registry truth; this capstone **certifies** that the shared change-orchestration
truth holds on every claimed M5 **Git, review, AI, work-item, provider, help, and support / export surface** — the
selected change object, its worktree / base identity, stack membership and order, landing state, validation
freshness, and cleanup evidence — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_change_orchestration_surface_certification/`
- **Schema:** `schemas/teamwork/m5-change-orchestration-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-change-orchestration-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/teamwork/m5-change-orchestration-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **consumer profile** a change-object owner, a stack / landing flow, a provider
merge-queue consumer, or a support / export consumer reads a change orchestration through, not on the underlying
object class it renders:

1. **Fully-certified change-orchestration lane** — a tracked change object whose selected worktree / base identity,
   stack membership and order, landing state, validation freshness, and cleanup evidence all converge on one
   export-safe, provider-authoritative, internally consistent record identical across every consumer. The **only**
   profile that may certify a `certified_change_orchestration_truth` claim.
2. **Reviewable change-object record structure** — a self-sufficient, inspectable change-object-bound landing
   candidate / review record; certifies at most `reviewable_change_orchestration_record`.
3. **Unbound-worktree-binding profile** — a change object whose selected worktree and base-or-dirty-tree identity
   can no longer be confirmed bound; auto-narrows to `worktree_binding_unverified_projection`.
4. **Inferred-stack-membership profile** — a patch-stack member whose stack membership cannot be confirmed
   explicitly declared (it risks being inferred from a branch name alone); auto-narrows to
   `stack_membership_unverified_projection`.
5. **Silently-reordered-stack profile** — a stack-edit review sheet whose proposed order cannot be confirmed
   reviewed; auto-narrows to `stack_order_unverified_projection`.
6. **Ambient-branch-landing profile** — a landing candidate whose queue authority or protected-branch gate cannot
   be proven (merge queue unavailable, ambiguous queue position, stale base, or unverifiable protected-branch
   rule); auto-narrows to `landing_authority_unverified_projection`.
7. **Stale-validation-shelf profile** — a portable shelf whose packaged validation or approval evidence is stale;
   auto-narrows to `validation_freshness_unverified_projection`.
8. **Partial-cleanup-evidence profile** — a worktree cleanup preview whose affected-work and recovery evidence is
   partial; auto-narrows to `cleanup_evidence_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and change-orchestration-truth behavior — and resolves to
a derived verdict:

- **green** — every axis certified, every invariant held, the claimed change-orchestration tier delivered;
- **yellow** — a truth axis is not current, so the change-orchestration claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-lane profile claims a certified change-orchestration record, or the narrowing is
  inconsistent.

The eight seeded rows cover all six frozen object classes (change object and landing-candidate sheet each appear
on a green and a yellow row), so the certification runs across the full matrix rather than a single class.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_change_orchestration_truth` / `reviewable_change_orchestration_record` claim while one of its truth
   axes is not current over-claims and blocks.
2. **Only a fully-certified change-orchestration lane may certify a certified change-orchestration record.** Every
   other profile is at most a reviewable change-object record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the selected
   change object, worktree / base identity, stack membership and order, landing state, validation freshness, and
   cleanup evidence as text / JSON / Markdown.
4. **Every B154 hard invariant holds per row.** No profile may infer stack membership from branch names alone;
   mutate another worktree without an explicit selected change object and worktree binding; silently reorder,
   collapse, or retarget stack members; land from ambient branch state; or delete an orphaned worktree or stale
   member without previewing running tasks, open editors, uncommitted changes, recovery checkpoints, and
   export-safe evidence.
5. **One canonical proof bundle.** Every row cites exactly one canonical change-orchestration matrix proof bundle
   (`artifacts/release/m5-change-orchestration-proof/support_export.json`) — the frozen change-orchestration matrix
   proof — so support, docs / help, release, and public-proof surfaces consume a single change-orchestration
   certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_CHANGE_ORCHESTRATION_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_change_orchestration_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
