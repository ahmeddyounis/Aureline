# Symbolication contract evidence

This packet is the checked-in proof path for exact-build symbol and
source-map manifests, local or mirrored symbolication reports, and the
shared fidelity labels rendered by debug, profiler, preview,
browser-runtime, and support/export surfaces.

## Canonical surfaces

| Surface | Required truth |
|---|---|
| Debug frame stack | Fidelity label, symbol sources used, exact-build mismatch disclosure before navigation |
| Crash dump card | Fidelity label, unresolved-frame count, mirrored-use disclosure when applicable |
| Profiler hotspot workspace | Fidelity label, mirrored-source disclosure, redaction class |
| Profiler trace viewer | Fidelity label, mirrored-source disclosure, redaction class |
| Preview runtime frame | Build-mismatch disclosure before navigation |
| Browser-runtime stack | Build-mismatch disclosure before navigation |
| Support export packet | Symbol sources used, redaction class, unresolved-frame truth |
| Incident crash card | Symbol sources used, redaction class, unresolved-frame truth |

## Proof claims

| Claim | Evidence |
|---|---|
| Exact-build local symbols are admitted only when verified field-for-field | `report.debug_frame_stack.shell_crash_exact` in `artifacts/debug/symbolication_contract.json` |
| Mirrored source maps stay subordinate to local-first lookup and remain visible to the user | `policy.enterprise.renderer_mirror` plus `report.profiler_hotspot.renderer_approximate` |
| Symbol-only reports do not overclaim source lines | `report.support_export.provider_symbol_only` |
| Exact-build mismatches narrow to unresolved and block navigation truthfully | `report.preview_runtime.renderer_unresolved_mismatch` |
| Debug, profiler, preview/browser-runtime, and support all preserve one shared fidelity vocabulary | `surfaces[]` in `artifacts/debug/symbolication_contract.json` |

## Verification

```sh
cargo test -p aureline-debug
```
