# Badge/chip/pill taxonomy, overflow budget, and text-plus-icon semantics contract

This document freezes the **cross-surface contract** for compact status
tokens rendered as badges, chips, or pills. It exists to stop surfaces
from collapsing **support class**, **scope**, **confidence**,
**freshness**, and **permissions** into color-only hints or
position-only folklore when layouts become dense.

The contract is normative. Where it disagrees with the source product
documents in `.t2/docs/`, those sources win and this contract (plus its
schema and fixtures) MUST update in the same change.

## Companion artifacts

- [`/schemas/ux/status_pill.schema.json`](../../schemas/ux/status_pill.schema.json)
  — machine-readable family/term taxonomy, overflow budget constraints,
  and fixture case shapes.
- [`/fixtures/ux/badge_cases/`](../../fixtures/ux/badge_cases/)
  — worked crowded-row, export-parity, high-contrast, and mixed
  support-class/freshness cases.

## Upstream contracts this contract composes with

This contract is a **composition contract**. It does not replace
domain-specific badge contracts; it defines the shared *shape* and
stacking rules that those owners project into compact tokens.

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — “Badges and pills” and
  component rules: **not more than three inline before overflow** and
  **never color-only**.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` — badge families,
  “one row rarely shows more than one badge from the same family”, and
  “support claims must carry evidence age”.
- [`/docs/ux/decoration_precedence_contract.md`](./decoration_precedence_contract.md)
  — precedence bands, collapse rules, and no-hue-only requirements.
- [`/docs/governance/capability_axis_matrix.md`](../governance/capability_axis_matrix.md)
  — support-class / scope / freshness separation for capability claims.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  — view-level freshness truth. (This contract defines the *compact
  token family*; view freshness owns its full record shape.)
- [`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  — environment/transport posture objects pills may summarize.
- [`/docs/ux/file_state_badge_and_write_review_contract.md`](./file_state_badge_and_write_review_contract.md)
  and [`/docs/governance/provenance_badge_contract.md`](../governance/provenance_badge_contract.md)
  — examples of domain-specific badge owners that still must project
  into the shared families without relying on hue.

## Definitions

- **Badge** — compact, primarily informative token. Not interactive by
  default.
- **Chip** — compact token that is interactive (toggle, filter, or
  selection). Must expose pressed/selected state and keyboard
  affordances.
- **Pill** — a shape convention (rounded token) used by either badges or
  chips. In this repository, *pill* refers to the shared **record**
  shape; the UI chooses the shell.

## Core rules (must-haves)

1. **Never hue-only.** Color may reinforce meaning, but the meaning must
   survive high-contrast, forced-colors, screenshots, and export. Every
   token has a short text label; icons are reinforcement, not the only
   channel.
2. **One fact per family.** Each pill belongs to one family. A row
   should rarely show more than one pill from the same family. If
   multiple facts exist for the same family, the surface collapses them
   into one visible pill plus a details route that enumerates the hidden
   facts.
3. **Inline density budget.** A surface MUST NOT render more than **3**
   pills inline on interactive rows/cards/tabs before overflow treatment.
   (Support-export and machine logs are exempt and may render the full
   set.)
4. **Overflow must be recoverable.** When pills exceed the inline
   budget (or available space), the remainder collapses into an
   inspectable summary affordance (for example, `3 more`). The summary
   must be keyboard reachable and must expand into a list that names the
   families and the pill labels in precedence order.
5. **Support claims must carry freshness and scope.** A surface MUST
   NOT render a support-class pill without also disclosing (a) the
   evidence freshness class and (b) the scope the claim applies to (for
   example, “All matching results”, “Workspace”, or a bounded selection
   set). A “Certified” claim without “as-of” truth is non-conforming.

## Pill families and controlled terms

The schema publishes the controlled vocabulary. Families are stable and
closed; adding a new family or repurposing a term is breaking.

| Family | Purpose | Examples (labels) |
|---|---|---|
| `status` | A top-level readiness/health cue for the object/row. | `Ready`, `Blocked`, `Degraded` |
| `mode` | A currently active operating mode that changes interpretation. | `Review`, `Presentation` |
| `environment` | Where the thing runs (local/remote/managed/etc). | `Local`, `Remote`, `Managed` |
| `provider` | Which subsystem produced the fact (index, LSP, AI, etc). | `Project graph`, `Language server`, `AI assist` |
| `scope` | Which population/scope the claim applies to. | `Visible rows`, `All matching` |
| `confidence` | How confident the product is in the underlying fact. | `High`, `Low`, `Unknown` |
| `permissions` | Whether the user/system is permitted to act/see. | `Granted`, `Policy blocked` |
| `freshness` | Evidence freshness floor for claims. | `Live`, `Warm`, `Stale`, `Unverified` |
| `support_class` | Support tier as a user-facing claim. | `Certified`, `Community` |

## Stacking and precedence

Pills are not a pile of stickers. When selecting which pills can remain
inline under the budget, surfaces follow precedence:

1. `permissions` (when constraining or blocking)
2. `status` (blocked/degraded/warn)
3. `freshness` and `confidence` (when they change interpretation)
4. `support_class` (only when paired with freshness + scope)
5. `scope`, `environment`, `provider`, `mode` (metadata; overflow first)

If a surface already has a stronger decoration from the precedence
contract (for example a trust/policy critical banner), it may omit the
lower-precedence pills from inline view but MUST preserve them in the
overflow/detail route and in accessible names.

## Expansion contract (plain-language and detail routes)

Every pill must expand into plain-language explanation. The expansion is
allowed to be a tooltip, focus popover, hovercard, inline explainer row,
or sheet, but it MUST be reachable without pointer-only hover.

At minimum, the expansion provides:

- the pill’s canonical label (same words as the compact pill);
- one short explanation sentence (no generic “not supported” when a more
  precise reason is knowable);
- a details route or command id when the state affects actionability;
- evidence references when the pill is a claim (support class, scope,
  permissions, freshness, confidence).

## Aging and downgrade behavior (support and freshness)

When freshness floors lapse or capability/support windows change, the UI
must **downgrade visibly** rather than silently retaining an earlier
strong claim:

- A `support_class = certified` pill MUST NOT remain the only visible
  cue once `freshness = stale` or `unverified`; the surface either shows
  both pills inline (budget permitting) or keeps `Stale` inline and
  collapses the support claim into overflow with a downgrade sentence.
- If a surface can no longer justify the previous support class for the
  current scope (scope changed, policy narrowed, environment changed),
  it must render a scope pill and a downgrade note in the expansion
  route rather than relying on color changes alone.

## Acceptance checklist

A surface or fixture conforms when:

1. Every rendered pill is traceable to a controlled family+term
   vocabulary entry, not to color alone.
2. Dense rows do not render more than three inline pills without an
   overflow/summary affordance.
3. High-contrast/forced-colors rendering retains labels/icons/shape and
   the state remains discoverable via keyboard.
4. Support-class pills never ship without freshness and scope disclosure.
5. Exported/CLI text uses the same pill labels and ordering as the UI
   record, so reviewers do not need screenshots to understand meaning.

