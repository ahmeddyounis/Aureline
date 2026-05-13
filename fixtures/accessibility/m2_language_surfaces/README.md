# Language Surface Accessibility Fixtures

These fixtures protect the launch-language accessibility review for
diagnostics, completion assistance, and refactor preview surfaces. They cite
the existing language, editor-assist, safe-preview, keyboard, screen-reader,
and reduced-motion contracts instead of forking those source models.

Run:

```sh
cargo test -p aureline-shell --test language_surface_accessibility
```
