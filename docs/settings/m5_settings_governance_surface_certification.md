# M5 settings-governance surface certification (M05-1203)

This contract is the **closing B143 surface-certification capstone** over the frozen M5 settings-governance
matrix (`m5_settings_governance_matrix`). Where the freeze matrix defines the five governed
configuration-runtime families — **resolve-setting, write-setting, sync-scope, migrate-schema, and
rollout-capability** — the 1197–1201 implementation lanes resolve their per-surface setting-definition,
effective-resolution, write-intent, policy-constraint, sync-conflict, schema-migration, and
capability-lifecycle truth, and the 1202 shared-consumer lane aligns their grammar across the settings-resolver,
shell, sync-service, policy-service, capability-service, diagnostics, docs / help, CLI / export, and
support-export consumers and proves keyboard / screen-reader / high-zoom / high-contrast / localization /
CLI-export parity, this capstone **certifies** that the shared settings-governance truth holds on every claimed
M5 **configuration-bearing profile** and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_settings_governance_surface_certification/`
- **Schema:** `schemas/config/m5-settings-governance-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-settings-governance-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/config/m5-settings-governance-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a user, reviewer, admin, or support engineer reads a
setting-definition, effective-resolution, write-intent, policy-constraint, sync-conflict, schema-migration, or
capability-lifecycle surface through, not on the reusable configuration-runtime family it renders:

1. **Live trusted settings surface** — a live, first-party, fully-current resolve-setting surface. The **only**
   profile that may certify a `trusted_settings_surface` claim.
2. **Reviewable settings structure** — a self-sufficient, inspectable setting-definition / effective-value /
   schema-migration record; certifies at most `reviewable_settings_surface`.
3. **Disclosed write-intent profile** — a write-setting surface whose preview / checkpoint / rollback evidence
   can only be partially disclosed; auto-narrows to `write_intent_disclosed_projection`.
4. **Unverified sync-conflict profile** — a sync-scope surface whose field-level conflict resolution cannot be
   confirmed; auto-narrows to `sync_conflict_unverified_projection`.
5. **Unverified capability-lifecycle profile** — a rollout-capability surface whose dependency marker or
   kill-switch cause has aged out or is policy-blocked; auto-narrows to
   `capability_lifecycle_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and settings-governance-component-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed configuration tier delivered;
- **yellow** — a truth axis is not current, so the configuration claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh trusted claim, a hard invariant breaks, CLI/export
  parity drops, a non-live profile claims a trusted settings surface, or the narrowing is inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `trusted_settings_surface` / `reviewable_settings_surface` claim while one of its truth axes is not current
   over-claims and blocks.
2. **Only a live first-party profile may certify a trusted settings surface.** Every other profile is at most a
   reviewable settings structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the canonical
   setting definition, effective value, write intent, policy constraint, sync conflict packet, schema migration,
   capability record, and registry reference as text / JSON / Markdown.
4. **Every B143 hard invariant holds per row.** No profile may recycle a retired setting ID, rewrite a scoped
   (Workspace/Profile) write into a broader (User/Machine) scope, silently overwrite locked or machine-only
   state during sync, hide a lifecycle or experiment dependency behind unpublished markers, or hide a
   kill-switch or policy-disable cause behind generic unavailable copy.
5. **One canonical proof bundle.** Every row cites exactly one canonical settings-governance proof bundle
   (`artifacts/release/m5-settings-governance-proof/support_export.json`) — the frozen settings-governance
   matrix proof — so release, docs, and support consume a single configuration-runtime certification source
   rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_SETTINGS_GOVERNANCE_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_settings_governance_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
