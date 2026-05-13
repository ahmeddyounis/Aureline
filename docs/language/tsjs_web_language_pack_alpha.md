# TS/JS Web Language-Pack Alpha

This page records the first bounded language-pack artifact for the TypeScript,
JavaScript, and web-document launch wedge.

## Canonical Artifact

The pack artifact is `artifacts/language_packs/tsjs_web_alpha.yaml`. It names:

- Tree-sitter grammar rows for TypeScript, TSX, JavaScript, JSX, HTML, CSS,
  JSON, and YAML;
- LSP and syntax fallback provider routes;
- diagnostics defaults for language-service, web-document, config, quality, and
  test evidence;
- formatter, linter, and test-adapter hooks;
- file icon refs for tabs, file trees, and quick open;
- docs pack refs; and
- known gaps that keep the alpha claim bounded.

## Runtime Consumer

The first consumer lives in `crates/aureline-language/src/packs/tsjs_web.rs`.
`TsJsWebLanguagePack` reads the manifest shape and emits a
`tsjs_web_language_pack_enablement_snapshot` that resolves the declared
grammars through the shared registry, lists provider and tool-hook refs, exposes
activation globs, and records whether the protected flows can enable without
manual per-file assembly.

The consumer reuses the existing runtime contracts instead of defining private
pack truth:

- Tree-sitter grammar registry: `crates/aureline-language/src/tree_sitter/`
- LSP routing and provider health vocabulary:
  `crates/aureline-language/src/lsp_router/`
- TS/JS assistance and rename-preview records:
  `crates/aureline-language/src/tsjs/`
- TS/JS quality-tool records:
  `crates/aureline-language/src/tsjs/quality/`
- Suspicious-content and safe-preview trust classes:
  `crates/aureline-content-safety/`

## Claim Boundary

The pack is intentionally alpha-limited. It does not claim React, Next, route
graph, visual editing, source-map debug, managed workspace, certified archetype,
or stable broad-refactor depth. HTML, CSS, JSON, and YAML are web-adjacent
language rows with syntax, diagnostics labels, formatting hooks, icons, docs,
and content-integrity cues; they are not framework-expert rows.

Mutating actions remain preview-first. Generated, external, protected, stale, or
partial provider state must stay labeled in the record that reaches editor,
review, CLI, and support surfaces.

## Protected Proof Path

The protected fixture is
`fixtures/language/packs/tsjs_web_alpha/pack_enablement_cases.yaml`. It verifies
that the manifest enables eight languages, resolves eight grammar rows, exposes
all provider routes and quality hooks, preserves safe-preview trust posture, and
keeps known gaps visible.

Run:

```sh
cargo test -p aureline-language --test tsjs_web_pack_alpha
ruby -ryaml -e 'YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false)' artifacts/language_packs/tsjs_web_alpha.yaml
```
