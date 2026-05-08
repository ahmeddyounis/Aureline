# Deprecation packet template

Copy this template when deprecating a user-facing surface, schema, command
family, or compatibility/claim-bearing artifact.

This is a reviewer-facing packet: it exists so deprecations do not ship as
scattered notes across docs, release notes, and support handoffs.

## Scope

- **What is being deprecated?** (surface, schema, command family, API, workflow)
- **Why?** (security, correctness, replace-with, consolidation, support cost)
- **Successor / replacement** (stable id / ref; include migration posture)
- **Deprecation window** (start date and earliest removal date)

## Canonical references

- Deprecation metadata schema: `schemas/governance/deprecation_metadata.schema.json`
- Support-window / migration policy: `docs/release/end_of_support_and_migration_contract.md`
- Claim-publication gate policy: `ci/claim_publication_gate.yaml`

## Required checklist

- Update the canonical record to mark the item deprecated (do not rely on prose).
- Update docs/help/About/service-health projections so the deprecation state is
  visible and consistent.
- Update any claim or badge surfaces so they do not widen posture past a
  deprecated row.
- Ensure the successor path is discoverable (docs anchors, command aliases, or
  migration bridge refs as applicable).

