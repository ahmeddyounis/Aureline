# M5 change-intent surface certification (M05-1293)

This contract is the **closing B153 surface-certification capstone** over the frozen M5 change-intent and
engineering-lifecycle matrix (`m5_change_intent_and_engineering_lifecycle_matrix`). Where the freeze matrix
defines the six governed change-intent object classes — **change-intent record, start-work sheet, linked-change
panel, ready-for-review handoff sheet, resolve-or-close sheet, and blocked-or-escalate card** — the
M05-1285–1290 implementation lanes resolve their change-intent record / start-work, linked-change panel /
relation, ready-for-review handoff / publish-action, resolve-or-close sheet / resolution-outcome,
blocked-or-escalate card / escalation-outcome, and lifecycle-state / reconcile-flow registry truth; this
capstone **certifies** that the shared change-intent truth holds on every claimed M5 **work-item, start-work,
review, provider, help, and support / export surface** — provider ownership, local-versus-provider commit state,
linked branch / worktree / review identity, relation source, blocker / resolution state, and validation
evidence — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_change_intent_surface_certification/`
- **Schema:** `schemas/teamwork/m5-change-intent-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-change-intent-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/teamwork/m5-change-intent-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **consumer profile** a work-item owner, a start-work / handoff flow, a
provider handoff consumer, or a support / export consumer reads a change intent through, not on the underlying
object class it renders:

1. **Fully-certified change-intent lane** — a tracked change intent whose provider ownership,
   local-versus-provider commit state, linked branch / worktree / review identity, relation source,
   blocker / resolution state, and validation evidence all converge on one export-safe, provider-committed,
   internally consistent record identical across every consumer. The **only** profile that may certify a
   `certified_change_intent_truth` claim.
2. **Reviewable change-intent record structure** — a self-sufficient, inspectable tracked-item-bound record;
   certifies at most `reviewable_change_intent_record`.
3. **Local-only-or-reconcile-required commit-state profile** — a record whose local-versus-provider commit state
   can no longer be confirmed provider-committed (local-only draft, queued publish, or reconcile-required);
   auto-narrows to `commit_state_unverified_projection`.
4. **Undisclosed-start-work-side-effect profile** — a start-work sheet whose branch / worktree / review-draft /
   provider-link side effects cannot be confirmed separately disclosed; auto-narrows to
   `side_effect_disclosure_unverified_projection`.
5. **Flattened-linked-relation-source profile** — a linked-change panel whose relation source (linked-by-provider,
   linked-locally, suggested-by-Aureline, or stale-or-broken) cannot be confirmed distinct; auto-narrows to
   `linked_relation_source_unverified_projection`.
6. **Blocked-handoff-publishability profile** — a ready-for-review handoff whose publishability is blocked
   (offline, missing write scope, policy-blocked, or partially writable); auto-narrows to
   `handoff_publishability_unverified_projection`.
7. **Local-only-resolution-authority profile** — a resolve-or-close sheet whose final-resolution authority is
   local-only or has an unresolved engineering blocker; auto-narrows to
   `resolution_authority_unverified_projection`.
8. **Unresolved-blocker-continuity profile** — a blocked-or-escalate card whose blocker / resolution state is
   unstated or whose retained local evidence is at risk after a failed provider write; auto-narrows to
   `blocker_continuity_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and change-intent-truth behavior — and resolves to a
derived verdict:

- **green** — every axis certified, every invariant held, the claimed change-intent tier delivered;
- **yellow** — a truth axis is not current, so the change-intent claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-lane profile claims a certified change-intent record, or the narrowing is
  inconsistent.

The eight seeded rows cover all six frozen object classes (change-intent record and ready-for-review handoff
sheet each appear on a green and a yellow row), so the certification runs across the full matrix rather than a
single class.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_change_intent_truth` / `reviewable_change_intent_record` claim while one of its truth axes is not
   current over-claims and blocks.
2. **Only a fully-certified change-intent lane may certify a certified change-intent record.** Every other
   profile is at most a reviewable change-intent record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the provider
   ownership, local-versus-provider commit state, linked branch / worktree / review identity, relation source,
   blocker / resolution state, and validation evidence as text / JSON / Markdown.
4. **Every B153 hard invariant holds per row.** No profile may let start work silently create a branch,
   worktree, review draft, or provider link without separately disclosing each side effect; let a local handoff
   packet or queued publish masquerade as a provider-committed update; flatten linked-by-provider, linked-locally,
   suggested-by-Aureline, and stale-or-broken relation into one generic relation badge; auto-resolve tracked work
   while engineering blockers remain unresolved; or drop local notes, handoff packets, or linked evidence when a
   provider write fails.
5. **One canonical proof bundle.** Every row cites exactly one canonical change-intent lifecycle matrix proof
   bundle (`artifacts/release/m5-change-intent-proof/support_export.json`) — the frozen change-intent lifecycle
   matrix proof — so support, docs / help, release, and public-proof surfaces consume a single change-intent
   certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_CHANGE_INTENT_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_change_intent_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
