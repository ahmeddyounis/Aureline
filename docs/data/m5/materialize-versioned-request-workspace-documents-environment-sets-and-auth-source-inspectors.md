# Materialize versioned request-workspace documents, environment sets, and auth-source inspectors

## Overview

This document describes the M5 canonical implementation for versioned request-workspace documents, layered environment sets, and auth-source inspectors. The implementation lives in:

- **Rust types**: `crates/aureline-api/src/materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors/`
- **Schema**: `schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json`
- **Checked-in packet**: `artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`

## Design Principles

1. **Inspectability first**: Auth source, write posture, and environment provenance stay visible on every surface that can send a request.
2. **No raw secrets in shared files**: Environment sets and auth sources reference secrets by broker handle; raw credential bodies are excluded from portable export by default.
3. **Versioned and diffable**: Request documents carry semantic versions and are diffable for UI, CLI, and automation reuse.
4. **Explicit degradation**: Stale schema snapshots, policy-blocked operations, and narrowed surfaces are labeled rather than hidden.
5. **Effective-request inspector**: Before send, the user can see which values came from the request file, workspace defaults, policy injection, ad hoc overrides, or secret handles.

## Core Types

### RequestWorkspaceDocumentRow

A versioned request document (HTTP or GraphQL) with:
- `document_kind`: `HttpRequest` or `GraphQlOperation`
- `document_version`: semantic version (`major`, `minor`, `patch`)
- `method_kind` and `path_template`
- `header_refs`, `body_ref`, `variable_refs`, `assertion_refs`
- `environment_set_ref` and `auth_source_ref`
- `write_posture`: `ReadOnly`, `WriteCapable`, or `PolicyBlocked`
- `diffable` and `cli_reusable` flags

### EnvironmentSetRow

A layered environment set with:
- `base_url_ref`: opaque endpoint reference
- `layers`: ordered list of `EnvironmentLayerRow` entries showing provenance
- `secret_handle_refs`: opaque broker handles
- `effective_fingerprint_ref`: resolved fingerprint for preview
- `previewable` and `excludes_raw_secrets_from_export`

### AuthSourceInspectorRow

An auth-source inspector with:
- `auth_mode`: `NoAuth`, `SecretBrokerHandle`, `DelegatedIdentity`, `PolicyInjectedCredential`, `ManagedServiceIdentity`, `Mtls`, `ImportedNoLiveAuth`, `PolicyBlocked`
- `broker_handle_refs` and `mtls_signing_refs`: opaque handles
- `visible_without_secret`: always true for valid rows
- `provenance`: where the auth source originated

### EffectiveRequestInspectorRow

Shows value attribution before send:
- `shows_document_values`, `shows_workspace_defaults`, `shows_policy_injected`, `shows_ad_hoc_overrides`, `shows_secret_handles`
- `visible_before_send` and `preserves_provenance_in_export`

### SchemaSnapshotRow

A schema snapshot with:
- `source_kind`, `digest_ref`, `freshness_state`, `target_endpoint_ref`
- `stale_labeled`: true when the snapshot is not current
- `may_masquerade_as_live`: must always be false

## Qualification Packet

The `RequestQualificationPacket` binds together:
- `surfaces`: governed surface rows with labels, guards, and proof packets
- `documents`, `environment_sets`, `auth_sources`, `effective_inspectors`, `schema_snapshots`
- `summary`: counts that must match the computed summary

### Validation Rules

- `schema_version` must equal `1`
- `record_kind` must equal the canonical string
- All IDs within a family must be unique
- Stable surfaces must have a proof packet and complete guard truth
- Narrowed stable claims must have `downgrade_if_missing: true`
- Documents must cover HTTP and GraphQL kinds
- Write postures must cover `ReadOnly`, `WriteCapable`, and `PolicyBlocked`
- Auth sources must cover at least `NoAuth`, `SecretBrokerHandle`, `DelegatedIdentity`, and `PolicyBlocked`
- All auth sources must be `visible_without_secret`
- Environment sets must be previewable and exclude raw secrets from export
- Effective inspectors must show all value sources
- Schema snapshots must have digest and freshness, and stale schemas must not masquerade as live

## Downgrade and Rollback

If any of the following conditions are met, the affected surface narrows below `Stable`:
- Proof packet is missing or stale
- Guard truth is incomplete
- Required document kind, write posture, auth mode, or environment layer is missing
- Auth source hides secret material
- Environment set exports raw secrets or is not previewable
- Stale schema may masquerade as live truth

## Integration

Downstream UI, CLI, support, and export surfaces consume the typed crate (`aureline-api`) and the checked-in JSON packet. They do not re-describe state manually.

## Verification

Run the crate tests and validation:

```bash
cargo check -p aureline-api
cargo test -p aureline-api
```

The embedded JSON artifact is validated at compile time via `include_str!`, so any drift between the artifact and the typed model will fail `cargo check`.
