//! Canonical M5 build-intelligence, host-boundary, and managed-workspace
//! execution-truth matrix.
//!
//! New M5 execution, preview, infrastructure, and managed-runtime surfaces must
//! stay explicit about how a target was discovered, how confident the product is
//! in it, where work is running, which control plane still owns mutable truth,
//! and what recovery path applies when the target or service plane changes. This
//! crate freezes one machine-readable [`SurfaceGovernanceRow`] per marketed M5
//! execution surface — local build targets, framework-pack builds, remote preview
//! sessions, managed-workspace runtimes, connector-backed services, cluster-context
//! execution, live-resource targets, and incident-replay targets — so every later
//! depth lane inherits one authoritative execution-truth model instead of cloning
//! product-copy variants of "runs here" or "connected".
//!
//! The model is a release-control gate, not a status badge. The
//! [`ExecutionClaim`] a surface may *publish* is derived deterministically from its
//! observed states: a surface whose target was never discovered, whose adapter
//! confidence is unverified, whose host is unbound, whose control-plane owner is
//! unknown, whose managed workspace is unavailable, whose mutation is destructive,
//! whose approval was bypassed, whose evidence is stale, or whose rollback is
//! incomplete cannot publish an authoritative claim, and its [`ClaimDecision`]
//! records whether the gate published it, narrowed it to a qualified or provisional
//! claim, or withheld the claim entirely. Because
//! [`SurfaceGovernanceRow::published_claim`],
//! [`SurfaceGovernanceRow::claim_decision`], and the recomputed
//! [`SurfaceGovernanceRow::narrowing_reasons`] are all validated against the gate,
//! desktop, CLI, support-export, and release-evidence surfaces can prove that
//! underqualified surfaces narrow automatically and that no surface claims more
//! certainty, host stability, or control-plane ownership than its own evidence
//! supports.
//!
//! Governance stays surface-specific and provenance-bound. The packet pins the
//! execution-surface vocabulary and requires exactly one row per claimed surface,
//! so a verified local build target never lends its confidence to a live-resource
//! mutation or an externally owned cluster context, and no surface inherits a
//! stronger claim simply because an adjacent one is authoritative.
//!
//! The packet is checked in at
//! `artifacts/execution/m5/m5-build-and-host-governance.json` and embedded here, so
//! this typed consumer and any CI gate agree on every surface without a cargo build
//! in CI. The model is metadata-only: every field is a typed state or an opaque
//! ref. It carries no credential bodies, raw provider payloads, host tokens, or
//! control-plane secrets.
//!
//! The companion [`m5_target_discovery`] module narrows the same execution-truth
//! discipline to a single question — *how was this target discovered and how certain
//! is that answer?* — across the M5 lanes that pick build targets, notebook kernels,
//! preview runtimes, profiler sessions, framework generators, request/browser
//! runtimes, API runtimes, and incident or pipeline-linked reruns. Its non-inheriting
//! confidence gate keeps an approximate or heuristic target from masquerading as a
//! confident exact one and makes target changes reviewable instead of silent.
//!
//! The companion [`m5_host_boundary`] module narrows the same discipline to a third
//! question — *where did this work actually run, and how certain is that answer?* —
//! across the M5 lanes that can cross from the local shell to a remote, container,
//! managed-workspace, browser-bridge, or service-plane host. It pins one closed
//! [`m5_host_boundary::HostKind`] vocabulary, carries an execution-origin receipt and
//! host-boundary context strip per lane, and runs every lane through a non-inheriting
//! attribution gate so a missing receipt, a bridged or reconnecting connection, a
//! stale context, an unbound host, or a broken export continuity narrows, flags, or
//! withholds the published origin instead of letting a browser, companion, preview,
//! or managed surface imply that work ran locally.
//!
//! The companion [`m5_mutation_and_handoff_review`] module narrows the same discipline to
//! a fourth question — *who is acting, was it approved, how is it recovered, and what does
//! it do before it commits?* — across the M5 mutation paths that can widen authority or
//! side effects: request-workspace mutations, browser-runtime actions, preview-route
//! actions, live-resource operations, remote mutations, and the browser/companion and
//! vendor-console handoffs that carry them. Its non-inheriting commit gate resolves each
//! path through one reviewed preview/apply/handoff sheet — surfacing actor, approval,
//! target context, duration, time-bound route effect, rollback class, and a fallback or
//! open-in-provider path — so an inherited authority, a bypassed approval, an unknown
//! rollback, an unbounded route, or a severed handoff narrows, flags, or withholds the
//! commit instead of letting a request, browser, or handoff path inherit hidden authority
//! or bypass review, and it exports a machine-readable mutation receipt so support and
//! audit can reconstruct which reviewed action class actually ran.

pub mod m5_build_and_host_governance;
pub mod m5_host_boundary;
pub mod m5_mutation_and_handoff_review;
pub mod m5_target_discovery;
