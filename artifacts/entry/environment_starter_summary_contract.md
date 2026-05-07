# Environment-starter summary + starter side-effect diff contract (entry surfaces)

This artifact publishes the canonical **environment-starter summary** and **starter side-effect diff** vocabulary used by the Start Center, template/prebuild entry, open/import flows, and any “starter-assisted” entry lane.

Its goal is cross-surface honesty: starters, templates, and warm starts MUST remain accelerative without hiding side effects, trust widening, port exposure, or lock-in.

This file does **not** define final UI composition. It freezes **what must be disclosed before commit** so every surface can render the same truthful preflight story.

## 1. Canonical sources (quoted by reference)

Normative product requirements:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` (templates/starters/prebuilds and side-effect envelope)
- `.t2/docs/Aureline_Technical_Design_Document.md` (entry/switching rules; managed provisioning honesty)

Starter disclosure and bypass invariants:

- `docs/ux/template_and_prebuild_contract.md` (starter summary axes, bypass paths, “summary before commit”)
- `docs/workflow/starter_side_effect_envelope.md` (diff-first side-effect envelope)
- `artifacts/workflow/open_without_starter_parity_audit.md` (same-weight bypass checklist)

Trust, secrets/auth, and port exposure disclosures (composed by reference):

- `docs/ux/trust_prompt_contract.md`
- `docs/auth/credential_state_and_secret_prompt_contract.md`
- `docs/remote/attach_tunnel_port_forward_contract.md`
- `docs/runtime/container_engine_and_preflight_contract.md`

Machine-readable starter action diff (exportable / supportable boundary):

- `schemas/entry/starter_action_diff.schema.json`
- `fixtures/entry/starter_side_effect_cases/`

## 2. Preflight boundary (must happen before commit)

Every route that advertises a starter lane (template, scaffold, prebuild/warm start, managed provisioning assist, or any “continue with starter” recovery lane) MUST render a **preflight summary** *before* any durable state changes or authority changes occur.

Preflight invariants:

1. **No silent side effects.** The surface MUST NOT install extensions, request secrets/auth, grant trust, expose/forward ports, provision remote resources, or run tasks/scripts merely because the summary rendered.
2. **Commit is explicit.** Any action that will execute automatically MUST be disclosed before the user commits (no “Create” / “Continue” that hides concrete effects).
3. **Cancel is real.** Until commit, the user MUST be able to cancel without mutating workspace files, profile state, trust posture, port exposure, or remote resources.
4. **Bypass is same-weight.** If a surface claims an open-without-starter path exists, it MUST be present at equal weight on that surface (keyboard reachable; not buried behind overflow or reduced-contrast affordances), even when the starter lane is blocked by offline state, policy narrowing, signature review, or trust review.

## 3. Required environment-starter summary fields (minimum set)

Every environment-starter summary MUST answer the questions below using stable fields (not free-form copy-only claims). A surface MAY disclose more, but it MUST disclose at least this minimum set.

### 3.1 Identity and provenance (required)

The summary MUST identify what starter is being offered:

- **Template/starter identity**: a display name for the template/prebuild/starter lane, or the declared runtime image/capsule name when the lane is runtime-first.
- **Source class**: who authored it (first-party, team-managed, community, local-only, mirrored, uncertified) using the vocabulary frozen in `docs/ux/template_and_prebuild_contract.md`.
- **Support class**: how it is supported over time using `support_class` (`docs/ux/template_and_prebuild_contract.md`).
- **Runtime/toolchain scope**: where the resulting runtime/toolchain lives (local/devcontainer/container/remote/managed) using `runtime_and_toolchain_scope` (`docs/ux/template_and_prebuild_contract.md`).

### 3.2 Expected setup cost (required)

The summary MUST disclose the expected cost band:

- `starter_setup_cost_class` and an expected time/connectivity cue (as frozen by the template/prebuild contract).

### 3.3 Starter side-effect diff (required)

The summary MUST include (or link to) a **starter action diff record** that:

- compares a same-weight plain-open lane against the starter lane; and
- partitions actions into **run now** vs **deferred / user-invoked / recommend-only**; and
- names at least one same-weight bypass path.

The diff payload is defined by:

- `schemas/entry/starter_action_diff.schema.json`

### 3.4 Extension install/restore preview (required when applicable)

If the starter lane includes extension install/restore/activation:

- the summary MUST enumerate the extension identities (ids/refs) before commit; and
- the diff MUST include an `extension_install` action row (never collapsed into “prepare environment”).

### 3.5 Task/script execution preview (required when applicable)

If the starter lane will run tasks/scripts/hooks:

- the summary MUST disclose that execution will occur (or that it will be offered later) before commit; and
- the diff MUST include a `script_or_task_execution` action row (never collapsed into “run setup tasks” without naming execution).

### 3.6 Port exposure / forwarding preview (required when applicable)

If the starter lane will expose or forward ports (local binds, forwarded endpoints, remote tunnel routes, shareable preview routes):

- the summary MUST disclose that port exposure will occur before commit; and
- the diff MUST include a `port_exposure` action row.

Port exposure details compose with the port-forward and endpoint contracts; the summary must not embed raw hostnames, raw URLs, or raw port numbers when privacy-reduction rules forbid them.

### 3.7 Secrets/auth and trust prompts (required when applicable)

If the starter lane requires secret handles, browser sign-in, auth callbacks, trust grants, or admission widening:

- the summary MUST disclose the prompts that will appear before any prompt is triggered; and
- the diff MUST include `secret_or_auth_request` and/or `trust_grant` action rows, with deferred actions marked as requiring later reapproval when applicable.

## 4. Non-conforming examples (for reviewers)

The following behaviors violate this contract:

- a generic `Create` / `Continue` action that installs extensions, grants trust, exposes/forwards ports, requests secrets/auth, provisions remote resources, or runs tasks/scripts without disclosing those effects before commit;
- a preflight summary that lists actions but does not distinguish **run now** vs **deferred / user-invoked / recommend-only**;
- a surface that claims “Open without starter” exists but hides it behind overflow, reduced contrast, or below-the-fold placement;
- a bypass path that disappears when the starter lane is blocked by offline state or policy narrowing.

