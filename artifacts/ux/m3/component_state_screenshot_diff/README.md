# Component-State Screenshot Diff Packet

`packet.json` is the release-review projection for component-state screenshot
diff coverage on launch-critical surfaces. It carries refs to baseline,
comparison, and diff artifacts without embedding image bytes.

The checked packet must:

- cover shell chrome, Start Center, command palette, search, dialogs,
  trust prompts, notifications, Help/About, settings, and activity rows;
- exercise dark, light, high-contrast, compact, standard, comfortable,
  standard-motion, reduced-motion, low-motion, power-saver, and critical
  hot-path rows across the matrix;
- keep hover-only critical actions, missing focus visibility, spinner-only
  blocked states, and color-only state meaning absent.

Regenerate with:

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- screenshot-diff > artifacts/ux/m3/component_state_screenshot_diff/packet.json
```
