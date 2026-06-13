# M5 Safe-Preview Limited Mode Fixtures

These fixtures are valid, export-safe safe-preview limited-mode packets that
exercise open-mode resolution, oversized/generated banners, open-raw and
open-canonical-source actions, and expensive-render guards across the new M5 log,
lockfile, snapshot, bundle, evidence-packet, and generated artifact families.
Each one keeps every family present, keeps the default view cheap, keeps open-raw
reachable immediately, preserves the canonical-source/generator relationship, and
gates every expensive or unsafe render behind an explicit opt-in without leaking
raw bytes.

## all_small_no_guard.json

Every artifact is small, in-budget, inert, and cheap to render. No artifact is
oversized and no expand action is expensive or unsafe, so nothing needs a guarded
render. The non-generated build-log capture opens fully inline; the inherently
generated families still open in limited mode (with a generated-artifact banner)
but expand immediately and cheaply. Demonstrates that the resolution holds — and
validates — when nothing needs to narrow, so a fully cheap run does not silently
behave like a guarded one. Regenerate with `m5_safe_preview_limited_mode --clean`.
