# AI pack promotion validator

Validate the stable AI-pack rollout packet, local-model publication manifest, and
support projection:

```sh
python3 tools/release/ai-pack-promotion/validate_ai_pack_promotion.py
```

The validator checks that stable route rows expose provider/model identity,
prompt-pack versions, tool-schema ranges, local-model provenance when
applicable, independent rollback refs, downgrade receipts, and mirror/offline
publication metadata without vendor-network dependence.
