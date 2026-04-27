# High-risk field rules contract

This document freezes the shared contract for fields whose value cannot
be treated as ordinary text. It exists so settings, request workspaces,
package-manager auth, notebook kernels, transport configuration, repair
flows, and support exports all render, validate, redact, copy, export,
and apply high-risk values through one rule model.

The contract is normative. Where this document disagrees with the
source UI / UX, architecture, security, transport, request, package, or
repair contracts it cites, the source contract wins and this document,
its schema, matrix, and fixtures update in the same change. Where this
document disagrees with a downstream surface's private widget behavior,
this document wins and the surface is non-conforming.

Companion artifacts:

- [`/schemas/ux/field_rule.schema.json`](../../schemas/ux/field_rule.schema.json)
  - boundary schema for one `field_rule_record`. It binds a field-rule
    class to value representation, evaluation context, validation hooks,
    review affordances, unsafe-value warnings, multi-value semantics, and
    export posture.
- [`/artifacts/ux/field_risk_classes.yaml`](../../artifacts/ux/field_risk_classes.yaml)
  - machine-readable matrix for the reserved risk classes, rule-family
    defaults, cross-surface mapping, and required warning semantics.
- [`/fixtures/ux/field_rule_cases/`](../../fixtures/ux/field_rule_cases/)
  - worked YAML records for secret handles, raw secret input, filesystem
    paths, URI endpoints, code-backed expressions, command templates,
    environment refs, certificate refs, and provider/resource ids.

This contract composes with, and does not replace:

- [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md)
  for staged review, validation classes, probe attribution, stale/skipped
  admission, and apply gating.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  and [`/artifacts/security/secret_class_rows.yaml`](../../artifacts/security/secret_class_rows.yaml)
  for secret classes, storage modes, projection modes, and redaction
  defaults.
- [`/docs/fs/path_truth_packet.md`](../fs/path_truth_packet.md)
  for canonical path, presentation path, alias, symlink, and path-truth
  disclosure.
- [`/docs/api/request_workspace_contract.md`](../api/request_workspace_contract.md)
  for request environments, auth source classes, stale schema handling,
  retarget protection, and raw-secret denial.
- [`/docs/package/package_action_contract.md`](../package/package_action_contract.md)
  for registry auth, script risk, lockfile impact, mirror/offline state,
  and rollback review.
- [`/docs/network/transport_governance_seed.md`](../network/transport_governance_seed.md)
  for proxy, trust-store, certificate, endpoint, route, and egress
  posture.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  for repair preview, checkpoint, reversal, forbidden-action, and support
  handoff rules.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for high-risk preview, representation-labeled copy/export, authority
  disclosure, and deny-closed behavior.

Normative source sections projected here include the form and
parameter-source templates in
`.t2/docs/Aureline_UI_UX_Spec_Document.md`, the secret-broker,
execution-context, transport, request, package, notebook, and repair
matrices in `.t2/docs/Aureline_Technical_Design_Document.md`, and the
schema-governance and export/redaction guardrails in
`.t2/docs/Aureline_PRD.md`.

## Scope

This contract applies whenever a field value can:

- carry, reference, reveal, or project a secret;
- point at filesystem, archive, remote, generated, or provider-owned
  state;
- resolve differently depending on workspace root, current working
  directory, selected target, profile, environment layer, kernel,
  request runtime, package manifest, trust store, or policy epoch;
- evaluate code, shell text, templates, expressions, assertions, or
  commands;
- change network target, certificate, route, proxy, registry, endpoint,
  account, tenant, or provider/resource identity;
- participate in drag/drop, bulk paste, multi-value list editing,
  import/export, support handoff, or copy to clipboard.

Low-risk text fields may stay outside this contract only while the
value is inert, local, non-secret, non-executable, non-targeting, and
export-safe as entered. The moment a surface adds a secret posture, a
targeting posture, an evaluation posture, or a copy/export restriction,
it emits a `field_rule_record`.

## Rule Families

Every high-risk field names exactly one `field_rule_class`.
The class is the risk family, not a collapse into one generic widget.
Each record also names `value_shape_classes` so later file-vs-folder,
generic path, JSON, YAML, code, endpoint, key/value, and multi-value
surfaces can reuse the same contract without losing shape-specific
behavior.

