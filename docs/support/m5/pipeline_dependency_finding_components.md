# M5 pipeline, dependency, and finding component support packet

This support packet index points support and incident workflows to the single
certification bundle for reusable M5 pipeline, annotation, dependency,
manifest-diff, and security-finding components.

- Matrix: `artifacts/design/m5-pipeline-dependency-finding-component-matrix.md`
- Release proof: `artifacts/release/m5-pipeline-dependency-finding-proof/proof_packet.json`
- Support export: `artifacts/release/m5-pipeline-dependency-finding-proof/support_export.json`
- Fixtures: `fixtures/ui/m5-pipeline-dependency-finding-components/`

Support exports must preserve the same controlled labels and states as the
primary UI: provider/run identity, trigger, direct/transitive dependency truth,
manifest hooks and constraints, severity, confidence, freshness, suppression,
remediation, and audit actions. A support packet that drops these fields or
falls back to screenshot-only evidence narrows the affected review, package,
health, or companion claim.
