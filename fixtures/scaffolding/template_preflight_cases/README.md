# Template-card, generation-preflight, and template-health worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/scaffolding/template_health_and_preflight_contract.md`](../../../docs/scaffolding/template_health_and_preflight_contract.md)
and the schemas at
[`/schemas/scaffolding/template_card.schema.json`](../../../schemas/scaffolding/template_card.schema.json)
and
[`/schemas/scaffolding/generation_preflight.schema.json`](../../../schemas/scaffolding/generation_preflight.schema.json).
The closed `template_health_state_class` matrix lives at
[`/artifacts/scaffolding/template_health_states.yaml`](../../../artifacts/scaffolding/template_health_states.yaml).

Every file is a single YAML document carrying a `__fixture__`
prelude summarising the scenario, the contract sections it
exercises, and the acceptance bullets it backs. The runtime
payload conforms to one of these shapes:

- `template_card_record`
- `generation_preflight_record`

No fixture embeds raw bearer tokens, raw API keys, raw
passwords, raw signing keys, raw certificate / key material,
raw absolute filesystem paths, raw repository URLs, raw author
email addresses, raw container registry URLs, raw devcontainer
image tags, raw lockfile bodies, raw extension marketplace
urls, raw stdout / stderr, raw post-install script bodies, or
raw user-supplied parameter values. Every such field is an
opaque ref into a per-classification registry, an integer-
bucket count, a typed enum value, or a redaction-aware
reviewable sentence.

## Cases

### Template-card cases (acceptance bullets 1, 3)

- [`template_card_first_party_fresh.yaml`](./template_card_first_party_fresh.yaml)
  — First-party Rust CLI starter card with
  `template_health_state_class = fresh` and
  `card_disposition_class = card_admissible_for_generation`.
  All five `side_effect_summary` classes are typed and the
  bypass list (open-folder + create-empty-workspace) renders at
  equal weight.
- [`template_card_validation_stale_bypass_intact.yaml`](./template_card_validation_stale_bypass_intact.yaml)
  — Same card after the freshness check returned
  `stale_or_invalid`. `template_health_state_class =
  validation_stale`; the card remains admissible and the bypass
  routes (folder, workspace, clone, create-empty) render at
  equal weight regardless of the stale signal.
- [`template_card_known_issue_blocking.yaml`](./template_card_known_issue_blocking.yaml)
  — Team-managed Go backend-service card whose
  `known_issue_disclosure_class =
  known_issue_active_blocking_user_review_required` forces
  `template_health_state_class =
  known_issue_active_blocking_user_review_required` and
  `card_disposition_class =
  card_visible_disabled_known_issue_blocking`. Bypass routes
  remain at equal weight.

### Generation-preflight cases (acceptance bullets 2, 3)

- [`preflight_first_party_local_apply_admitted.yaml`](./preflight_first_party_local_apply_admitted.yaml)
  — Preflight against the first-party local Rust CLI card.
  Enumerates the seven required preflight axes
  (parameter / environment / file-write / dependency-impact /
  execution-step / immediate-vs-deferred / checkpoint /
  delete-recovery / bypass) so generation never collapses into a
  generic Create chip. `preflight_disposition_class =
  preflight_admissible_apply_admitted`.
- [`preflight_known_issue_workaround_acknowledged.yaml`](./preflight_known_issue_workaround_acknowledged.yaml)
  — Preflight against a card whose health state is
  `known_issue_active_workaround_documented`. Apply is admitted
  only after the user acknowledges the workaround; the
  acknowledgement is recorded as a `pre_write_phase` execution
  step.
- [`preflight_unsupported_template_blocked_bypass_intact.yaml`](./preflight_unsupported_template_blocked_bypass_intact.yaml)
  — Preflight against an unsupported template revision.
  `preflight_disposition_class = preflight_blocked_unsupported`,
  `denial_reason_class =
  template_revision_archived_no_apply_path`. The bypass list
  still names folder, workspace, clone, and create-empty
  routes — health warnings never remove the
  open-without-starter alternative.
- [`preflight_ai_proposed_pending_admission.yaml`](./preflight_ai_proposed_pending_admission.yaml)
  — Preflight whose bound card is AI-tool-proposed and pending
  user admission. `preflight_disposition_class =
  preflight_pending_ai_admission`,
  `ai_tool_proposed_review_ticket_ref` non-null, bypass routes
  preserved.

## Acceptance mapping

| Acceptance bullet | Demonstrating fixtures |
|---|---|
| Users can inspect source/support class, trust, side effects, and template health before generation | `template_card_first_party_fresh.yaml`, `template_card_known_issue_blocking.yaml`, `preflight_known_issue_workaround_acknowledged.yaml`, `preflight_ai_proposed_pending_admission.yaml` |
| Preflights enumerate write/dependency/execution impact instead of collapsing them into a generic Create action | `preflight_first_party_local_apply_admitted.yaml`, `preflight_known_issue_workaround_acknowledged.yaml` |
| Template-health warnings never remove the plain open-without-starter alternative | `template_card_validation_stale_bypass_intact.yaml`, `template_card_known_issue_blocking.yaml`, `preflight_unsupported_template_blocked_bypass_intact.yaml`, `preflight_ai_proposed_pending_admission.yaml` |
