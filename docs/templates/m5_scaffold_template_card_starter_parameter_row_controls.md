# M5 scaffold template cards and starter parameter rows

The scaffold template card and the starter parameter row are two of the six governed scaffold /
project-entry components frozen by the
[M5 scaffold-component matrix](m5_scaffold_component_matrix.md). This lane implements those two
families as two co-equal control vectors in one export-safe packet,
[`ScaffoldTemplateCardStarterParameterRowControlsPacket`](../../crates/aureline-templates/src/implement_scaffold_template_cards_and_starter_parameter_rows_with_source_support_host_boundary_and_portability_truth_across_claimed_m5_project_entry_surfaces/mod.rs),
so a claimed M5 start-center, template-gallery, parameter-form, or CLI surface can project a
template card and a parameter row that make starter source, support, host boundary, and parameter
precedence **explicit before a user commits** — never inferred, and never routing creation through
a generic Create that hides side effects or exposing a raw secret value.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never asserted.

### `resolve_template_posture`

Given a template card's frozen **starter source class** and **template support class**, the
resolver derives a **source class** and a **support posture**:

- `first_party_starter` → `first_party_template` (governed first-party)
- `team_managed_starter` → `team_managed_template` (governed first-party)
- `community_starter` → `community_template` (must carry a community-source note), not governed
  first-party
- `local_only_starter` / `mirrored_starter` → `local_template` (must carry a local-source note),
  not governed first-party
- `unknown_source_starter` → `source_unknown` (must carry an unknown-source note), not governed
  first-party

and

- `officially_supported` → `fully_supported` (exact first-party support)
- `community_supported` → `community_supported` (not exact first-party support)
- `experimental` / `bridge_behavior` → `experimental_or_bridge` (not exact first-party support)
- `deprecated` / `unsupported` → `unsupported_or_deprecated` (not exact first-party support)

A user can therefore always tell **who authored a starter and how it is supported** before
committing; a community, local, mirrored, or unknown starter can never read as a governed
first-party starter, and bridge or heuristic behavior can never read as exact first-party support.
Every card also names its **target runtime and toolchain, its host boundary, its setup-task or
extension impact,** and an **open-manifest** action, so the network, dependency-install,
remote-provisioning, trust, and managed-workspace side effects behind a create are never hidden.

### `resolve_parameter_disclosure`

Given a parameter row's source-precedence **origin class** and its frozen **action timing**, the
resolver derives a **portability class**:

- `template_default` → `portable_template_value` (portable)
- `user_input` → `portable_user_value` (portable)
- `workspace_value` → `workspace_scoped_value` (must carry a local-only note), not portable
- `policy_value` → `policy_managed_value` (the workspace does not own it), not portable
- `secret_reference` → `secret_reference_not_persisted` (must carry a secret note), not portable

The five origin classes are the exact acceptance-criteria labels — **`Template default`, `User
input`, `Workspace value`, `Policy value`, and `Secret reference`** — so source precedence is
never hidden inside implementation detail. A workspace-scoped or policy-managed value can never
read as a portable user value, and a **secret reference never reveals a raw secret value**: the row
carries only an opaque reference and routes to the secret manager. Each row also names its
**required / optional state, its validation state, and whether its action is applied immediately or
deferred,** with an explicit persistence / portability cue.

## Hard invariants

Every template card and parameter row keeps four hard invariants `false`:

- `hides_starter_source_or_support_class` — the source and support (or parameter precedence) is
  always named.
- `hides_side_effect_or_host_boundary` — a create never hides a side effect behind a generic
  Create, and the host boundary is always visible.
- `exposes_secret_or_raw_value_by_default` — a raw secret value or file body is never exposed.
- `invents_alternate_state_label` — no surface invents a second grammar for a governed state.

## Coverage

The canonical packet ships six template cards covering every starter source class, every template
support class, every derived source class, and every support posture; and six parameter rows
covering every parameter source layer, every action timing, all five source-precedence origin
classes, and every derived portability class. Two scenario fixtures spotlight the two honesty
edges: a community template card that must never read as governed first-party, and a
secret-reference parameter row that must never reveal a raw secret value.

## Truth source

The checked-in support export and the two scenario fixtures are generated only from the canonical
seed builders (`cargo run -p aureline-templates --example dump_scaffold_entry_controls`). The
[schema](../../schemas/ui/m5-scaffold-template-card-starter-parameter-row-controls.schema.json),
this doc, and the
[release proof bundle](../../artifacts/release/m5-scaffold-template-card-starter-parameter-row-proof/)
are the canonical source of truth for this lane; raw file bodies, raw secret values, pasted local
paths, repository URLs, credentials, and secrets never cross the export boundary.
