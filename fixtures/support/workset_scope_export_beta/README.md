# Support / audit export workset / scope beta corpus

Frozen corpus that promotes the support / audit **export** surface's workset /
sparse-slice / policy-limited-view truth from incidental coverage to beta. It
proves a support/audit export honours the declared scope boundary the same way
the workspace, search, graph, refactor, and AI surfaces do:

- the export **declares** the active named workset / sparse scope / policy-limited
  view it was produced under — an embedded scope declaration that itself
  validates and quotes the active scope class, workset name, and scope lineage;
- out-of-scope roots and policy-hidden members are **disclosed** through the
  scope truth's `excluded_roots` (`not_in_workset_root_list` / `policy_hidden`)
  rather than silently dropped;
- policy-limited content is **redacted / labeled** with the support redaction
  vocabulary (`policy_locked` / `omitted_policy_locked` / `policy_denied`)
  rather than silently embedded.

## What each case asserts

Each `*_export.json` case is replayed by
`crates/aureline-support/tests/workset_scope_export_beta.rs`. The test
deserializes the case's `artifact` directly into an
`aureline_workspace::WorksetArtifactRecord`, validates it, and projects an
`aureline_workspace::WorksetScopeBetaTruth` for the `Export` and `SupportPacket`
consumer surfaces. It bundles both truths into an
`aureline_workspace::WorksetScopeBetaSupportExport` — the support-export
projection — which becomes the export's scope declaration. It then assembles a
support/audit export from the declaration plus the case's redacted content rows
and asserts:

- the support `scope_label` is derived from the shared `ScopeClass` chip-label
  vocabulary, not a private label set (`support_scope_class` resolves through the
  manifest map onto a real workspace `ScopeClass`);
- the scope declaration is present and validates, declares the active scope
  class / workset name / lineage, and (for the policy-limited view) preserves the
  underlying workset as a lineage ancestor;
- out-of-scope roots are disclosed in the Export truth's `excluded_roots`
  (`not_in_workset_root_list`) and policy-hidden members in `policy_hidden`;
- the scope truth narrows or blocks `export_artifact` / `support_archive` for the
  active scope class (`narrowed_to_scope` for named workset / sparse slice,
  `blocked_by_portability` for the managed-provider-locked policy-limited view);
- every in-scope metadata row uses the canonical local-first default redaction
  posture (`not_required_metadata` / `included_default`), and every
  policy-limited row carries the policy redaction label
  (`policy_locked` / `omitted_policy_locked` / `policy_denied`) — never silently
  embedded.

The test also drives the two acceptance failures directly: an export that omits
the scope declaration fails validation, and an export that embeds a
policy-limited content row without a policy label fails validation, so neither a
missing scope declaration nor leaked policy-limited content can pass silently.

## Shared vocabularies

`manifest.json`'s `scope_class_vocabulary_map` pins the mapping between the
support scope-class tokens and the `aureline-workspace` `ScopeClass` vocabulary.
The export surface reuses the workspace vocabulary verbatim, so the map is the
identity over the shared classes and `support_only_scope_classes` is empty.
`workspace_scope_class_vocabulary` mirrors `ScopeClass::as_str()` and
`redaction_vocabulary` mirrors the policy-locked tokens from
`aureline-support` `bundle::vocabulary`. The Rust test reuses
`aureline_workspace::ScopeClass` and the `aureline_support` redaction enums
directly (`aureline-support` already depends on `aureline-workspace`) and proves
the map is a bijection over the shared classes;
`ci/check_beta_support_workset_scope.py` re-derives both vocabularies from crate
source and fails closed if the map, the mirrors, the policy-redaction labels, or
the required scope-class coverage drift, so the export and workspace surfaces
cannot fork the scope or redaction vocabulary without breaking the gate.
