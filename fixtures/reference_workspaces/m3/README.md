# Beta Reference-Workspace Packets

This directory contains the governed reference-workspace packets for the
beta launch wedges. The workspace descriptors under
`fixtures/workspaces/reference/` remain the byte-shape source of truth.
The packets here add owners, toolchain pins, privacy/license posture,
workflow coverage, harness bindings, and consumer links so compatibility,
benchmark, migration, support, and release evidence can cite one register.

The canonical machine register is
`artifacts/compat/m3/reference_workspace_register.yaml`.

| Packet | Reference workspace | Archetype row |
|---|---|---|
| [`jvm_service/workspace.yaml`](./jvm_service/workspace.yaml) | `refws.java_kotlin_service_archetype_seed` | `archetype_row:java_or_kotlin_service` |
| [`rust_workspace/workspace.yaml`](./rust_workspace/workspace.yaml) | `refws.small_rust_self_host_slice` | `archetype_row:rust_workspace` |
| [`go_service/workspace.yaml`](./go_service/workspace.yaml) | `refws.go_service_archetype_seed` | `archetype_row:go_service_or_monorepo_slice` |
| [`cpp_native/workspace.yaml`](./cpp_native/workspace.yaml) | `refws.c_cpp_native_archetype_seed` | `archetype_row:c_or_cpp_native_project` |
