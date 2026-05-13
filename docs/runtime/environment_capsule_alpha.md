# Environment-Capsule Alpha Seed

This document defines the alpha seed lane for environment capsules and
workspace templates. It turns launch-bundle setup into one inspectable object
model before the full capsule resolver, template registry, and prebuild service
exist.

## Companion Artifacts

- [`/schemas/runtime/environment_capsule_alpha.schema.json`](../../schemas/runtime/environment_capsule_alpha.schema.json)
  defines the alpha seed schema for `environment_capsule_alpha_record` and
  `workspace_template_seed_manifest`.
- [`/artifacts/templates/workspace_template_seed.yaml`](../../artifacts/templates/workspace_template_seed.yaml)
  is the first seed manifest. It binds the TypeScript web and Python
  launch-bundle rows to concrete capsule refs and template refs.
- [`/crates/aureline-shell/src/start_center/mod.rs`](../../crates/aureline-shell/src/start_center/mod.rs)
  is the first consumer. Start Center projects the seed into reviewable
  template rows and refuses raw environment values.
- [`/ci/check_environment_capsule_alpha.py`](../../ci/check_environment_capsule_alpha.py)
  validates the schema, seed artifact, launch-bundle backrefs, protected
  fixture ids, docs, and shell consumer.
- [`/fixtures/runtime/environment_capsule_alpha/manifest.json`](../../fixtures/runtime/environment_capsule_alpha/manifest.json)
  fixes the protected capsule and template ids used by the validator.

The full capsule body remains
[`/schemas/runtime/environment_capsule.schema.json`](../../schemas/runtime/environment_capsule.schema.json).
The alpha schema is the launch-wedge seed shape that lets templates and launch
bundles point at one vocabulary while the resolver implementation matures.

## Contract

Each alpha capsule names:

- scope: workspace id, scope class, included roots, excluded roots, and cwd ref;
- target plan: target class, capsule location class, host-boundary class, target
  identity ref, requested source artifact, materialized runtime ref, and network
  posture;
- toolchain plan: toolchain class, id, version requirement, activation strategy,
  source ref, and unsupported-gap refs;
- environment variables: variable names, source class, redaction class, digest or
  credential alias, secret class, and `raw_value_included = false`;
- trust and policy: trust state, identity mode, policy epoch, hook gating, and
  explicit no-secret-value default;
- lifecycle hooks: hook refs, effect class, trust gate, and `raw_command_included
  = false`;
- compatibility fingerprint: capsule hash, source digest set, platform/arch,
  policy epoch, and critical toolchain hashes;
- prebuild reuse policy: whether reuse is not applicable, eligible, or rejected
  by drift, policy, or trust.

Each workspace template names its capsule ref, launch-bundle refs, scaffold
template refs, docs refs, bypass paths, setup cost, runtime/toolchain scope, and
prebuild fallback behavior. Launch bundles reference the same template ids under
`workspace_template_refs`, so review surfaces can navigate in either direction.

## Guardrails

- Raw secret values are never present in the alpha seed. Secret-bearing variables
  use `credential_alias_ref` plus `secret_class`.
- Raw command lines are not present on lifecycle hooks. Hooks point to command or
  bootstrap refs that downstream review sheets can resolve.
- Templates and prebuilds are accelerators, not authorities. Stale or drifted
  prebuilds downgrade to cold or partial-warm materialization.
- The Start Center consumer reads the seed; it does not restate toolchain,
  variable, capsule, or template truth in local constants.

## Verification

Run:

```sh
python3 ci/check_environment_capsule_alpha.py --repo-root . --render-template-gallery
cargo test -p aureline-shell start_center::tests::workspace_template_seed
```
