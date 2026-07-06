# M5 Bundle Class-Disclosure Primitive: Class-Disclosure Card and Claim-Narrowing Row

- Packet: `m5-bundle-class-disclosure-primitive:stable:0001`
- Label: `M5 Bundle Class-Disclosure Primitive: Class-Disclosure Card and Claim-Narrowing Row`
- Class-disclosure surfaces: 6 / 6
- Disclosure classes: native_first_party, imported_user_handoff, managed_approved, design_partner_certified, community, local_draft
- Capability confidences: native, exact, capability_mapped, approximate, unsupported_gap

## Class-disclosure surfaces

- **Start-center class card**
  - Owner: Start-center guild
  - Scope: Start-center class card disclosing a native first-party bundle's class, native compatibility, and fresh certification before launch
  - Worked cases: 1
    - `disclosure:rust-service:0001` → native_first_party class, native compatibility, certified claim
- **Bundle detail class panel**
  - Owner: Bundle detail guild
  - Scope: Bundle detail class panel disclosing a design-partner certified bundle whose capabilities map exactly rather than natively
  - Worked cases: 1
    - `disclosure:web-app:0002` → design_partner_certified class, exact compatibility, certified claim (narrowed)
- **Migration class-disclosure row**
  - Owner: Migration guild
  - Scope: Migration class-disclosure row disclosing an imported user-handoff bundle as bridged, pending review, and narrowed from native parity
  - Worked cases: 1
    - `disclosure:imported-monorepo:0003` → imported_user_handoff class, capability_mapped compatibility, imported claim (narrowed)
- **Docs / help class block**
  - Owner: Docs / help guild
  - Scope: Docs / help class block explaining a community bundle as capability-mapped and community-reviewed using the shared class vocabulary
  - Worked cases: 1
    - `disclosure:framework-pack:0004` → community class, capability_mapped compatibility, community claim (narrowed)
- **Diagnostics class report**
  - Owner: Diagnostics guild
  - Scope: Diagnostics class report covering a managed org-approved bundle with disclosed policy / mirror dependencies and a local draft with no external claim
  - Worked cases: 2
    - `disclosure:managed-web:0005` → managed_approved class, native compatibility, probable claim (narrowed)
    - `disclosure:local-draft:0006` → local_draft class, native compatibility, local_draft claim (narrowed)
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing an imported bundle's class as approximate, mirror-bounded, and stale, keeping the class stable offline
  - Worked cases: 1
    - `disclosure:offline-replay:0007` → imported_user_handoff class, approximate compatibility, imported claim (narrowed)
