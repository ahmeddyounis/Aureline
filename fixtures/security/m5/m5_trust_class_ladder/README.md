# M5 Trust-Class Ladder Fixtures

These fixtures are valid, export-safe trust-class ladder packets that exercise
the runtime resolution across the new M5 preview and embedded surfaces. Each one
keeps every surface present, covers every downgrade trigger in the rule catalog,
confines active content to its declared trust class, keeps embedded/review
surfaces non-executing and strong-decision surfaces in strict identity, keeps raw
inspection and raw copy reachable, and degrades unsafe states to sanitized or
compare-only visibility without leaking raw bytes.

## all_trusted_no_downgrade.json

Every surface's requested trust class is honored: all trust signals are clear, so
no downgrade rule fires and no surface degrades. Active-capable surfaces execute
within their declared trusted-local or isolated-remote class; embedded/review
surfaces stay at sanitized rich by construction. Demonstrates that the resolution
holds — and validates — when nothing needs to narrow, so a fully trusted run does
not silently behave like a degraded one. Regenerate with
`m5_trust_class_ladder --clean`.
