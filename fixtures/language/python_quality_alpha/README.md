# Python Quality Alpha Fixtures

`quality_cases.json` seeds the Python formatter, linter, and test-adapter
quality wedge. The cases keep raw source and tool output out of fixture payloads
while preserving provider ids, interpreter state, diagnostic refs, task-event
refs, freshness, scope, and rerun posture.

The fixture covers:

- missing interpreter selection, which blocks all Python quality hooks with an
  interpreter-specific rerun posture;
- missing Ruff lint tooling while Black formatting and pytest hooks remain
  usable; and
- missing pytest tooling while Black formatting and Ruff linting remain usable.
