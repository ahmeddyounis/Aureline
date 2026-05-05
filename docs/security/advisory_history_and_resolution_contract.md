# Advisory history timeline and resolution contract

This document freezes the pre-implementation contract for **advisory
history timelines**, **resolved-state downgrade behavior**, and the
**current-mitigation linkage rules** that keep advisories auditable after
an incident.

The goal is to preserve auditability without keeping resolved advisories
at emergency prominence:

- resolved/superseded/withdrawn advisories remain inspectable and
  copy-safe;
- reviewers can move from “current mitigation” to prior history without
  broken identity or missing chronology; and
- mirror/offline history views disclose when they are stale or partial
  instead of implying a complete record.

Companion artifacts:

- [`/schemas/security/advisory_timeline_entry.schema.json`](../../schemas/security/advisory_timeline_entry.schema.json)
  - machine boundary for `advisory_timeline_entry_record` and
    `advisory_history_timeline_record`.
- [`/fixtures/security/advisory_history_cases/`](../../fixtures/security/advisory_history_cases/)
  - worked examples covering emergency state, resolved downgrade,
    supersedence chains, and mirror-stale history projections.
- [`/docs/security/advisory_surface_contract.md`](./advisory_surface_contract.md)
  and [`/schemas/security/advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json)
  - how advisories render across product UI, docs/help, and exports (this
    contract adds the timeline/history rules those surfaces must honor).
- [`/docs/security/advisory_identity_and_install_assessment_contract.md`](./advisory_identity_and_install_assessment_contract.md)
  plus [`/schemas/security/advisory_identity.schema.json`](../../schemas/security/advisory_identity.schema.json)
  and [`/schemas/security/affected_install_assessment.schema.json`](../../schemas/security/affected_install_assessment.schema.json)
  - stable advisory identity, copy-safe IDs, fixed-version linkage, local
    continuity notes, and mirror freshness.
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  - canonical advisory record and append-only `decision_history[]` that
    timeline entries summarize.
- [`/docs/security/intake_and_triage.md`](./intake_and_triage.md)
  and [`/schemas/security/private_triage_workspace_packet.schema.json`](../../schemas/security/private_triage_workspace_packet.schema.json)
  - `current_mitigation_status` and postmortem linkage reserved on the
    triage packet.
- [`/docs/security/postmortem_and_compensating_control_contract.md`](./postmortem_and_compensating_control_contract.md)
  and [`/schemas/security/postmortem_record.schema.json`](../../schemas/security/postmortem_record.schema.json)
  - signed postmortems and compensating controls referenced by
    `resolved_by_ref` and post-incident history rows.

Normative source alignment:

- `.t2/docs/Aureline_PRD.md` §10.9 — vulnerability disclosure and
  response.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.8 and
  Appendix AS — incident response artifacts, mirror/offline distribution,
  and auditability.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13 and Appendix
  BS — advisory state, supersedence, and durable linkage without re-keying.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` “Security advisory cards,
  emergency notices, and disclosure links” — history and disclosure
  behavior must be explicit and provenance-aware.

If this contract disagrees with `.t2/docs/` sources, the `.t2/docs/`
source wins and this document, schema, and fixtures update together.

## Scope

Frozen at this revision:

- the timeline entry vocabulary: `draft`, `published`, `mitigated`,
  `superseded`, `resolved`, `withdrawn`;
- per-surface retention rules for those entries (product, docs/help,
  support exports, admin exports);
- resolved-state downgrade rules: “step down prominence” MUST NOT become
  disappearance;
- linkage rules that preserve:
  - stable advisory IDs and copy-safe ID rows,
  - fixed build / compensating control references,
  - local continuity notes, and
  - postmortem references
  across the same advisory history chain; and
- mirror/offline history posture: stale or partial snapshots MUST be
  disclosed as such.

Out of scope:

- building a full advisory center UI;
- publishing a public external disclosure website; and
- transport, syncing, or storage implementations for mirrors/offline
  bundles.

## Definitions

- **Advisory identity**: `advisory_identity_record` keyed by
  `advisory_identity.aureline_advisory_id` (stable for the life of the
  advisory).
- **Advisory family / history chain**: the navigable graph created by
  `history.supersedes_*` and `history.superseded_by_*` linkage fields
  across advisories.
- **Timeline entry**: one append-only, export-safe “history row” that
  summarizes a major lifecycle state for one advisory and declares which
  surfaces must retain it.
- **Timeline snapshot**: one renderable history object containing
  timeline entries plus freshness/completeness disclosure for mirror or
  offline contexts.

## Timeline entry classes

Each timeline entry MUST declare its `entry_class`:

- `draft` — a drafted advisory identity/record not yet published.
- `published` — advisory is published in its declared visibility class.
- `mitigated` — mitigation is available/applied/shipped (including
  compensating controls and emergency actions) but the advisory remains
  part of the active response chain.
- `superseded` — this advisory is no longer the “current mitigation”
  advisory because another advisory superseded it.
- `resolved` — the response for this advisory reached mitigation-complete
  (fixed build promoted, compensating control retired, or the finding is
  no longer applicable) and the advisory is downgraded to history.
- `withdrawn` — the advisory was explicitly retracted (duplicate,
  non-security, or erroneous); it remains visible as a withdrawn history
  row (never silently removed).

### Surface retention matrix (minimum)

Timeline entries MUST remain visible on the following surfaces:

| Entry class | Product | Docs/help | Support export | Admin export |
|---|---:|---:|---:|---:|
| `draft` | allowed (restricted) | forbidden | required | required |
| `published` | required | required | required | required |
| `mitigated` | required | required | required | required |
| `superseded` | required (history section) | required | required | required |
| `resolved` | required (history section) | required | required | required |
| `withdrawn` | required (history section) | allowed | required | required |

Rules:

1. “History section” means the product MUST keep the row reachable and
   clearly labeled as not-active; it MUST NOT keep it at emergency
   prominence.
2. “Allowed (restricted)” means the product MAY show draft history to
   authorized reviewers, but MUST NOT widen visibility by doing so.
3. Support/admin exports MUST retain enough timeline to preserve
   chronology even when product UI chooses to collapse/compact history
   rendering.

## Resolved-state downgrade (non-negotiable)

When an advisory transitions out of an active/emergency state:

1. **No silent disappearance.** The advisory MUST remain inspectable
   under the same stable ID family (`aureline_advisory_id`), and copy-safe
   IDs MUST remain copyable subject to the visibility boundary.
2. **Prominence changes, not identity.** The advisory’s emergency/action
   state MAY clear, and surfaces MUST step down prominence (for example:
   remove banner placement), but they MUST NOT re-key, rename, or hide the
   advisory as if it never existed.
3. **Resolution is linkable.** `resolved_at` and `resolved_by_ref` (fixed
   build, postmortem, disable-bundle retirement, compensating control row,
   or release-evidence packet) MUST remain present on resolved history
   rows.

This document does not define UI styling; it defines the invariants the
UI and exports MUST preserve.

## Current-mitigation linkage rules

“Current mitigation” is the live response path a user/operator is
expected to follow *now*. The contract for navigating from current
mitigation to prior advisory history is:

1. **Stable join key.** All history navigation begins from
   `advisory_identity.aureline_advisory_id`. CVE/GHSA are aliases and MUST
   NOT be used as primary keys.
2. **Bidirectional supersedence.** If advisory B supersedes advisory A:
   - advisory B MUST list advisory A in `history.supersedes_*`; and
   - advisory A MUST carry `history.superseded_by_* = B`.
3. **Fixed-version and continuity retention.** Resolved/superseded
   advisories MUST remain able to surface:
   - the fixed build reference (`resolved_by_ref` or fixed build identity
     reference); and
   - the local continuity note (what still works safely while mitigations
     were active).
4. **Postmortem continuity.** When a signed postmortem exists, the
   timeline MUST carry a stable link to it (directly on the timeline
   snapshot and/or via `resolved_by_ref`) so exports and offline bundles
   can bind follow-up ownership to the same advisory chain.

## Mirror/offline stale or partial history disclosure

History views built from mirrors, offline bundles, manual imports, or
local cache MUST disclose:

- `mirror_freshness_class` (up-to-date vs stale/expired/unknown); and
- `history_completeness_class` (complete vs partial/unknown).

Rules:

1. A stale or partial history snapshot MUST NOT be rendered as “complete”
   history. Consumers MUST show a clear label/note indicating missing
   chronology risk.
2. A mirror/offline snapshot MUST NOT imply public-network reachability
   when it does not exist.

## Worked examples

Worked examples live in:
[`/fixtures/security/advisory_history_cases/`](../../fixtures/security/advisory_history_cases/).

They cover:

- active emergency action with timeline rows linking to current mitigation;
- resolved advisory with reduced prominence and durable copy-safe IDs;
- superseded advisory chain with bidirectional linkage; and
- mirror-stale history view disclosing partial/stale data.

## Change control

Adding a new `entry_class`, `history_completeness_class`, or
surface-retention rule is additive-minor and requires an
`advisory_timeline_schema_version` bump plus fixture coverage.

Repurposing an existing value is breaking and requires a governance
decision co-signed by security/trust and release owners. Existing history
rows are never deleted; they are superseded.

