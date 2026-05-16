# Network egress-truth beta

Network-capable beta surfaces must disclose where code or data might leave the
machine. The egress-truth beta page promotes that disclosure into one
inspectable record that the shell network strip, admin / settings center,
support export, headless inspector, and reviewer fixtures all read by
reference — so updates, AI, providers, extensions, and docs / help cannot
quietly disagree about where a request is going.

## Contract

The shared contract ref is `network:network_badges_beta:v1`.

The source of truth lives at
[`crates/aureline-shell/src/network_badges/mod.rs`](../../../crates/aureline-shell/src/network_badges/mod.rs)
and the headless inspector at
[`crates/aureline-shell/src/bin/aureline_shell_network_badges.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_network_badges.rs).

The page exports:

- `network_network_badges_beta_row_record` for each network-capable surface
  (`update`, `ai`, `provider`, `extension`, `docs_help`).
- `network_network_badges_beta_profile_binding_record` for the per-profile
  egress class, origin scope, locality, explainer, and route disclosure.
- `network_network_badges_beta_support_row_record` for the export-safe
  support projection of each live row.
- `network_network_badges_beta_defect_record` for validator findings.
- `network_network_badges_beta_page_record` and
  `network_network_badges_beta_support_export_record` for the complete page
  and its export wrapper.

## The badge trio

Every profile binding carries three stable badges:

| Badge | Vocabulary | Source |
| --- | --- | --- |
| **Egress class** | `local_only`, `target_local`, `org_approved_external`, `public_internet`, `mirror_only`, `deny_all` | TDD §7.11.5 transport-governance enum |
| **Origin scope** | `desktop_client`, `remote_target`, `managed_service`, `extension_host`, `headless_runner` | TDD §7.11.5 transport-governance enum |
| **Locality** | `local_only`, `mirrored`, `self_hosted`, `managed`, `public_cloud` | Egress-truth disclosure required by the M3 trust gate |

Locality is the user-facing label: it answers "where might this data leave
the machine to?" using procurement-friendly vocabulary. The validator
enforces a consistency rule between locality and egress class so the two
badges cannot drift:

| Locality | Allowed egress classes |
| --- | --- |
| `local_only` | `local_only`, `deny_all` |
| `mirrored` | `mirror_only` |
| `self_hosted` | `org_approved_external`, `target_local` |
| `managed` | `public_internet`, `org_approved_external` |
| `public_cloud` | `public_internet` |

`public_cloud` locality additionally requires a non-empty `route_label` so
the public destination is never anonymous.

## Required behavior

**Each network-capable action discloses one of: local-only, mirrored,
self-hosted, managed, or public-cloud routed.** The page covers the
`update`, `ai`, `provider`, `extension`, and `docs_help` surfaces; the
validator emits `missing_surface_coverage` when any of them is absent.

**Badges and explainers are shared across surfaces.** Every binding carries
the same `egress_class`, `origin_scope`, and `locality` vocabulary plus an
`explainer_label` and `route_label`. The shell render summary, admin /
settings center, support export wrapper, and headless inspector read the
same fields rather than minting per-surface copy.

**Wrong or missing egress labels block beta promotion.** The validator emits
typed `NetworkBadgeBetaDefectKind` records that hold beta promotion on the
affected row:

| Defect | When it appears |
| --- | --- |
| `missing_surface_coverage` | A claimed network-capable surface is absent. |
| `missing_profile_coverage` | A row is missing a connected, mirror-only, offline, or enterprise-managed binding. |
| `surface_token_drift` | The surface token does not match the surface class. |
| `egress_class_token_drift` | The egress-class token does not match the egress class. |
| `origin_scope_token_drift` | The origin-scope token does not match the origin scope. |
| `locality_token_drift` | The locality token does not match the locality class. |
| `locality_inconsistent_with_egress` | Locality is not in the allowed set for the declared egress class. |
| `hidden_public_cloud_routing` | A `public_cloud` locality has no `route_label`. |
| `hidden_public_endpoint_fallback` | A row or binding permits an undeclared public-cloud fallback. |
| `empty_explainer_label` | A binding has an empty `explainer_label`. |
| `support_row_vocabulary_drift` | The support / export row drifted from the live row vocabulary. |
| `raw_secret_or_private_material_exposed` | A row sets `raw_secret_or_private_material_excluded = false`. |

## Profile coverage

Each row covers four profiles:

- **`connected`** — the normal connected beta profile.
- **`mirror_only`** — public endpoints are not a fallback target; signed
  mirrors carry every network-capable surface.
- **`offline`** — air-gapped; `local_only` or `deny_all` egress only,
  locality is `local_only`.
- **`enterprise_managed`** — admin-managed policy; surfaces are routed
  through customer-hosted (`self_hosted`) endpoints.

## Drills

The headless inspector exposes three negative drills used by the validator
suite and the fixture set:

- `drill-locality-egress-mismatch` — declare `mirrored` locality on a
  `public_internet` binding to prove `locality_inconsistent_with_egress`
  fires.
- `drill-hidden-public-cloud` — drop the `route_label` on a `public_cloud`
  binding to prove `hidden_public_cloud_routing` fires.
- `drill-missing-surface` — remove the docs / help surface to prove
  `missing_surface_coverage` fires.

## Fixtures

Reviewer fixtures live at
[`fixtures/network/m3/egress_truth/`](../../../fixtures/network/m3/egress_truth/).
See the [README](../../../fixtures/network/m3/egress_truth/README.md) for the
generation commands and per-file purpose.
