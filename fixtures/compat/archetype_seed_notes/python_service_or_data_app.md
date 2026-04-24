# Python service or data app

## Row binding

- Archetype row id: `archetype_row:python_service_or_data_app`
- Archetype id: `python_data_app`
- Initial support class: `experimental`
- Target support class: `certified`
- Inclusion target: `first_stable`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

Python with `uv`, `venv`, or Poetry; `pytest` as the in-repo test
runner; FastAPI or Django as the service framework. Notebook adjacency
is intentionally handled by
`archetype_row:notebook_first_data_workflow` so the Python service row
does not silently inherit notebook-specific cell-aware compare
expectations.

## Required-mode rationale

- `local_only` — covers interpreter-select, in-repo test, debug, and
  refactor-basics flows on a developer machine.
- `local_plus_devcontainer_or_container` — Python toolchain isolation
  (interpreter version, native dependencies, OS packages) is the
  dominant operational pattern. Certifying the row without a
  container or devcontainer mode would understate the real-world
  scope.

## Evidence already on file

- Reference workspace: `refws.python_data_app_archetype_seed`
  ([fixture](../../workspaces/reference/python_data_app_archetype_seed.json)).
- Corpus scenarios:
  `archetype.python_data_app_first_open_certified`,
  `workflow.first_useful_edit_python_data_app`.
- Reserved task-success corpus id:
  `fixture.archetype_python_data_app_seed`.

## Open evidence questions

- Materialise a devcontainer or container scenario before the row may
  move out of `experimental`. The seed fixture is bare bytes; a
  container-mode workflow needs its own corpus row.
- Stand up a certified-archetype report and a claim-manifest row;
  neither exists yet for this archetype.
- Decide which interpreter manager (`uv`, `venv`, Poetry) is the
  canonical certified path; the row currently admits all three with no
  caveat.
