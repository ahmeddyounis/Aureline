# AI Spend And Route Receipt Fixtures

## cancelled_branch_agent.json

A receipt catalogue captured after a user cancelled a long-running side-branch
agent, alongside the inline assist and patch review rows that keep the lane
coverage complete.

The inline assist explain run resolves to local at Preview: an unmetered,
no-charge preflight that needs no acknowledgement, a local route with no change,
and a measured bundled band that reconciles within projection.

The patch review managed pass resolves to managed at Stable: its requested model
was deprecated, so the route receipt records a `model_substituted` change with the
precise `model_deprecated` reason, and its measured cost ran above the projected
band — disclosed as `over_projection_disclosed` rather than hidden.

The background branch agent is the focus: the user cancelled it at the implement
checkpoint. Its cumulative cost budget is hard-stop bounded and in its
`warning_raised` state, its three checkpoints are strictly ordered with a
cumulative band that accumulates from low to medium, and its cancellation export
names checkpoint `2`, carries an export-safe receipt readable on the desktop,
CLI, support-export, and diagnostics surfaces, and gives the precise
`user_requested` reason. The run dropped to Beta from a verified
checkpoint-reversible rollback posture.

This demonstrates that every lane carries preflight, route, and spend receipts,
that a route downgrade names a precise reason instead of a generic provider
error, that a measured overrun is disclosed, that a side-branch agent is bounded
by a cumulative ceiling and ordered checkpoints, and that a cancellation is an
export-safe, checkpoint-aware receipt rather than an opaque kill.
