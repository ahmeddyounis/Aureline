# Schema Registry And Record Classes

This document is the reviewer-facing companion for the governed JSON
registries in:

- [`/schemas/registry/schema_registry.json`](../../schemas/registry/schema_registry.json)
- [`/schemas/registry/record_class_registry.json`](../../schemas/registry/record_class_registry.json)

The existing consent-ledger seed and record-class seed remain the source
context. The JSON registries are the compact product-facing entry points
used by release checks, support/export previews, CLI/headless output, and
surface inspectors.

## Contract

Every governed payload family names:

- owner and schema version;
- version-change rationale;
- consent class and endpoint class;
- record-class binding;
- retention posture;
- lifecycle state;
- downgrade rules for old or unknown versions;
- surfaces that must expose the same schema metadata.

Telemetry remains opt-in for open builds. Support-bundle manifests,
usage exports, offboarding packets, CLI/headless output, and SDK result
schemas remain separate rows even when redaction, export, or transport
plumbing is shared.

## Record Classes

Every governed artifact class names local-versus-managed truth,
export/delete/hold semantics, redaction posture, retention posture, and
offboarding role. Local-only artifacts and managed copies are intentionally
different classes of truth; a delete, export, hold, or offboarding flow
must not collapse them into a single vague status.

## Surface Parity

Settings, Help/About, Support Center, admin/export, release packets, and
CLI/headless output read the same registry fields:

- `schema_id`
- `schema_version`
- `lifecycle_state`
- `consent_class`
- `endpoint_class`
- `record_class_id_refs`

Those surfaces may render different layouts, but they must not invent
different consent, endpoint, retention, or lifecycle vocabulary for the
same row.

## Version Handling

Readers classify packet versions as:

- `supported` when the packet version equals the registered schema
  version;
- `deprecated` when it is older but still inside the readable window;
- `limited` when it is a near-future or otherwise unknown version that
  can only be inspected with manual review;
- `unsupported` when it is outside the declared readable window.

Unknown or deprecated packet versions must be visible as one of those
states. Silent parsing and silent failure are both invalid.

## CI Gate

Run:

```sh
python3 tools/ci/schema_registry_governance.py --repo-root .
```

The gate rejects missing owners, missing schema refs, missing record-class
bindings, missing downgrade rules, telemetry rows that are not opt-in for
open builds, and rows that do not expose schema metadata to the required
surface set.
