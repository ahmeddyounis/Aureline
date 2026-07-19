<!--
SPDX-FileCopyrightText: 2026 Aureline contributors
SPDX-License-Identifier: Apache-2.0
-->

# Coding-agent guidance

This file applies to the entire repository. Human contributor requirements in
[`CONTRIBUTING.md`](./CONTRIBUTING.md) also apply to agent-authored changes.

## Read before changing code

For non-trivial work, search the authoritative specifications in `.t2/docs/`
before editing. The files are large, so use targeted `rg` searches and read the
surrounding sections. Start with the document that owns the affected concern:

- `Aureline_PRD.md` for requirement and verification intent;
- `Aureline_Technical_Architecture_Document.md` for system boundaries;
- `Aureline_Technical_Design_Document.md` for component contracts;
- `Aureline_UI_UX_Spec_Document.md` and
  `Aureline_UX_Design_System_Style_Guide.md` for interaction and visual rules;
- `Aureline_Security_Threat_Model.md` for trust and abuse boundaries.

Then consult the checked-in contract, schema, fixture, artifact, ADR, and RFC
that governs the exact surface. [`docs/README.md`](./docs/README.md) is the
repository-local index. When prose, code, and a machine-readable contract
disagree, preserve the safer/narrower behavior and update the owning artifacts
together; do not silently broaden a stable or protected surface.

## Implementation guardrails

- Respect [`docs/repo/dependency_rules.md`](./docs/repo/dependency_rules.md)
  and `artifacts/governance/package_inventory.yaml`. New crates or dependency
  edges require the topology, inventory, and ownership updates described there.
- Preserve public record kinds, schema versions, command IDs, result codes,
  source anchors, redaction rules, and compatibility windows unless the owning
  migration/decision artifacts change in the same commit.
- Never place raw secrets, credentials, private payloads, or unredacted user
  content in logs, fixtures, snapshots, telemetry, support exports, or commits.
- Keep mutations previewable, scoped, auditable, and recoverable. Do not bypass
  approval, trust, policy, protected-path, or stale-evidence gates.
- Maintain the workspace's declared Rust 1.75 language/library compatibility
  even though the reproducible build uses the newer pinned compiler.
- Preserve unrelated working-tree changes. Do not commit generated `target/`
  output or local credentials.

## Validation

Run the narrowest affected tests while iterating, then the applicable shared
gates. The baseline commands are:

```sh
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -m pip install --requirement tools/requirements-contract-validation.txt
./ci/contract_validation.sh --out-dir target/contract-validation
```

Changes to build identity or release assembly should also use
`./tools/build/build.sh`. Specialized lanes under `ci/` and `tools/ci/` remain
mandatory when their owning docs or workflows name them.

## Commits

Use conventional commit subjects and include the Developer Certificate of
Origin sign-off with `git commit -s`. Keep commits coherent and report the exact
validation commands that passed.
