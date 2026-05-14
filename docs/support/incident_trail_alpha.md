# Alpha crash incident trail

This contract joins alpha-channel crash capture, exact-build symbolication,
trace IDs, and support-bundle linkage into one redaction-safe incident
trail.

Owned implementation and fixtures:

- [`/crates/aureline-crash`](../../crates/aureline-crash/) - Rust
  incident-trail model and acceptance tests.
- [`/fixtures/support/incident_trail_alpha/`](../../fixtures/support/incident_trail_alpha/)
  - exact, partial, and missing-symbolication fixture path.
- [`/crates/aureline-support/src/bundle/crash_linkage.rs`](../../crates/aureline-support/src/bundle/crash_linkage.rs)
  - support-bundle preview row that consumes the trail.
- [`/crates/aureline-shell/src/support_seed/mod.rs`](../../crates/aureline-shell/src/support_seed/mod.rs)
  - first shell support/export consumer.

## Trail shape

`crash_incident_trail_alpha_record` carries:

- `crash_envelope_ref`, `crash_dump_ref`, optional
  `symbolication_report_ref`, and `support_bundle_manifest_ref`;
- `primary_exact_build_identity_ref`, `trace_ids`, `fault_domain_id`,
  and `chronology_capture_state`;
- per-module mapping state: `exact`, `partial`, `missing`, or
  `build_mismatch`;
- support linkage state: `linked`, `missing_manifest_ref`, or
  `mismatched_bundle_ref`;
- safe next actions: safe mode, open without restore, export evidence,
  and retry one fault domain.

Raw dump bytes, raw stack bodies, raw memory pages, raw absolute paths,
raw command lines, and secrets are excluded from this trail. The raw
dump remains governed by `support.item.crash_dump_or_core`.

## Acceptance states

- Exact match: every module maps to the same exact-build family and the
  support-bundle manifest ref is present.
- Partial: unresolved module mappings stay visible as `partial`; the
  trail remains linked to the support bundle.
- Missing: absent symbolication report is labeled `missing`; the trail
  still preserves crash envelope, dump manifest, trace IDs, support
  bundle refs, and safe next actions.
- Build mismatch: differing exact-build refs produce
  `build_mismatch`, never `exact`.

## Verification

```sh
cargo test -p aureline-crash --test incident_trail_alpha
cargo test -p aureline-support --test crash_incident_trail_support_preview
cargo test -p aureline-shell --test support_bundle_alpha_manifest shell_support_surface_consumes_crash_incident_trail
```

The tests prove the trail links the crash to exact-build symbolication
and a support-bundle manifest, labels partial or missing symbolication
honestly, and preserves the safest next actions without enabling reset
shortcuts.
