# Interaction-Integrity Conformance Enforcement

This page is the release-review entrypoint for the interaction-integrity
conformance corpus. The corpus turns focus/batch truth, preview drift,
permission expiry, safe-preview/copy-export parity, host-boundary cues, and
responsive fallback into one current packet family.

## Sources

- Corpus manifest:
  `fixtures/qe/m3/interaction_integrity_corpus/manifest.yaml`
- Generated conformance packet:
  `artifacts/qe/m3/interaction_integrity_packets/conformance_packet.json`
- Release snapshot:
  `artifacts/qe/m3/interaction_integrity_packets/release_snapshot.json`
- Support projection:
  `artifacts/qe/m3/interaction_integrity_packets/support_export_projection.json`
- Validation capture:
  `artifacts/qe/m3/interaction_integrity_packets/captures/interaction_integrity_validation_capture.json`
- Validator:
  `ci/check_m3_interaction_integrity_corpus.py`

## Control Coverage

The corpus must keep all six control classes current and passing:

| Control class | What it proves |
|---|---|
| `focus_batch_scope` | Focus, active/current item, selected state, hidden/filtered members, and resulting target ids remain distinct before batch or destructive actions. |
| `preview_drift_invalidation` | Apply paths re-check target, scope, host boundary, route, policy, lifecycle, approval, and representation bindings before commit. |
| `permission_expiry` | Expired or drifted approval tickets deny spend attempts and route to typed reapproval instead of silent reuse. |
| `safe_preview_copy_export_parity` | Raw, rendered, sanitized, redacted, escaped, and metadata-only transfers remain representation-labeled. |
| `host_boundary_cues` | Runtime-heavy surfaces preserve discovery source, host boundary, lifecycle, route, and wrong-target reapproval truth. |
| `responsive_fallback` | Resize, split, detach, and compact fallback preserve required identity and safety fields or deny the action. |

## Blocking Rule

For every marketed row listed in the manifest, missing, stale, red, or
expired-waiver packet state blocks beta widening and release-candidate
promotion. Active waivers must be named, time-boxed, and carried in the packet
instead of clearing the gate silently.

## Refresh

Run:

```sh
python3 ci/check_m3_interaction_integrity_corpus.py --repo-root .
```

Use `--check` in CI to fail when the generated conformance packet, release
snapshot, support projection, or validation capture drift from the checked-in
corpus.

