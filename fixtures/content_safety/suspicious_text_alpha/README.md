# Suspicious text alpha fixtures

These fixtures protect the shared suspicious-text detector projection across
editor, diff, search, and review surfaces. Each case feeds one source snippet
through `aureline-content-safety` and expects every surface to expose the same
warning classes, exact anchors, raw-vs-safe copy choices, and safe export
continuity.

The fixture bodies keep suspicious codepoints escaped so reviewers can inspect
the files without relying on editor rendering behavior.
