# Title/context bar examples

These fixtures exercise the canonical
`title_context_bar_state_record` described by
[`docs/ux/title_context_bar_contract.md`](../../../docs/ux/title_context_bar_contract.md)
and validated by
[`schemas/ux/title_context_bar_state.schema.json`](../../../schemas/ux/title_context_bar_state.schema.json).

| Fixture | Case covered |
| --- | --- |
| [`restricted_mode_local_workspace.json`](./restricted_mode_local_workspace.json) | Restricted local workspace with trust visible across title chrome, native title, status item, and support export. |
| [`partial_open_workspace.json`](./partial_open_workspace.json) | Partial open with one root still warming and mixed branch summary. |
| [`detached_repo_metadata.json`](./detached_repo_metadata.json) | Detached repository metadata where branch fields are omitted instead of guessed. |
| [`multi_root_mixed_state.json`](./multi_root_mixed_state.json) | Multi-root workspace with mixed trust and mixed repository state. |
| [`missing_host_details.json`](./missing_host_details.json) | Cached remote restore with missing host details and read-only degraded posture. |
| [`mixed_local_plus_remote_session.json`](./mixed_local_plus_remote_session.json) | Local editing plus reconnecting remote execution context. |
