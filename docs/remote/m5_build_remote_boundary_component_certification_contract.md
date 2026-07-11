# M5 build/remote-boundary component profile certification contract (M05-1083)

This is the closing **profile-certification capstone** for the B128
build/remote/managed-workspace boundary component lane. Where the freeze matrix defines
the eight reusable components, the four implement lanes narrow each one, the consumer
lane adopts them, and the accessibility lane proves keyboard / screen-reader /
reduced-motion / high-contrast / CLI-export parity, this capstone *certifies* that the
shared component truth holds on every claimed M5 build / remote / managed execution
profile — and auto-narrows any profile that cannot sustain it.

- **Schema:** [`schemas/ui/m5-build-remote-boundary-component-certification.schema.json`](../../schemas/ui/m5-build-remote-boundary-component-certification.schema.json)
- **Canonical proof bundle:** `artifacts/release/m5-build-remote-boundary-component-certification-proof/support_export.json`
- **Cited build/remote-boundary proof bundle (one, shared):** `artifacts/release/m5-build-remote-boundary-proof/support_export.json`
- **Accessibility evidence:** `artifacts/release/m5-build-remote-boundary-component-accessibility-proof/support_export.json`

## What is certified

The packet is keyed on the **claimed execution / deployment profile** a user, operator,
or support engineer reads build/remote-boundary truth through, not on the reusable
component family it renders. The eight certified profiles are:

| Profile | Reads |
| --- | --- |
| `local_execution` | host-boundary strip, execution-origin receipt row |
| `ssh_execution` | host-boundary strip, adapter-confidence chip |
| `container_execution` | adapter-confidence chip, discovery-diff card |
| `devcontainer_execution` | discovery-diff card, execution-origin receipt row |
| `managed_workspace` | managed-workspace lifecycle card, host-boundary strip |
| `suspend_resume` | suspend/resume/rebuild review sheet, managed-workspace lifecycle card |
| `rebuild_recreate` | suspend/resume/rebuild review sheet, managed-workspace lifecycle card |
| `expiry_local_safe` | workspace-expiry banner, local-safe continuation card |

Every frozen component family — adapter-confidence chip, discovery-diff card,
host-boundary strip, execution-origin receipt row, managed-workspace lifecycle card,
suspend/resume/rebuild review sheet, workspace-expiry banner, and local-safe
continuation card — is certified on at least one profile.

## The six truth axes

Each profile is scored on exactly six axes, each appearing once:

1. **`visual`** — adapter confidence, discovery drift, host ownership, execution origin,
   lifecycle state, persistence class, continuity, expiry timing, and local-safe
   continuation are shown on-surface.
2. **`keyboard`** — the same inspect / review / reconnect / export-before-loss / renew
   actions are keyboard-reachable.
3. **`screen_reader`** — the same boundary truth is announced non-visually, never
   color/glyph-only.
4. **`cli_export`** (always-on) — the profile state is reconstructable as text / JSON /
   Markdown for support and automation. This axis must always certify.
5. **`degraded_state`** — a stale, unverified, or unsupported reading honestly downgrades
   a `full_truth` / `resolved_truth` claim rather than reading as fresh first-party local
   truth.
6. **`boundary_truth`** — host ownership, execution origin, lifecycle, continuity, and
   expiry stay explicit and never collapse into generic status wording, imply exact
   continuity after a material change, hide local-safe / companion handoff in overflow
   only, or let lower-confidence discovery overwrite a resolved target.

## The three spec guardrails

Every certified profile carries the three B128 guardrails, all of which must stay
`false`; any breach blocks the profile (red):

- `implies_exact_continuity_after_material_change`
- `hides_local_safe_or_companion_handoff_in_overflow_only`
- `lower_confidence_overwrites_resolved_target_without_review`

## The invariant: a degraded axis must produce a visible claim narrowing

The boundary-support claim ceiling ranks, strongest first: `full_truth` (5),
`resolved_truth` (4), `degraded` (3), `stale` (2), `unverified` (1), `unsupported` (0).

- **Green** — every axis certified, the claimed ceiling delivered.
- **Yellow** — an axis is `disclosed_narrowed` with a bound reason and a frozen downgrade
  trigger, and the profile visibly narrows its claim from `claimed_claim` to a weaker
  `certified_claim` via `claim_auto_narrow` (bound to the narrowed, non-always-on axis,
  with a non-generic label).
- **Red** — a degraded axis is hidden behind a fresh first-party full claim
  (`undisclosed_drift`, or a disclosed narrowing with no claim reduction), a guardrail is
  breached, a non-local profile certifies a live `full_truth` claim, the CLI/export axis
  drops, the copy / export parity is incomplete, the certified claim exceeds the claimed
  one, or the narrowing is inconsistent. Red profiles are not publishable.

Only the local, first-party-local profile may certify a live `full_truth` claim; every
remote / managed profile certifies at most `resolved_truth` and narrows visibly when a
boundary dimension weakens. A stale, unverified, or unsupported profile can never keep a
fresh first-party `full_truth` claim. The stored `derived_status` is always recomputed
and compared on validation, so the verdict can never be hand-asserted.

## Metadata-only boundary

Raw provider tokens, credential material, and bearer secrets never cross this boundary.
The validator rejects any export carrying obviously forbidden material.

## Regenerating the proof

```
GEN_BUILD_REMOTE_BOUNDARY_CERT_ARTIFACTS=1 cargo test -p aureline-remote --lib \
  certify_adapter_confidence...::tests::generate_artifacts
```

This writes the canonical `support_export.json`, `matrix.csv`, and `report.md` under
`artifacts/release/m5-build-remote-boundary-component-certification-proof/` and mirrors
them into `fixtures/ui/m5-build-remote-boundary-component-certification/`. The
`checked_in_export_matches_seeded_builder` test byte-locks the on-disk export to the
seeded builder.
