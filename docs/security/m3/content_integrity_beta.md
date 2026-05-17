# Content Integrity Beta Packet

The beta content-integrity packet is the governed handoff between the shared
suspicious-content detector and declared product surfaces. It prevents a beta
row from claiming green content-integrity posture unless the same detector
vocabulary, warning refs, and representation labels survive across editor,
diff, search, review, docs, safe-preview, install-review, AI-context, and
support-export projections.

## Runtime Path

The implementation lives in
`crates/aureline-content-safety/src/suspicious_content/`.

The packet builder runs `detect_suspicious_content` once for the inspected
UTF-8 body, then projects shared `content_integrity_warning_record` rows onto
each declared surface. Consumers validate the packet through:

```sh
cargo run -q -p aureline-content-safety --bin content_integrity_beta -- fixtures/content_safety/m3/shared_detector/shared_beta_surfaces.json
```

Use `--packet` with the same fixture to render the checked packet JSON.

## Green Gate

A packet is green only when:

- every declared beta surface is present exactly once;
- the detector outcome and suspicious-content classes match on every surface;
- each surface emits shared `content_integrity_warning_record` rows;
- no projection normalized or stripped source bytes;
- raw, rendered, sanitized, and redacted copy/export labels are explicit;
- warning refs stay attached to review and support/export transfers; and
- trust class, raw/rendered state, representation labels, review continuity,
  and support-export continuity are all visible.

Any violation makes the validation report `blocked`. Beta admission treats a
missing or blocked report as a claim blocker.

## Fixture And Artifact

Protected fixtures live under
`fixtures/content_safety/m3/shared_detector/`.

The checked beta packet lives at
`artifacts/security/m3/content_integrity_beta_packet.json` and is referenced by
the claimed-surface register so admission checks can fail closed if the packet
is absent or no longer green.
