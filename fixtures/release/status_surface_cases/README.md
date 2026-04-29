# Release Status Surface Cases

These fixtures exercise the release-status surface object family from
[`/docs/release/release_status_surface_contract.md`](../../../docs/release/release_status_surface_contract.md).
They are structural examples for release-candidate cards, version-bump
rows, promotion timeline entries, artifact provenance links, and rollback
or revocation panels.

Schema companions:

- [`/schemas/release/release_candidate_card.schema.json`](../../../schemas/release/release_candidate_card.schema.json)
- [`/schemas/release/promotion_timeline_entry.schema.json`](../../../schemas/release/promotion_timeline_entry.schema.json)
- [`/schemas/release/rollback_revocation_panel.schema.json`](../../../schemas/release/rollback_revocation_panel.schema.json)

Cases:

| Fixture | Coverage |
|---|---|
| `local_build.yaml` | Local build output, incomplete proof layers, no public support claim, discard-local-build recovery. |
| `staged_candidate.yaml` | Staged stable candidate with stale compatibility evidence, blocker visibility, next gate, and repin path. |
| `promoted_stable.yaml` | Promoted stable channel with current evidence, support window, provenance links, and rollback panel. |
| `revoked_yanked_artifact.yaml` | Revoked update metadata plus yanked registry/marketplace artifact with separate action verbs, consumer scopes, historical visibility, and recovery paths. |
