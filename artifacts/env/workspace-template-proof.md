# Workspace-template proof

This report is the human-readable proof for the declarative workspace-template
layer that composes a typed environment capsule with workflow-bundle
references, certified-archetype defaults, and docs / onboarding references. The
canonical implementation is
[`crates/aureline-env/src/workspace_templates/mod.rs`](../../crates/aureline-env/src/workspace_templates/mod.rs);
the template corpus and its expected inspection outcomes are checked in under
[`fixtures/env/workspace-template/`](../../fixtures/env/workspace-template/) and
validated by `crates/aureline-env/tests/workspace_template.rs`.

## What the template is

A `WorkspaceTemplate` is a declarative, diffable, mirrorable launch artifact —
not an opaque starter code path. It composes, as inspectable data:

- a `TemplateIdentity` — id, version, label, and a versioned digest,
- the embedded `EnvironmentCapsule` it hydrates (the same typed object the rest
  of the environment-truth lane consumes),
- typed `workflow_bundle_refs` — workflow bundles the template expects, each
  pinned by a digest and flagged as never widening the execution scope,
- `archetype_defaults` — certified-archetype defaults the template seeds,
- `docs_refs` — onboarding, start-center, and reference docs the template links,
- a `trust` posture — source / signer / mirror class plus an attestation
  evidence state,
- a `support` posture — support class and freshness, and
- explicit `guardrails` proving the composition injects no proprietary service
  dependence, no ungated lifecycle hooks, and no hidden bundle / runtime
  widening.

The template never stores secrets or raw bodies: every composed layer is
reduced to an id, a digest, and an evidence state.

## Hydration does not fork the execution model

The template *embeds* the typed `EnvironmentCapsule`. `inspect_template` folds
that embedded capsule through the **same** `inspect_environment` path a direct
local or remote run uses, and only then narrows the result by the composition
layers and the trust posture. The why-this-template report carries the embedded
`capsule_inspection` verbatim, so a template can never tell a greener story than
the environment it composes — and a stale fingerprint or ungated hook in the
capsule downgrades the template exactly as it downgrades the capsule.

The composition layers can only **narrow** the claim:

- The maturity floor is taken from the same `EvidenceState` floors the capsule
  engine uses — partial evidence narrows to beta, stale to preview, missing
  withholds.
- Warm start is governed **only** by the capsule (its source digest and prebuild
  fingerprint), so the composition layers never touch the warm-start posture.

The template's claimed maturity and warm-start posture must equal the embedded
capsule's; a template cannot claim more than the environment it hydrates.

## One inspector, one engine

`desktop_template_inspection`, `headless_template_inspection`, and
`support_template_inspection` all read the **same** `WhyThisTemplate` object, so
desktop, CLI / headless, and support narrow identically.
`export_template_metadata` projects a redaction-safe support view (ids, digests,
classes, states only) that wraps both the template inspection and the capsule's
own metadata export.

## Certified corpus

| Source class | Template | Embedded capsule | Claimed | Effective | Verdict | Warm start |
| --- | --- | --- | --- | --- | --- | --- |
| `first_party` | `env.template.first_party` | `env.capsule.local` | `stable` | `stable` | `certified` | `cold_build` → `cold_build` |
| `managed_approved` | `env.template.managed_approved` | `env.capsule.managed_workspace` | `beta` | `beta` | `certified` | `warm_full_reuse` → `warm_full_reuse` |
| `community` | `env.template.community` | `env.capsule.vm` | `stable` | `stable` | `certified` | `warm_partial_reuse` → `warm_partial_reuse` |
| `local_draft` | `env.template.local_draft` | `env.capsule.local` | `stable` | `stable` | `certified` | `cold_build` → `cold_build` |

Every required source class — first-party, managed-approved, community, and
local-draft — is represented by a template that certifies at its embedded
capsule's claim on fully current evidence and a clean guardrail posture.

## Failure / recovery scenarios

| Scenario | Source class | Injected | Verdict | Maturity | Warm start |
| --- | --- | --- | --- | --- | --- |
| `community_attestation_partial` | `community` | partial trust attestation | `narrowed` | `stable` → `beta` | `warm_partial_reuse` (unchanged) |
| `managed_workflow_bundle_stale` | `managed_approved` | stale workflow-bundle reference | `narrowed` | `beta` → `preview` | `warm_full_reuse` (unchanged) |
| `local_draft_attestation_missing` | `local_draft` | missing attestation | `withheld` | `stable` → `withdrawn` | `cold_build` |
| `community_capsule_fingerprint_stale` | `community` | stale embedded-capsule fingerprint | `narrowed` | `beta` → `preview` | `warm_full_reuse` → `cold_build` |

These prove the guardrails end-to-end:

- **Composition narrows, never silently widens.** A partial trust attestation or
  a stale workflow-bundle reference narrows the maturity through the composition
  layer, and the guardrail flags (`injects_proprietary_service_dependence`,
  `introduces_ungated_lifecycle_hooks`, `widens_bundle_or_runtime_scope`) must
  all stay `false` or the template fails validation.
- **A draft is withheld, not faked.** A local draft with missing attestation is
  withheld rather than presented as installable.
- **The template inherits the capsule's downgrade.** A stale embedded-capsule
  fingerprint narrows the template to `preview` and forces a cold build, because
  the template runs the capsule through the same engine instead of forking it.
- **Composition does not move warm start.** The partial-attestation and
  stale-bundle scenarios narrow maturity while leaving the warm-start posture
  untouched, because only the capsule governs warm reuse.

## Install / update / remove review

`diff_templates` compares two templates field-by-field (identity, trust, support,
claim, composition layers) and embeds the capsule diff, and `plan_template_change`
produces a reviewable, rollback-aware `TemplateChangePlan` for `install`,
`update`, and `remove`. Each plan lists exactly which layers the template
composes, carries the diff (for updates), reports the resulting claim, and
explains how to roll the change back.

## How to verify

```
cargo test -p aureline-env
cargo run -p aureline-env --example dump_workspace_template fixtures
```
