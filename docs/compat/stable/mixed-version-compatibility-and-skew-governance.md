# Mixed-Version Compatibility and Skew Governance

This page is the reviewer-facing companion to
`artifacts/compat/stable/mixed-version-compatibility-and-skew-governance.json`.
The JSON artifact is authoritative for release-center, compatibility-packet,
Help/About, diagnostics, and support-export consumers.

Stable cross-version claims are allowed only when the boundary row, negotiated
fields, skew-window drill, rollback-order proof, owner signoff, and downstream
bindings are current. A stale or missing row narrows only that boundary; it does
not inherit an adjacent green row.

| Boundary | Supported skew | Upgrade order | Rollback order | Unsupported label |
|---|---|---|---|---|
| Launcher and local sidecars | Coordinated artifact set only | Full artifact set | Full artifact set | `coordinated_upgrade_required` |
| Desktop/CLI and remote agent/helper | Declared adjacent window | Client, then agent | Agent, then client | `file_only_fallback` |
| Desktop/CLI/browser and managed control plane | Current plus previous minor/LTS | Control plane, then clients | Control plane, then clients | `read_only_downgrade` |
| Extension host and SDK/runtime ABI | Published SDK support window | Host/runtime, then extension | Extension, then host/runtime | `extension_quarantined` |
| Saved artifact and schema readers/writers | Same schema epoch, additive-only | Writer, then reader | Writer, then reader | `attributed_compatibility_error` |
| Provider adapters and provider-linked packets | Current plus previous minor/LTS | Service family, then client | Service family, then client | `read_only_downgrade` |
| Audit and event producers/consumers | Same schema epoch, additive-only | Producer, then consumer | Producer, then consumer | `attributed_compatibility_error` |

The provider-adapter row is intentionally narrowed below Stable in the current
snapshot because its skew-window and rollback-order proofs are stale. Stable
widening remains held until those packets are refreshed.
