# M5 Trust-Decision Identity Fixtures

These fixtures are valid, export-safe trust-decision identity packets that
exercise strong-decision identity rendering, suspicious-cue detection, warnings,
and raw-identity inspection affordances across the new M5 install/update,
remote-attach, collaboration-invite, route-share, and policy-review surfaces.
Each one keeps every surface present, keeps every surface in the strong-decision
render mode that is strictly stronger than ordinary browsing, keeps the full
identity shown without truncation, keeps open-raw and copy-escaped reachable, and
preserves the stricter rendering in product, export, and support handoff without
leaking raw identity bytes.

## all_clean_identities.json

Every identity is plain ASCII, so the shared suspicious-content detector flags
nothing: `suspicious_surface_count` is `0`, no surface carries a warning, and no
surface offers a codepoint inspector. Demonstrates that strong-decision rendering
and reachable raw inspection hold — and validate — even when there is nothing to
flag, so a fully clean run does not silently behave like an ordinary browsing
pane. Regenerate with `m5_trust_decision_identity --clean`.
