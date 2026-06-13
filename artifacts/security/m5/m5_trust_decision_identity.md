# M5 Trust-Decision Identity Rendering

- Packet: `m5-trust-decision-identity:stable:0001`
- Case: `case:m5-trust-decision-identity:stable`
- Surfaces: 5 · suspicious: 3
- Strength: strong-decision 2 > ordinary-browsing 1

## Surfaces

- **publisher_package_name** → decision `install_or_update` (Install / Update), render `strong_decision`
  - Identity: `Aurelіne Labs` (13 chars) · suspicious: true · raw ref: `marketplace:publisher:formatter@3.1.0:identity`
  - Threat classes: mixed_script_confusable
  - Inspection: open_raw_identity, copy_escaped_identity, inspect_codepoints
- **remote_host_label** → decision `remote_attach` (Attach), render `strong_decision`
  - Identity: `build\u{200B}-01.internal` (18 chars) · suspicious: true · raw ref: `remote:host:build-01:identity`
  - Threat classes: invisible_formatting
  - Inspection: open_raw_identity, copy_escaped_identity, inspect_codepoints
- **collaborator_identity** → decision `collaboration_invite` (Invite), render `strong_decision`
  - Identity: `Dana Okonomou` (13 chars) · suspicious: false · raw ref: `collab:invite:9f2:identity`
  - Inspection: open_raw_identity, copy_escaped_identity
- **route_share** → decision `route_share` (Share), render `strong_decision`
  - Identity: `team-observability` (18 chars) · suspicious: false · raw ref: `route:share:dashboard:target`
  - Inspection: open_raw_identity, copy_escaped_identity
- **policy_review** → decision `policy_review` (Approve), render `strong_decision`
  - Identity: `Allow egress to \u{202E}gpc.example\u{202C}` (29 chars) · suspicious: true · raw ref: `policy:review:egress-allow-list:text`
  - Threat classes: bidi_control
  - Inspection: open_raw_identity, copy_escaped_identity, inspect_codepoints
