# M5 Fault / Crash Certification

## Overview

This packet is the canonical certification index for M5 host-failure truth. It
does not introduce new runtime behavior; it binds the already-landed M5
fault-domain, restart-budget, crash-store, symbolication, schema-registry,
recovery-review, forensic, and drill packets into one shared decision surface.

The machine-readable truth source is:

- `fixtures/support/m5/fault_crash_certification/packet.json`

Downstream consumers must ingest that same index rather than re-deriving
host-failure maturity from local copy:

- Help/About
- service health
- support export
- release manifest / publication truth

## Profiles covered

The certification covers these claimed M5 profiles:

- `desktop_local_first`
- `hybrid_remote_attach`
- `managed_cloud`
- `self_hosted_sovereign`
- `air_gapped_mirror_only`

Each host/profile row names one published state:

- `qualified`
- `limited_profile_scoped`
- `experimental_local_only`
- `not_marketed`
- `blocked_unverified`

Rows may narrow, withhold, or block when the named profile cannot honestly
claim the required control plane, browser bridge, symbolication posture, or
schema-governed export story.

## What each row proves

Every certification row carries:

- the governing `fault_domain_class` and `restart_class`
- the restart posture packet ref
- the crash-artifact / forensic packet ref
- the symbolication packet or report ref
- the diagnostics-schema packet ref
- the field-readiness drill packet ref
- the published state plus any active `stale_proof_tokens`
- the active downgrade-rule ids that explain the narrowed state

That means a support, release, or Help/About surface can answer exactly what is
qualified for a host family on a given profile without inventing local wording.

## Required downgrade behavior

The packet freezes downgrade automation for these cases:

- `restart_evidence_stale`
- `crash_artifact_proof_stale`
- `symbolication_not_exact_build`
- `diagnostic_schema_stale`
- `field_readiness_drill_stale`
- `profile_capability_absent`
- `consumer_binding_missing`

No stale host-failure, crash-forensics, or schema-governance proof may keep a
broader M5 claim green.

## Shared-surface binding

The certification includes one `surface_binding` row for:

- `help_about`
- `service_health`
- `support_export`
- `release_manifest`

Every binding must preserve these fields verbatim:

- `certification_row_id`
- `host_family_id`
- `profile`
- `published_state`
- `stale_proof_tokens`
- `downgrade_rule_ids`

If one consumer stops ingesting the certification by reference, the broad claim
blocks until parity is restored.
