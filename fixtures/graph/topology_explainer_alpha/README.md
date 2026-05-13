# Graph topology explainer alpha fixtures

This directory holds the protected alpha fixture for the first
runtime impact explainer packet produced by `crates/aureline-graph`.

The fixture does not redefine graph identity. It expects the runtime
packet to reuse canonical graph node ids, graph edge ids, freshness,
confidence, and evidence-state vocabulary from the graph seed and the
alpha query-family envelope. The packet must also carry a non-canvas
table fallback that uses the same node and edge ids as the visual
projection.

