# Task Event Alpha Fixtures

These fixtures exercise the canonical runtime task-event stream. They prove
that build, test, and debug lanes share one event grammar, that all required
task lifecycle states are representable, and that every typed event retains a
raw adapter-origin envelope reference with workspace, task, trace, and
redaction identity.

