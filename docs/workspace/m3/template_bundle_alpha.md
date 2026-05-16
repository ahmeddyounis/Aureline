# Workspace-template bundle alpha

The alpha workspace-template bundle is the durable, reviewable record that
Start Center, CLI / headless entry, docs, and support packets read **before
a template is used**. It restates the source class, support class, target
runtime, side-effect classes, trust posture, and open-without-starter bypass
routes named by the underlying signed manifest, and projects them as one
closed record so every disclosure surface quotes the same vocabulary.

The bundle is intentionally narrower than the template scaffold-run packet
documented in
[`/docs/scaffolding/template_and_scaffold_contract.md`](../../scaffolding/template_and_scaffold_contract.md):

- the bundle owns **review** — what the user is shown about a template's
  identity, side effects, and bypass options ahead of any choice to apply;
- the scaffold-run packet (`template_scaffold_alpha_packet`) owns
  **generation** — the preflight write plan, scaffold run, and generated
  project lineage.

The companion schema lives at:

- [`/schemas/workspace/template_bundle.schema.json`](../../../schemas/workspace/template_bundle.schema.json)

The canonical fixtures live under:

- [`/fixtures/workspace/m3/template_bundle/`](../../../fixtures/workspace/m3/template_bundle/)

The headless validator that gates every fixture lives at:

- [`/ci/check_template_bundle_alpha.py`](../../../ci/check_template_bundle_alpha.py)

The Rust types are exported from `aureline_workspace::templates`. The
integration test
[`crates/aureline-workspace/tests/template_bundle_alpha.rs`](../../../crates/aureline-workspace/tests/template_bundle_alpha.rs)
replays every fixture and proves the closed acceptance states. The first
shell consumer is
[`crates/aureline-shell/src/start_center/template_bundles/mod.rs`](../../../crates/aureline-shell/src/start_center/template_bundles/mod.rs),
which renders a deterministic bundle row directly from the checked-in
first-party fixture.

## 1 Why freeze this now

The signed template-manifest and scaffold-run packets already answer
*what a scaffold-run did*. They do not answer the **review** question every
template surface asks first:

- who authored or signed this template, and which trust root vouches for it;
- whether the template is officially supported, community supported, or
  experimental;
- what runtime the template expects (local, container, remote image,
  managed cloud);
- which side effects the template requires before it can be applied —
  network egress, extension install, remote provisioning, managed service,
  or credential handles;
- whether the user can keep working without applying the template at all.

This bundle freezes those answers without inventing a registry, gallery,
or generator runtime.

## 2 Record shape

Every bundle is one
`workspace_template_bundle_alpha_record` carrying:

| Block | Required content |
| --- | --- |
| `source_review` | `source_class`, `source_distribution_class`, `signature_state`, `publisher_label`, `trust_root_ref`. |
| `support_review` | `support_class` plus the lifecycle class re-exported from the manifest. |
| `target_runtime_review` | `runtime_scope_class`, `host_boundary_class`, non-empty supported ecosystem and platform class lists. |
| `side_effect_review` | Closed network egress, extension install, remote provisioning, managed service, and credential provisioning classes plus declared hook / task counts and short reviewable sentences. |
| `trust_review` | Trust posture class, egress posture class, and reviewable trust notes. |
| `bypass_review` | At least one open-without-starter route id, `bypass_continuity_class = equal_weight_with_apply`. |
| `consumer_surfaces` | Non-empty list drawn from `start_center`, `cli_headless_entry`, `docs_workspace`, `support_export`; must include `start_center`. |
| `support_export` | Packet refs and the closed `raw_secret_export_allowed = raw_command_export_allowed = raw_url_export_allowed = false`. |
| `review_invariants` | All of `reviewed_before_use`, `inspectable_before_execution`, `no_writes_before_review` must be `true`. |

## 3 Frozen rules

The validator and the integration test both enforce:

1. **Bypass continuity.** Every bundle MUST carry at least one
   `open_without_starter_route_id` from the closed bypass vocabulary, and
   `bypass_continuity_class` MUST be `equal_weight_with_apply`. The user
   can always open the workspace plainly when the product contract allows
   it.
2. **No hidden disclosure widening.** Side-effect classes are drawn from
   the closed vocabulary. A `managed_cloud_required` runtime scope MUST
   pair with `managed_workspace_required` remote provisioning, a non
   `no_managed_service_required` managed-service class, and a non
   `no_network_egress_required` egress class. A `local_only` runtime scope
   MUST NOT claim remote provisioning or a managed service.
3. **Community / uncertified disclosure.** A `community` or `uncertified`
   `source_class` MUST carry at least one trust note so the missing signer
   continuity is not hidden behind a generic posture.
4. **Closed export.** Raw secrets, raw command lines, and raw URLs are
   never exported through the bundle. Support packets quote refs only.
5. **Pre-execution review.** The bundle is inspectable before execution
   and writes nothing on its own; it never substitutes for the scaffold
   preflight or scaffold-run contracts.

## 4 Reusable consumers

The bundle is the single record every disclosure consumer reads:

- **Start Center** projects the bundle into a card row that surfaces
  author, source class, support class, target runtime, side effects, and
  bypass routes before the user opens a generation preflight.
- **CLI / headless entry** prints the bundle's projection as
  deterministic plaintext so scripted or remote workflows see the same
  review surface.
- **Docs / workspace help** lifts the bundle's display label, summary,
  and bypass guidance to keep documentation in lockstep with the
  product surface.
- **Support exports** quote the bundle id, manifest ref, and packet refs
  so a support ticket never invents parallel review vocabulary.

## 5 Boundary placement

The bundle is durable and exportable. It is also intentionally
underspecified: it does not include the preflight write plan, the
scaffold run, or the generated-project lineage. Those records live in
the existing scaffold contract and remain authoritative once a user
chooses to apply a template. The bundle covers only the disclosure surface
that must be honest **before** that choice is offered.
