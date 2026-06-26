# M5 Live-Announcement Grammar Fixtures

These fixtures are valid, export-safe announcement-grammar catalogs that exercise
the auto-narrowing behavior the canonical support export keeps green. Each one keeps
a class for every governed dynamic-event class, the shared and grammar vocabulary
sets intact, and the conformance-review, consumer-projection, and release-posture
invariants satisfied — the difference is which class narrows and why. They are
minted from the same seed builder as the canonical export by
`aureline_shell_m5_announcement_grammar`.

## proof_stale_narrowed.json

The success-with-recovery class's assistive-tech proof has gone stale. The class
narrows from Stable to Beta and keeps its `proof_stale` downgrade trigger, so the
gap is disclosed while the class stays present with its message template, channel,
coalescing budget, suppression rules, and durable fallback intact. Demonstrates that
stale proof narrows the claim rather than hiding the class.

## live_region_unavailable_narrowed.json

The progress-milestone class's OS live region is unavailable. The class narrows from
Stable to Preview, shifts its `fallback_durability` to `durable_surface_only`, and
keeps its `bridge_unavailable` downgrade trigger and its reopenable `run_header`
durable fallback. Demonstrates that when the live region is gone the announcement
still has a durable counterpart the user can reopen, rather than the meaning being
dropped.
