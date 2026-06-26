# Action-Label and Count/Scope-Language Parity Fixtures

These fixtures are valid, export-safe action-label/scope catalog packets. They are
minted from the same seed builder as the canonical support export by
`aureline_shell_m5_action_label_scope`, and each one passes every validation
invariant. They exercise the two parity properties the catalog must keep green:
locale neutrality and offline-mirror identity.

## localized_overlay.json

A localized overlay of the canonical catalog. Every verb label, scope phrase, object
noun, and reference template is pseudo-localized (human text runs are wrapped in
`⟦ ⟧` locale markers), while every label id, verb/scope/object id, count-variable
name, and `{verb}` / `{count:...}` / `{scope:...}` / `{object_one}` /
`{object_many}` placeholder stays byte-for-byte identical. The `reference_locale`
flips from `en` to `qps-ploc`. Demonstrates that the resolved human prose localizes
freely while the machine-facing identity — the part the product, exports, and screen
readers key off — never moves, so a translation can never fork the meaning of a
scope or hide the object class an action mutates.

## offline_mirror.json

An offline-mirror variant of the canonical catalog. The scopes, verbs, objects,
labels, and disclosures are identical; only the catalog id and the release/mirror
refs differ. Demonstrates that the catalog survives an offline mirror with its
verb-first labels and controlled scope phrases intact.
