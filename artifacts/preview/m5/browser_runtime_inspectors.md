# M5 Browser-Runtime Inspectors

- Packet: `m5-browser-runtime-inspectors:stable:0001`
- Label: `M5 Browser-Runtime Inspectors`
- Inspectors: 6 (1 mutation-capable, 1 downgraded)
- Inspector kinds: 5 / 5
- Target kinds: 6 / 6
- Mapping qualities: 4 / 4

## Inspectors

- **inspector:dom:0001** (dom)
  - DOM inspector on an embedded preview mapped exactly to its canonical-source span; the override previews the real source diff before commit
  - inspector=dom target=embedded_preview attach=dom_only mapping=exact freshness=live continuity=fresh_attach redaction=non_sensitive_passthrough
  - Mutation: side_effect=dom_mutation review=review_required target=`target:embedded-preview:0001`
- **inspector:dom:0002** (dom)
  - DOM inspector on an external browser mapped approximately to source; jump-to-source lands near the span
  - inspector=dom target=external_browser attach=dom_only mapping=approximate freshness=live continuity=fresh_attach redaction=non_sensitive_passthrough
- **inspector:css:0001** (css)
  - CSS inspector on a simulator showing a generated stylesheet with no hand-authored span; inspect-to-source falls back to the generator input
  - inspector=css target=simulator_or_emulator attach=dom_and_styles mapping=generated_only freshness=live continuity=fresh_attach redaction=non_sensitive_passthrough
- **inspector:console:0001** (console)
  - Console inspector on a device browser; message bodies are redacted by default so tokens never leak into diagnostics
  - inspector=console target=device_browser attach=dom_only mapping=runtime_only freshness=live continuity=fresh_attach redaction=redacted_by_default
- **inspector:network:0001** (network)
  - Network inspector on a remote preview re-attached after a transport drop; only request metadata crosses, and the prior session stays attributable
  - inspector=network target=remote_preview_session attach=dom_styles_network mapping=runtime_only freshness=reconnected continuity=reconnected redaction=metadata_only
- **inspector:storage:0001** (storage)
  - Storage inspector over an imported captured snapshot; storage entries are carried as opaque hashes, not raw values
  - inspector=storage target=captured_snapshot attach=dom_styles_network_storage mapping=runtime_only freshness=captured_snapshot continuity=imported_snapshot redaction=hashed_reference
  - Downgraded: This view is an imported captured snapshot, not a live runtime; storage shown is from the capture and has no live session to mutate
