# Experiment Run Identities, Environment Fingerprints, Dataset Cards, and Artifact Lineage

## Overview

This document describes the M05-028 implementation for **experiment run
identities, environment fingerprints, dataset cards, and artifact lineage** in
the Aureline notebook subsystem.

The implementation enforces local-first experiment traceability: run identity,
dataset provenance, artifact lineage, and environment fingerprint stay
inspectable and portable without requiring a hosted tracking product to become
the hidden source of truth.

## Records

### `ExperimentRunIdentity`

Canonical experiment run identity record. Carries human-readable run identity,
source reference, timestamps, outcome class, commit/revision provenance,
execution origin, and environment fingerprint reference.

| Field | Type | Required | Description |
|---|---|---|---|
| `record_kind` | string | yes | `"notebook_experiment_run_identity"` |
| `notebook_experiment_lineage_schema_version` | integer | yes | `1` |
| `run_id` | opaque id | yes | Stable opaque run identifier |
| `title` | string | yes | Human-readable run title |
| `source_ref` | opaque ref | yes | Originating notebook/script/task/test |
| `started_at` | ISO 8601 | yes | UTC start timestamp |
| `ended_at` | ISO 8601 | no | UTC end timestamp |
| `outcome_class` | enum | yes | `success`, `failure`, `cancelled`, `partial`, `policy_blocked` |
| `commit_or_revision_ref` | opaque ref | no | Git commit or workspace revision |
| `execution_origin_label` | string | yes | Human-readable origin |
| `environment_fingerprint_ref` | opaque ref | yes | Ref to environment fingerprint |
| `summary` | string | yes | Export-safe summary |

**Validation rule:** `success` outcome requires `ended_at`.

### `ExperimentEnvironmentFingerprint`

Canonical experiment environment fingerprint record. Communicates
human-readable environment identity, interpreter/kernel label,
package/toolchain summary, target origin, policy epoch, and freshness.

| Field | Type | Required | Description |
|---|---|---|---|
| `record_kind` | string | yes | `"notebook_experiment_environment_fingerprint"` |
| `notebook_experiment_lineage_schema_version` | integer | yes | `1` |
| `fingerprint_id` | opaque id | yes | Stable opaque fingerprint identifier |
| `environment_identity_label` | string | yes | Human-readable identity |
| `interpreter_kernel_label` | string | yes | Interpreter or kernel label |
| `package_toolchain_summary` | string | no | Package/toolchain summary |
| `target_origin_label` | string | yes | Human-readable target origin |
| `policy_epoch_ref` | opaque ref | no | Policy epoch reference |
| `freshness_class` | enum | yes | `fresh`, `stale`, `unresolved`, `policy_blocked` |
| `last_known_good_at` | ISO 8601 | no | Last known good timestamp |
| `summary` | string | yes | Export-safe summary |

**Validation rules:**
- `fresh` requires `last_known_good_at`.
- `policy_blocked` requires `policy_epoch_ref`.

### `DatasetCard`

Canonical dataset card record. Carries dataset identity, source class, version
or snapshot note, size estimate, sensitivity/redaction class, and location
class.

| Field | Type | Required | Description |
|---|---|---|---|
| `record_kind` | string | yes | `"notebook_dataset_card"` |
| `notebook_experiment_lineage_schema_version` | integer | yes | `1` |
| `dataset_id` | opaque id | yes | Stable opaque dataset identifier |
| `dataset_label` | string | yes | Human-readable label |
| `source_class` | enum | yes | `local_file`, `remote_url`, `database`, `api_endpoint`, `versioned_store`, `generated`, `unknown` |
| `snapshot_version_label` | string | no | Version or snapshot label |
| `size_estimate_label` | string | no | Size estimate |
| `sensitivity_redaction_class` | enum | yes | `public`, `internal`, `confidential`, `redacted_preview`, `blocked` |
| `location_class` | enum | yes | `local_workspace`, `remote_storage`, `managed_cache`, `provider_only` |
| `summary` | string | yes | Export-safe summary |

**Validation rule:** `redacted_preview` or `blocked` sensitivity requires `size_estimate_label`.

### `ArtifactLineage`

Canonical artifact lineage record. Carries artifact identity, producing run
reference, generator step label, environment fingerprint reference, save
location, and lineage state.

| Field | Type | Required | Description |
|---|---|---|---|
| `record_kind` | string | yes | `"notebook_artifact_lineage"` |
| `notebook_experiment_lineage_schema_version` | integer | yes | `1` |
| `artifact_id` | opaque id | yes | Stable opaque artifact identifier |
| `producing_run_ref` | opaque ref | yes | Ref to producing run |
| `generator_step_label` | string | yes | Human-readable generator step |
| `environment_fingerprint_ref` | opaque ref | no | Ref to environment fingerprint |
| `save_location_class` | enum | yes | `local_workspace`, `remote_storage`, `managed_artifact_store`, `export_buffer` |
| `lineage_state_class` | enum | yes | `current`, `stale`, `diverged`, `orphaned`, `imported` |
| `stale_diverged_note` | string | no | Explanation for stale/diverged state |
| `summary` | string | yes | Export-safe summary |

**Validation rules:**
- `stale` or `diverged` requires `stale_diverged_note`.
- `orphaned` requires `producing_run_ref` to be `"orphaned"`.

### `ExperimentLineagePacket`

Checked-in packet that downstream surfaces ingest. Contains closed vocabulary
lists and worked examples for every record type.

## Artifacts

- JSON packet: `/artifacts/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.json`
- Markdown summary: `/artifacts/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.md`

## Schema

- `/schemas/notebook/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.schema.json`

## Fixtures

- `/fixtures/notebook/m5/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage/`

## Downstream consumers

- `crates/aureline-notebook` — primary module and integration tests
- `crates/aureline-review` — may reference experiment lineage in review packs
- `crates/aureline-support` — support bundle and evidence timeline ingestion
