# AI Review Assist And Publish Truth

- Packet: `ai-review-assist-truth:stable:0001`
- Review workspace: `review-workspace:serializer-pr:1842`
- Review-pack digest: `sha256:2222222222222222222222222222222222222222222222222222222222222222`
- Scope selectors: 3
- Findings: 3 (1 outdated)
- Publish sheets: 2 (1 provider, 1 fallback)
- Resolution rows: 3
- Consumer projections: 4

This packet preserves selected diff, uncommitted changes, and hosted review scopes with explicit base/head identity. Provider publication is only admitted when provider write access is present; missing provider write access downgrades to local, copy, or export actions.
