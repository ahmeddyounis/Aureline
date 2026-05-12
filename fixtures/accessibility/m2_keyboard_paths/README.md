# External Alpha Keyboard Path Fixtures

These fixtures protect the launch-critical keyboard audit consumed by
the shell help surface. They keep command ids, resolver source
attribution, focus return, preset coverage, conflict reporting, and
actionable remaining gaps tied to one fixture lane.

Run:

```sh
cargo test -p aureline-shell --test keyboard_gap_audit
```
