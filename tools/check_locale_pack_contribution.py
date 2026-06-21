#!/usr/bin/env python3
"""Gate for the locale-pack contribution tooling and governance artifacts.

This is the release-gated proof that the contribution tooling actually holds the
line. It runs the standalone validator over three things and fails on any
surprise:

* the terminology governance glossary validates against its own invariants;
* every shipped authoring template under ``templates/locale-packs/`` validates
  with zero errors (so contributors copy a known-good starting point); and
* every rejected fixture under
  ``fixtures/i18n/locale-pack-contribution/rejected/`` is rejected, and raises
  the finding codes its ``expected.json`` declares (so the guardrails provably
  fire on incompatible packs and forbidden label replacement).
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
PKG_PARENT = SCRIPT_DIR / "i18n"
if str(PKG_PARENT) not in sys.path:
    sys.path.insert(0, str(PKG_PARENT))

from validate_locale_pack.validator import (  # noqa: E402
    DEFAULT_GLOSSARY_REL,
    DEFAULT_REGISTRY_REL,
    load_message_registry,
    load_terminology_glossary,
    validate_locale_pack,
    validate_terminology_glossary,
)

TEMPLATE_DIRS = ("first-party", "community", "extension-owned")
TEMPLATES_REL = "templates/locale-packs"
REJECTED_REL = "fixtures/i18n/locale-pack-contribution/rejected"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--report", default=None, help="Write a JSON capture to this repo-relative path.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    glossary = load_terminology_glossary(repo_root / DEFAULT_GLOSSARY_REL)
    registry = load_message_registry(repo_root / DEFAULT_REGISTRY_REL)

    capture: dict = {"glossary": {}, "templates": [], "rejected": []}
    ok = True

    # 1. The governance glossary validates against its own invariants.
    glossary_findings = validate_terminology_glossary(glossary)
    glossary_errors = [f for f in glossary_findings if f.severity == "error"]
    capture["glossary"] = {
        "errors": len(glossary_errors),
        "codes": sorted({f.code for f in glossary_errors}),
    }
    if glossary_errors:
        ok = False
        print(f"FAIL terminology glossary: {len(glossary_errors)} error(s)")
        for f in glossary_errors:
            print(f"    {f.code}: {f.location}: {f.message}")
    else:
        print("ok   terminology glossary")

    # 2. Every shipped template validates clean.
    for name in TEMPLATE_DIRS:
        pack_dir = repo_root / TEMPLATES_REL / name
        findings = validate_locale_pack(pack_dir, registry=registry, glossary=glossary)
        errors = [f for f in findings if f.severity == "error"]
        warnings = [f for f in findings if f.severity == "warning"]
        capture["templates"].append(
            {"name": name, "errors": len(errors), "warnings": len(warnings),
             "codes": sorted({f.code for f in errors})}
        )
        if errors:
            ok = False
            print(f"FAIL template {name}: {len(errors)} error(s)")
            for f in errors:
                print(f"    {f.code}: {f.location}: {f.message}")
        else:
            print(f"ok   template {name} ({len(warnings)} warning(s))")

    # 3. Every rejected fixture is rejected with the expected finding codes.
    rejected_root = repo_root / REJECTED_REL
    for pack_dir in sorted(p for p in rejected_root.iterdir() if p.is_dir()):
        name = pack_dir.name
        expected_path = pack_dir / "expected.json"
        expected_codes = []
        if expected_path.exists():
            expected_codes = json.loads(expected_path.read_text(encoding="utf-8")).get("expected_codes", [])
        findings = validate_locale_pack(pack_dir, registry=registry, glossary=glossary)
        error_codes = {f.code for f in findings if f.severity == "error"}
        missing = [c for c in expected_codes if c not in error_codes]
        case_ok = bool(error_codes) and not missing
        capture["rejected"].append(
            {"name": name, "expected_codes": expected_codes,
             "observed_codes": sorted(error_codes), "ok": case_ok}
        )
        if case_ok:
            print(f"ok   rejected {name} (raised {len(error_codes)} error code(s))")
        else:
            ok = False
            if not error_codes:
                print(f"FAIL rejected {name}: expected rejection but no error was raised")
            else:
                print(f"FAIL rejected {name}: missing expected code(s): {', '.join(missing)}")

    capture["passed"] = ok
    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    print(f"\n{'PASS' if ok else 'FAIL'}: locale-pack contribution gate")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
