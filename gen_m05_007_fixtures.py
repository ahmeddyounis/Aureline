#!/usr/bin/env python3
"""Generate fixture files for M05-007."""

import json
import copy

with open("artifacts/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.json") as f:
    base = json.load(f)

# Fixture 1: duplicate entry id
dup = copy.deepcopy(base)
dup["rows"][0]["entry_id"] = dup["rows"][1]["entry_id"]
dup["summary"] = None  # invalidate summary so only DuplicateEntryId fires
with open("fixtures/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules/duplicate_entry_id.json", "w") as f:
    json.dump(dup, f, indent=2)

# Fixture 2: missing rollback path ref on a non-missing state
missing_ref = copy.deepcopy(base)
for row in missing_ref["rows"]:
    if row["rollback_path_state"] != "missing":
        row["rollback_path_ref"] = ""
        break
missing_ref["summary"] = None
with open("fixtures/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules/missing_rollback_path_ref.json", "w") as f:
    json.dump(missing_ref, f, indent=2)

# cases.json mapping
cases = {
    "cases": [
        {
            "file": "missing_rollback_path_ref.json",
            "expected_check_id": "EmptyField"
        },
        {
            "file": "duplicate_entry_id.json",
            "expected_check_id": "DuplicateEntryId"
        }
    ]
}
with open("fixtures/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules/cases.json", "w") as f:
    json.dump(cases, f, indent=2)

print("Fixtures generated.")
