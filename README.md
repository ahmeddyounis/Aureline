# Aureline

Aureline is an open-source next-generation IDE (working name). The
repository is under active pre-release implementation and contract
maturation. It contains a Rust workspace, executable validation lanes,
schemas, fixtures, and release/governance artifacts; it is not yet a
production release.

- Docs index: [`docs/README.md`](./docs/README.md).
- Decision records: [`docs/adr/`](./docs/adr/) and
  [`docs/rfc/`](./docs/rfc/).
- Decision backlog: [`docs/governance/decision_backlog.md`](./docs/governance/decision_backlog.md).
- DRI and escalation: [`docs/governance/dri_map.md`](./docs/governance/dri_map.md).
- Contributing: [`CONTRIBUTING.md`](./CONTRIBUTING.md) (setup, ADR/RFC workflow, protected-path evidence, DCO, REUSE/SPDX, dependency hygiene).
- Dogfood intake: [`docs/governance/dogfood_issue_taxonomy.md`](./docs/governance/dogfood_issue_taxonomy.md).
- Supportability concept: [`docs/support/support_center_concept.md`](./docs/support/support_center_concept.md).
- Contributor notes: [`AGENTS.md`](./AGENTS.md), [`CLAUDE.md`](./CLAUDE.md).

## Build and validate

Use the pinned toolchain and checked-in entry points:

```sh
./tools/build/bootstrap.sh
./tools/build/build.sh
cargo test --locked --workspace
python3 -m pip install --requirement tools/requirements-contract-validation.txt
./ci/contract_validation.sh --out-dir target/contract-validation
```

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for protected-path,
dependency, evidence, DCO, and licensing requirements.
