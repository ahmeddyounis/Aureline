# AI Route and Spend Truth

- Packet: `ai-route-spend-truth:stable:0001`
- Action id: `ai-action:route-spend:stable:0001`
- Evidence id: `ai-evidence:route-spend:stable:0001`
- Flow: `review` (material: true)
- Intended route: `managed` / `Aureline managed hosted AI` (`Hosted review model`)
- Policy and packs: `routing-policy:stable-review:v4` / `prompt-pack:review-stable:v7` / `tool-schema-pack:docs-connector:v3` (range `tool-schema-range:docs-connector:v3`)
- Actual route: `managed` (outcome `completed_clean`, cost `vendor_hosted_entitlement_band` / `actual_measured`)
- Decision cause: `policy_admitted`; lever `deny-lever:route-policy:stable-review:v4`; fallback chain `fallback:managed-review|fallback:manual-review-checklist`
- Downgrade: false (cause `no_route_change`, both routes preserved: true)
- Quota families: 5 rows (0 blocked this action)
- Non-AI fallback available: true (`Open the manual review checklist`)
