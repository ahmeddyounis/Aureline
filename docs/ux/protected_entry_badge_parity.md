# Protected-entry badge parity audit (terminal, tasks, debug, remote attach, provider entry, browser handoff)

Protected entry points (terminal paste review, task launch, debug launch/attach, remote attach/resume, provider-bearing actions, and browser handoff) cross **host boundaries**, **ownership domains**, and **authority planes**. This audit freezes one parity contract so target/origin/route badges do not drift across those entry surfaces.

This document is normative. Where it disagrees with upstream contracts in `.t2/docs/` or the component contracts it cites, the upstream source wins and this audit (plus its matrix and fixtures) MUST update in the same change.

Machine-readable companions:

- [`/artifacts/ux/protected_entry_badge_matrix.yaml`](../../artifacts/ux/protected_entry_badge_matrix.yaml)
  — the badge-family definitions, ordering rules, tooltip field requirements, and surface parity joins.
- [`/fixtures/ux/protected_entry_badge_cases/`](../../fixtures/ux/protected_entry_badge_cases/)
  — parity cases covering identical route across entry surfaces, browser-only continuation, provider-backed entry with local fallback, wrong-target recovery, and restricted/policy-limited entry.

This audit composes with (and does not replace):

- [`/docs/ux/command_diagnostics_contract.md`](./command_diagnostics_contract.md) and
  [`/schemas/commands/diagnostic_projection.schema.json`](../../schemas/commands/diagnostic_projection.schema.json)
  — the canonical `protected_entry_badge_record` shape and its frozen vocabularies.
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md) and
  [`/artifacts/runtime/action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml)
  — the canonical origin/target/route/exposure vocabulary; badges quote these tokens rather than surface-local copy.
- [`/docs/ux/host_identity_contract.md`](./host_identity_contract.md),
  [`/docs/verification/target_and_host_boundary_packet.md`](../verification/target_and_host_boundary_packet.md),
  [`/artifacts/remote/host_boundary_matrix.yaml`](../../artifacts/remote/host_boundary_matrix.yaml), and
  [`/artifacts/runtime/target_graph_badge_projection.yaml`](../../artifacts/runtime/target_graph_badge_projection.yaml)
  — host-boundary cue stacks, wrong-target posture, and the canonical host-boundary label set.
- [`/docs/execution/terminal_truth_contract.md`](../execution/terminal_truth_contract.md),
  [`/docs/execution/run_and_attempt_contract.md`](../execution/run_and_attempt_contract.md), and
  [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md)
  — entry-specific truth models that the protected-entry badge must agree with (never mint parallel “where did this run?” fields).
- [`/docs/providers/provider_link_header_and_handoff_contract.md`](../providers/provider_link_header_and_handoff_contract.md) and
  [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — browser handoff packets, reason codes, destination classes, and provider actor vocabulary used by `browser_handoff_entry`.
- [`/docs/ux/badge_pill_contract.md`](./badge_pill_contract.md)
  — density budget, overflow affordances, accessibility, and “never hue-only” constraints.
- [`/docs/ux/capability_lifecycle_badge_contract.md`](./capability_lifecycle_badge_contract.md)
  — stale/degraded vocabulary and evidence-aging downgrade rules.

## Scope

This audit governs the **protected-entry badge cluster**: the small, repeatable set of badges/chips/pills that answer:

1. **Target** — what is being targeted (shell, task runner, debug adapter, remote attach session, provider mutation, browser handoff).
2. **Host boundary & trust** — what boundary is crossed and what trust posture applies.
3. **Origin & actor** — who/what is initiating and (for provider-bearing lanes) which provider actor identity is in use.
4. **Route & exposure** — how the user arrived at the entry and whether exposure/risk is local-only vs externally visible.
5. **Client scope** — which client/runtime plane is entitled to render or execute the entry.
6. **Browser-handoff state** — when the entry transitions into a system-browser handoff, the handoff packet remains inspectable.
7. **Degraded/stale** — when the underlying route/target/provider is stale or degraded, the downgrade is explicit (not implied by missing UI).

Out of scope: final widget styling, per-platform icon artwork, and any “perfect” layout for every badge stack. This audit freezes semantics, ordering, and field sources.

## Badge families (what each badge answers)

The badge cluster is a composition of **families**. Families are stable; surfaces MUST NOT introduce ad-hoc synonyms.

### 1) Target family (what is being touched)

Canonical owner: `protected_entry_badge_record.target_kind_class` and `protected_entry_badge_record.target_identity_ref`.

- Required across all protected entries.
- Must be export-safe: labels are resolved through `export_safe_target_label_ref` and MUST follow the row’s `export_safe_wording_class`.
- Must never expose raw hostnames, raw URLs, raw account handles, or raw project ids; those remain behind resolvers keyed by `target_identity_ref`.

### 2) Host boundary & trust family (what boundary is crossed)

Canonical owners:

- `protected_entry_badge_record.target_trust_class` (trust posture of the target lane), and
- host-boundary cue stacks referenced by the entry’s canonical target artifacts (e.g. `host_identity_chip_record.route_truth` and the host-boundary matrices).

Rules:

- A surface MUST NOT collapse `browser_runtime_bridge` / `service_plane` / `managed_workspace` into generic “remote” in exports or logs.
- When the host boundary is unknown, the correct rendering is `unknown_requires_review` (block mutation until repaired), not a silent “local” assumption.

### 3) Origin family (where the entry was initiated)

Canonical owners:

- `protected_entry_badge_record.origin_authority_class` (authority plane: user/AI/extension/automation/collaboration/admin-policy), and
- `protected_entry_badge_record.target_route_class` (entry route: palette/menu/keybinding/CLI/AI/deep-link/restore).

Rules:

- The route badge is **not** a surface-local label; it is a controlled vocabulary token.
- Deep links and restored-session reconnects always reopen origin/target disclosure (even when other fields would normally allow direct apply).

### 4) Actor family (who the entry acts as, provider-bearing lanes)

Canonical owner: `protected_entry_badge_record.origin_actor_class` (ADR-0010 provider actor class).

Rules:

- Provider-bearing and browser-handoff entries MUST name the actor class explicitly; a generic “Connected” claim is forbidden.
- Actor class is distinct from the *local* authority class; both may be present in the tooltip/narration.

### 5) Route/exposure family (how work becomes visible)

Canonical owners:

- entry route: `protected_entry_badge_record.target_route_class`, and
- exposure/risk: the canonical route-truth / capability artifacts (e.g. command scope, preview class, and route-exposure taxonomies).

Rule:

- If the entry is externally visible or irreversible, the entry surface MUST render an explicit “protected” disclosure (preview/approval posture) alongside the badge cluster; it MUST NOT rely on a badge color or an icon-only cue.

### 6) Client-scope family (which clients can render/execute)

Canonical owner: `protected_entry_badge_record.origin_client_scope`.

Rules:

- Client-scope mismatches are explicit: if a client cannot execute an entry (e.g. CLI vs desktop), the entry is disabled with the typed reason `client_scope_excludes_surface`; the badge cluster must not be removed as a side-effect.

### 7) Browser-handoff state family (when the entry leaves the product)

Canonical owner:

- the `browser_handoff_packet_record` (ADR-0010) referenced by `target_identity_ref` for `browser_handoff_entry` targets.

Minimum required fields to keep inspectable (in tooltip, details, export, and logs):

- `destination_class`, `reason_code`, `expected_authority_on_destination`, `replay_posture`, and a return anchor/return target label.

### 8) Degraded/stale family (when truth is narrowed)

Canonical owners:

- `status_pill` / capability-lifecycle badges for degraded/stale vocabulary, and
- target availability/freshness projections (e.g. target-graph availability `stale/partial/unavailable`).

Rule:

- When a protected entry’s target/provider/route is degraded or stale, the entry surface must render an explicit downgrade pill (e.g. `Stale`, `Degraded`, `Blocked`) rather than letting the badge silently disappear or implying the issue is “just UI”.

## Ordering and tooltip discipline

Ordering is fixed so reviewers can compare screenshots and logs mechanically:

1. **Degraded/Stale** (if present) — lead with the narrowing cue.
2. **Host boundary & trust** — the boundary must be visible before the action label.
3. **Target** — what will be affected.
4. **Actor** (provider-bearing lanes) — who the provider-side actor is.
5. **Origin/Route** — how the user arrived (palette/menu/keybinding/CLI/AI/deep-link/restore).
6. **Client scope / floors** — shown inline only when it changes availability; otherwise in tooltip/details.

Tooltip/narration rules:

- Tooltips MUST be field-derived (label refs + canonical packet fields); free-text tooltips are non-conforming.
- The accessibility narration referenced by `badge_narration_ref` must include (at minimum) the visible family labels plus any hidden families folded into overflow.
- Export-safe representations MUST quote class tokens and label refs; they MUST NOT scrape UI text.

## Surfaces under audit (where parity must hold)

The following entry surfaces MUST render the same badge semantics for the same underlying route/target/boundary:

- **Terminal**: terminal header strip, paste safety sheet, terminal context menu protected actions.
- **Tasks**: task launcher, task history rows that include a run-here / rerun entry, and any protected task apply surface.
- **Debug**: debug launcher and attach sheets, rerun/reattach affordances, and debug “open in browser” fallbacks.
- **Remote attach**: attach/resume entry cards (including restored-session reconnect).
- **Provider object entry**: provider tiles/cards and provider-backed action rows (including local fallback states).
- **Embedded docs links**: embedded docs/help lanes that deep-link to protected entry actions; links must not mint surface-local route labels.
- **Browser handoff**: the handoff sheet and its audit/event trail; the handoff packet is the only URL-bearing source.

## Parity cases

The case corpus under [`/fixtures/ux/protected_entry_badge_cases/`](../../fixtures/ux/protected_entry_badge_cases/) exercises:

- the same entry route token (`palette_invocation_route`) shown across multiple protected entry surfaces;
- browser-only continuation with typed handoff packet preservation;
- provider-backed entry with truthful local fallback and degraded state disclosure;
- wrong-target recovery and reapproval-required presentation that preserves target/origin truth; and
- restricted/policy-limited entry where the badge remains inspectable in support exports and logs.

## Acceptance checklist

A surface set conforms when:

1. The same route/host-boundary token projects the same badge semantics regardless of entry surface.
2. Browser-handoff, provider-entry, and local-only cases remain inspectable in screenshots, support exports, and textual logs (no hue-only or hover-only meaning).
3. Reviewers can compare two entry surfaces and explain any intentional difference (density/overflow is allowed; semantic drift is not).
