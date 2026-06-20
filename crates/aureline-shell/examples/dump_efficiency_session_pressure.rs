//! Conformance dump for active-session low-power continuity.
//!
//! Emits, for every representative posture, the typed inputs together with the
//! session-continuity posture the efficiency state produces for active tasks,
//! debug sessions, remote attaches, notebook kernels, traces, and long-running
//! captures. The output backs the checked-in fixtures under
//! `fixtures/efficiency/session-pressure/` so the active-session behavior provably
//! derives from the same canonical efficiency-state objects as the status,
//! diagnostics, support, and disclosure surfaces.

use aureline_shell::efficiency::session_pressure::seeded_session_pressure_cases;

fn main() {
    let cases = seeded_session_pressure_cases();
    println!(
        "{}",
        serde_json::to_string_pretty(&cases).expect("session-pressure cases serialize")
    );
}
