# Framework pack headers and framework status strips

- Packet: `m5-framework-pack-header-status-strip-controls:stable:0001`
- Surface: `M5 framework pack headers and framework status strips: pack identity, version range, support class, provider source, workspace scope, freshness, health, and local-versus-remote scope truth across claimed framework surfaces`
- Framework pack headers: 6 (4 bridged or heuristic)
- Framework status strips: 6 (5 not local scope)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Framework pack headers

- **Next.js app pack** — support `officially_supported` → `fully_supported`, experience `core_native`, scope `local_scope`, freshness `current`, deep link `pack_manifest`
- **Django pack** — support `community_supported` → `community_supported`, experience `pack_backed`, scope `container_scope`, freshness `imported`, deep link `provider_registry_entry`
- **Svelte pack** — support `experimental` → `experimental_or_bridge`, experience `heuristic`, scope `remote_scope`, freshness `stale`, deep link `compatibility_reference`
- **Rails bridge pack** — support `bridge_only` → `experimental_or_bridge`, experience `bridged`, scope `managed_scope`, freshness `never_scanned`, deep link `docs_anchor`
- **Legacy PHP pack** — support `deprecated` → `unsupported_or_deprecated`, experience `heuristic`, scope `remote_scope`, freshness `unknown`, deep link `docs_anchor`
- **Unidentified pack** — support `unsupported` → `unsupported_or_deprecated`, experience `heuristic`, scope `unknown_scope`, freshness `stale`, deep link `no_deep_link`

## Framework status strips

- **Next.js** — support `officially_supported`, identity `identified_versioned`, experience `core_native`, scope `local_scope`, health `healthy`, deep link `pack_manifest`
- **Django** — support `community_supported`, identity `version_pinned`, experience `pack_backed`, scope `container_scope`, health `degraded`, deep link `provider_registry_entry`
- **SvelteKit** — support `experimental`, identity `version_drifted`, experience `heuristic`, scope `remote_scope`, health `compatibility_warning`, deep link `compatibility_reference`
- **Rails** — support `bridge_only`, identity `multiple_detected`, experience `bridged`, scope `managed_scope`, health `broken`, deep link `docs_anchor`
- **PHP** — support `deprecated`, identity `unversioned`, experience `heuristic`, scope `remote_scope`, health `unknown`, deep link `docs_anchor`
- **Unresolved framework** — support `unsupported`, identity `unknown_pack`, experience `heuristic`, scope `unknown_scope`, health `degraded`, deep link `compatibility_reference`
