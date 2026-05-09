# Shell chrome appearance goldens

This folder contains the **shell chrome** appearance golden harness seed:

- screenshot baselines under `baselines/`
- token-adoption baseline at `token_adoption_baseline.json`

## Regenerating screenshots (developer local)

These captures require a working desktop environment because they run the native
shell and exit after the first rendered frame.

Run:

```bash
python3 tools/ci/capture_appearance_goldens.py --out-dir tests/golden/appearance/shell_chrome/baselines
```

## Comparing against the repo baselines

```bash
python3 tools/ci/compare_appearance_goldens.py --baseline-dir tests/golden/appearance/shell_chrome/baselines
```

## Updating the token-adoption baseline (expected require_* calls)

```bash
python3 tools/ci/check_token_adoption.py --update --baseline tests/golden/appearance/shell_chrome/token_adoption_baseline.json
```

