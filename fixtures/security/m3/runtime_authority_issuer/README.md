# Runtime-authority issuer conformance fixtures

Reviewer-facing fixtures for the runtime-authority issuer-boundary projection
owned by
[`/crates/aureline-policy/src/runtime_authority_issuers/mod.rs`](../../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs).
The canonical record kind is
`security_runtime_authority_issuer_page_record`. The schema lives at
[`/schemas/security/runtime_authority_issuer.schema.json`](../../../../schemas/security/runtime_authority_issuer.schema.json)
and the remembered-rule schema at
[`/schemas/security/remembered_decision_rule.schema.json`](../../../../schemas/security/remembered_decision_rule.schema.json).

The reviewer-facing landing page is
[`/docs/security/m3/runtime_authority_issuer_boundaries.md`](../../../../docs/security/m3/runtime_authority_issuer_boundaries.md)
and the release-evidence packet is
[`/artifacts/security/m3/runtime_authority_lineage_packet.md`](../../../../artifacts/security/m3/runtime_authority_lineage_packet.md).

The red/green conformance roll-up is
[`/artifacts/security/m3/runtime_authority_conformance_report.md`](../../../../artifacts/security/m3/runtime_authority_conformance_report.md).

## How to regenerate

The fixtures are pure functions of the seed in
`runtime_authority_issuers::seeded_runtime_authority_issuer_page()`. To rebuild
every file in this directory:

```sh
FX=fixtures/security/m3/runtime_authority_issuer
for arg in page issuers requesting-surfaces remembered-rules requests decisions defects summary lineage-packet \
           drill-admitted-self-authorization drill-admitted-ambient-privilege drill-admitted-broadened-remembered-rule \
           drill-remembered-rule-broadened-scope drill-remembered-rule-lifetime-exceeds-budget \
           drill-remembered-rule-forbidden-class drill-shell-root-authority-overreach \
           drill-refused-without-reason drill-local-only-admitted-to-provider \
           drill-admitted-beyond-rule-expiry drill-admitted-without-root-proof \
           drill-refused-without-recovery-guidance; do
  cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- "$arg" \
    > "$FX/${arg//-/_}.json"
done
```

## Files

| File | Purpose |
| --- | --- |
| `page.json` | Full seeded runtime-authority-issuer page: issuers, requesting surfaces, remembered rules, requests, decisions, defects (empty), summary. |
| `issuers.json` | One record per allowed issuer (`shell`, `policy_service`, `supervisor`) with its closed set of mintable ticket classes, allowed requesting surfaces, and attestable authority sources. |
| `requesting_surfaces.json` | One record per registered requesting surface; each surface routes through one or more issuers and never mints authority itself. |
| `remembered_rules.json` | The narrow remembered-decision rule(s) compiled from prior approvals. |
| `requests.json` | Boundary requests submitted by non-issuer surfaces. Covers AI tool plans, CLI scripts, extensions, browser companions, recipes, remote helpers, admin consoles, and local admin tooling. |
| `decisions.json` | Boundary decisions returned for each request: granted, remembered-decision-narrowed, or refused with closed reason tokens. |
| `defects.json` | Validator defects on the seeded page; empty on the green fixture. |
| `summary.json` | Aggregate summary: counts by decision class, requesting-surface classes present, rejection reasons present, defect counts by kind. |
| `lineage_packet.json` | Metadata-only privileged-action lineage packet projected from the page; raw credentials, raw authority bodies, and plaintext secret material are excluded by construction. |
| `drill_admitted_self_authorization.json` | Drill: the extension self-authorization refusal is forged to `granted`. Surfaces `admitted_self_authorization` and `decision_admitted_without_chain`. |
| `drill_admitted_ambient_privilege.json` | Drill: the browser-companion ambient-privilege refusal is forged to `granted`. Surfaces `admitted_ambient_privilege`. |
| `drill_admitted_broadened_remembered_rule.json` | Drill: the recipe broaden refusal is forged to `remembered_decision_narrowed` against a different target. Surfaces `decision_admitted_on_source_mismatch`. |
| `drill_remembered_rule_broadened_scope.json` | Drill: the seeded remembered rule has its `scope_ref` cleared. Surfaces `remembered_rule_not_narrow`. |
| `drill_remembered_rule_lifetime_exceeds_budget.json` | Drill: the remembered rule's renewable lifetime is pushed past the `local_mutation` ticket-class budget. Surfaces `remembered_rule_lifetime_exceeds_budget`. |
| `drill_remembered_rule_forbidden_class.json` | Drill: the remembered rule is moved onto a `credential_projection` ticket class. Surfaces `remembered_rule_forbidden_class`. |
| `drill_shell_root_authority_overreach.json` | Drill: the shell issuer claims `policy_trust_admin_change` mint and `may_mint_root_authority_changes`. Surfaces `unauthorized_root_authority_claim` and `issuer_overreach`. |
| `drill_refused_without_reason.json` | Drill: a refused decision drops every rejection reason. Surfaces `refused_decision_missing_reason`. |
| `drill_local_only_admitted_to_provider.json` | Drill: the remote-helper local-only refusal is forged to `granted` against a provider target. Surfaces `decision_admitted_on_source_mismatch`. |
| `drill_admitted_beyond_rule_expiry.json` | Drill: the remembered rule expires before the renewal decision timestamp. Surfaces `decision_admitted_beyond_rule_expiry`. |
| `drill_admitted_without_root_proof.json` | Drill: the admin trust-root rotation request drops `root_authority_proof_present`. Surfaces `decision_admitted_without_chain`. |
| `drill_refused_without_recovery_guidance.json` | Drill: a refused decision drops `local_editing_preserved` and `reprompt_required`. Surfaces `decision_dropped_recovery_guidance`. |

