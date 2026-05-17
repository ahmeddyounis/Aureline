# Reference Workspace Fixture Packets

This directory holds review packets for reference-workspace fixtures that need
more context than the shape descriptors under `fixtures/workspaces/reference/`.

The descriptor files remain the canonical workspace shapes. Packets here bind
those shapes to launch-wedge workflows, privacy clearance, provenance, and
repeatability notes so benchmark and release packets can cite one fixture
register row instead of copying paths.

The beta packets under `m3/` add exact toolchain manifests, pass/fail
harness entries, and consumer bindings for the JVM, Rust, Go, and C/C++
launch wedges. Their canonical register is
`artifacts/compat/m3/reference_workspace_register.yaml`.
