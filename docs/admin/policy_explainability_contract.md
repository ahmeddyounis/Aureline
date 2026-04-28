# Admin policy explainability contract

This contract freezes the vocabulary and record shapes used when
Aureline explains managed or policy-driven behavior inside the product.
It exists so a locked setting, disabled command, endpoint posture card,
policy diff, decision-history row, CLI JSON result, support packet, and
admin handoff all answer the same questions with the same terms:

- what source is in force;
- who owns the current lock or next step;
- whether the source is current, stale, mirrored, offline, revoked, or
  verification-failed;
- which setting, command, action, endpoint capability, retention row, or
  export route is affected;
- what changed from the previous state to the effective state;
- where the user can inspect the matching audit event, retention or
  deletion constraint, and machine-readable handoff packet; and
- how to escalate without copying raw policy bundles or unrelated tenant
  data.

## Companion artifacts

- [`/schemas/admin/effective_policy_card.schema.json`](../../schemas/admin/effective_policy_card.schema.json)
  - machine-readable boundary for effective-policy cards, endpoint
  posture cards, policy diff views, decision-history rows,
  locked-state explanations, and admin/support handoff exports.
- [`/fixtures/admin/policy_history_rows/`](../../fixtures/admin/policy_history_rows/)
  - worked decision-history row examples that preserve previous and
  effective state, source provenance, redaction, and deep-link targets.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  - upstream signed policy-bundle vocabulary for source class,
  freshness, validation, epoch, and local explainability.
- [`/schemas/identity/admin_audit_packet.schema.json`](../../schemas/identity/admin_audit_packet.schema.json)
  - upstream audit/export packet vocabulary for actor, target, tenant
  scope, and admin event classes.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  and [`/schemas/policy/`](../../schemas/policy/)
  - local `aureline.policy.json` artifact and signed bundle-cache
  vocabulary for precedence, last-known-good, emergency-disable,
  refresh-history, and local reconstruction evidence.
- [`/schemas/settings/effective_setting.schema.json`](../../schemas/settings/effective_setting.schema.json)
  - upstream effective-setting and lock vocabulary consumed by settings
  rows and by "why is this locked" explanations.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  - retention, deletion, export, hold, chronology, and policy
  simulation vocabulary reused by the retention/deletion matrix links.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  - support-bundle packet vocabulary reused by admin/support handoff
  exports.

Normative product sources for this contract are the enterprise policy
requirements, the admin-plane architecture, the effective policy and
audit center UX, the admin/policy center layout, and the policy-diff
templates in `.t2/docs/`. If this document disagrees with those
sources, the `.t2/docs/` source wins and this contract must be updated
in the same change.

## Scope

Frozen at this revision:

- one shared terminology set for policy source, owner, source
  freshness, validation, effective state, lock target, lock reason,
  decision class, endpoint posture, deep link, action, export format,
  and redaction summary;
- one strict JSON Schema file that can represent:
  - `effective_policy_card_record`;
  - `endpoint_posture_card_record`;
  - `policy_diff_view_record`;
  - `policy_decision_history_row_record`;
  - `locked_state_explanation_record`;
  - `admin_policy_handoff_export_record`;
- decision-history fixtures that show state transitions and provenance
  rather than anonymous event dumps;
- deep-link rules between policy cards, lock explanations, audit event
  explorer rows, retention/deletion matrix rows, and handoff exports;
  and
- export behavior for paired human-readable and machine-readable
  handoff.

Out of scope:

- implementing a policy evaluator;
- implementing a full admin console;
- authoring or signing policy bundles;
- retaining raw audit trails from external providers; and
- creating a hosted-console-only explanation path.

## Shared terms

Every admin-policy surface MUST preserve these fields by name when it
renders, logs, exports, or emits CLI output:

