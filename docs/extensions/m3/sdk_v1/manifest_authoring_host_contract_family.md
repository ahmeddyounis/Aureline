# SDK v1 manifest authoring guide: picking a host contract family

This guide is the canonical walkthrough for picking a host contract
family and binding the claim to the runtime v1 beta contract. It is
referenced as
`manifest_guide:host_contract_family_walkthrough:1.0.0` in the
[SDK v1 starter pack](./README.md).

## Step 1: pick a host contract family

The closed `HostContractFamilyClass` vocabulary covers:

- `wasm_component_model` — capability-bounded Wasm component-model
  extensions running in the in-process isolated world.
- `wasm_core_module` — capability-bounded Wasm core-module
  extensions.
- `external_host_process` — separately supervised host processes
  attached through a typed envelope.
- `helper_binary` — short-lived helper processes attached through a
  typed envelope.
- `remote_side_component` — components attached over a remote agent
  envelope.
- `compatibility_bridge` — legacy extension API calls translated
  through a compatibility bridge.

The same vocabulary appears as `SdkV1ApiSurfaceClass` in the starter
pack; the `host_contract_family_for_api_surface` projection is total
and closed.

## Step 2: bind to a published SDK v1 API surface

Every claimed surface row in the starter pack carries:

- `api_surface_class` (one of the surface classes above);
- `host_contract_family_class` that matches the surface class through
  `host_contract_family_for_api_surface`;
- `sdk_release_bundle_ref` prefixed `sdk_release_bundle:`;
- `covered_wit_world_refs` for wasm surfaces, or
  `external_host_contract_ref` for external-host / helper-binary
  surfaces; and
- an `availability_class` from the closed
  `SdkV1ApiAvailabilityClass` vocabulary.

A wasm surface row without any covered WIT world refs is denied at
schema validation with
`sdk_v1_api_surface.wasm_surface_must_cite_wit_world_refs`. An
external-host / helper-binary surface row without an external host
contract ref is denied with
`sdk_v1_api_surface.external_host_surface_must_cite_contract_ref`.

## Step 3: prove the claim through the runtime contract

[`evaluate_runtime_v1_beta_contract`](../../../../crates/aureline-extensions/src/runtime/mod.rs)
validates that the manifest's declared host contract family matches an
authorized `HostPlacementClass` / `HostSupervisionClass` pair. A
manifest that claims `wasm_component_model` but is placed as
`external_host_supervised_process` is refused with
`host_placement_unsupported`.

## Repair affordance

If the runtime contract evaluator emits a refusal, fix the manifest's
declared host contract family or fix the host placement/supervision
binding and rerun:

```text
cargo test -p aureline-extensions runtime
cargo test -p aureline-extensions supervision
```

The starter-pack lane refuses a sample row whose
`host_contract_family_class` disagrees with its `api_surface_class`
with `refused_sample_host_family_disagrees_with_api_surface`.