| Rule family | Primary risk | Required context before apply | Default export posture |
|---|---|---|---|
| `secret_handle_field` | Secret projection through a broker or vault handle. | Storage mode, secret class, consumer scope, projection mode, and reveal policy. | Handle type or alias only; never raw secret body. |
| `raw_secret_input` | User-entered credential material before broker admission. | Session-only entry surface, intended secret class, destination store, and conversion/deny path. | Excluded; support export records detection and omission only. |
| `filesystem_path_ref` | Path authority, aliasing, portability, or path escape. | Absolute/relative basis, root authority, canonical vs presentation path, and target execution location. | Path with basis or redacted path summary, depending on scope. |
| `uri_endpoint` | Network target, retargeting, credential leakage, or egress authority. | Endpoint class, origin scope, route/egress class, auth posture, and policy epoch. | Endpoint summary with sensitive components omitted. |
| `code_backed_expression` | Evaluated logic, query, assertion, script fragment, or generated expression. | Runtime/evaluator, language or format, authority scope, input refs, and preview/diff basis. | Source ref plus review metadata; raw source only under explicit review. |
| `command_template` | Shell or process execution, interpolation, and target mutation. | Execution location, shell/process class, variables, working directory, environment delta, and preview/dry-run posture. | Template ref plus redacted variable summary. |
| `environment_ref` | Variable precedence, secret shadowing, and environment-dependent behavior. | Layer source, precedence, resolution time, winning scope, and unresolved/shadowed keys. | Keys, sources, and redaction report; values only when safe. |
| `certificate_ref` | Trust material, identity proof, expiry, and rotation. | Store/bundle source, fingerprint, issuer/epoch, scope, expiry, and refresh rule. | Fingerprint, handle ref, issuer/epoch; never private key material. |
| `provider_resource_identifier` | Account, tenant, project, registry, package, model, cluster, or resource targeting. | Provider class, account/tenant scope, stable id path, display label, freshness, and duplicate policy. | Stable id path and display label when policy allows; no embedded credentials. |

A surface that collapses these rule families into a generic text box is
non-conforming. A surface may render a compact control, but the full
field-rule record must remain reachable by review sheets, CLI/headless
output, support export, and validators.

## Shared Expectations

### Review affordances

Every field-rule record declares the affordances a surface must show
before apply. Common affordances include:

- `storage_mode_label` for secret-bearing and certificate-backed fields;
- `basis_disclosure` for paths, commands, and evaluated expressions;
- `scope_authority_disclosure` for endpoints, provider/resource ids,
  repair actions, package auth, notebook kernels, and support export;
- `preview_diff` or `dry_run_preview` for code-backed expressions,
  command templates, package actions, repair flows, and broad settings;
- `redaction_preview` for any value that may be copied, exported,
  shared, attached to support, or sent through CLI/headless output;
- `duplicate_resolution` for multi-value lists where order, id, alias,
  or repeated values matter;
- `lossy_normalization_warning` when a surface canonicalizes case,
  Unicode, path separators, URL escaping, header casing, certificate
  formatting, or provider ids in a way the original value cannot be
  reconstructed from the normalized display.

If a required affordance cannot render in the current layout, the
surface promotes to a sheet, full review, CLI JSON summary, or support
packet projection. Tooltip-only disclosure is insufficient for
high-risk field behavior.

### Redaction and export

Secret-bearing fields are never downgraded to plaintext export by a
presentation choice. Raw secret bytes, raw delegated tokens, private
keys, raw certificate material, raw request bodies, raw auth headers,
raw registry credentials, raw proxy credentials, and raw provider
payloads stay out of default export. A field may expose an elevated
export path only when the field-rule class explicitly allows it and the
review surface records the class change, destination class, omitted
data classes, and redaction report.

Support exports must preserve enough metadata to diagnose behavior:
field rule class, risk class, source class, storage mode, validation
state, evaluation context, freshness, authority scope, redaction mode,
and omitted-value reason. They do not need the dangerous value itself.

### Evaluation context

Path-backed and code-backed values must declare evaluation context and
risk before apply. The record names:

- who resolves the value (`resolver_actor_class`);
- where it resolves (`execution_location_class`);
- when it resolves (`resolution_time_class`);
- which basis fields matter, such as workspace root, containing file,
  current working directory, active profile, environment layer,
  selected target, request environment, package manifest scope, kernel
  identity, certificate store, provider account, or support-bundle
  scope.

When context is missing, stale, or ambiguous, mutation-class apply is
blocked or requires explicit stale/skipped admission from the staged
review contract. No path, command, expression, endpoint, certificate,
or provider/resource id may be treated as an inert string when context
changes its meaning.

### Validation hooks

Field rules use the validation classes from the form-validation
contract rather than minting local "valid" flags. Typical hooks:

- `local_syntax` for parse shape, path shape, URI form, env key form,
  expression parse, command template parse, or certificate fingerprint
  shape;
- `schema` for settings definitions, request schemas, package
  manifests, environment-layer schemas, or provider resource schemas;
- `secret_resolution` for broker handle existence, projection mode,
  delegated credential validity, and raw-secret denial;
- `external_probe`, `remote_auth`, and `target_discovery` for endpoints,
  registries, kernels, transport routes, provider resources, and
  certificate stores;
- `policy` for authority ceilings, egress limits, export limits, or
  managed lock states;
- `dry_run` for command templates, package actions, repair
  transactions, request replay, and broad code-backed changes.

A validation hook that would mutate target state is not a validation
hook. It must become a preview/apply or dry-run record with declared
side effects.

### Unsafe-value warnings

