# Docs Search, Symbol Reference Cards, and Code-Anchor Deep Links

- Packet: `packet:m5:docs_search_link:http_client_publish`
- Search: docs search: http client publish example
- Promotion: `stable` (0 findings)
- Rows: 3 | Symbol cards: 2 | Deep links: 2 | Disclosures: 1

## Results

1. `result:project_docs:http_client_get` [symbol_reference_result] — project_docs / exact_build_match / authoritative_live
   - Reason: exact symbol match in project docs at the active build with strong overlap
2. `result:mirrored:http_publish_guide` [docs_page_result] — mirrored_official_docs / compatible_minor_drift / warm_cached
   - Reason: pinned, signed mirror of the official publish guide within the compat window
3. `result:curated:http_patterns` [curated_pack_result] — curated_knowledge_pack / unknown_target_build / degraded_cached
   - Reason: curated knowledge pack match; target build unknown so version is disclosed

## Symbol reference cards

- `symref:project:symbol:http-client-get` [symbol/exact_symbol_match]: http::Client::get (project_authoritative_only)
- `symref:project:guide:http-overview` [symbol/package_level_guide_fallback]: http::Client::publish (project_outranks_vendor_default)

## Code-anchor deep links

- `deeplink:symbol:http-client-get` [symbol_ref]: Open http::Client::get (anchor preserved: true)
- `deeplink:file:http-publish-example` [file_line_range]: Open publish example (anchor preserved: true)

## Resolution disclosures

- `symref:project:guide:http-overview` [package_guide_fallback/advisory]: no dedicated symbol page exists yet; the package-level guide is shown
