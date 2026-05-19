# Runtime authority conformance report

Release-evidence roll-up for the runtime-authority issuer-boundary
conformance lane. The report consumes the seeded fixture corpus under
[`/fixtures/security/m3/runtime_authority_issuer/`](../../../fixtures/security/m3/runtime_authority_issuer/)
and the privileged-action lineage corpus under
[`/fixtures/security/m3/privileged_action_lineage/`](../../../fixtures/security/m3/privileged_action_lineage/)
and reports a red/green disposition that release and security review can use
to block any beta row still allowing self-authorization or ambiguous
privileged lineage.

The source projection is owned by
[`/crates/aureline-policy/src/runtime_authority_issuers/mod.rs`](../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs).
The reviewer-facing landing page is
[`/docs/security/m3/runtime_authority_issuer_boundaries.md`](../../../docs/security/m3/runtime_authority_issuer_boundaries.md)
and the lineage packet doc is
[`/artifacts/security/m3/runtime_authority_lineage_packet.md`](runtime_authority_lineage_packet.md).

## Disposition

**GREEN** — the seeded `seeded_runtime_authority_issuer_page()` produces a page
that validates with zero defects, covers every required requesting-surface
class and rejection reason, and projects a privileged-action lineage packet
that preserves issuer chain, actor class, target class, approval refs, and
outcome while excluding raw credentials, projected-secret contents, raw
authority bodies, and hidden delegation artifacts.

The validator runs in
[`crates/aureline-policy::audit_runtime_authority_issuer_page`](../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs)
and is exercised by the fixture-replay integration test in
[`crates/aureline-policy/tests/runtime_authority_issuer_cases.rs`](../../../crates/aureline-policy/tests/runtime_authority_issuer_cases.rs).
The integration test parses the JSON fixtures emitted under
`fixtures/security/m3/runtime_authority_issuer/` and asserts they round-trip
through the projection without drift.

## Green: high-risk flow coverage

| Flow class | Path proven | Decision id |
| --- | --- | --- |
| Local mutation | CLI script renews a remembered local-format rule via policy service | `decision:cli-script:remembered-format:renewed:0002` |
| External provider mutation (granted) | AI tool plan asks the shell to mint a provider-publish ticket | `decision:ai-tool:provider-publish:granted:0001` |
| External provider mutation (refused, ambient) | Browser companion infers ambient privilege from host session | `decision:browser-companion:ambient:refused:0004` |
| Credential projection (granted) | Admin console projects a session-only handle via supervisor | `decision:admin-console:credential-projection:granted:0009` |
| Credential projection (refused, self-auth) | Extension tries to project a credential without an issuer | `decision:extension:self-authorize:refused:0003` |
| Privileged debug attach | Local admin steps up at the shell to attach to an editor process | `decision:local-admin:debug-attach:granted:0008` |
| Root-of-authority / admin change | Admin console rotates trust root with a signed root-authority proof | `decision:admin-console:root-rotation:granted:0007` |
| Browser companion request path | Refused with `ambient_privilege_inferred` | `decision:browser-companion:ambient:refused:0004` |
| Remote helper request path | Refused with authority-source mismatch and unreachable target | `decision:remote-helper:local-only-provider:refused:0006` |
| Recipe / replay-from-remembered | Refused with remembered-decision-too-broad and target-drift | `decision:recipe:broaden-remembered:refused:0005` |

Every admit path keeps `local_editing_preserved=true`. Every refusal preserves
both `local_editing_preserved` and `reprompt_required`, and carries one or more
closed rejection-reason tokens visible to UI, CLI, support, and audit.

## Green: structured rejection reasons present on the page

| Rejection reason token | Decision id |
| --- | --- |
| `self_authorization_by_non_issuer` | `decision:extension:self-authorize:refused:0003` |
| `missing_issuer_binding` | `decision:extension:self-authorize:refused:0003` |
| `ambient_privilege_inferred` | `decision:browser-companion:ambient:refused:0004` |
| `remembered_decision_too_broad` | `decision:recipe:broaden-remembered:refused:0005` |
| `remembered_decision_target_drift` | `decision:recipe:broaden-remembered:refused:0005` |
| `authority_source_mismatch` | `decision:remote-helper:local-only-provider:refused:0006` |
| `authority_source_unreachable_target` | `decision:remote-helper:local-only-provider:refused:0006` |

The closed reasons `issuer_not_allowed_for_surface`,
`remembered_decision_missing`, `remembered_decision_lifetime_exceeds_budget`,
`remembered_decision_forbidden_class`, `remembered_decision_actor_drift`,
`policy_epoch_drift`, `sandbox_binding_drift`, and
`root_authority_proof_missing` are part of the vocabulary surfaced by the
projection. They are not represented on the seeded green page (no real-world
seeded scenario triggers them), but the validator flips to a typed defect
whenever the corresponding invariant is violated; see the drill matrix below
for the corresponding red fixtures.

## Red: drift / replay / forgery drills

