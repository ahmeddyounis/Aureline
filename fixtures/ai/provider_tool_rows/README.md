# AI provider / model / local-model-pack / external-tool worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/provider_model_registry_contract.md`](../../../docs/ai/provider_model_registry_contract.md)
and the schemas at
[`/schemas/ai/provider_registry.schema.json`](../../../schemas/ai/provider_registry.schema.json),
[`/schemas/ai/model_registry.schema.json`](../../../schemas/ai/model_registry.schema.json),
and
[`/schemas/ai/external_tool_registry.schema.json`](../../../schemas/ai/external_tool_registry.schema.json).

Every file is a multi-document YAML stream. The first document is
a `__fixture__` prelude summarising the scenario, the contract
sections it exercises, and the record kinds it produces. The
remaining documents are individual `ai_provider_registry_entry_record`,
`local_model_pack_entry_record`, `model_registry_entry_record`,
`provider_selection_disclosure_record`,
`ai_provider_registry_audit_event_record`,
`external_tool_registry_entry_record`,
`external_tool_invocation_disclosure_record`, and
`external_tool_registry_audit_event_record` instances that conform
to the schemas.

No fixture embeds raw URLs, raw endpoint hostnames, raw spawn
commands, raw environment variables, raw API keys, raw OAuth
tokens, raw mTLS material, raw model weights, raw pack bytes, raw
provider payloads, raw request / response bodies, raw stdio frames,
or raw credential material. Every such field is an opaque ref or
structured readout.

## Cases

| Scenario file                                            | Axis exercised                                                                 | Covered route / locus                                                                                          |
|----------------------------------------------------------|--------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| `local_model_pack_first_party_bundle.yaml`               | Local-model pack + local-in-process provider + model registry + disclosure     | First-party bundled signed pack; `execution_locus_class = local_in_process`, `offline_only_no_network_egress`  |
| `byok_vendor_direct_provider.yaml`                       | BYOK provider + model registry + disclosure (approval-ticket cited)            | BYOK vendor direct; `execution_locus_class = byok_remote_vendor_direct`, `auth_mode_class = byok_api_key`       |
| `enterprise_gateway_brokered_provider.yaml`              | Enterprise-gateway provider + disclosure; shares `model_entry_id` with BYOK    | Customer-operated enterprise gateway; `execution_locus_class = enterprise_gateway_brokered`, `deployment_profile = managed_fleet` |
| `vendor_hosted_first_party_managed_provider.yaml`        | Vendor-hosted first-party-managed provider + model registry + disclosure       | Vendor-hosted managed; `execution_locus_class = vendor_hosted_first_party_managed`, `cost_visibility = bundled_no_incremental_cost` |
| `stdio_mcp_external_tool.yaml`                           | External tool (stdio MCP) + invocation disclosure + audit trail                | Local MCP over stdio; `tool_transport_class = remote_mcp_over_stdio`, `tool_execution_locus_class = local_subprocess_same_device` |
| `local_http_external_tool.yaml`                          | External tool (loopback HTTP) + invocation disclosure + audit trail            | Local loopback HTTP; `tool_transport_class = local_http_loopback`, `tool_execution_locus_class = local_companion_service_loopback` |
| `remote_http_external_tool.yaml`                         | External tool (remote streamable-HTTP MCP) + invocation disclosure + audit     | Remote MCP streamable-HTTP; `tool_transport_class = remote_mcp_over_streamable_http`, `tool_execution_locus_class = enterprise_gateway_brokered_service` |

Every fixture declares its canonical values via the
`exercised_classes` block so later coverage audits can confirm
each vocabulary member is hit at least once.

## Acceptance-criteria coverage

The seeded cases cover the four required examples named in the
task's acceptance criteria:

- **Local model pack** — `local_model_pack_first_party_bundle.yaml`
  (a `local_model_pack_entry_record` plus its serving local-in-
  process provider and model entries).
- **BYOK provider** — `byok_vendor_direct_provider.yaml`.
- **Enterprise-gateway route** — `enterprise_gateway_brokered_provider.yaml`.
- **Stdio external tool** — `stdio_mcp_external_tool.yaml`.
- **Local-HTTP external tool** — `local_http_external_tool.yaml`.
- **Remote-HTTP external tool** — `remote_http_external_tool.yaml`.

The vendor-hosted first-party-managed fixture is included for
parity so all four provider-side route classes (local, BYOK,
enterprise gateway, vendor-hosted managed) emit the same
identity spine on every evidence surface.
