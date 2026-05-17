# Descriptor And Invocation Fixtures

These fixtures exercise the canonical command object boundary:

- `workspace_open_folder.descriptor.json` declares the public descriptor metadata expected by command surfaces and extension admission.
- `palette_open_folder.invocation.json` records the invocation-session envelope for the same command.
- `palette_open_folder.result.json` records the structured result packet consumed by support, automation, export, and parity tooling.
- `deprecated_cli_alias.result.json` records a successful invocation through a deprecated CLI alias without changing canonical command identity.

The fixtures use opaque refs only. They intentionally avoid raw paths, raw labels, raw arguments, or raw provider payloads.
