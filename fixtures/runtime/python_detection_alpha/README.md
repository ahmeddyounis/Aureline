# Python detector alpha fixtures

These protected fixtures exercise the read-only Python environment detector
for the Python launch wedge. The detector reads manifests, version files, and
local environment markers; it does not execute Python, `uv`, Poetry, shell
startup files, or repo-owned hooks.

| Fixture | Purpose |
|---|---|
| `uv_workspace` | `uv.lock`, `[tool.uv]`, `.python-version`, and `.venv/pyvenv.cfg` agree on the selected Python environment. |
| `poetry_workspace` | Poetry lock and `pyproject.toml` dependency metadata select Poetry and a Python requirement. |
| `venv_only` | A plain `.venv` names the interpreter and manager without a higher-level project manager. |
| `ambiguous_interpreter_pins` | Same-precedence Python version files disagree and remain ambiguous before launch. |
| `malformed_pyproject` | A malformed relevant `pyproject.toml` field surfaces as a detector failure card. |
