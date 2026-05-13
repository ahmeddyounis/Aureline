# Identity Mode Alpha Fixtures

These protected fixtures exercise the account-free local, self-hosted
organization, and managed convenience identity-mode baseline.

Each packet parses into `IdentityModeBaselinePacket` and proves:

- local core remains available without account creation;
- self-hosted and managed rows disclose policy source, freshness, entitlement
  state, and offline behavior;
- managed-only outages or grace windows pause only managed actions and keep
  local work available;
- the current deployment boundary is stated narrowly enough for the active
  profile.

Fixtures use refs, aliases, and export-safe labels only. They do not include
raw credentials, raw tenant names, raw user email addresses, hosted-console
payloads, or signed policy/entitlement bundle bodies.
