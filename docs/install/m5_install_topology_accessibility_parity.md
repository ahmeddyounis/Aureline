# M5 install-topology accessibility & auto-narrowing parity (M05-1178)

This contract is the accessibility-localization-support-export parity and auto-narrowing capstone over the
frozen M5 install-topology matrix (`m5_install_topology_matrix`). Where the freeze matrix defines the five
governed delivery-topology families — **per-user-managed, per-machine-managed, side-by-side-stable-preview,
portable-mode, and offline-air-gap-bundle** — and the 1173–1176 implementation lanes resolve their per-surface
install-mode, updater-ownership, state-root, channel-isolation, managed-operation, and rollback truth, this
lane certifies — per delivery-topology family — that every install-mode / updater-owner / state-root /
repair-verify / rollout-ring / rollback claim survives beyond the installer screenshot and **auto-narrows when
its state-boundary proof, repair/verify coverage, or rollout-ring evidence is stale, partial, or
policy-blocked**.

- **Module:** `crates/aureline-ui/src/m5_install_topology_accessibility_parity_and_narrowing_when_install_topology_state_root_repair_verify_or_rollout_evidence_is_stale/`
- **Schema:** `schemas/install/m5-install-topology-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-install-topology-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/install/m5-install-topology-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every family exposes a keyboard-reachable,
   screen-reader-announced, high-zoom-reflowing (200–400%), high-contrast / larger-text-legible,
   localization-safe, and CLI/headless-reachable path into the same install-topology identity, semantic role,
   registry reference, install mode, state root, and rollback target the rendered surface shows — never a
   pointer-only affordance hidden in installer chrome, an unlabeled control, or an updater owner / state root
   that only lives in a screenshot. The support / release / CLI export reconstructs each family's meaning from
   typed tokens and opaque refs **without a raw payload**, so support and release proof can state which
   delivery-topology truth class was active without leaking a raw secret blob or a machine-specific sensitive
   path.

2. **Honest auto-narrowing.** When a side-by-side family's state-boundary proof can only be partially disclosed,
   a portable / offline family's repair/verify coverage cannot be confirmed, or a family's rollout-ring evidence
   has aged out or is policy-blocked, the claim auto-narrows from `trusted_delivery_surface` /
   `reviewable_delivery_surface` to the matching projection, discloses the narrowing with a precise trigger and
   binding dimension, and preserves the canonical identity / last-known registry reference. A family with every
   dimension intact must **not** carry a spurious narrowing, and a weakened family can never keep a trusted,
   stable delivery claim — install-topology meaning is never conveyed by an installer-chrome-only affordance, a
   mislabeled screenshot, or an unlabeled control alone.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the updater service, shell / About,
   diagnostics, admin, installer, docs/help, CLI-export, support-export, and product surfaces so product, help,
   and release publication stay aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_delivery_surface` | Fully current, registry-bound, ownership-inspectable, state-isolated, repair/verify-covered, rollout-evidenced — trusted and stable. |
| `reviewable_delivery_surface` | Self-sufficient, inspectable read-only install-topology projection (a static per-machine policy-control / registry reference an admin can inspect), not an authoritative live-resolving surface. |
| `state_boundary_disclosed_projection` | Side-by-side stable/preview state-boundary proof can only be partially disclosed — an **honest disclosed-absence**, not a truth overstatement (side-by-side-stable-preview). |
| `repair_verify_unverified_projection` | Portable / offline repair/verify coverage cannot be confirmed (portable-mode). |
| `rollout_evidence_unverified_projection` | Rollout-ring promotion / rollback evidence has aged out or is policy-blocked (offline-air-gap-bundle). |

## Weakening dimensions and their frozen triggers

Each family maps 1:1 to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (family) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `install_ownership_clarity` (per-user-managed) | *(green — fully qualified trusted)* | — | — |
| `policy_control_clarity` (per-machine-managed) | *(reviewable — high-zoom reflow disclosed)* | — | — |
| `state_boundary_clarity` (side-by-side-stable-preview) | `state_boundary_disclosed_partial` | `proof_stale` | no (honest disclosed-absence) |
| `repair_verify_clarity` (portable-mode) | `repair_verify_unconfirmed` | `deployment_claim_outpaced_ring_or_repair_verify_evidence` | yes |
| `rollout_evidence_clarity` (offline-air-gap-bundle) | `rollout_evidence_unconfirmed` | `deployment_claim_outpaced_ring_or_repair_verify_evidence` | yes |

The `state_boundary_disclosed_partial` state is deliberately **excluded** from `cannot_be_shown_trusted`: a
partial boundary proof shown honestly with the last-known isolated state root is a disclosed-absence operation,
not a truth overstatement.

## Structure-heavy families

The **side-by-side-stable-preview** (channel-isolation table), **portable-mode** (root-inventory table), and
**offline-air-gap-bundle** (artifact-graph rollback table) families render a dense structured surface, so they
must additionally bind their structured layout to an equivalent flat list / textual / CLI path (a `structured`
fallback modality **plus** a non-visual list / textual / CLI path).

## Certified rows

Five rows, one per family: **1 green** (per-user-managed — install mode / updater ownership stays inspectable,
trusted) and **4 yellow** — the per-machine-managed family stays a fully-qualified reviewable surface but
discloses a high-zoom reflow reduction, and the remaining three auto-narrow to their permitted projections. **No
red rows may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_INSTALL_TOPOLOGY_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_install_topology_accessibility_parity_and_narrowing_when_install_topology_state_root_repair_verify_or_rollout_evidence_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
