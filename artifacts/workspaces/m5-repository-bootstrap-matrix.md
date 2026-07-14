# M5 Repository-Bootstrap, Checkout-Plan, Trust-Stage, and Post-Open-Queue Matrix

- Packet: `m5-repository-bootstrap:stable:0001`
- Label: `M5 repository-bootstrap, checkout-plan, trust-stage, and post-open-queue matrix`
- Repository-bootstrap families: 5 (5 stable)
- Repository-bootstrap roles: source_locator, checkout_plan, credential_posture, evidence_packet, staged_trust, resumable_acquisition, post_open_queue
- Open-local roles: local_checkout_root_located, existing_checkout_detected_not_recloned, working_tree_and_git_dir_distinguished, read_only_partial_root_offered_when_incomplete, bound_to_repository_bootstrap_registry, reclone_over_existing_local_checkout_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Repository-bootstrap families

- **open_local**: `stable`
  - Owner: Repository-acquisition owner
  - Canonical schema: `schemas/workspaces/m5-source-locator.schema.json`
  - Scope: One open-local profile naming the located local checkout root, the existing checkout detected rather than recloned, the working-tree-versus-git-dir distinction, and the read-only partial root offered when incomplete so opening a local checkout stays a distinct verb and never rewrites clone into open because a local checkout already exists
  - Required labels: identity, semantic_role, registry_reference, source_locator
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **clone_remote**: `stable`
  - Owner: Git-service owner
  - Canonical schema: `schemas/workspaces/m5-checkout-plan.schema.json`
  - Scope: One clone-remote profile naming the resolved remote source locator, the checkout cost and topology shown before the fetch, the credential posture disclosed before network access, and the declared sparse or partial checkout plan so a remote clone shows checkout cost, topology, and credential posture before any network or disk mutation and never runs a repo-owned action implicitly during the clone
  - Required labels: identity, semantic_role, registry_reference, source_locator, checkout_plan, credential_posture
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **open_archive**: `stable`
  - Owner: Repository-acquisition owner
  - Canonical schema: `schemas/workspaces/m5-source-locator.schema.json`
  - Scope: One open-archive profile naming the located archive container, the archive digest verified before extract, the extraction plan shown before disk mutation, and the disclosed nested-archive topology so opening an archive stays a distinct verb, shows its extraction plan before disk mutation, and never silently overwrites a working tree
  - Required labels: identity, semantic_role, registry_reference, source_locator
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **import_bundle**: `stable`
  - Owner: Trust-service owner
  - Canonical schema: `schemas/workspaces/m5-bootstrap-evidence.schema.json`
  - Scope: One import-bundle profile naming the verified bundle signer continuity, the preserved mirror and air-gap provenance, the bundle digest verified before import, and the recorded offline-import evidence so importing a bundle preserves signer and mirror provenance across offline or mirrored fetches and stages trust rather than running repo-owned actions implicitly
  - Required labels: identity, semantic_role, registry_reference, credential_posture
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **resume_snapshot**: `stable`
  - Owner: Workspace-service owner
  - Canonical schema: `schemas/workspaces/m5-bootstrap-evidence.schema.json`
  - Scope: One resume-snapshot profile naming the resumable partial-acquisition state, the offered Resume / Discard / Open-read-only-partial-root choice, the typed post-open bootstrap queue, and the preserved resume evidence so an interrupted or partial acquisition stays resumable or discardable with evidence and never strands partial state without a choice or auto-executes a post-open queue
  - Required labels: identity, semantic_role, registry_reference, checkout_plan
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
