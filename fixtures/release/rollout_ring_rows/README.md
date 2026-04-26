# Rollout-row fixtures

These fixtures exercise the `rollout_ring_row_record` branch of
[`schemas/release/install_row.schema.json`](../../../schemas/release/install_row.schema.json).
They keep rollout labels, owners, promotion state, rollback state, and
preserved evidence links explicit for fleet and release-center views.

The files distinguish deployment-exposure rows (`canary`, `pilot`,
`broad`) from channel-population rows (`stable`, `preview`, `beta`) and
the long-support population row (`lts`).
