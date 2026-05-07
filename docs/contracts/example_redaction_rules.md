# Example Redaction & Fidelity Rules

This document defines how contract example payloads remain:

- redaction-safe (exportable, loggable, and linkable without leaking secrets or
  internal-only material),
- explicit about fidelity (so docs/tests do not overclaim),
- schema-checkable in CI (so examples stay executable as schemas evolve).

Authoritative index: `artifacts/contracts/example_pack_index.yaml`.

## Fidelity classes

Every indexed example MUST declare one `fidelity_class`:

- `canonical`: Preferred example shape for implementers to follow.
- `intentionally_partial`: Deliberately omits optional sections to stay small.
- `redacted`: Sensitive values are removed or replaced with safe placeholders.
- `deprecated`: Historical shape retained for migration/compat review only.
- `illustrative_only`: Narrative-only shape sketch; do not treat as a contract.

## Redaction-safe content rules

Examples MUST NOT include:

- raw secret material (API keys, tokens, cookies, private keys, OAuth codes),
- real user identifiers (emails, full names, personal handles),
- real hostnames, internal endpoints, or private network topology,
- machine-local absolute paths that identify a real user or machine.

Preferred substitutions:

- Use opaque ids already modeled by the schema (e.g. `*_id`, `*_ref` fields).
- Use placeholder domains (e.g. `example.com`) when a domain is required.
- Use placeholder home directories (e.g. `/Users/example/...`, `C:\\Users\\Example\\...`).
- Prefer typed redaction vocabularies already present in the contract
  (`redaction_class`, omission blocks, retention-by-reference stubs).

## Schema validity and fixture-only metadata

Examples are allowed to carry fixture-only metadata to explain intent:

- `$schema` (editor/schema hint)
- `__fixture__` (scenario prelude used by fixtures)

Validation MUST ignore these keys when checking schema conformance.

Some examples are packaged as a small wrapper with `records: [...]` (a list of
schema-shaped records). Validation MUST apply schema checks to each record in
order, not to the wrapper object.

## Versioning and deprecation

- Examples MUST carry the schema version fields required by their contract
  (`*_schema_version`, `schema_version`, etc.).
- When a breaking schema change lands, affected examples MUST either be updated
  in the same change or reclassified as `deprecated` with a replacement example.