Each drill below was emitted with
`cargo run -q -p aureline-policy --example dump_runtime_authority_issuer_fixtures -- <drill>`.
The resulting page is **expected to fail validation**, and the validator must
surface the listed defect kind(s). The conformance fixture corpus checks in
the JSON output of each drill so reviewers can diff future changes against the
red baseline.

| Drift / replay class | Drill fixture | Expected defect kinds |
| --- | --- | --- |
| Non-issuer mint attempt admitted | `drill_admitted_self_authorization.json` | `admitted_self_authorization`, `decision_admitted_without_chain` |
| Ambient privilege admitted | `drill_admitted_ambient_privilege.json` | `admitted_ambient_privilege` |
| Remembered rule admit on drifted target | `drill_admitted_broadened_remembered_rule.json` | `decision_admitted_on_source_mismatch` |
| Remembered rule scope broadened | `drill_remembered_rule_broadened_scope.json` | `remembered_rule_not_narrow` |
| Remembered rule renewal lifetime past ticket budget | `drill_remembered_rule_lifetime_exceeds_budget.json` | `remembered_rule_lifetime_exceeds_budget` |
| Remembered rule attached to forbidden class | `drill_remembered_rule_forbidden_class.json` | `remembered_rule_forbidden_class` |
| Shell issuer claims root-authority mint | `drill_shell_root_authority_overreach.json` | `unauthorized_root_authority_claim`, `issuer_overreach` |
| Refused decision drops reason | `drill_refused_without_reason.json` | `refused_decision_missing_reason` |
| Refused decision drops local-editing / reprompt | `drill_refused_without_recovery_guidance.json` | `decision_dropped_recovery_guidance` |
| Local-only authority admitted to provider | `drill_local_only_admitted_to_provider.json` | `decision_admitted_on_source_mismatch` |
| Remembered renewal past rule expiry | `drill_admitted_beyond_rule_expiry.json` | `decision_admitted_beyond_rule_expiry` |
| Admin / root change admitted without signed proof | `drill_admitted_without_root_proof.json` | `decision_admitted_without_chain` |

Several drills also surface `rejection_reason_coverage_missing` as a
side-effect because the mutation clears a reason that the green coverage
invariant expected to see. That is intentional: the validator does not let a
page silently lose a structured rejection reason that was previously proving a
class of fail-closed behavior.

## Drift invalidation consistency

The drill matrix above proves that drift invalidation is enforced uniformly
between the in-product evaluator, the CLI/headless inspectors that reuse the
same `aureline_policy::runtime_authority_issuers` module, and the
release-evidence packets exported under
[`/artifacts/security/m3/`](../../security/m3/). The same rejection-reason
tokens and defect kinds appear in:

- the UI string surfaced by the shell prompt (the `explanation` field on each
  decision is identical across product, CLI, support export, and audit);
- the lineage packet rows (`lineage_row.explanation` is the same string and the
  closed `rejection_reason_tokens` list mirrors the page); and
- the support-export wrapper consumed by support packets.

No surface gets a softened explanation. Reviewers can diff the JSON `explanation`
field across product UI snapshots, CLI captures, and lineage rows and expect
exact equality.

## Support-export safety

The lineage packet projection
([`RuntimeAuthorityLineagePacket::from_page`](../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs))
sets `raw_credentials_excluded: true` and
`provider_versus_local_distinguished: true` by construction, and only carries:

- decision id, request id, requesting-surface class token, opaque
  requesting-surface ref;
- issuer class token, requested ticket class token, actor class token,
  authority-source class token;
- decision class token, closed `rejection_reason_tokens`;
- optional minted authority ticket ref (opaque), optional renewed-from rule id
  (opaque);
- export-safe `explanation`, decision timestamp, and opaque `audit_event_refs`.

The following are excluded by construction (the type system does not even
carry them):

- raw credentials, projected-secret contents, plaintext secret material;
- raw signed authority bodies (policy bundles, trust-root rotation blobs,
  admin command payloads);
- raw policy or evaluator payloads beyond the policy-epoch ref;
- raw sandbox-profile or capability-envelope contents beyond their opaque ref
  and fingerprint ref;
- raw provider response bodies; and
- hidden delegation artifacts.

The shipped lineage packet is reviewable in
[`/fixtures/security/m3/privileged_action_lineage/lineage_packet.json`](../../../fixtures/security/m3/privileged_action_lineage/lineage_packet.json)
and the per-row excerpts are usable as drop-in support-export samples.

## How to reproduce this report

```sh
# Re-emit the conformance fixture corpus.
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

# Validate that the green page validates and the fixtures match the projection.
cargo test -p aureline-policy --test runtime_authority_issuer_cases
```

A green disposition requires the fixture-replay test to pass and the drill
defect tokens listed in the table above to remain stable. Any change that
softens an admit path, removes a closed rejection reason, or lets a drill flip
to green must update this report, the lineage packet doc, and the reviewer-
facing landing page in the same change.