## High-risk flow classes covered

The seeded page demonstrates one or more representative scenarios for every
high-risk runtime-authority flow named by the spec:

- **Local mutation** — `decision:cli-script:remembered-format:renewed:0002` (remembered-decision-narrowed via policy service).
- **External provider mutation** — `decision:ai-tool:provider-publish:granted:0001` (granted via shell) and `decision:browser-companion:ambient:refused:0004` (refused, ambient privilege).
- **Credential projection** — `decision:admin-console:credential-projection:granted:0009` (granted via supervisor, broker handle) and `decision:extension:self-authorize:refused:0003` (refused, self-authorization).
- **Privileged debug attach** — `decision:local-admin:debug-attach:granted:0008` (granted via shell after local-admin step-up).
- **Root-of-authority / admin change** — `decision:admin-console:root-rotation:granted:0007` (granted via supervisor with a recorded root-authority proof).
- **Browser companion path** — refused via the ambient-privilege rejection.
- **Remote helper path** — refused via authority-source mismatch and authority-source-unreachable-target.
- **Recipe / replay-from-remembered path** — refused via remembered-decision-too-broad and remembered-decision-target-drift.

## Drift / failure scenarios proven

| Drift / replay class | Drill fixture |
| --- | --- |
| Non-issuer mint attempt | `drill_admitted_self_authorization.json` |
| Ambient-privilege inference | `drill_admitted_ambient_privilege.json` |
| Drifted target | `drill_admitted_broadened_remembered_rule.json`, `drill_remembered_rule_broadened_scope.json` |
| Changed provider scope / source mismatch | `drill_local_only_admitted_to_provider.json` |
| Expired ticket / rule | `drill_admitted_beyond_rule_expiry.json` |
| Changed sandbox profile | `drill_remembered_rule_forbidden_class.json` (forbidden class for the rule's sandbox), `drill_remembered_rule_broadened_scope.json` |
| Changed policy epoch | covered by `drill_admitted_broadened_remembered_rule.json` (renewal must keep policy-epoch and sandbox-profile refs consistent) |
| Stale / replay from remembered decision | `drill_remembered_rule_lifetime_exceeds_budget.json`, `drill_admitted_beyond_rule_expiry.json` |
| Missing signed root proof | `drill_admitted_without_root_proof.json` |
| Issuer overreach | `drill_shell_root_authority_overreach.json` |
| Refused without structured guidance | `drill_refused_without_reason.json`, `drill_refused_without_recovery_guidance.json` |

## Redaction posture (lineage_packet.json)

The lineage packet preserves only metadata-class fields verbatim:

- decision id, request id, requesting-surface class token and opaque surface ref;
- issuer class token mediating the decision;
- requested ticket class token;
- actor class token and authority-source class token;
- decision class token and closed rejection reason tokens;
- optional minted authority ticket ref and renewed-from rule id (opaque);
- export-safe explanation, decision timestamp, and audit-event refs.

The packet explicitly excludes raw credentials, raw authority bodies (signed
policy bundles, trust-root rotation blobs, admin command payloads), raw policy
or evaluator payloads beyond the policy-epoch ref, raw sandbox-profile or
capability-envelope contents beyond their opaque refs, raw provider response
bodies, and plaintext secret material. The `provider_versus_local_distinguished`
flag is `true` because the explicit ticket-class, authority-source-class, and
target-class tokens never collapse local-only and provider-linked actions into a
single bucket.
