# Fixtures: M5 explainer-and-architecture-maps packet

This directory contains fixture metadata for the `m5_explainer_and_architecture_maps_packet`.

The canonical packet is checked in at:

`artifacts/graph/m5/m5-explainer-and-architecture-maps.json`

and validated by the typed model in the `aureline-graph` crate
(`m5_explainer_and_architecture_maps`) and the JSON Schema at
`schemas/graph/m5-explainer-and-architecture-maps.schema.json`.

## Coverage

- **Distinct source classes.** The corpus declares snapshots of every source class — `curated`,
  `imported`, and `generated` — so a generated explanation never collapses into curated truth.
  Every binding preserves the source-class label.
- **Generated prose never stands without citations.** Every snapshot cites at least one citation
  and carries freshness and confidence cues; the `generated` login-flow snapshot is fully cited and
  carries `cached`/`low` cues, so it never reads as a free-floating source of architecture truth.
- **Every citation kind.** The corpus cites a `file`, `symbol`, `doc_pack`, `adr`, `curated_note`,
  and `graph_object`, so an explanation can rest on any of them.
- **Architecture maps are never canvas-only.** Every snapshot offers the `keyboard`, `list_table`,
  and `screen_reader` paths; the imported billing-gateway snapshot omits the optional `canvas` to
  show the non-canvas paths are sufficient.
- **Visibility never widens.** The corpus carries `public`, `internal`, and `private` snapshots.
  The restricted (`private`) secrets-vault snapshot is visible in the in-product review explainer
  but is withheld from the support export and redacted entirely from the export projection. The
  support-export binding carries every export-safe snapshot and no private one.
- **Carried beyond one panel.** Each of `onboarding_tour`, `review_explainer`, `docs_browser`,
  `ai_context_inspector`, and `support_export` carries exactly one binding, stamped with the active
  snapshot and scope; onboarding, review, docs, and AI all point at the same snapshots.
- **Upstream provenance.** The packet binds to the canonical graph-depth governance matrix
  (`artifacts/graph/m5/m5-graph-governance.json`), the workset-scope packet
  (`artifacts/graph/m5/m5-workset-scope.json`), and the topology-identity packet
  (`artifacts/graph/m5/m5-topology-identity.json`) whose node identity space it reuses.

## Guardrails proven

- A snapshot with no citations, or no freshness/confidence cue, fails validation
  (`SnapshotMissingCitations`, `SnapshotMissingFreshnessOrConfidence`).
- A snapshot that offers only a canvas fails validation (`CanvasOnlyNavigation`).
- A snapshot that cites an undeclared citation, or references an undeclared follow-up, fails
  validation (`UnresolvedCitationRef`, `UnresolvedFollowUpRef`).
- A binding that flattens source labels, or carries a snapshot beyond its visibility ceiling, fails
  validation (`SourceLabelsNotPreserved`, `VisibilityExceedsBinding`).
- An export-safe snapshot not carried by the support-export binding, or a `private` snapshot
  carried by it, fails validation (`ExportSafeSnapshotMissingFromSupportExport`,
  `PrivateSnapshotInSupportExport`).
