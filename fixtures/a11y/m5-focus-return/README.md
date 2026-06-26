# M5 Focus-Return and Stable-Selection Fixtures

These fixtures are valid, export-safe focus-and-selection contracts that exercise the
auto-narrowing behavior the canonical support export keeps green. Each one keeps a
zone row for every governed focus zone, the shared and focus vocabulary sets intact,
and the conformance-review, consumer-projection, and release-posture invariants
satisfied — the difference is which zone narrows and why. They are minted from the
same seed builder as the canonical export by `aureline_shell_m5_focus_return`.

## proof_stale_narrowed.json

The dense-collection zone's assistive-tech proof has gone stale. The zone narrows from
Stable to Beta and drops its keyboard-complete claim, keeping its `proof_stale`
downgrade trigger, its explicit focus-return rule, its stable-item-identity rule, and
its roving-tabindex rule intact. Demonstrates that stale proof narrows the claim — and
withdraws the keyboard-complete claim — rather than hiding the zone or silently keeping
an unsupportable claim.

## bridge_unavailable_narrowed.json

The multi-window-layout zone's OS accessibility bridge is unavailable. The zone narrows
from Stable to Preview, drops its `non_visual_fidelity` to `degraded_accessible`, drops
its keyboard-complete claim, and keeps its `bridge_unavailable` downgrade trigger. Its
stable-item-identity rule still preserves focus and selection across
`multi_window_restore`, so restored windows keep their item identity rather than
degrading into row-index focus loss. Demonstrates that when the bridge is gone the zone
still preserves stable item identity and a safe focus-return fallback, rather than the
context being dropped.
