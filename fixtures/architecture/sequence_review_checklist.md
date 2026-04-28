# Critical Sequence Review Checklist

Use this checklist when a change touches startup, large-repository
navigation, AI-assisted apply, remote execution, or collaboration /
shared-debug behavior. The reviewer should open the narrative document
and the latency pack together:

- [`/docs/architecture/critical_sequence_diagrams.md`](../../docs/architecture/critical_sequence_diagrams.md)
- [`/artifacts/perf/critical_sequence_latencies.yaml`](../../artifacts/perf/critical_sequence_latencies.yaml)

## Required Checks

- [ ] Every changed user-visible step appears in the sequence diagram
      and has exactly one matching `stage_id` in the latency pack.
- [ ] Every protected stage names a budget or contract gate, a deadline,
      cancellation authority, fallback behavior, trace/span identity,
      and evidence refs.
- [ ] Every backgroundable or optional stage proves it cannot block the
      protected deadline.
- [ ] Every forbidden hot-path action is still outside the hot path:
      blocking filesystem scans, network/provider calls, process
      launch, full indexing, support export, unapproved extension
      activation, raw secret projection, or hidden replay.
- [ ] Stale data, missing providers, degraded remote state, and degraded
      collaboration state use one of the latency pack's
      `truth_contract_vocabulary` values and have a user-visible truth
      statement.
- [ ] Every benchmark or fitness reference resolves to a current row or
      is explicitly marked as a pending dedicated row in the latency
      pack.
- [ ] Every sequence has at least one failure-injection case class and
      at least one fixture ref that exercises the degraded or failed
      posture.
- [ ] Every sequence has a support reconstruction hook that maps to a
      scenario class in
      [`/artifacts/support/reconstruction_checklist.yaml`](../../artifacts/support/reconstruction_checklist.yaml).
- [ ] Any changed support hook names the required reconstruction axes:
      command, route/target/origin/exposure where applicable, docs
      version, exact build identity, claim row, known-limit note, and
      redaction posture.
- [ ] If the change widens a capability, the review includes the policy,
      approval-ticket, retention, and support-export impact.

## Sequence-Specific Checks

### Warm Startup To First Edit

- [ ] First paint, command-entry readiness, first interactive editor,
      and first edit paint still map to the warm-path and latency-budget
      ledgers.
- [ ] Shell paint is not blocked by restore, indexing, extensions, AI,
      collaboration, Git status, network, or support export.
- [ ] Restore failures route to safe mode, open-without-restore,
      layout-only restore, or compatible restore with a visible recovery
      note.

### Large Repository To First Useful Navigation

- [ ] Partial tree, first result, and first symbol jump keep separate
      deadlines and trace names.
- [ ] Cached, scanner-backed, graph-backed, and provider-backed results
      carry freshness/source/confidence labels.
- [ ] Full indexing, full Git status, package install, or language-server
      cold start cannot block the first useful result.

### AI-Assisted Multi-File Change With Approval

- [ ] Context assembly records omitted, redacted, policy-blocked, stale,
      and tainted segments explicitly.
- [ ] Provider egress happens only after policy, route, spend, and tool
      gates.
- [ ] The diff review is required before any multi-file write.
- [ ] Apply creates a checkpoint or states the compensating rollback
      posture before mutation.
- [ ] Replay and evidence packets can explain provider unavailability,
      model/tool drift, and retained or omitted artifact classes.

### Remote Attach And Run Targeted Test

- [ ] Attach produces a remote shell or actionable failure within the
      published deadline.
- [ ] Target identity witness, capability envelope, policy epoch,
      credential handle, and execution context refs are present before
      any remote mutation.
- [ ] Target changes, provider loss, reconnect degradation, and public
      tunnel denial block or downgrade instead of silently continuing.
- [ ] Test dispatch and first task event preserve run/attempt,
      cancellation-authority, and artifact-event truth.

### Collaboration Join And Shared Debug Follow

- [ ] Session join, presence update, shared debug metadata, follow
      projection, and control-grant stages stay distinct.
- [ ] Relay loss or participant degradation never freezes or rolls back
      local buffers.
- [ ] Presenter/follow state never confers mutating terminal or debugger
      authority.
- [ ] Mutating shared-debug action requires an explicit control grant,
      and revocation is immediate and non-replayable.
- [ ] Retention defaults to metadata-only unless a session policy
      manifest admits broader retention.

## Change-Control Prompt

If any answer above is "no", the change must either narrow back to the
existing contract or update all linked artifacts in the same review:

- narrative sequence diagram;
- latency pack stage row;
- benchmark or fitness linkage;
- failure-injection fixture;
- support reconstruction hook;
- release or support evidence packet that cites the changed path.
