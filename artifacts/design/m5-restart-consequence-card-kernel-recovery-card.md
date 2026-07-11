# Restart consequence cards and kernel recovery cards

- Packet: `m5-restart-consequence-card-kernel-recovery-card-controls:stable:0001`
- Surface: `M5 restart consequence cards and kernel recovery cards: preserved-versus-lost state, debugger/session impact, reconnect/restart-clean/choose-another-kernel recovery, and no-hidden-rerun truth across claimed notebook restore and failure flows`
- Restart consequence cards: 6 (2 lose live state)
- Kernel recovery cards: 6 (5 not recovered)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Restart consequence cards

- **Restart kernel (live state lost)** — action `restart_kernel`, consequence `state_lost` → `live_state_lost_impact`, scope `ends_session`, deep link `kernel_manager`
- **Restart and run all (variables cleared)** — action `restart_and_run_all`, consequence `variables_cleared` → `variables_cleared_impact`, scope `ends_session`, deep link `notebook_location`
- **Interrupt kernel (state preserved)** — action `interrupt_kernel`, consequence `state_preserved` → `state_preserved_impact`, scope `keeps_session`, deep link `kernel_manager`
- **Shut down kernel (outputs retained)** — action `shutdown_kernel`, consequence `outputs_retained` → `outputs_retained_impact`, scope `ends_session`, deep link `support_bundle`
- **Reconnect kernel (outputs cleared)** — action `reconnect_kernel`, consequence `outputs_cleared` → `outputs_cleared_impact`, scope `keeps_session`, deep link `kernel_manager`
- **Clear outputs (no session consequence)** — action `clear_outputs`, consequence `no_consequence` → `no_restart_impact`, scope `outputs_only`, deep link `docs_anchor`

## Kernel recovery cards

- **Kernel recovered by reconnect** — action `reconnect`, state `recovered` → `recovered_clean` / `continues_session`, deep link `kernel_manager`
- **Reattach session (reconnect available)** — action `reattach_session`, state `reconnect_available` → `reconnect_offered` / `continues_session`, deep link `kernel_manager`
- **Restart clean (restart required)** — action `restart_clean`, state `restart_required` → `restart_needed` / `clean_session`, deep link `kernel_manager`
- **Choose another kernel (no kernel available)** — action `choose_another_kernel`, state `no_kernel_available` → `no_kernel_available` / `clean_session`, deep link `kernel_manager`
- **Start local fallback (recovery blocked)** — action `start_local_fallback`, state `recovery_blocked` → `recovery_blocked` / `clean_session`, deep link `support_bundle`
- **Wait for managed kernel (recoverable)** — action `wait_for_managed`, state `recoverable` → `recoverable_now` / `awaits_managed`, deep link `docs_anchor`
