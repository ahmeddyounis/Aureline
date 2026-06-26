# M5 Dynamic-Surface Accessibility Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every governed
object present, the frozen vocabulary set intact, and the conformance-review,
consumer-projection, and release-posture invariants satisfied — the difference is
which object is narrowed and why. They are minted from the same seed builder as the
canonical export by `aureline_shell_m5_dynamic_surface_a11y_matrix`.

## bridge_unavailable.json

The bridge-diagnostics packet is held after the OS accessibility bridge goes
unavailable on the target platform. Held objects no longer carry a public claim, so
the evidence requirement relaxes to `recommended`, but the object stays present with
its bridge-state and non-visual-fidelity vocabularies intact. The
accessibility-surface descriptor, screen-reader label model, live-announcement
class, focus-return contract, and dense-surface non-visual summary remain at their
canonical qualifications. Demonstrates that an unavailable bridge narrows to held
and discloses the degradation rather than claiming silent screen-reader
completeness.

## dense_summary_narrowed.json

The dense-surface non-visual summary is narrowed from Stable to Preview after a
non-visual-fidelity finding. The object keeps all of its declared vocabularies and
the `non_visual_fidelity_lost` downgrade trigger, so the gap is disclosed while the
claim narrows. Demonstrates that a dense surface whose non-visual summary regresses
narrows the claim rather than shipping a surface that exposes only the visible rows.
