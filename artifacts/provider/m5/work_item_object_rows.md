# Work-item object rows

- Packet: `providers.work_item_object_rows.packet`
- Source page: `providers.work_item_transition_beta.page`
- Schema: `schemas/work_items/object_rows.schema.json`
- Contract: `docs/work_items/object_rows.md`

The checked packet proves three first-consumer row classes:

- issue/work-item row: `AUR-241` keeps provider identity, exact provider state, freshness, and branch/review/run/incident/validation relations visible;
- task row: `TASK-17` keeps explicit local-draft truth instead of masquerading as provider-committed state;
- incident row: `INC-246` keeps imported/offline-captured truth visible while preserving relation identity refs.

The export-safe row vocabulary preserves:

- provider family and provider label;
- object class and canonical id;
- exact provider or local state token;
- sync scope (`provider_committed`, `local_draft_only`, `offline_captured`);
- relation-strip item identities and source/freshness posture;
- no raw provider URLs or hidden account material.
