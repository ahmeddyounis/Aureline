# Certify API, database, and browser-runtime workflows with mutation, redaction, and scale drills

## Scope

This artifact is the canonical M5 certification qualification packet for API, database, and browser-runtime workflows with mutation, redaction, and scale drills.

## Checked-in packet

- JSON: `artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json`
- Schema: `schemas/data/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.schema.json`

## Surface claims

| Surface | Claim | Displayed |
|---|---|---|
| API workflow certification | stable | stable |
| Database workflow certification | stable | stable |
| Browser-runtime certification | stable | stable |
| Mutation drill | stable | stable |
| Redaction drill | stable | stable |
| Scale drill | stable | stable |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof narrows to `preview`.

## Integration status

All 10 upstream B3 packets are referenced and verified.
