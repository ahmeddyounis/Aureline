# Infrastructure Surface Qualification Packet

This file is the artifact-level companion document for the checked-in
infrastructure surface qualification packet.

- **Canonical JSON**: `artifacts/infra/infrastructure-surface-qualification/support_export.json`
- **Schema**: `schemas/infra/infrastructure-surface-qualification.schema.json`
- **Typed consumer**: `crates/aureline-infra/src/infrastructure_surface_qualification/mod.rs`

The packet is the canonical infrastructure evidence index for the currently
claimed infrastructure and incident-adjacent surface families: source
intelligence, live counterpart graph flows, plan and validation viewers,
live-resource target context, provider-overlay and vendor-console handoff
continuity, incident/support reopen parity, and the shared public evidence
index consumed by docs/help, Help / About, support playbooks, and public-truth
surfaces.

The packet auto-narrows any row that loses relationship proof, target-context
proof, live-counterpart proof, plan/viewer proof, handoff-boundary proof, or
export parity instead of allowing a generic DevOps/SRE claim to stay greener
than the checked evidence.
