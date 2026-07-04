# M5 Visual-Designer Surface Certification Contract (M05-810)

This contract certifies that every **claimed M5 visual-design surface** keeps its
public claim honest with the underlying mapping / round-trip / preview-runtime
truth, and that support and release exports preserve exactly the truth visible
in-product. It is the closing certification capstone over the B94 visual-designer
component lane: the freeze matrix froze the reusable primitives (M05-804), the
805–807 lanes resolved their per-target truth, the 808 lane certified
accessibility fallback, and the 809 lane adopted them across handoff consumers.

- **Module:** `crates/aureline-preview/src/certify_export_and_support_packet_parity_and_auto_narrowing_across_claimed_visual_design_surfaces/`
- **Boundary schema:** `schemas/ui/m5-visual-designer-surface-certification.schema.json`
- **Support / release export:** `artifacts/release/m5-visual-designer-surface-certification-proof/support_export.json`
- **CSV matrix:** `artifacts/release/m5-visual-designer-surface-certification-proof/matrix.csv`
- **Report:** `artifacts/components/m5-visual-designer-surface-certification.md`
- **Fixtures:** `fixtures/ui/m5-visual-designer-surface-certification/`

## Model

Each row keys on one `M5VisualDesignClaimedSurface` (design-canvas workspace,
structure-tree panel, property-inspector panel, source round-trip rail,
breakpoint / device-preview deck, framework-pack preview, browser-runtime
inspection, docs / help embeds, support export, release proof) and certifies four
truth dimensions:

| Dimension | Reused frozen vocabulary | Supported claim ceiling |
| --- | --- | --- |
| Mapping quality | `M5BreakpointMappingQuality` | `exact` → writable, `approximate` → inspect-only, `unmapped` → source-only |
| Round-trip support | `RoundTripCapabilityClass` | write-back → writable, inspect / none → inspect-only, source-only fallback → source-only |
| Preview-runtime freshness | `PreviewFreshnessClass` | `fresh` → writable, `aging` → inspect-only, `stale` / `unknown` → read-only |
| Export parity (always on) | `CopyExportParity` + `M5VisualDesignCertExportField` | text / JSON / Markdown + mandatory truth fields, never a screenshot alone |

A surface declares the claim (`M5VisualDesignClaimTier`:
`fully_interactive_writable` → `inspect_only` → `read_only` → `source_only`) it
makes when every dimension is healthy. The lane derives the **effective** claim by
narrowing the declared claim to the weakest dimension's supported ceiling.

## Statuses

- **Certified (green):** the observed truth supports the declared claim on every
  dimension; the effective claim equals the declared claim.
- **Narrowed / disclosed (yellow):** a dimension weakened, so the claim narrowed
  to what the dimension supports and disclosed an honest `ClaimAutoNarrow` (frozen
  trigger, weakened dimension, precise reason, preserved source truth).
- **Blocked (red):** the claim exceeds what the truth supports (drift is hidden),
  the export drops a mandatory truth field, or a narrowing is undisclosed. Blocked
  rows may not ship.

## Acceptance criteria

- **AC1 — a stale / partial / unsupported lane can no longer present as fully
  writable or fully mapped.** `claim_tracks_truth` rejects any effective claim
  above the weakest supported ceiling (`ClaimHidesDrift`) and any claim narrowed
  further than the truth requires (`OverNarrowedClaim`).
- **AC2 — support and release exports preserve the same mapping / runtime truth
  visible in-product.** `export_preserves_truth` requires text / JSON / Markdown
  copy parity plus every `M5VisualDesignCertExportField::MANDATORY` field
  (selection identity, mapping quality, round-trip state, runtime origin, preview
  freshness, effective claim, narrowed reason); each row cites the one
  certification bundle.
- **AC3 — claim publication and field triage stay aligned on downgrade
  behavior.** `narrowing_disclosed` requires the auto-narrow to match the binding
  dimension and effective claim, and the docs / help, support, and release
  evidence surfaces are all certified so the narrowed states surface identically
  in UI, docs / help, release packets, and support artifacts.

## Boundary

The packet is metadata-only: raw source bodies, diff hunks, credentials, and
provider payloads never cross the boundary. Only typed class tokens, opaque
summary / evidence refs, booleans, and redacted labels are carried, and a
forbidden-material scan rejects any export that leaks secret-like strings.
