# Stabilized extension-bisect, suspect-runtime quarantine, and bounded repair orchestration — Artifact

## Status

**Stable** — hardened M4 orchestration profile with extension-bisect binding,
suspect-runtime quarantine binding, bounded repair binding, retained
capabilities, accessibility posture, and recovery-ladder integration.

## Checked-in outputs

| Output | Path |
|--------|------|
| Implementation | `crates/aureline-support/src/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded/mod.rs` |
| Boundary schema | `schemas/support/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded.schema.json` |
| Reviewer doc | `docs/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded.md` |
| Fixture corpus | `fixtures/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded/` |

## What is stabilized

The beta extension-bisect, quarantine, and repair contracts are promoted to a
stable orchestration contract by adding:

1. **Extension-bisect binding** — every profile cites the bisect session, steps,
   finding, restore plan, and support packet with a closed status class.
2. **Suspect-runtime quarantine binding** — every profile cites the quarantine
   record, lane, owner, reason, expiry, clear action, re-enable action, and
   evidence refs with a closed reason class.
3. **Bounded repair binding** — every profile cites the repair transaction,
   preview, outcome, blast radius, compensation, and status with closed
   vocabularies.
4. **Retained capability records** — each of the eight required capabilities
   carries a rationale and user-facing support guidance.
5. **Accessibility posture rows** — every touched surface and retained
   capability attests keyboard, screen-reader, IME/grapheme/bidi, zoom,
   high-contrast, and reduced-motion behavior.
6. **Recovery-ladder bindings** — every required rung, including the new
   `extension_bisect` and `bounded_repair` rungs, carries a support class,
   review gate, and evidence refs.

## Seeded support scenarios

The fixture corpus covers three profile classes:

- `post_crash_loop_orchestration` — entered after startup crash-loop budget breach.
- `user_invoked_orchestration` — user explicitly chose orchestration from recovery surface.
- `policy_forced_orchestration` — managed policy forced orchestration.

Each fixture includes:
- one stabilized orchestration profile,
- three subsystem bindings (extension bisect, quarantine, bounded repair),
- eight retained capability records,
- accessibility posture rows for all touched capabilities,
- nine recovery-ladder binding rows.

## Verification

Run the protected tests:

```bash
cargo test -p aureline-support --test stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded
```

## Risks and follow-ups

- Live runtime enforcement (extension bisect, quarantine supervisor, repair
  executor) is out of scope and lands with the chrome/runtime consumers.
- Auto-resolution of quarantined lanes is not covered; only explicit
  user-confirmed or policy-reviewed actions are certified.
- Cross-tenant policy reconciliation remains unsupported.
