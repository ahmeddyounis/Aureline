# M5 Scaffold-Component Accessibility & Auto-Narrowing (M05-1026)

This lane is the accessibility / keyboard / screen-reader / CLI / export parity and honest
auto-narrowing capstone over the frozen M5 scaffold-component matrix
(`freeze_the_m5_scaffold_template_card_...`). Where the freeze matrix defines the reusable scaffold
template card, starter parameter row, scaffold preflight card, template health row, generated-project
diff card, and scaffold handoff banner primitives — and the 1021–1025 implementation / boundary /
consumer lanes resolve their per-surface truth — this lane certifies, per component family, that
scaffold claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
self-narrowing**.

## What it guarantees

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and CLI/headless-reachable path into the same starter source class,
  support class, host boundary, side-effect disclosure, parameter source, health freshness, and
  generated-versus-user-owned recovery boundary the rich component shows — never a hover-only chip.
  The hierarchy-heavy generated-project diff card (nested created / modified / renamed / deleted file
  tree) additionally binds its tree to a flat list / textual path.
- **Export parity.** The support / release / CLI export reconstructs each component's meaning from
  typed tokens and opaque refs **without a raw value** — never a raw secret parameter value or raw
  generated file payload — preserving the stable component identity, source / support posture,
  side-effect disclosure, health freshness, recovery boundary, and narrowing reasons — so support,
  docs, and release proof can reconstruct exactly what the user was actually shown without leaking a
  blocked secret value.
- **Honest auto-narrowing.** When a template's freshness drifts, a prerequisite health check is
  blocked, a starter parameter is secret-bound and cannot travel, a generation diff's truth is
  partial, or a validation state is cached / not checked, the component's readiness claim auto-narrows
  from `qualified_starter` to a `secret_bound_parameter_projection` / `blocked_prerequisite_projection`
  / `drifted_template_projection` / `partial_generation_projection` / `unchecked_validation_projection`,
  discloses the narrowing with a precise trigger and binding dimension, and preserves the canonical
  starter source / support / recovery boundary. A drifted-template / partial-generation /
  unchecked-validation state can never keep a fully-qualified starter claim — incomplete readiness
  evidence never presents a starter as ready.
- **Cross-surface disclosure.** The same narrowed state surfaces in the start center, template
  gallery, parameter form, preflight, diff review, workspace, health dashboard, CLI, and
  support-export surfaces so product, docs, and release publication stay aligned on downgrade
  behavior.

## Model

- **Readiness claim tiers** (strongest first): `qualified_starter`, `secret_bound_parameter_projection`,
  `blocked_prerequisite_projection`, `drifted_template_projection`, `partial_generation_projection`,
  `unchecked_validation_projection`.
- **Claim dimensions** (1:1 with the six families): `starter_trust_integrity`,
  `parameter_portability`, `prerequisite_health`, `template_freshness`, `generation_diff_evidence`,
  `handoff_validation_clarity`.
- **Condition states**: `starter_verified_ready` (baseline) plus the operational / privacy states
  `secret_bound_parameter` and `prerequisite_blocked`, and the three "cannot-be-proven"
  incomplete-readiness-evidence narrowing axes `freshness_drifted`, `generation_diff_partial`, and
  `validation_stale`.

Each condition state maps 1:1 to a permitted claim ceiling and names an on-topic frozen downgrade
trigger (`parameter_source_unstated`, `host_boundary_unstated`, `health_freshness_stale`,
`generated_boundary_blurred`) so certified reasons stay byte-identical to the freeze matrix. Only the
three cannot-be-proven incomplete-readiness-evidence states can never keep a qualified-starter claim;
a secret-bound parameter and a blocked prerequisite are honest privacy / operational states, not
readiness overstatements.

## Artifacts

- Schema: `schemas/ui/m5-scaffold-component-accessibility-fallback.schema.json`
- Support export (canonical): `artifacts/release/m5-scaffold-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-scaffold-component-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-scaffold-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-scaffold-component-accessibility-fallback/`

Regenerate the checked-in artifacts with:

```
GEN_SCAFFOLD_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-templates generate_artifacts
```