Unsafe-value warnings are structured triggers, not prose. The shared
trigger vocabulary covers raw secret literals, plaintext export
attempts, missing relative-path basis, path escapes or symlink aliases,
target-context path misses, endpoint retargeting, evaluated code with
workspace authority, shell interpolation, unresolved or shadowed
environment refs, expired/untrusted certificates, ambiguous provider
resources, duplicate collisions, lossy normalization, bulk paste, drag
and drop, and policy-narrowed authority.

Warnings are review inputs. They do not silently transform the value.
If the safe path is to normalize, the surface preserves a raw ref or
reports the lossy normalization. If the safe path is to reject, the
surface blocks with the structured warning trigger and recovery path.

### Scope and authority disclosure

Every high-risk field must disclose the scope it affects and the actor
or authority that can use it. Examples:

- secret handle fields show consumer class, projection mode, and reveal
  posture;
- path refs show root authority, remote/local location, and alias state;
- endpoint fields show route, origin scope, and egress class;
- command templates show shell/process class, working directory, and
  environment delta;
- environment refs show layer precedence, winner, and unresolved keys;
- certificate refs show store source, issuer/epoch, fingerprint, and
  expiry;
- provider/resource ids show provider, account/tenant, stable id path,
  and duplicate policy.

## Cross-surface Mapping

| Surface | Rule classes typically used | Non-negotiable behavior |
|---|---|---|
| Settings | `secret_handle_field`, `filesystem_path_ref`, `uri_endpoint`, `environment_ref`, `certificate_ref`, `provider_resource_identifier` | Effective source, policy lock, redaction class, restart/apply timing, and export posture stay visible. Raw secrets are excluded from profiles and support bundles. |
| Request workspaces | `secret_handle_field`, `raw_secret_input`, `uri_endpoint`, `environment_ref`, `code_backed_expression`, `provider_resource_identifier` | Auth values resolve through handles; raw bearer/header secrets deny or convert through review. Endpoint retargeting, stale schema, and environment precedence are reviewable before send/replay. |
| Package-manager auth | `secret_handle_field`, `raw_secret_input`, `uri_endpoint`, `command_template`, `filesystem_path_ref`, `provider_resource_identifier` | Registry credentials use broker/delegated handles. Script/native build templates disclose execution context and cannot appear as metadata-only changes. Manifest scope and lockfile basis remain visible. |
| Notebook kernels | `filesystem_path_ref`, `code_backed_expression`, `command_template`, `environment_ref`, `provider_resource_identifier` | Kernel identity, interpreter/runtime, working directory, environment, and trust state are declared before execution or re-run. Captured output never implies live kernel authority. |
| Transport config | `uri_endpoint`, `certificate_ref`, `secret_handle_field`, `filesystem_path_ref`, `provider_resource_identifier` | Proxy, trust store, CA bundle, SSH host proof, client-certificate binding, route, egress, and policy epoch are inspectable. Certificate export uses refs/fingerprints only. |
| Repair flows | `filesystem_path_ref`, `command_template`, `environment_ref`, `certificate_ref`, `provider_resource_identifier` | Preview, checkpoint, reversal class, forbidden-action set, and affected state classes are visible before apply. Skipped or stale probes route through the staged-review gate. |
| Support exports | all rule classes | Export preserves metadata, context, validation state, redaction report, and omitted-value reasons. It does not reconstruct raw secret bodies or private trust material. |

## Multi-value, paste, drop, and normalization semantics

Fields that accept more than one value must declare:

- whether bulk paste is supported, blocked, or promoted to review;
- whether drag/drop is compatible with the target context;
- how duplicates are handled: rejected, preserved with warning, merged
  by stable id, or accepted only with review;
- whether order is meaningful;
- whether normalization is lossless or lossy.

Relative paths require a declared basis. Valid bases include workspace
root, containing file, selected target, current working directory, or an
explicit user-chosen basis. A relative path without a basis cannot
apply. Drag/drop of a path into a remote, package, notebook, or repair
target must disclose whether the dropped path is local-client,
workspace-host, remote-target, archive-internal, or provider-owned.

Bulk paste into secret, command, code, endpoint, environment, or
provider/resource fields must not silently split, deduplicate, trim, or
normalize values. The record must name the split rule, duplicate rule,
lossy normalization posture, and review surface used before apply.

## Conformance

A conforming surface:

1. Emits one `field_rule_record` for every high-risk field.
2. Reuses the frozen `field_rule_class`, `risk_class`, validation,
   redaction, review-affordance, evaluation-context, warning-trigger,
   and export-posture vocabularies.
3. Blocks mutation-class apply when required evaluation context is
   missing, stale, unsupported, or policy-blocked.
4. Preserves raw-secret exclusion by default and records any elevated
   export path as a reviewable class change.
5. Routes drag/drop, bulk paste, duplicate handling, and lossy
   normalization through the same field rule as ordinary manual entry.
6. Projects the same rule metadata to desktop UI, CLI/headless output,
   support export, and future inspectors.

Adding a new enum value is additive-minor and requires updates to the
schema, this document, the risk-class matrix, and at least one fixture.
Repurposing an existing value is breaking and requires a new governance
decision row before any surface consumes it.
