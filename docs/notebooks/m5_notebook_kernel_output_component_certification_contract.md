# M5 Notebook Document/Kernel/Output Component Profile Certification (M05-1091)

This is the **closing surface-certification capstone** for the B129 notebook-kernel-output component
lane. Where the frozen matrix (`schemas/ui/m5-notebook-kernel-output-component-matrix.schema.json`)
defines the eight reusable **notebook-document header**, **kernel-state strip**, **kernel-picker
row**, **kernel-origin pill**, **output-trust banner**, **output-provenance chip group**,
**restart-consequence card**, and **kernel-recovery card** components, the M05-1085..1088 implement
lanes narrow each one, the M05-1090 consumer lane proves they are reusable across the claimed
notebook-editor / diff-review / debug / AI-context / CLI / support-export consumers, and the
M05-1089 accessibility / auto-narrowing capstone certifies keyboard / screen-reader / high-zoom /
reduced-motion / CLI / export parity per family, this capstone **certifies that the shared notebook
document / kernel / output / trust / recovery component truth holds on every claimed M5 local /
remote / managed notebook profile** — and auto-narrows any profile that cannot sustain it.

- **Module:**
  `crates/aureline-notebook/src/certify_notebook_document_kernel_output_trust_recovery_and_output_boundary_component_truth_on_every_claimed_m5_local_remote_managed_notebook_profile/`
- **Boundary schema:** `schemas/ui/m5-notebook-kernel-output-component-certification.schema.json`
- **Release proof:** `artifacts/release/m5-notebook-kernel-output-component-certification-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-notebook-kernel-output-component-certification/`
- **Canonical bundle every row cites:** `artifacts/release/m5-notebook-kernel-output-proof/support_export.json`
  (the frozen notebook-kernel-output component matrix release proof — the canonical M5 evidence index
  entry for this lane)

## What is certified

The packet is keyed on the **notebook runtime / output profile** a user, operator, or support
engineer reads notebook document / kernel / output / trust / recovery truth through — not on
component family or implement lane. Eight claimed profiles are certified exactly once:

| Profile | Meaning |
| --- | --- |
| `local_trusted_kernel` | A local, first-party trusted kernel with a live, trusted output — the only profile that may certify a live-trusted claim. |
| `remote_isolated_kernel` | An isolated remote kernel (SSH / container) with an explicit remote origin. |
| `managed_kernel` | A managed-workspace kernel with an explicit managed origin. |
| `trusted_local_output` | A trusted local output rendered through the sanitized / active trust classes. |
| `stale_output` | An output whose trust evidence has gone stale and must not read as live. |
| `degraded_origin_kernel` | A kernel whose origin is degraded (unstated or approximate). |
| `restarted_kernel` | A kernel restarted clean, clearing live results without a hidden rerun. |
| `disconnected_reconnecting_kernel` | A kernel that disconnected and is reconnecting with only partial parity. |

Each profile is scored across six truth axes:

| Axis | What it proves |
| --- | --- |
| `visual` | Document identity, kernel origin / class / liveness, output trust class, output provenance, restart / reconnect consequence, and preserved-vs-lost recovery state are shown on-surface. |
| `keyboard` | The same select-kernel / inspect-origin / open-raw / reconnect / restart-clean / choose-another-kernel / export actions are keyboard-reachable. |
| `screen_reader` | The same truth is announced non-visually, never color / glyph / hover-only. |
| `cli_export` | **Always-on.** The certified profile state is reconstructable as text / JSON / Markdown for support and automation. |
| `degraded_state` | A stale, disconnected, or kernel-free reading honestly downgrades the claim rather than reading as a fresh live-trusted result. |
| `notebook_truth` | Document identity, kernel origin, output trust class, output provenance, stale-vs-live honesty, restart / reconnect consequence, and recovery continuity stay explicit and never let a recovery card imply a rerun, present stale output as live, hide the trust class behind hover only, or collapse local / remote / managed kernels into one unlabeled badge. |

## Verdicts

Each row's `derived_status` is **recomputed, never authored**:

- **green** — every axis certified and the claimed result tier delivered.
- **yellow** — a truth axis is not current, and the result claim narrows **visibly** to the weakest
  supported ceiling, bound to the narrowed axis with a non-generic disclosure label and a frozen
  downgrade trigger.
- **red** — a degraded axis is hidden behind a fresh live-trusted claim inherited from a healthier
  profile, CLI/export parity dropped, a spec guardrail is breached, a non-local profile claims a
  live-trusted result, or the narrowing is inconsistent. A red profile is **not publishable**.

## Invariants

- **A degraded axis must produce a visible claim narrowing.** A profile keeping a
  `live_trusted_result` / `reviewable_result` claim while an axis is not current over-claims and
  blocks.
- **Only the local trusted-kernel profile may certify a `live_trusted_result` claim.** A stale,
  disconnected, or kernel-free profile can never keep a fresh live-trusted claim.
- **The `cli_export` axis is always-on** and must certify on every row so support and automation can
  reconstruct the certified truth from the same object identity the user saw.
- **The four B129 guardrails** must all hold (false) on every row:
  `recovery_card_implies_rerun`, `presents_stale_output_as_live`,
  `hides_trust_class_behind_hover_only`, `collapses_kernel_origins_into_one_badge`.
- Every row cites exactly one canonical notebook-kernel-output proof bundle. The packet is
  metadata-only: raw notebook cell material, credential material, and bearer secrets never cross this
  boundary.

## Certified result

Eight profiles: **4 green** (local trusted kernel delivers `live_trusted_result`; remote isolated
kernel, managed kernel, and trusted local output deliver `reviewable_result`) and **4 yellow** (stale
output narrows to `stale_output_projection`; degraded-origin kernel narrows to
`degraded_origin_projection`; restarted kernel narrows to `no_kernel_projection`; disconnected /
reconnecting kernel narrows to `partial_kernel_parity_projection`). No red. All eight frozen
component families are certified on some profile.

## Regenerating the proof

```
GEN_NOTEBOOK_KERNEL_OUTPUT_CERT_ARTIFACTS=1 cargo test -p aureline-notebook --lib \
  certify_notebook_document_kernel...::tests::generate_artifacts
```

The seed builder in the module is the single source of truth; the checked-in
`support_export.json` and the fixtures mirror are byte-aligned to it and re-verified by the
`checked_in_export_matches_seeded_builder` test.
