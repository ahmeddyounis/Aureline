# Queue Governor And Admission Control Evidence

This reviewer artifact summarizes the stable M4 queue-governor packet.

- Packet: `artifacts/runtime/m4/queue_governor_and_admission_control_packet.json`
- Schema: `schemas/runtime/queue-governor-and-admission-control.schema.json`
- Runtime contract: `crates/aureline-runtime/src/queue_governor_and_admission_control/`
- Fixture corpus: `fixtures/runtime/m4/queue-governor-and-admission-control/`

The packet proves:

- five shared queue lanes are present with explicit collapse, checkpoint,
  retry-budget, and starvation rules;
- duplicate hot-set jobs collapse by key instead of growing the queue;
- upload/replication resumes from a chunk checkpoint;
- provider overlay and replication work have separate retry budgets;
- protect-core truth is projected consistently to shell, activity center,
  diagnostics, CLI/headless inspection, and support export;
- support export excludes raw user content and reports lane depth, oldest age,
  collapse count, checkpoint, shed work, pause reason, resume owner, and
  resume condition.
