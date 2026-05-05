# Emergency disable bundle and local-continuity card fixtures

These fixtures anchor the emergency disable bundle and local-continuity
card contract frozen in:

- `/docs/security/emergency_disable_bundle_contract.md`
- `/schemas/security/emergency_disable_bundle.schema.json`
- `/schemas/security/local_continuity_card.schema.json`

They exist so emergency disablement remains durable and inspectable
across connected, mirrored, manual-import, and offline environments.

**Scope rules**

- Fixtures validate as either `emergency_disable_bundle_record` or
  `local_continuity_card_record`.
- Every disable bundle includes explicit scope, explicit expiry or
  follow-up rule, signer continuity, and at least one distribution row.
- Local continuity cards always state what still works, what is blocked,
  what triggered the state, and the next safe recovery path.
- Raw bundle bytes, raw signatures, raw trust roots, raw registry
  payloads, raw URLs, hostnames, absolute paths, and secret material
  never appear.