| Field | Meaning |
|---|---|
| `source` | The effective source bundle or local derived explanation. Carries `policy_bundle_ref`, `bundle_version`, `policy_epoch`, `policy_source_class`, source label, scope, freshness, validation state, refresh time, expiry, and stale/offline note. |
| `owner_class` | Who controls the next step: end user, workspace owner, organization admin, managed admin, extension publisher, release/channel policy, security emergency policy, support operator, system resolver, or unresolved owner. |
| `effective_state` | Product-facing state: current authoritative, current mirrored, stale within grace, stale past grace, offline last-known-good, offline unverified, partially managed, revoked, verification failed, or unmanaged local. |
| `affected_target` / `target` | The setting, command, action, feature area, endpoint capability, provider route, retention/deletion path, support export, extension install, or update channel being explained. |
| `lock_state` / `lock_reason` | Lock or constraint state and the typed reason behind it. Disabled controls MUST NOT collapse policy locks into generic disabled styling. |
| `previous_state` / `effective_state` in diffs and history | Redaction-aware projections of before and after. Raw values are not embedded when they are secret-bearing or irrelevant to the current tenant/scope. |
| `deep_links` | Stable route refs into effective card, diff view, decision row, lock explanation, audit event explorer, retention/deletion matrix, handoff export, settings detail, command diagnostics, support packet, CLI JSON, or docs anchor. |
| `redaction_summary` | The included and omitted data classes plus the redaction class applied to the record. |

The schema intentionally mirrors source/freshness/validation vocabulary
from the signed policy-bundle schema and actor/target vocabulary from
the admin-audit packet schema. New surfaces SHOULD import these terms
instead of minting near-synonyms.

## Effective-policy card

An `effective_policy_card_record` is the product-facing summary of the
current enforced state for a policy area or bundle. It MUST include:

- source bundle, version, epoch, owner class, scope, freshness, and
  validation state;
- effective state;
- endpoint posture summary;
- every locked or constrained target relevant to the card;
- at least one action that either opens details, exports a handoff, or
  explains why no mutation is available;
- deep links to audit, retention/deletion, and handoff routes when those
  lifecycle constraints are relevant; and
- a redaction summary.

The card MAY be rendered in a compact settings row, policy center card,
endpoint posture page, Project Doctor finding, or CLI table, but the
underlying fields remain the same. A compact surface can hide fields
behind "details", but it cannot drop the source, owner, freshness, lock
reason, or escalation path.

## Endpoint posture card

An `endpoint_posture_card_record` summarizes machine and fleet-relevant
state. It MUST preserve:

- install mode;
- update ring;
- mirror posture;
- trust-root state;
- identity status;
- bundle freshness class;
- diagnostics actions; and
- a reviewable posture summary.

Private mirrors for updates, extensions, docs packs, model packs, and
policy bundles are visible through this surface by stable refs and
reviewable labels. Raw mirror URLs, raw hostnames, private tenant names,
and raw signing material are forbidden.

## Policy diff view

A `policy_diff_view_record` compares a baseline source and effective
source. Each diff row MUST carry:

- affected target;
- previous state projection;
- effective state projection;
- user-visible consequence;
- redaction summary;
- audit event refs; and
- retention/deletion matrix refs when the diff changes lifecycle,
  export, hold, or delete posture.

Diffs MUST compare previous and effective state without exposing raw
secrets, raw policy rule bodies, raw tenant-private data, or unrelated
policy content. Values use `value_projection.projection_class` so a
reader can distinguish plain summaries from redacted shape, class label,
or omitted irrelevant scope.

## Decision-history row

A `policy_decision_history_row_record` is the durable row a recent
decision timeline renders. It MUST distinguish:

- decision code;
- decision class and outcome;
- actor class;
- owner class;
- affected target;
- previous state;
- effective state;
- source bundle;
- provenance chain;
- explanation ref;
- policy diff ref;
- audit event ref;
- retention/deletion matrix refs;
- handoff export refs;
- deep links; and
- next escalation action.

The row is not an anonymous event dump. It preserves how the decision
was reached: bundle observed, mirror sync, admin action, policy
evaluation, client limitation, provider limitation, user action,
emergency action, retention lifecycle event, and final resolver step are
separate provenance steps.

