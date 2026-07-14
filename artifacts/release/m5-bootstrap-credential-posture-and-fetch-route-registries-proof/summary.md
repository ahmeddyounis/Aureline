# M5 Credential-Posture and Fetch-Route Registries

- Packet: `m5-bootstrap-credential-posture-and-fetch-route-registries:stable:0001`
- Label: `M5 bootstrap credential-posture and fetch-route registries with one stable credential-posture object resolving per acquisition path, the posture staying handle-only with no raw secret embedded and host trust disclosed, canonical / accessible / audit resolution-form coverage, and the complete route-endpoint / signer-continuity / digest-continuity / mirror-provenance / recovery-language / trust-proof fetch-route object across acquisition-engine, git, trust, diagnostics, CLI, and support surfaces`
- Consumer surfaces: 6
- Credential auth sources: anonymous_public, delegated_token, stored_handle_credential, host_key_or_tls_pinned, air_gap_offline, kind_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **acquisition_engine**: `stable`
  - Owner: Acquisition-engine owner
  - Scope: The acquisition engine resolves the anonymous-public credential posture to one stable object — auth-source reference, proxy / mirror route, host-key / TLS-pin state, delegated-token policy, handle-only secret reference, and mirror / signer provenance — from the shared registry and derives the public-upstream fetch route; a posture object missing its proxy / mirror route and a route that drops signer continuity across a mirrored fetch degrade honestly instead of reading as a clean pass
  - Credential-posture entries: 2 / fetch-route entries: 2
- **git_service**: `stable`
  - Owner: Git-service owner
  - Scope: The git service resolves the delegated-token credential posture while keeping the secret handle-only, and renders the approved-mirror fetch route with signer continuity preserved; a resolution-form gap on a posture entry and on a fetch route is caught before a screenshot can reintroduce a false-truth reading
  - Credential-posture entries: 2 / fetch-route entries: 2
- **trust_service**: `stable`
  - Owner: Trust-service owner
  - Scope: The trust service reports the host-key / TLS-pinned credential posture and the air-gap bundle-import route without manual reconstruction; a delegated-token posture that embedded raw secret material in the portable manifest instead of a handle-only reference is caught as a secret embed
  - Credential-posture entries: 2 / fetch-route entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics resolves the stored-handle credential posture while keeping the secret handle-only and bound to the registry, and renders the managed-snapshot resume route; a posture that is a hand-copied per-entry assumption and a fetch route on an unclassified class degrade honestly
  - Credential-posture entries: 2 / fetch-route entries: 2
- **cli_export**: `stable`
  - Owner: CLI-export owner
  - Scope: The CLI export renders the same resolved credential-posture and fetch-route truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied credential table
  - Credential-posture entries: 2 / fetch-route entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved credential-posture and fetch-route truth without embedding raw secrets, so a hand-copied constant, an unstated registry token, an embedded secret, or a dropped signer continuity is visible in evidence rather than hidden behind a screenshot
  - Credential-posture entries: 2 / fetch-route entries: 1
