# Declarative workspace templates

This document describes the declarative workspace-template layer and its
why-this-template inspector. The canonical implementation is
[`crates/aureline-env/src/workspace_templates/mod.rs`](../../crates/aureline-env/src/workspace_templates/mod.rs);
the corpus and expected inspection outcomes are checked in under
[`fixtures/env/workspace-template/`](../../fixtures/env/workspace-template/) and
the human-readable proof is
[`artifacts/env/workspace-template-proof.md`](../../artifacts/env/workspace-template-proof.md).

It builds directly on the typed environment capsule described in
[`docs/env/environment-capsule.md`](environment-capsule.md) and the
environment-capsule governance matrix in
[`docs/env/m5-env-governance.md`](m5-env-governance.md): the capsule lane
*materializes the environment object*, the governance lane *certifies it*, and
this lane *composes a reviewable launch artifact around it without forking the
execution model*.

## Why this exists

Workspace templates and starters used to be opaque code paths with hidden
execution assumptions: a starter could imply a trustworthy environment while it
quietly stood up services, ran lifecycle hooks, or widened the runtime scope. A
reviewer could not see what a template composed or why.

This lane turns templates into declarative, diffable, mirrorable artifacts. A
`WorkspaceTemplate` composes one embedded `EnvironmentCapsule` with the workflow
bundles it expects, the certified-archetype defaults it seeds, and the docs it
links — and nothing it composes can be hidden, because every layer is typed,
digested, and inspected.

## What the template composes

- **`identity`** — a `TemplateIdentity` with a stable id, a monotonic version,
  a label, and a versioned digest of the template's defining inputs.
- **`environment_capsule`** — the embedded typed `EnvironmentCapsule` the
  template hydrates. This is the *same* object a direct local or remote run
  consumes, so hydration cannot fork the runtime or trust semantics.
- **`workflow_bundle_refs`** — typed references to the workflow bundles the
  template expects, each pinned by a digest and carrying a
  `widens_execution_scope` flag that must be `false`.
- **`archetype_defaults`** — certified-archetype defaults, each pinned by a
  digest, so the workspace opens with reviewed defaults.
- **`docs_refs`** — onboarding, start-center, and reference docs the template
  links.
- **`trust`** — a `TemplateTrust` posture: the source class (first-party,
  managed-approved, community, local-draft), the signer class, the mirror class,
  and the evidence state of the attestation backing them.
- **`support`** — a `SupportPosture`: the support class and the freshness state.
- **`guardrails`** — explicit `CompositionGuardrails` booleans, all required to
  be `false`, proving the composition injects no proprietary service
  dependence, no ungated lifecycle hooks, and no hidden bundle / runtime
  widening.

## Hydration reuses the same execution model

`inspect_template` is the single explainability path. It runs the embedded
capsule through the **same** `inspect_environment` engine the capsule lane uses,
then narrows the result by the composition layers:

- each `workflow_bundle_ref`, `archetype_default`, and `docs_ref` contributes
  its `coverage` evidence state,
- the `trust.attestation_state` and `support.freshness_state` contribute their
  evidence states,

and each is folded through the same `EvidenceState` maturity floors the capsule
engine uses — partial narrows to beta, stale to preview, missing withholds. The
result is one `WhyThisTemplate` report carrying the effective maturity, verdict,
narrowing tokens, the embedded `capsule_inspection`, and a per-layer reason
list.

Two invariants keep the template honest:

1. **No widening.** The template's `claimed_maturity` and
   `claimed_warm_start_posture` must equal the embedded capsule's; the
   composition can only narrow from there, never widen.
2. **Warm start belongs to the capsule.** Only the capsule's source digest and
   prebuild fingerprint govern warm reuse, so the composition layers never move
   the warm-start posture. A template inherits the capsule's warm-start
   downgrade rather than inventing its own.

`desktop_template_inspection`, `headless_template_inspection`, and
`support_template_inspection` all delegate to `inspect_template`, so every
surface reads the same object.

## Metadata-first by construction

The template never stores secrets or raw bodies. Every composed layer is reduced
to an id, a digest, and an evidence state, and `export_template_metadata`
projects a redaction-safe `TemplateExport` (always `metadata_only`) that wraps
the canonical inspection and the capsule's own metadata export — never secrets,
raw env bodies, hook commands, or provider payloads.

## Inspect, diff, plan

- **Inspect** — `inspect_template` returns the canonical `WhyThisTemplate`
  report.
- **Diff** — `diff_templates` compares two templates field-by-field (identity,
  trust, support, claim, workflow-bundle / archetype / docs layers) and embeds
  the capsule diff, reporting the changes as metadata tokens.
- **Plan** — `plan_template_change` produces a reviewable, rollback-aware
  `TemplateChangePlan` for `install`, `update`, and `remove`, listing exactly
  which layers the template composes, the diff (for updates), the resulting
  claim, and how to roll the change back.

## Source classes covered

The fixture corpus covers every required source class — `first_party`,
`managed_approved`, `community`, and `local_draft` — each certifying at its
embedded capsule's claim on current evidence, plus degraded variants that drive
the inspector's composition narrowing, withholding, and inherited warm-start
downgrade. See the proof report for the full table.

## Guardrails

- Template composition may not silently inject proprietary service dependence,
  hidden lifecycle hooks, or bundle / runtime widening: the `guardrails` flags
  must all be `false`, a `widens_execution_scope` bundle ref fails validation,
  and a signer class inconsistent with the source class fails validation.
- A template only narrows; the inspector never promotes a template above its
  embedded capsule's claimed maturity or warm-start posture.
- The template does not redesign the execution context; it composes the
  environment *definition* the existing runtime materializes.
