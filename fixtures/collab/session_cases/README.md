# Collaboration Session Layer Worked Cases

This directory contains worked examples for the architecture seed at
[`docs/architecture/collaboration_session_layer_adr.md`](../../../docs/architecture/collaboration_session_layer_adr.md)
and the boundary schemas in
[`schemas/collab/session_topology.schema.json`](../../../schemas/collab/session_topology.schema.json)
and
[`schemas/collab/shared_object_authority.schema.json`](../../../schemas/collab/shared_object_authority.schema.json).

The cases exercise the boundary between canonical project state,
shared-session artifacts, ephemeral presence, and captured export
evidence. They intentionally use opaque refs and reviewable labels only:
no raw source text, terminal bytes, debug payloads, URLs, absolute paths,
user identifiers, tokens, or provider payloads are embedded.

## Cases

- [`relay_outage_local_continuation.yaml`](./relay_outage_local_continuation.yaml)
  — relay loss degrades presence and shared convergence while local
  editing remains available and shared-session state narrows visibly.
- [`viewer_fallback_unsent_work.yaml`](./viewer_fallback_unsent_work.yaml)
  — a policy downgrade moves a participant to viewer while preserving
  unsent local proposals for review/export.
- [`presenter_follow_presence_ephemeral.yaml`](./presenter_follow_presence_ephemeral.yaml)
  — presenter/follow state degrades through the presence plane and does
  not queue a hidden presenter handoff or control grant.
- [`provider_outage_archive_deferred.yaml`](./provider_outage_archive_deferred.yaml)
  — managed archive publication is deferred while local journals and
  redacted archive manifests remain attributable and recoverable.
- [`anchor_drift_metadata_export.yaml`](./anchor_drift_metadata_export.yaml)
  — an ambiguous/deleted anchor stays metadata-only and must be
  superseded by a new object instead of silently relocating.
- [`archive_seal_storage_boundary.yaml`](./archive_seal_storage_boundary.yaml)
  — a sealed archive captures attributed evidence without becoming the
  canonical project database.
