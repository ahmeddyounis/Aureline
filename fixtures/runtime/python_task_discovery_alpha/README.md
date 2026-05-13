# Python Pytest Task Discovery Alpha Fixtures

These protected fixtures exercise the bounded pytest discovery and run-contract
lane in `crates/aureline-runtime/src/discovery/pytest`.

- `ready_uv` resolves a local `.venv` plus `uv` manager, discovers function and
  class-method pytest selectors, and emits direct `uv run pytest` contracts.
- `missing_python_runtime` contains pytest tests but no interpreter or manager
  facts, so discovery succeeds while launch contracts are blocked with explicit
  runtime blockers.
- `unsupported_conda` keeps discovered pytest source refs while blocking launch
  because Conda is outside the alpha pytest runner contract.

Verify with:

```sh
cargo test -p aureline-runtime pytest
```
