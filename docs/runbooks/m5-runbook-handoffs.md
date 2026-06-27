# Runbook control-plane handoffs

Runbooks routinely cross out of Aureline's governed plane into provider consoles
and browser surfaces. Left implicit, that pivot can read as a hidden escape from
Aureline truth, or a browser reference document can quietly present itself as if it
were executable in-product control. This document is the contract for the
**control-plane handoff** model: how Aureline represents a pivot to a console or
browser surface as a first-class, attributable transition, and how a destination's
**reference-plane state** stops read-only documentation from masquerading as
in-product control.

The crate `aureline-runbooks` owns the model. The handoff packet itself is a
governance object (`m5_runbook_governance`,
[`ControlPlaneHandoffPacket`]); the `m5_runbook_handoffs` module publishes the
register every consuming surface reads. The machine-readable inventory lives at
[`artifacts/runbooks/m5-runbook-handoff-register.json`](../../artifacts/runbooks/m5-runbook-handoff-register.json)
(human summary:
[`artifacts/runbooks/m5-runbook-handoff-register.md`](../../artifacts/runbooks/m5-runbook-handoff-register.md)),
and the schema is
[`schemas/runbooks/m5-runbook-handoff-register.schema.json`](../../schemas/runbooks/m5-runbook-handoff-register.schema.json).
The handoff packet's source of truth is the execution schema it is embedded in,
[`schemas/runbooks/m5-runbook-execution.schema.json`](../../schemas/runbooks/m5-runbook-execution.schema.json).

## A pivot is a first-class, attributable transition

A console/browser pivot is never a hidden escape. Every handoff packet names:

- a **destination class** — what the far side actually is;
- a **reason class** — why the pivot happens;
- the **object identity** it crosses to (`destination_object_ref`) and the
  **attribution** of the pivot to a session/actor;
- a **return anchor** — the initiating Aureline object, plus the target and
  evidence identity preserved across the pivot, so the operator returns to the
  runbook/incident without losing context;
- any **narrowed authority** that applies on the far side (for example, read-only
  console access).

A handoff never mints a hidden privileged mutate channel
(`creates_hidden_mutate_channel` is always false).

### Destination classes

| Destination class | Boundary | Reference plane |
|-------------------|----------|-----------------|
| `vendor_console` | `vendor_console_handoff` | **Handoff required** — the true external control plane. |
| `browser_app_surface` | `browser_handoff` | **Handoff required** — a hosted surface that is itself the control plane. |
| `external_auth_authority` | `auth_boundary_cross` | **Handoff required** — an external IdP/SSO challenge. |
| `browser_reference_doc` | `browser_handoff` | **Reference only** — read-only documentation; never control. |

### Reason classes

`execute_out_of_plane_action`, `consult_reference_documentation`,
`inspect_vendor_state`, `complete_auth_challenge`, `retrieve_export_artifact`.

## Reference plane: what the destination *is*

Each destination carries a **reference-plane state**, and the destination class
fixes it — the packet cannot lie about it:

- **`handoff_required`** — the destination is the **true control plane**, but it
  lives *outside* Aureline. Aureline hands off explicitly and attributably, with a
  return anchor back to the runbook. The action happens on the far side; it is not
  in-product control.
- **`reference_only`** — the destination is **read-only browser documentation**.
  It can never present itself as executable in-product control, and it can never
  claim that control is *exercised* on the far side: a reference-only handoff whose
  reason is `execute_out_of_plane_action` or `complete_auth_challenge` is rejected
  (`reference_only_handoff_claims_control`).
- **`governed_in_app`** completes the vocabulary for the in-plane case (no
  handoff).

No handoff destination is ever executable *in product* (`executable_in_product`
is always false): either the control plane is external, or the destination is
read-only reference.

The register also publishes a **reference-plane catalog** naming the provider
consoles that remain the true control plane and the browser-only reference docs
that stay reference-only, so a reader can see at a glance which destinations are
control planes and which are documentation.

## Return anchors preserve target and evidence identity

A pivot must not lose the initiating context. Every handoff's **return anchor**
carries:

- the **initiating object** (`runbook_execution`, `runbook_step`, or
  `incident_workspace`) and its ref;
- the **target continuity ref** — the same target selector the initiating step
  acted on;
- the **evidence continuity ref** — one of the evidence outputs the initiating
  step produced;
- a **return message id** for the "return to Aureline" affordance.

For a handoff embedded in an execution record, the anchor is validated against the
step that ran: if its target continuity ref does not match the step's target, or
its evidence continuity ref is not one of the step's evidence outputs, the record
is rejected (`return_anchor_breaks_continuity`). The operator can always return to
the initiating Aureline object with target and evidence linkage intact.

## One truth, every surface

The handoff register projects every governed handoff — including the live ones
embedded in the operator-scenario execution records — into one surface-independent
projection, and the incident workspace, operator history, support exports, and
docs/help all render the same truth. A conformance check asserts that every
handoff embedded in an execution record is represented in the register, so a live
pivot can never be quietly dropped. The register carries no credential bodies or
raw console/browser payloads.

## Invariants

- Console/browser pivots are explicit, attributable product transitions, never
  hidden escapes from Aureline truth.
- Reference-only browser documentation can never present itself as executable
  in-product control.
- A user can return to the initiating Aureline object with target/evidence linkage
  intact.

[`ControlPlaneHandoffPacket`]: ../../crates/aureline-runbooks/src/m5_runbook_governance/mod.rs
