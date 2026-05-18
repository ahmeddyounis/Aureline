# Resource Governor And Queue Truth Fixtures

These fixtures exercise the beta resource-governor truth packet and shell
projection. They focus on pressure visibility, lane-state honesty, checkpoint
and collapse metadata, override explanations, protected foreground continuity,
and support-export parity.

The fixtures are expectation records rather than raw scheduler traces. Runtime
tests replay them through `seeded_resource_governor_snapshot` and assert that
the code-level packet exposes the same vocabulary and invariants.
