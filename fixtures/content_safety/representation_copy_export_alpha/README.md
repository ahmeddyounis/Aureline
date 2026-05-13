# Representation copy/export alpha fixtures

These fixtures validate representation-labeled copy and export behavior across
diff, review, search, and package/install review surfaces.

The protected case feeds one packet into `aureline-content-safety` and expects
the validator to prove:

- each surface has a raw or plain-text safe default copy action;
- rendered, context-bearing, and export-packet variants are explicitly typed;
- sensitive values require a preview before clipboard commit;
- every copy/export action mints a shell interaction-safety
  `copy_export_representation_record`; and
- at least one reconciliation group spans multiple surfaces with
  representation, target-boundary, provenance, and recovery affordances intact.
