# M5 Mutation-Path Fix-Flow Fixtures

These fixtures are valid, export-safe mutation-path fix-flow packets that exercise
preview-first bidi/invisible/confusable fix flows, the no-silent-byte-rewrite
guard, and scope-aware auditable suppressions across the new M5 save, format,
organize-imports, and AI-apply paths. Each one keeps every mutation path present
exactly once, keeps every path blocking silent suspicious-byte rewrites, keeps
every fix flow requiring a preview before bytes change, and preserves the guard in
product, export, and support handoff without leaking raw artifact bytes.

## all_clean_no_findings.json

Every mutation path touches plain ASCII content, so the shared suspicious-content
detector flags nothing: `paths_with_findings_count` is `0`, no path offers a fix
kind, and no path records a suppression. Demonstrates that the silent-rewrite
guard and the preview-first fix flow hold — and validate — even when there is
nothing to fix, so a fully clean save/format/organize-imports/AI-apply never
silently rewrites bytes. Regenerate with `m5_mutation_path_fix_flow --clean`.
