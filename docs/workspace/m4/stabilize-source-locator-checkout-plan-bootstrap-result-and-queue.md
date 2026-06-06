# Stable Project-Entry Acquisition Truth

This contract stabilizes the source-locator, checkout-plan, bootstrap-result, and bootstrap-queue objects used by project-entry lanes. Desktop, CLI/headless, deep-link, restore, policy-guided deployment, diagnostics, Help/About, migration, and support-export surfaces read the same packet:

- schema: [`/schemas/workspace/source-locator-checkout-plan-bootstrap-result.schema.json`](../../../schemas/workspace/source-locator-checkout-plan-bootstrap-result.schema.json)
- Rust projection: `aureline_workspace::stabilize_source_locator_checkout_plan_bootstrap_result_and_queue`
- fixture: [`/fixtures/workspace/m4/stabilize-source-locator-checkout-plan-bootstrap-result-and-queue/interrupted_partial_mirror_resume_packet.json`](../../../fixtures/workspace/m4/stabilize-source-locator-checkout-plan-bootstrap-result-and-queue/interrupted_partial_mirror_resume_packet.json)

## Contract

The stable packet wraps the existing acquisition seed records without replacing them:

- `source_locator` preserves locator kind, canonical target ref, forge/mirror/proxy/import class, protocol, target-ref intent, auth-source class, and trust seed hint.
- `checkout_plan` preserves destination ref, full/partial/sparse/shallow/live/import mode, clone depth/filter posture, sparse profile ref, submodule/LFS mode, cost band, resumability token, and policy/mirror constraints.
- `credential_descriptor` is handle-only. It may name credential handle refs, approval-ticket refs, proxy/mirror route refs, delegated-auth mode, host-key/TLS posture, offline fallback rule, and reauth state. It must not carry raw secrets.
- `bootstrap_result` preserves outcome lineage, resulting roots, warnings, evidence refs, interrupted/completed/partial-authority state, next-step queue refs, and trust stage.
- `bootstrap_queue` keeps each deferred setup item attributable, resumable/cancelable/reviewable, and scoped to a checkout plan instead of collapsing setup into one opaque task.

## Honesty Rules

Partial acquisition, sparse checkout, shallow history, interrupted resume, mirror/offline posture, queued setup, and non-admitted trust all force `partial_authority: true`. A packet with partial authority may not report `completion_state: completed`.

Mirror/proxy/offline sources preserve `mirrored` in `outcome_lineage`; interrupted partial acquisitions preserve `partially_acquired`; local opens preserve `opened`; import/archive/template paths preserve `imported`; restore/live-session paths preserve `resumed`.

Every queue item must carry an evidence ref or evidence summary. Support export renders the typed queue rows directly rather than inferring meaning from logs.
