# Alpha Route-Origin Reconstruction

This contract publishes the alpha route/origin vocabulary used by live
action surfaces, support bundles, incident packets, Help/About copy, and
release evidence. It exists so a reviewer can answer the same questions
from an export that the user saw live: where the action originated, what
target it reached, which route was chosen, which traffic origin was used,
which policy admitted or denied it, who acted, when it happened, what the
outcome was, and whether fallback was declared.

## Canonical Artifacts

- Matrix: [`artifacts/routes/alpha_route_origin_matrix.yaml`](../../artifacts/routes/alpha_route_origin_matrix.yaml)
- Transport decision schema: [`schemas/transport/transport_decision_alpha.schema.json`](../../schemas/transport/transport_decision_alpha.schema.json)
- Support reconstruction packet: [`artifacts/support/command_route_reconstruction_alpha.yaml`](../../artifacts/support/command_route_reconstruction_alpha.yaml)
- Protected fixtures: [`fixtures/runtime/route_origin_alpha/`](../../fixtures/runtime/route_origin_alpha/)
- First support consumer: [`crates/aureline-support/src/route_origin_alpha/mod.rs`](../../crates/aureline-support/src/route_origin_alpha/mod.rs)

## Required Packet Fields

Every alpha transport decision record carries these stable fields:

- `origin.origin_scope` and `origin.traffic_origin`
- `target.target_class`, `target.endpoint_class`, and `target.target_identity_ref`
- `route.route_choice`, `route.egress_class`, and `route.route_change_reason_code`
- `policy.policy_source_class`, `policy.policy_source_ref`, and `policy_epoch`
- `actor.actor_class` and `actor.actor_ref`
- `timestamps.planned_at`, `timestamps.decided_at`, and `timestamps.completed_at`
- `decision_result.decision_outcome` and `decision_result.route_truth_state`
- `fallback.fallback_posture`, `declared_to_user`, and `direct_fallback_allowed`

Support bundles project the same fields into
`action_reconstruction_contexts[]`; they do not scrape rendered UI copy.
Raw URLs, hostnames, request/response bodies, command bodies, provider
payloads, cookies, tokens, and private keys remain excluded.

## Alpha Coverage

The protected fixture corpus covers:

- local task/debug route with `local_only` and `no_fallback`;
- provider preflight with `org_approved_direct`;
- browser handoff with `browser_handoff_required` and a route-change reason;
- publish-capable pipeline with `publish_pipeline`;
- declared tunnel exposure with `tunnel_exposed_route`, `tunnel_session_ref`,
  and `tunnel_exposed_public`;
- wrong-target denial with intended and observed target refs;
- wrong-origin denial with expected and observed origin scopes;
- hidden managed relay denial; and
- hidden public fallback denial.

Wrong-target, wrong-origin, hidden-relay, and hidden-fallback cases are
distinct denial states. They must never collapse into a generic transport
failure or appear as ordinary local continuity.

## Support Export Rule

If the route changes, the support packet changes. A managed relay,
browser handoff, publish pipeline, mirror-only fallback, or provider route
must be visible through `route_choice`, `traffic_origin`,
`decision_outcome`, and `fallback_posture` in both live surfaces and
exported reconstruction packets.
