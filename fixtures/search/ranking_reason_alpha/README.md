# Ranking-reason alpha fixtures

These fixtures prove that quick-open and symbol-search rows expose stable
result IDs plus structured ranking reasons that can be rendered as `Why this
result?` cards and included in support exports without scraping UI strings.

| Fixture | Coverage |
|---|---|
| `quick_open_hot_set_file_card.json` | Quick open surfaces a lexical hot-set file with stable lexical result identity, partial readiness, and a compact ranking signal. |
| `symbol_search_structural_fallback_card.json` | Symbol search uses structural fallback while graph proof is warming and exports result ID, fallback reason, and dominant signals. |
