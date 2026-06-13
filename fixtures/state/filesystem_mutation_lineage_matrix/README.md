# Filesystem mutation-lineage matrix corpus

Protected fixture corpus for the filesystem identity, watch fidelity,
mutation lineage, and deferred-intent matrix.

Each JSON fixture binds one checked-in scenario to:

- one expected matrix row,
- one root class,
- one path identity class,
- one watch state,
- one save fallback,
- one undo class,
- one corruption state,
- one connectivity state,
- one reconciliation posture, and
- one explicit capability-coverage block.

The corpus exists so later notebook, request workspace, preview,
profiler, provider-draft, infrastructure-overlay, and offline packet
work can map new behaviors to frozen row vocabulary instead of assuming
ordinary editor file semantics apply automatically.

Boundary schema:
[`schemas/state/filesystem_mutation_lineage_matrix.schema.json`](../../../schemas/state/filesystem_mutation_lineage_matrix.schema.json)

Reviewer doc:
[`docs/state/filesystem_mutation_lineage_matrix.md`](../../../docs/state/filesystem_mutation_lineage_matrix.md)

Artifact packet:
[`artifacts/state/filesystem_mutation_lineage_matrix.json`](../../../artifacts/state/filesystem_mutation_lineage_matrix.json)
