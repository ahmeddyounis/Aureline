# State Label And Naming Fixtures

Worked YAML fixtures for
[`/docs/copy/naming_and_state_label_contract.md`](../../../docs/copy/naming_and_state_label_contract.md)
and
[`/schemas/copy/label_term.schema.json`](../../../schemas/copy/label_term.schema.json).

Each file is a `label_review_case_record` showing the surfaces under
review, candidate labels, approved labels, forbidden labels, expected
decision, and conformance expectations.

## Cases

- [`command_rename_review.yaml`](./command_rename_review.yaml)
  - Command rename review that approves a verb-first canonical label
    and requires the old phrase to remain only as a typed alias.
- [`state_chip_surface_comparison.yaml`](./state_chip_surface_comparison.yaml)
  - Cross-surface comparison rejecting reuse of `Stale` for a different
    meaning in CLI output.
- [`client_scope_label_mapping.yaml`](./client_scope_label_mapping.yaml)
  - Client-scope mapping across UI, docs, exports, and support packets.
- [`rejected_decorative_label.yaml`](./rejected_decorative_label.yaml)
  - Mechanical rejection of decorative naming for a state and command.
