# M5 framework pack headers and framework status strips

This contract implements the frozen `framework_pack_header` component family from the
[M5 framework-component matrix](m5_framework_component_matrix.md) as two reusable, co-equal
control vectors — the full **framework pack header** and the compact **framework status strip** —
so a user can orient before they trust a framework lens.

The Rust validator in
`crates/aureline-templates/src/implement_framework_pack_headers_and_framework_status_strips_with_pack_identity_version_support_range_provider_source_freshness_compatibility_and_local_versus_remote_scope_truth`
is the authoritative gate; the
[boundary schema](../../../schemas/ui/m5-framework-pack-header-status-strip-controls.schema.json)
documents the export shape.

## What the pack header names

A framework pack header names, before a user trusts a framework-aware feature:

- **Pack identity and version range** — the pack name, the detected framework, and the
  framework / version range.
- **Support class** — one of `officially_supported`, `community_supported`, `experimental`,
  `bridge_only`, `deprecated`, or `unsupported`.
- **Provider source** — who provides the pack (first-party, community registry, mirror, bridge
  adapter, or unresolved).
- **Selected workspace scope** — which workspace the header is scoped to.
- **Freshness** — whether the pack signal is `current`, `imported`, `stale`, `never_scanned`, or
  `unknown`.
- A first-class **open-compatibility-details** action.

## What the status strip preserves

A framework status strip preserves, wherever a framework-aware feature is claimed:

- The **detected framework and version**.
- The **pack health** — `healthy`, `degraded`, `compatibility_warning`, `broken`, or `unknown`.
- The **compatibility notes**.
- The **bridge-or-heuristic posture**.

## Derived truth (never asserted)

Both components carry three derived axes computed by `resolve_framework_pack_posture` from the
frozen support class, pack identity state, certainty disposition, and execution boundary:

- **Support posture** — `fully_supported`, `community_supported`, `experimental_or_bridge`, or
  `unsupported_or_deprecated`. Only `fully_supported` is exact first-party support.
- **Framework-experience class** — `core_native`, `pack_backed`, `bridged`, or `heuristic`. This
  is the acceptance-criteria axis: a user can tell at a glance whether the active experience is
  core native, pack-backed, bridged, or heuristic.
- **Scope posture** — `local_scope`, `container_scope`, `remote_scope`, `managed_scope`, or
  `unknown_scope`. Only `local_scope` is local; every other scope is remote to this machine.

Because these are derived, bridge or heuristic behavior can never read as exact first-party
support, a drifted or multiple-detected pack can never leave its identity implicit, and a remote,
managed, container, or unknown scope can never read as local.

## Hard invariants

Every pack header and status strip keeps these `false`:

- `hides_pack_identity_or_support_class`
- `lets_heuristic_masquerade_as_exact`
- `hides_local_container_ssh_or_managed_boundary`
- `invents_alternate_state_label`

The validator additionally rejects any component whose bridged or heuristic experience claims exact
first-party support (`heuristic_claims_exact_support`).

## Deep links and recovery

Every next step names one stable `pack_manifest`, `provider_registry_entry`, `docs_anchor`, or
`compatibility_reference` deep link rather than an ephemeral overlay.

## Export safety

Raw file bodies, raw manifests, pasted local paths, repository URLs, credentials, and secrets never
cross the export boundary. The canonical proof bundle lives at
`artifacts/release/m5-framework-pack-header-status-strip-proof/` and the scenario fixtures at
`fixtures/ui/m5-framework-pack-header-status-strip-controls/`, both regenerated deterministically
from the seed builders via
`cargo run -p aureline-templates --example dump_framework_pack_header_controls`.
