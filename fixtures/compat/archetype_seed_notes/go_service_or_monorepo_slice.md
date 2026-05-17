# Go service or monorepo slice

## Row binding

- Archetype row id: `archetype_row:go_service_or_monorepo_slice`
- Archetype id: `go_service_or_monorepo_slice`
- Initial support class: `experimental`
- Target support class: `certified`
- Inclusion target: `first_stable`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

Go modules or workspaces with in-repo tests and the `dlv` debugger.
The row is intentionally written to admit either a single-module
service or a monorepo slice that exposes one or more modules; both
shapes share the same workflow set.

## Required-mode rationale

- `local_only` — Go workflows are fully covered by an in-repo build,
  test, and debug on a developer machine. The row deliberately does
  not promise a remote-attach baseline at first stable.

## Evidence already on file

- Reference workspace: `refws.go_service_archetype_seed`
  ([fixture](../../workspaces/reference/go_service_archetype_seed.json)).
- Beta packet and harness:
  `fixtures/reference_workspaces/m3/go_service/workspace.yaml`,
  `fixtures/reference_workspaces/m3/go_service/harness.yaml`.
- Corpus scenarios: `corpus.reference.go_service_archetype_seed`,
  `corpus.archetype.go_service_seed`,
  `corpus.workflow.first_useful_edit_go_service`.
- Design-partner input class: `sanitised_repo_admissible`.

## Open evidence questions

- Capture current pass/fail results for the seeded workflow harness
  before the row may move out of `experimental`.
- Decide whether the certified path requires both go-modules and
  go-workspaces evidence rows or whether a workspace is the canonical
  shape with module support inheriting through it.
- Capture the dependency-view workflow shape; the row mentions it but
  the seeded harness does not yet carry a dedicated dependency-view
  pass/fail capture.
