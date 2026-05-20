# Appearance evidence consumption guide (beta)

This guide explains how shiproom, support, docs, marketplace review, and
design QA consume the appearance-parity evidence packet so they can
answer "is this surface appearance-safe?" from the packet alone, without
reconstructing visual truth from screenshots.

The packet is a join of three already-governed upstream truth sources.
It does not re-derive appearance facts; it consolidates them into one
current, build-linked packet and a metadata-safe support export.

## Canonical records

- Machine-readable packet:
  `fixtures/ux/m3/appearance_evidence/appearance_evidence_packet.json`
- Support export (metadata-safe):
  `fixtures/ux/m3/appearance_evidence/support_export.json`
- Human reports (generated, never hand-edited):
  - `artifacts/ux/m3/appearance_parity_evidence_packet.md`
  - `artifacts/ux/m3/high_contrast_and_live_change_audit.md`
  - `artifacts/extensions/m3/extension_inheritance_gap_packet.md`

Every record carries `ux:appearance_parity_evidence:v1`. The packet,
support export, and the three reports are generated from upstream truth,
so hand-curated prose and stale screenshots can never outrank the
current packet.

## Upstream sources

| Source | Carries | Ref |
| --- | --- | --- |
| Extension appearance conformance | Contributed-UI inheritance gaps, per-axis support, host-stable labels. | `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json` |
| Component-state / token conformance | First-party dark/light/high-contrast structured state reports and screenshot-state diffs. | `artifacts/ux/m3/component_state_screenshot_diff/packet.json` |
| Appearance-session contract | Live OS appearance-change audit and imported-theme mapping coverage. | `fixtures/ux/m3/theme_import_and_live_change/appearance_session_beta_contract.json` |

Each source is governed by its own gate; this packet only consolidates
their current truth. The `source_packets` block records each source's
ref, captured timestamp, and freshness class.

## Build attribution

Every appearance claim is attributable to an exact build identity and
package version:

- First-party rows and the high-contrast signoff and live-update audit
  carry the candidate `exact_build_identity_ref`, `version_label`, and
  `source_revision_ref` from the release artifact graph.
- Contributed rows carry the extension `package_version_label` parsed
  from the registry descriptor (or the mirrored bundle identity).

If a row or signoff is not attributable to a build identity, the gate
refuses the packet.

## Appearance decision vocabulary

| Decision | Meaning |
| --- | --- |
| `appearance_safe` | Proven across the claimed modes; safe to imply parity. |
| `appearance_safe_with_caveats` | Inherits most axes; reduced support on disclosed axes. |
| `appearance_unverified` | Claim exceeds proof; do not badge until verified. |
| `appearance_private_styling` | Renders private styling; does not inherit host appearance. |

First-party rows resolve to `appearance_safe` only when their structured
state report (token conformance + screenshot-state diff + semantic
stability) passes. Contributed decisions are derived from the upstream
support and decision classes.

## How each consumer reads the packet

- **Shiproom**: read `packet_state` and `summary`. A `current_packet`
  state with zero downgraded rows, a passed high-contrast signoff, and a
  passed live-update audit means every claimed beta appearance row is
  proven for the candidate build.
- **Support**: read `support_export.json`. It is metadata-safe
  (`raw_private_material_excluded: true`,
  `manual_private_path_lookup_required: false`) and answers per-surface
  appearance-safety, build attribution, and remediation links without
  opening private paths.
- **Docs / Help**: link the consumption guide and the generated reports.
  Help/About surfaces cite the packet for appearance truth.
- **Marketplace review**: read `extension_inheritance_gap_rows`. Each row
  names the exact contributed surface, its unsupported states, the
  downgrade note shown at install, whether the caveat persists after
  install, and the remediation link.
- **Design QA / Accessibility**: read `high_contrast_signoff` and
  `live_update_audit` for focus/cue legibility, protected-cue
  preservation, contrast targets, and live OS appearance-change posture.

## Freshness and downgrade propagation

The packet carries a `freshness` window (`stale_after`) and a per-source
freshness class. The gate enforces two rules:

- A row that rests on stale evidence must move to a downgraded
  `packet_state`; it may not stay `current_packet`.
- A `needs_review` extension row or a row with no passing evidence may
  never imply full appearance inheritance on any consuming surface.

The `consumer_surfaces` block lists the help, docs, marketplace, and
support surfaces that must downgrade their claimed appearance rows when
the packet is stale or red. Each entry records the conditions
(`downgrades_on`) under which the surface narrows or holds its claim.

## Verify

```sh
python3 ci/check_m3_appearance_evidence.py --repo-root . --check
```

The gate regenerates the packet, support export, the three reports, and
the validation capture from upstream truth, then fails when any output
would change, a referenced artifact is missing, an evidence source ages
past its review window without a downgrade, a claimed row carries
unresolved or red evidence, or the high-contrast signoff or live-update
audit is not attributable to a build identity.
