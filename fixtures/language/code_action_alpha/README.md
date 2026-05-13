# Code-action alpha fixtures

These fixtures exercise the code-action alpha records implemented in
`aureline-language::code_actions`.

The cases protect the first runtime contract for quick fixes, fix-all actions,
generated synchronization, configuration-changing actions, and read-only
validation actions. They verify that every action has a side-effect class,
that preview requirements are machine-readable, that broad or repo-truth
changes cannot apply silently, and that mutating actions carry named,
attributable undo groups.

| Fixture | Scenario |
|---|---|
| `action_cases.json` | Source-labeled action records built from diagnostic source descriptors, then projected into editor/review/export-safe surface state. |
