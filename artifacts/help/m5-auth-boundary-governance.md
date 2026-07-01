# M5 browser / device-code handoff cards

Card set: `m5_browser_handoff_card_set:default`

| Handoff kind | Reason | Data exit | Device code | Fallback | Return anchor |
| --- | --- | --- | --- | --- | --- |
| System browser sign-in | Authenticate with provider | `vendor_or_third_party_outbound` | false | Retry handoff in app | `anchor.return.after_sign_in` |
| Device-code authorization | Authorize device code | `no_payload_leaves_product` | true | Manual code entry | `anchor.return.after_device_code` |
| Provider content in browser | View provider content | `external_public_browse` | false | Copy link to open manually | `anchor.return.after_content_view` |
| Vendor / third-party link | Open vendor resource | `vendor_or_third_party_outbound` | false | Copy link to open manually | `anchor.return.after_vendor_link` |

Every card opens outside native chrome, never impersonates it, and preserves local continuity plus a truthful return anchor; device-code cards disclose the code and its expiry.

# M5 webview origin bars

Bar set: `m5_webview_origin_bar_set:default`

| Owner class | Origin disclosure | Permission | Open in browser | Capability limits |
| --- | --- | --- | --- | --- |
| Extension-owned | Named extension origin | Scoped permissions granted | true | 2 |
| Provider-owned | Named provider origin | No elevated permissions | true | 3 |
| First-party embedded | First-party origin | No elevated permissions | true | 2 |
| Unknown / untrusted | Undisclosed origin (blocked) | Permission denied | true | 5 |

Every bar is labeled embedded, never impersonates native chrome, holds every native-only messaging flag false, and discloses that it is not native trust chrome.
