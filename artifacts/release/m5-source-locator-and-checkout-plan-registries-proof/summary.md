# M5 Source-Locator and Checkout-Plan Registries

- Packet: `m5-source-locator-and-checkout-plan-registries:stable:0001`
- Label: `M5 source-locator and checkout-plan registries with one stable source-locator object resolving per entry flow, open and clone staying distinct verbs with a preserved literal target, the bootstrap credential posture disclosed before any network or mirror fetch, canonical / accessible / audit resolution-form coverage, and the complete ref-selection / depth-filter / submodule-mode / LFS-posture / destination-path / cost-band checkout-plan object across acquisition-engine, shell, workspace, git, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Source-locator kinds: local_path_source, remote_forge_url_source, archive_import_bundle_source, mirror_source, managed_snapshot_source, kind_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **acquisition_engine**: `stable`
  - Owner: Acquisition-engine owner
  - Scope: The acquisition engine resolves the local-path source locator to one stable object — literal target, resolved checkout root, staged-trust metadata, disclosed credential posture, signer provenance, and the distinct mirror / air-gap hint — from the shared registry and derives the full checkout plan; a locator object missing its resolved root and a checkout plan that would run a repo-owned action implicitly degrade honestly instead of reading as a clean pass
  - Source-locator entries: 2 / checkout-plan entries: 2
- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the remote-forge source locator while disclosing its credential posture before the fetch, and renders the partial checkout plan; a resolution-form gap on a locator entry and on a checkout plan is caught before a screenshot can reintroduce a false-truth reading
  - Source-locator entries: 2 / checkout-plan entries: 2
- **workspace_service**: `stable`
  - Owner: Workspace-service owner
  - Scope: The workspace service reports the archive / import-bundle source locator and the sparse checkout plan without manual reconstruction; a clone whose literal target was silently reopened over an existing local checkout is caught as a verb rewrite
  - Source-locator entries: 2 / checkout-plan entries: 1
- **git_service**: `stable`
  - Owner: Git-service owner
  - Scope: The git service resolves the mirror source locator while keeping signer / mirror provenance continuous and bound to the registry; a locator that is a hand-copied per-entry assumption and a checkout plan on an unclassified mode degrade honestly
  - Source-locator entries: 2 / checkout-plan entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved source-locator and checkout-plan truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied acquisition table
  - Source-locator entries: 2 / checkout-plan entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved source-locator and checkout-plan truth, so a hand-copied constant, an unstated registry token, a verb rewrite, or an implicit bootstrap is visible in evidence rather than hidden behind a screenshot
  - Source-locator entries: 2 / checkout-plan entries: 1
