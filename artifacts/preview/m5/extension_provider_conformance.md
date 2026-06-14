# M5 Extension-Provider Conformance

- Packet: `m5-extension-provider-conformance:stable:0001`
- Label: `M5 Extension-Provider Conformance`
- Rows: 5 (1 write-capable, 4 disclosed)
- Provider origins: 2 / 2
- Provider statuses: 4 / 4
- Operating profiles: 4 / 4

## Rows

- **conformance:embedded-preview:0001** (Embedded preview source-first inspect-and-edit)
  - First-party embedded-preview provider declared exact mapping, full attach, and hot reload; it backs the live source-first edit row
  - origin=first_party status=conformant profile=live target=embedded_preview mapping=exact attach=dom_only hot_reload=supported
- **conformance:external-browser:0001** (External browser preview parity)
  - First-party external-browser provider is conformant but its host is offline; the row presents a bounded mirror snapshot
  - origin=first_party status=conformant profile=mirror_offline target=external_browser mapping=approximate attach=dom_only hot_reload=supported
  - Repair: action=use_mirror_offline — The external browser host is offline; reconnect it to leave the mirror snapshot, or keep working against the mirror as read-only bounded truth
  - Disclosed: Only a mirror/offline snapshot is reachable; this row reflects the last captured state, not a live external browser
- **conformance:device-browser:0001** (Device browser inspect-to-source)
  - Contributed device-bridge provider declaration went stale; the prior declaration is preserved and the row is held inspect-only
  - origin=contributed status=stale_declaration profile=inspect_only target=device_browser mapping=approximate attach=dom_only hot_reload=restart_only
  - Repair: action=reverify_declaration — The device-bridge extension changed its declared capabilities; re-verify the provider declaration before this row can leave inspect-only
  - Disclosed: This contributed provider's capability declaration is stale and unverified; the row is held inspect-only until it is re-verified
- **conformance:external-browser:0002** (External browser network-depth inspection)
  - A weaker contributed lite-bridge provider would replace a stronger one; the swap is refused and the row is policy-limited
  - origin=contributed status=weaker_replacement profile=policy_limited target=external_browser mapping=approximate attach=dom_only hot_reload=unsupported
  - Repair: action=restore_stronger_provider — The lite-bridge extension declares weaker network depth and no hot reload than the provider it would replace; restore the stronger provider to regain full inspection
  - Disclosed: A weaker contributed provider was proposed in place of a stronger one; it is not allowed to silently take over, so this row is policy-limited
- **conformance:remote-preview:0001** (Remote preview session network inspection)
  - Contributed remote-runtime provider became unavailable; its prior declaration is preserved and the row falls back to a mirror snapshot
  - origin=contributed status=unavailable profile=mirror_offline target=remote_preview_session mapping=runtime_only attach=dom_styles_network hot_reload=not_applicable
  - Repair: action=reinstall_provider — The remote-runtime extension is no longer available; reinstall or re-enable it to restore live remote inspection, otherwise the mirror snapshot is read-only
  - Disclosed: The backing remote-runtime provider is unavailable; the row reflects its last captured declaration and a mirror snapshot, not a live session