## Why-is-this-locked explanation

A `locked_state_explanation_record` answers the contextual question
"why can I not change this?". Every locked or forced control in
settings, command palette, actions, endpoint posture, support export,
or retention/deletion views MUST be able to open one.

The explanation MUST include:

- target;
- source bundle;
- owner class;
- current scope;
- effective state;
- lock state;
- lock reason;
- reviewable explanation summary;
- stale/offline note;
- next escalation action;
- related command ids;
- related docs refs;
- deep links; and
- redaction summary.

If the source is stale or offline, the explanation still renders. It
must say whether the current behavior is last-known-good, paused until
refresh, local-only continuation, or blocked because the source is past
grace or failed verification.

## Handoff export

Every export path MUST offer a pair:

- a human-readable summary suitable for a user, admin, or support
  engineer; and
- a machine-readable packet with schema ref, source build/channel
  context when available, policy source, endpoint posture, affected
  targets, decision rows, redaction summary, and compatible consumer
  classes.

Machine-readable output MUST stay clean. CLI or headless JSON cannot be
polluted by progress text, instructional copy, screenshots, or rendered
Markdown. Human-readable context belongs in the paired summary.

Exports MUST NOT include whole policy bundles, raw policy rule bodies,
raw signatures, raw tenant directory payloads, raw provider payloads,
raw URLs/hostnames, raw user identifiers, raw command lines, raw paths,
raw tokens, or secret material. They may include opaque refs,
fingerprints, source labels, schema refs, redaction notes, and
reviewable summaries.

## Deep-link and reuse rules

1. Effective-policy cards and lock explanations MUST link to a decision
   history row or audit event whenever they describe a material change
   in trust, permission, provider routing, collaboration control,
   publish state, managed identity scope, retention, or deletion.
2. A policy diff that affects data lifecycle MUST link to the
   retention/deletion matrix row that explains what remains, where it
   remains, and when delete or export can complete.
3. A decision-history row MUST link to the exact explanation and diff
   that a user would see from the affected control.
4. A handoff export MUST link back to the source card, diff, history
   rows, lock explanations, and lifecycle matrix rows it summarizes.
5. Deep links are stable route refs, not browser-only console URLs.
   They must work in desktop, CLI-support projection, self-hosted,
   mirrored, offline, or partially managed contexts whenever the
   referenced record is locally available.
6. If a referenced record is unavailable offline, the link still renders
   with the unavailable reason and the next safe action.

## Offline, mirrored, and partially managed states

The policy center remains useful without a live hosted console:

- `current_mirrored` and `mirrored_current` render as current but still
  identify the mirror source and last successful sync.
- `stale_within_grace` may continue existing narrowed behavior, but new
  managed privilege must not appear from the stale source.
- `stale_past_grace`, `offline_unverified`, `verification_failed`, and
  `revoked` fail closed with a visible explanation and repair or
  escalation action.
- `offline_last_known_good` must cite the bundle or snapshot used as the
  fallback and must show which managed actions are paused.
- `partially_managed` must identify which targets are governed and which
  remain local or user-owned; it cannot imply fleet control over
  unmanaged areas.

## Fixture coverage

The seeded decision-history fixtures cover:

- a current managed policy that constrains an AI provider setting;
- an offline mirror using last-known-good policy while pausing a remote
  action; and
- a retention/deletion policy decision where a delete path is blocked by
  a hold and linked into the lifecycle matrix.

Each fixture carries previous/effective state, source provenance, audit
links, retention/deletion links where relevant, handoff export refs, and
redaction summaries. They are examples of the product contract, not
policy evaluator output.

## Change management

Adding a new enum value, optional property, or additive record kind is
additive-minor and bumps
`admin_policy_explainability_schema_version`. Renaming or repurposing an
existing value is breaking and requires a governance decision because it
would change the meaning of existing support packets, CLI output,
decision-history rows, and admin handoff exports.
