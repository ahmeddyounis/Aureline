# Help / About / service-health parity cases

Worked fixtures for the cross-surface parity packet frozen in:

- [`/docs/docs/docs_help_about_service_health_parity.md`](../../../docs/docs/docs_help_about_service_health_parity.md)
- [`/artifacts/docs/help_parity_matrix.yaml`](../../../artifacts/docs/help_parity_matrix.yaml)
- [`/schemas/docs/help_badge_projection.schema.json`](../../../schemas/docs/help_badge_projection.schema.json)

These cases are intentionally “small”: each fixture binds a single logical
condition (mirror stale, pack missing, version mismatch, provider outage, etc.)
to a set of `help_badge_projection_record` instances emitted by different
surfaces. The parity audit compares the projections field-for-field for the
axes listed in the matrix, so badge wording cannot drift across:

- embedded docs panes and docs search,
- Help / About,
- service-health,
- migration/help pivots,
- release center artifact rows, and
- support-bundle exports.

The fixtures carry only opaque ids, placeholder timestamps, and controlled
vocabulary. Raw URLs, hostnames, tenant identifiers, and secrets are forbidden.

