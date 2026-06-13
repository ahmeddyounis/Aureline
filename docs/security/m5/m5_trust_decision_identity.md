# M5 Trust-Decision Identity Rendering

This document is the contract for how the new M5 *trust-decision* surfaces
render identity. Install/update prompts, remote attach, collaboration invites,
route shares, and policy reviews are trust decisions, not ordinary browsing
panes: the user is about to grant trust, attach, install, or share based on a
publisher/package name, a remote-host label, a collaborator identity, a
route-share target, or a block of policy-review text. Those identity strings
render in a stronger *strong-decision* mode than ordinary browsing surfaces, so a
suspicious or ambiguous identity is inspectable before trust is granted.

This lane sits on the same shared content-integrity policy library as its
siblings: it runs the shared suspicious-content detector
(`aureline_content_safety::detect_suspicious_content`) over each identity and
derives the shared safe-inspection escape (`escape_for_safe_inspection`) rather
than inventing a parallel detector. The frozen content-integrity matrix locks the
static qualification each surface may claim, the trust-class ladder
(`m5_trust_class_ladder`) resolves the runtime trust class of active content, and
the safe-preview limited-mode lane (`m5_safe_preview_limited_mode`) guards
expensive or unsafe renders. This lane covers the orthogonal *identity-rendering*
gap they leave open.

- Record kind: `m5_trust_decision_identity_packet`
- Schema: [`schemas/security/m5-trust-decision-identity.schema.json`](../../../schemas/security/m5-trust-decision-identity.schema.json)
- Canonical support export: [`artifacts/security/m5/m5_trust_decision_identity/support_export.json`](../../../artifacts/security/m5/m5_trust_decision_identity/support_export.json)
- Summary artifact: [`artifacts/security/m5/m5_trust_decision_identity.md`](../../../artifacts/security/m5/m5_trust_decision_identity.md)
- Fixtures: [`fixtures/security/m5/m5_trust_decision_identity/`](../../../fixtures/security/m5/m5_trust_decision_identity/)
- Producer: `aureline_content_safety::project_m5_trust_decision_identity` /
  `frozen_m5_trust_decision_identity_packet`
- Headless tool: `m5_trust_decision_identity` (`--markdown`, `--clean`, `--validate <packet.json>`)

## Covered trust-decision surfaces

| Surface | Decision | Verb |
| --- | --- | --- |
| `publisher_package_name` | `install_or_update` | Install / Update |
| `remote_host_label` | `remote_attach` | Attach |
| `collaborator_identity` | `collaboration_invite` | Invite |
| `route_share` | `route_share` | Share |
| `policy_review` | `policy_review` | Approve |

The decision verb is always distinct from an ordinary browsing "Open", so the
affordance reads as a trust decision rather than a navigation.

## Render mode

Each surface resolves to an identity render mode with a strength rank:

| Mode | Rank | Meaning |
| --- | --- | --- |
| `ordinary_browsing` | 1 | The weaker baseline an everyday browsing pane uses. Recorded only for contrast. |
| `strong_decision` | 2 | The stricter mode every trust-decision surface must use. |

A trust-decision surface always resolves to `strong_decision`, whose rank
(`2`) is strictly higher than the `ordinary_browsing` baseline (`1`).
`validate` enforces the strict ordering so a surface can never quietly drop to
ordinary-browsing rendering.

## Suspicious-cue detection

The shared detector runs over each identity. When it reports findings, the
surface records the distinct threat-class tokens (`bidi_control`,
`invisible_formatting`, `mixed_script_confusable`, …), a warning, and an extra
codepoint-inspection affordance. The `identity_inspection_escaped` field is the
shared escaped/safe form (bidi and invisible codepoints become `\u{XXXX}`); it is
never the raw bytes and never masquerades as them — the raw identity is reachable
only through the `open_raw_identity` action's `raw_identity_ref`.

| Warning | When |
| --- | --- |
| `bidi_or_invisible_bytes` | The identity hides bidi-control or invisible-formatting codepoints. |
| `mixed_script_confusable` | The identity mixes scripts so a glyph is not the letter it appears to be. |

## Raw identity inspection

Every surface offers, at minimum, an open-raw and a copy-escaped affordance;
suspicious surfaces add a codepoint inspector:

| Action | Availability |
| --- | --- |
| `open_raw_identity` | Always — opens the raw identity bytes behind the rendered label. |
| `copy_escaped_identity` | Always — copies the shared escaped, safe-inspection form. |
| `inspect_codepoints` | When the detector flagged suspicious cues. |

## Invariants

The producer guarantees, and `validate` enforces, that:

- Every trust-decision surface uses `strong_decision`, strictly stronger than
  ordinary browsing.
- The full identity is shown without truncation (`displayed_char_len` equals
  `identity_char_len`).
- Open-raw and copy-escaped stay reachable on every surface; suspicious bytes are
  surfaced, never normalized away, and the escaped form never masquerades as raw.
- Every surface whose identity carries suspicious cues shows a warning, records
  its threat classes, and offers codepoint inspection.
- The same stricter rendering is preserved in product, exported review packets,
  and support handoff (`preserved_in_product`,
  `preserved_in_exported_review_packet`, `preserved_in_support_handoff`).

The packet is metadata only: no raw identity bytes, raw provider payloads, or
credentials cross the export boundary — only opaque refs and the escaped
inspection form.

## Consumers

The headless `m5_trust_decision_identity` tool is the first CLI/headless
consumer; it emits the canonical support export, the Markdown summary, the clean
fixture, and validates any packet. Support, diagnostics, review-packet, and
release tooling read the machine-readable packet, warnings, threat classes, and
inspection affordances directly rather than cloning prose.
