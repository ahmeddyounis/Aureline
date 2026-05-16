# Extension mirror and manual-import baseline fixtures

These fixtures exercise the beta mirror/manual import baseline in
`crates/aureline-extensions/src/mirror_import/`.

The fixture set proves that primary catalog, approved mirror, and manual
artifact imports preserve artifact identity, publisher continuity,
permission, compatibility, lifecycle, and trust metadata before install
review consumes the row.

Run:

```bash
cargo test -p aureline-extensions mirror_import::tests
```

