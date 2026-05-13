# Python detector alpha

This lane adds a read-only detector for Python launch-wedge repositories.
The implementation lives in
[`crates/aureline-runtime/src/detectors/python`](../../crates/aureline-runtime/src/detectors/python)
and emits a `python_environment_detection_record` that can be embedded on
the canonical execution context before task, test, debug, notebook, or
navigation flows depend on an interpreter.

## Contract

The detector reads repository declarations, local environment markers, and
caller-provided ambient facts. It never runs repo-owned hooks, shell startup
files, Python, `uv`, Poetry, or environment-manager binaries.

Interpreter precedence:

1. action-local override
2. workspace `.venv/pyvenv.cfg`
3. repo Python pins: `.python-version`, `.tool-versions`, `mise.toml`
4. `pyproject.toml` Python requirements
5. user/profile default
6. captured ambient `python` fact
7. detector fallback

Environment-manager precedence:

1. action-local override
2. `uv.lock`, `[tool.uv]`, `poetry.lock`, `[tool.poetry]`, or Conda env files
3. workspace `.venv`
4. user/profile default
5. captured ambient or detector fallback

When same-precedence sources disagree, the report records an
`unresolved_ambiguities[]` row and leaves that subject in the `ambiguous`
state instead of choosing a silent winner. Lower-precedence conflicts remain
as provenance cards so the execution-context inspector can explain why the
winning source was used.

## Inspector surface

`ExecutionContext::with_python_environment_detection` attaches the detector
record to the canonical context. The shell execution-context inspector adds a
**Python environment** section when that report is present, showing:

- interpreter state, selected value, winning source, and fallback path
- environment-manager state, selected value, winning source, and fallback path
- unresolved ambiguity rows
- one provenance-card row per observed source

Fallback, missing, unsupported, ambiguous, and unreadable-source states add
visible honesty markers before dispatch.

## Fixtures

Protected fixture repos live under
[`fixtures/runtime/python_detection_alpha`](../../fixtures/runtime/python_detection_alpha):

- `uv_workspace`
- `poetry_workspace`
- `venv_only`
- `ambiguous_interpreter_pins`
- `malformed_pyproject`

## How to verify

```sh
cargo test -p aureline-runtime python
cargo test -p aureline-shell python_environment
```
