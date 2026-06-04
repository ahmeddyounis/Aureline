# Stabilized About, Help, and Community Handoffs

Help/About now treats public issues, security disclosures, docs feedback, RFCs,
community support, and vendor/private support as typed handoff packets rather
than generic external links.

## Canonical Product Contract

- Code model: `crates/aureline-shell/src/help_about/mod.rs`
- Packet schema: `schemas/help/community-handoff-packet.schema.json`
- Fixture corpus: `fixtures/help/m4/stabilize-about-help-community-handoff/`

Each packet carries:

- destination class: `public`, `official_authenticated`, `community`,
  `vendor_managed`, or `local_only`;
- visibility boundary, auth expectation, and data-exit boundary before launch;
- exact originating object, anchor, and return path;
- build identity, service-health descriptor, and provenance descriptor refs;
- repro-packet redaction preview before share;
- durable retry, export, and open-later actions for browser-blocked or offline
  states.

## Redaction Continuity

The repro preview applies local-first redaction rules for local paths, usernames,
hostnames, tokens, extension inventory, deployment profile, and selected
diagnostics. Raw sensitive material is excluded by default and never leaves the
machine implicitly.

## Boundary Rules

Community and RFC lanes are feedback/community surfaces, not guaranteed official
support. Security disclosures stay on authenticated/private security lanes.
Vendor/private support is labeled as vendor-managed before any browser or packet
handoff.
