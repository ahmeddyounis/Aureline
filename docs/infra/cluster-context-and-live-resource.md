# Cluster-Context Strips, Truth-Mode Views, and Console Handoff

This document defines the checked-in qualification packet that keeps the Terraform, Kubernetes, and incident-adjacent surfaces explicit about target context, truth mode, and console handoff. It extends the [target-context and control-plane boundary](./target-context-and-control-plane-boundary.md) model rather than introducing a parallel infrastructure vocabulary.

The canonical machine-readable schema is [`/schemas/infra/cluster-context-and-live-resource.schema.json`](../../schemas/infra/cluster-context-and-live-resource.schema.json). The Rust validation model is in [`/crates/aureline-infra`](../../crates/aureline-infra/src/cluster_context_and_live_resource/mod.rs). Fixtures live in [`/fixtures/infra/cluster-context-and-live-resource`](../../fixtures/infra/cluster-context-and-live-resource).

## Qualification Rule

A cluster, Terraform, or incident-adjacent surface needs a current packet proving all of the following:

- every surface renders a **context strip** naming provider, account or subscription, project, cluster, namespace, region, tenant, execution origin, and credential class, and that strip matches the shared environment context;
- the **desired, rendered, plan, live, and provider-overlay** truth modes are shown as separate views, each with its own freshness and source label, never collapsed into one blended resource view;
- only a **live** view may be mutation-capable, and only while its freshness is live — stale, offline, cached, permission-limited, or unavailable live views are downgraded to read-only;
- the Terraform plan review, Kubernetes resource view, cluster live resource, incident runbook step, and support runbook export all render the full five-mode vocabulary;
- mutating or boundary-raising actions (`mutate` covering apply and destroy, `port_forward`, `shell_attach`, `exec`, `container_exec`, and `browser_console_launch`) pass through a **gate** that requires a reviewed preview or handoff, previews the exact target, and states the source-of-truth posture before execution;
- approved high-risk mutations carry a reviewed preview ref, and console launches carry an explicit handoff ref;
- console handoffs preserve the shared typed handoff reason, destination class, target identity snapshot, authority-boundary disclosure, structured return anchor, and return-safe breadcrumbs from the boundary packet;
- provider consoles are explicit handoff destinations that never claim Aureline as the authoritative control plane and never substitute for local source, plan, or observed truth.

Packets that fail any error-severity check are not promoted; their surfaces stay read-only, preview-pending, or handoff-only.

## Fixture Meaning

- `qualified_cluster_context_packet.json` proves a production-tagged checkout context where Terraform, Kubernetes, incident, CLI, console-handoff, and support-export surfaces share the same target strip, render all five truth modes separately, and gate apply, exec, and console-launch actions.
- `stale_live_downgraded_packet.json` keeps the five truth modes separate while marking plan and live state stale, offline, or unavailable; no view is mutation-capable and apply stays blocked pending fresh evidence.
- `wrong_target_blended_view_packet.json` intentionally fails validation: a Kubernetes strip points at the wrong cluster, a live view is blended with authored state, and an apply gate is approved without any preview.

## Support Export Posture

Support runbook exports may include packet ids, context refs, strip identity fields, credential handle classes, freshness and source labels, truth-mode names, preview refs, handoff audit refs, and redaction-safe summaries. They must not include raw secrets, kubeconfig bodies, cloud tokens, SSH private keys, browser cookies, raw provider responses, or private endpoint URLs.
