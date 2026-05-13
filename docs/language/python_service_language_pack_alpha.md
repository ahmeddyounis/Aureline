# Python Service Language Pack Alpha

This document records the first bounded Python service/data language-pack
artifact in `aureline-language`.

## Owned Runtime Surface

The canonical manifest is
`artifacts/language_packs/python_service_alpha.yaml`. The runtime consumer lives
under `crates/aureline-language/src/packs/python_service.rs` and exposes
`PythonServiceLanguagePack`. The consumer emits a
`python_service_language_pack_enablement_snapshot` that joins:

- Python and Markdown grammar rows resolved through the shared Tree-sitter
  registry;
- Python language-service, interpreter, formatter, linter, and pytest provider
  routes;
- Markdown CommonMark safe-preview and local Git review bridge routes;
- quality-tool hooks backed by the existing Python quality contract;
- docs, icon, known-gap, trust, and suspicious-content policy rows; and
- launch-bundle and archetype reporting refs.

Raw source bodies, raw Git diffs, command output, and provider logs are not
embedded. Records carry opaque refs, support class, freshness, scope, trust
class, representation labels, and export-safe summaries.

## Scope And Fallback Rules

- Python semantic answers are valid only for the selected interpreter and active
  workset.
- Missing, ambiguous, blocked, or drifted interpreter selection keeps Python
  assistance, quality hooks, tests, and notebook handoff visibly degraded.
- Markdown preview is CommonMark-baseline, sanitized, and raw/rendered
  copy-aware. Active rich content is not part of this pack.
- Local Git remains authoritative for status, diffs, commit metadata, and review
  packets. Python diagnostics and Markdown summaries may decorate those
  surfaces but may not replace Git truth.
- Commit-message, review-packet, and broad mutation flows require preview before
  writing or exporting.
- Framework-expert behavior, full notebook parity, debugger parity, and risky
  Git history mutation are explicit known gaps.

## Protected Proof Path

The protected fixture is
`fixtures/language/packs/python_service_alpha/pack_enablement_cases.yaml`. It
covers the pack enabling Python, Markdown, interpreter-sensitive provider
routes, quality hooks, local Git review surfaces, safe-preview posture, and
launch-bundle/archetype reporting from one artifact.

Run:

```sh
cargo test -p aureline-language --test python_service_pack_alpha
python3 - <<'PY'
from pathlib import Path
import yaml
yaml.safe_load(Path("artifacts/language_packs/python_service_alpha.yaml").read_text())
yaml.safe_load(Path("fixtures/language/packs/python_service_alpha/pack_enablement_cases.yaml").read_text())
PY
```
