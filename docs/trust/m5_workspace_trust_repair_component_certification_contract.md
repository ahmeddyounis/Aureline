# M5 workspace-trust / guided-repair component surface certification contract (M05-1099)

This is the closing **surface-certification capstone** for the B130 workspace-trust /
guided-repair component lane. Where the freeze matrix defines the eight reusable
components, the implement lanes (M05-1093..1097) narrow each one, and the accessibility
lane (M05-1098) proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export
parity and per-family auto-narrowing, this capstone *certifies* that the shared component
truth holds on every claimed M5 trust / repair operating profile — and auto-narrows any
profile that cannot sustain it.

- **Schema:** [`schemas/ui/m5-workspace-trust-repair-component-certification.schema.json`](../../schemas/ui/m5-workspace-trust-repair-component-certification.schema.json)
- **Canonical proof bundle:** `artifacts/release/m5-workspace-trust-repair-component-certification-proof/support_export.json`
- **Cited workspace-trust-repair proof bundle (one, shared):** `artifacts/release/m5-workspace-trust-repair-proof/support_export.json`
- **Accessibility evidence:** `artifacts/release/m5-workspace-trust-repair-component-accessibility-parity/support_export.json`

## What is certified

The packet is keyed on the **claimed profile** a user, operator, or support engineer
reads workspace-trust and guided-repair truth through, not on the reusable component
family it renders. The eight certified profiles are:

| Profile | Reads |
| --- | --- |
| `local_trusted_workspace` | workspace-trust banner, trust-fact grid |
| `remote_reviewed_workspace` | workspace-trust banner, root-trust strip |
| `managed_policy_workspace` | trust-fact grid, trust-elevation sheet |
| `exact_reversal_repair` | repair-transaction preview card, rollback-class strip |
| `restricted_workspace` | restricted-capability row, trust-fact grid |
| `mixed_root_workspace` | root-trust strip, workspace-trust banner |
| `checkpoint_missing_repair` | repair-transaction preview card, rollback-class strip |
| `manual_follow_up_repair` | repair-result receipt row, rollback-class strip |

Every frozen component family — workspace-trust banner, trust-fact grid, trust-elevation
sheet, restricted-capability row, root-trust strip, repair-transaction preview card,
rollback-class strip, and repair-result receipt row — is certified on at least one
profile. Support and export cite the same canonical proof set: the always-on CLI/export
axis and the metadata-only packet let a support engineer reconstruct the certified trust
and repair truth without a feature-local translation.

## The six truth axes

Each profile is scored on exactly six axes, each appearing once:

1. **`visual`** — grant source, policy epoch, trust scope, per-root trust, narrowed
   capability, repair-target ids, checkpoint availability, reversal class, partial
   success, and manual follow-up are shown on-surface.
2. **`keyboard`** — the same inspect-trust / review-transaction / reopen-restricted /
   request-approval actions are keyboard-reachable.
3. **`screen_reader`** — the same trust / repair truth is announced non-visually, never
   color/glyph-only.
4. **`cli_export`** (always-on) — the profile state is reconstructable as text / JSON /
   Markdown for support and automation. This axis must always certify.
5. **`degraded_state`** — a stale lineage, expired epoch, mixed-root trust, narrowed
   capability, missing checkpoint, or unproven reversal honestly downgrades a
   `full_trust_reviewed_result` / `reviewable_result` claim rather than reading as fresh,
   blanket first-party trust.
6. **`trust_repair_truth`** — grant source, policy epoch, trust scope, per-root trust,
   narrowed capability, checkpoint availability, reversal class, and repair outcome stay
   explicit and never collapse into generic chrome wording, imply blanket trust across
   roots or profiles, hide checkpoint absence or reversal limits, or collapse distinct
   reversal outcomes into a generic success.

## The B130 guardrails

Every row carries a `guardrails` object; all three fields must be `false`, and a breach
blocks the profile (red):

- `implies_blanket_trust_across_roots_or_profiles`
- `hides_checkpoint_absence_or_reversal_limits`
- `collapses_reversal_outcomes_into_generic_success`

## The invariant: a degraded axis must produce a visible claim narrowing

The trust / repair claim ceiling ranks, strongest first: `full_trust_reviewed_result`
(7), `reviewable_result` (6), `stale_lineage_projection` (5), `expired_epoch_projection`
(4), `mixed_root_projection` (3), `narrowed_capability_projection` (2),
`missing_checkpoint_projection` (1), `unproven_reversal_projection` (0).

- **Green** — every axis certified, every guardrail held, the claimed ceiling delivered.
- **Yellow** — an axis is `disclosed_narrowed` with a bound reason and a frozen downgrade
  trigger, and the profile visibly narrows its claim from `claimed_claim` to a weaker
  `certified_claim` via `claim_auto_narrow` (bound to the narrowed, non-always-on axis,
  with a non-generic label).
- **Red** — a degraded axis is hidden behind a fresh full-trust claim
  (`undisclosed_drift`, or a disclosed narrowing with no claim reduction), a guardrail
  breaks, a non-local profile certifies a `full_trust_reviewed_result`, the CLI/export
  axis drops, the copy / export parity is incomplete, the certified claim exceeds the
  claimed one, or the narrowing is inconsistent. Red profiles are not publishable.

Only a **local first-party profile** may certify a `full_trust_reviewed_result` claim —
a remote, managed, restricted, mixed-root, or repair profile is at most a reviewable
result or a narrowed projection. The stored `derived_status` is always recomputed and
compared on validation, so the verdict can never be hand-asserted.

## Metadata-only boundary

Raw credentials, session tokens, and grant secrets never cross this boundary. The
validator rejects any export carrying obviously forbidden material.

## Regenerating the proof

```
GEN_WORKSPACE_TRUST_REPAIR_CERT_ARTIFACTS=1 cargo test -p aureline-shell \
  certify_workspace_trust_and_guided_repair...::tests::generate_artifacts
```

This writes the canonical `support_export.json`, `matrix.csv`, and `report.md` under
`artifacts/release/m5-workspace-trust-repair-component-certification-proof/` and mirrors
them into `fixtures/ui/m5-workspace-trust-repair-component-certification/`. The
`checked_in_export_matches_seeded_builder` test byte-locks the on-disk export to the
seeded builder.
