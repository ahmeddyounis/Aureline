# Stable affordance mapping contract: Copy IDs, open schema/docs, run migration checks, and support pivots

Many Aureline surfaces communicate critical truth in prose: deprecation
copy, migration guidance, diagnostics, policy denials, and support
instructions. Prose alone is not a stable contract surface. This
document freezes the **stable follow‑up affordances** that let a user,
support engineer, or automation tool pivot from prose to contractual,
stable identifiers.

The contract is normative. Where it disagrees with upstream sources it
cites (PRD/TAD/TDD/UI/UX spec/ADRs), the upstream sources win and this
document plus its companion artifacts MUST update in the same change.

## Companion artifacts

- [`/schemas/ux/stable_affordance_row.schema.json`](../../schemas/ux/stable_affordance_row.schema.json)
  — boundary schema for `stable_affordance_row_record`.
- [`/artifacts/ux/stable_affordance_mappings.yaml`](../../artifacts/ux/stable_affordance_mappings.yaml)
  — the seed mapping catalog surfaces use to stay consistent.
- [`/fixtures/ux/stable_affordance_cases/`](../../fixtures/ux/stable_affordance_cases/)
  — worked cases demonstrating both available and unavailable mappings.

This contract composes with (and does not replace):

- [`/docs/commands/shareability_and_automation_contract.md`](../commands/shareability_and_automation_contract.md)
  — the governed copy‑form vocabulary for commands (copy id, copy CLI
  skeleton, deep links, docs anchors) and its no‑bypass rule set.
- [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  — progressive disclosure and the requirement that higher‑depth
  inspection paths resolve into registered commands rather than prose.
- [`/docs/ux/disabled_reason_grammar.md`](./disabled_reason_grammar.md)
  — stable “why unavailable” grammar and alternate‑route rules.
- [`/docs/migration/post_import_validation_contract.md`](../migration/post_import_validation_contract.md)
  — migration report decision states and explicit available actions.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and [`/docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md)
  — support packet families, redaction posture, and handoff rules.

## Scope

This contract applies to any surface that is stable, launch‑bearing, or
support‑relevant and can otherwise force guesswork:

- command results and “why unavailable” explainers;
- settings rows, schema/manifest validation findings, and deprecation notices;
- migration reports and migration‑bridge cards;
- diagnostics / doctor findings and policy denials;
- docs/help surfaces that present actionable guidance;
- support center cards and support exports.

Out of scope: implementing every mapped command or building every docs/help
surface. This contract freezes the **affordance inventory and mapping
expectations** so future implementations converge.

## Definitions

### “Stable affordance”

A **stable affordance** is an explicit follow‑up action that yields a
contract‑grade reference (a stable id, a schema ref, a docs anchor, or a
support packet ref) rather than “click here” prose. Stable affordances
exist so a user can share *what they are seeing* without screenshots.

### “Mapping”

A **mapping** is the association between:

1. a surface and subject (command, setting, schema/packet, diagnostic, policy, migration scope); and
2. the stable affordance slots that surface exposes for that subject.

Mappings are published as `stable_affordance_row_record` rows so desktop
UI, CLI/headless output, docs/help, migration reports, and support
exports can stay consistent.

## Stable affordance slots

Surfaces that claim a mapping MUST treat these slots as an explicit
inventory: a slot is either **available** (and resolves) or **disabled**
with a typed reason. Silent omission is non‑conforming when the slot is
relevant to the surface class.

Slots frozen by this contract:

| Slot | Purpose | Contract surface it resolves to |
|---|---|---|
| `copy_command_id` | Copy the canonical command id when the subject is command-backed. | Command registry / shareability metadata |
| `copy_cli_equivalent` | Copy a non-executable CLI/headless skeleton or canonical verb when one exists. | CLI help + shareability metadata |
| `open_schema_docs` | Open the governing schema and/or docs/help anchor for the subject. | Schema file + docs/help anchor |
| `run_migration_check` | Run the relevant migration/compatibility check and land on a machine-readable report. | Migration report / compatibility packet |
| `open_support_packet` | Open or generate the relevant support packet path for the subject. | Support bundle / object handoff packet |
| `copy_issue_or_advisory_ref` | Copy the stable issue/advisory reference when guidance is tied to a published advisory or tracked incident. | Advisory/incident record identity |

### Slot semantics

1. **Copy slots never copy local widget ids.** “Copy command id” and “copy
   CLI equivalent” must never copy row indices, DOM ids, local selection
   handles, or screenshot‑dependent tokens.
2. **Copy CLI equivalent never implies pre-approval.** When present, it
   is a skeleton or canonical verb that re-resolves through enablement,
   trust, policy, preview, and approval gates on paste.
3. **Open schema/docs is explicit about what exists.** If only schema or
   only docs exist, the surface must state which is available.
4. **Migration checks land on machine-readable output.** The follow-up
   route must produce a stable report id and schema version rather than
   an ephemeral toast.
5. **Support packets preserve redaction posture.** Support paths must
   clearly disclose whether the result is metadata-only, redacted by
   policy, or requires an explicit export review.

## Explicit degradation (fallback behavior)

When a slot is unavailable for the current profile, client scope, or
policy state, the surface MUST degrade explicitly:

- The slot remains present in the inventory.
- The slot is disabled with a short reason (policy blocked, UI-only,
  mapping unavailable, offline/degraded, scope excluded).
- The slot provides a contract-link fallback (at minimum, this document)
  so the user never dead-ends on “missing action” ambiguity.

Forbidden behaviors:

- silently hiding a slot that is expected on the surface class;
- rendering a slot that looks enabled but does not resolve (dead link);
- implying a hidden CLI command exists when the mapping is absent;
- replacing a missing mapping with prose that can drift per surface.

## Cross-surface stability invariants

Where the mapping exists, these stable identifiers MUST survive across
desktop UI, CLI/headless, docs/help, migration reports, diagnostics, and
support exports:

- **Command identity**: `cmd:…` command ids match across palette, menus,
  result cards, CLI help, docs references, and support exports.
- **CLI identity**: canonical verbs remain identical across CLI/help,
  docs, and copy affordances.
- **Schema identity**: schema refs are repo-stable (and optionally
  accompanied by a schema URI) and never replaced with “latest schema”
  free text.
- **Support/advisory identity**: issue/advisory refs remain copyable and
  stable wherever the product references them.

## Fixture expectations

The fixture corpus under `fixtures/ux/stable_affordance_cases/` MUST
demonstrate:

- a command-backed surface with `copy_command_id` and `copy_cli_equivalent`;
- a UI-only subject where `copy_cli_equivalent` is disabled explicitly;
- a deprecation/migration surface with `run_migration_check` and
  `open_schema_docs`;
- a diagnostic/policy denial surface with an `open_support_packet` route;
- an advisory/issue-linked surface with `copy_issue_or_advisory_ref`; and
- explicit degradation that never silently drops a relevant slot.

