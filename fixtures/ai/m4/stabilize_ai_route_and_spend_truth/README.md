# Stabilize AI Route and Spend Truth

This fixture set exercises the stable AI route and spend truth record owned by
`aureline_ai::stabilize_ai_route_and_spend_truth`. For one material AI action and
one evidence id, the packet binds the preflight estimate card, the live run
strip, the post-run receipt, the distinct quota-family summary, and the
route-downgrade banner — plus the non-AI fallback path, the typed
provider/model/external-tool registry resolution, and the cumulative-spend
posture a visible batch/agent lane must carry to claim the Stable line.

`route_spend_truth_packet.json` covers the clean managed-route case:

- a material `review` action whose preflight estimate card was shown before send
  with an intended `managed` route, a cost band, a latency band, the owning quota
  family flow, and an approval/policy note;
- a live run strip that discloses the current managed route, and a post-run
  receipt with an `actual_measured` cost class plus route and spend receipt refs;
- the typed registry resolution (provider id, model id/version, transport, auth
  mode, retention, region, quota family, execution locus) with the route class
  consistent with a vendor-hosted managed execution locus;
- the five distinct quota families (composer, review, agent/background,
  generation, tool/connector-assisted), each with its budget owner and state;
- a no-downgrade banner with both routes equal and no silent switch;
- a non-AI fallback path that remains reachable without any AI route; and
- the evidence export binding the in-product evidence id to the admin inspector
  and support export refs.

`route_downgrade_packet.json` covers the quota-blocked, route-downgraded case:

- the same action where the owning review flow's BYOK vendor quota is `exhausted`
  and `blocked_this_action`;
- a downgrade banner that preserves both the original `byok` route and the
  current `managed` route, names `fallback_after_quota_exhaustion` as the cause,
  carries a disclosure ref, and asserts no silent switch;
- a post-run receipt with a `completed_with_downgrade` outcome whose actual route
  matches the current downgraded route; and
- the non-AI fallback path that still remains.

Verify the checked packet with:

```sh
cargo test -p aureline-ai stabilize_ai_route_and_spend_truth --no-fail-fast
```

Regenerate the checked artifact, summary, and fixtures after intentional changes
with:

```sh
cargo test -p aureline-ai stabilize_ai_route_and_spend_truth::tests::emit_artifact -- --ignored
```
