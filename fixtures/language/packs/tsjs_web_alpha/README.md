# TS/JS Web Language-Pack Alpha Fixtures

These fixtures prove the TypeScript/JavaScript web language-pack artifact can be
loaded as one pack and projected into a runtime enablement snapshot without
manual per-file assembly.

Run:

```sh
cargo test -p aureline-language --test tsjs_web_pack_alpha
ruby -ryaml -e 'YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false)' artifacts/language_packs/tsjs_web_alpha.yaml
```
