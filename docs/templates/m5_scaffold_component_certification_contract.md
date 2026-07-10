# M5 Scaffold Component Surface Certification (M05-1027)

This is the **closing surface-certification capstone** for the B121 scaffold-component lane. Where
the frozen matrix (`schemas/ui/m5-scaffold-component-matrix.schema.json`) defines the six reusable
**scaffold-template-card**, **starter-parameter-row**, **scaffold-preflight-card**,
**template-health-row**, **generated-project-diff-card**, and **scaffold-handoff-banner**
components, the M05-1021..1024 primitive lanes narrow each one, the M05-1025 consumer lane proves
they are reusable across the claimed start-center / workspace-admission / template-registry /
framework-pack / workflow-bundle / help-support / safe-handoff-export consumers, and the M05-1026
accessibility / auto-narrowing capstone certifies keyboard / screen-reader / CLI / export parity per
family, this capstone **certifies that the shared scaffold-component truth holds on every claimed M5
stack-entry and project-generation surface** — and auto-narrows any surface that cannot sustain it.

- **Module:**
  `crates/aureline-templates/src/certify_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_truth_on_every_claimed_m5_stack_entry_and_project_generation_surface/`
- **Boundary schema:** `schemas/ui/m5-scaffold-component-certification.schema.json`
- **Release proof:** `artifacts/release/m5-scaffold-component-certification/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-scaffold-component-certification/`
- **Canonical bundle every row cites:** `artifacts/release/m5-scaffold-component-proof/support_export.json`
  (the frozen scaffold-component matrix release proof — the canonical M5 evidence index entry for
  this lane)

## What is certified

The packet is keyed on the **surface** a user reviews a starter, a preflight, a health signal, a
generated diff, or a workspace handoff on — not on component family or primitive lane. Eight claimed
surfaces are certified exactly once:

| Surface | Meaning |
| --- | --- |
| `start_center` | The start center (recent work / project entry / new-project surface). |
| `template_gallery` | The template gallery. |
| `scaffold_preflight` | The scaffold preflight surface. |
| `generation_diff_review` | The generation diff-review surface. |
| `workspace_handoff` | The workspace handoff surface (post-bootstrap). |
| `template_health` | The template-health dashboard surface. |
| `support_export` | The support / export bundle. |
| `cli_headless` | The CLI / headless surface. |

Each surface is scored on **six truth axes**: `visual`, `keyboard`, `screen_reader`, `export`
(always-on), `degraded_state`, and `source_side_effect_and_recovery`. Every one of the six frozen
component families is certified on at least one surface.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`qualified_starter` claim while one of its truth axes is not current — the template freshness has
drifted, the generation diff truth is partial, a prerequisite health check is blocked, or a starter
parameter is secret-bound and cannot travel — is over-claiming and **blocks (red)**. A surface that
discloses the reduction by narrowing its readiness claim (with a bound reason and a frozen downgrade
trigger) is honestly **yellow**. A surface with full parity delivers its claim (**green**).

The readiness-claim ladder, strongest first: `qualified_starter` (5) >
`secret_bound_parameter_projection` (4) > `blocked_prerequisite_projection` (3) >
`drifted_template_projection` (2) > `partial_generation_projection` (1) >
`unchecked_validation_projection` (0). Certification may only **narrow** a claim, never strengthen
it.

### Source and recovery preservation

Scaffold truth never loses its source or recovery: a narrowed surface always preserves its
**starter-source / support / side-effect / generated-versus-user-owned / recovery** continuity rather
than dropping it between a template card, a preflight card, a generated diff card, and a workspace
handoff banner. Dropping it blocks the surface (`SourceOrRecoveryDropped`).

### No hidden side effect, no raw value by default

No certified surface may let a generic `Create` **hide a network, dependency-install,
remote-provisioning, trust, or managed-workspace side effect** (`SideEffectHiddenBehindGenericCreate`).
No certified surface may **expose a secret-bound raw value by default**: secret references stay
redacted and a raw value is opt-in (`RawValueExposedByDefault`).

### Always-on export parity

The `export` axis must always stay certified, so support and automation can reconstruct the same
source / support / side-effect / health / recovery truth from the same component identity the user
saw. Export must offer text / JSON / Markdown reconstruction and prohibit a raw-value-only export.

## The four auto-narrow conditions

The seed packet certifies four green surfaces (full parity, claim delivered) and four yellow
surfaces — one for each spec auto-narrow condition (a blocked prerequisite, a drifted template
freshness, a partial generation diff, or a secret-bound starter parameter):

| Surface | Claimed → Certified | Binding axis | Trigger |
| --- | --- | --- | --- |
| `scaffold_preflight` | `qualified_starter` → `blocked_prerequisite_projection` | `source_side_effect_and_recovery` | `host_boundary_unstated` |
| `template_health` | `qualified_starter` → `drifted_template_projection` | `degraded_state` | `health_freshness_stale` |
| `generation_diff_review` | `qualified_starter` → `partial_generation_projection` | `degraded_state` | `generated_boundary_blurred` |
| `cli_headless` | `qualified_starter` → `secret_bound_parameter_projection` | `source_side_effect_and_recovery` | `parameter_source_unstated` |

No surface hides drift (red), no surface lets a generic `Create` hide a side effect, no surface
exposes a secret-bound raw value by default, and no surface drops its source / recovery continuity.

## Metadata-only boundary

The packet is metadata-only: typed class tokens, opaque refs, booleans, and redacted labels. Raw
parameter values, secret material, generated file bytes, and credential-bearing material never cross
this boundary (`RawStarterMaterialInExport`). A redacted secret **reference** (naming the parameter
and its source layer) is legitimate governed vocabulary and is retained.

## Regenerating the artifacts

The checked-in export is byte-aligned with the in-code seed builder
(`seeded_m5_scaffold_component_certification_packet`). A drift test fails if they diverge. To
regenerate after an intentional change:

```
GEN_SCAFFOLD_CERT_ARTIFACTS=1 cargo test -p aureline-templates --lib \
  -- certify_scaffold generate_artifacts
```

Then re-run the suite:

```
cargo test -p aureline-templates --lib -- certify_scaffold
```
