//! Headless emitter for the M5 presence-avatar-stack / role-or-follow-badge primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-presence-avatar-stack-proof/`, its matrix CSV, the Markdown
//! report `artifacts/components/m5-presence-avatar-stack-primitive.md`, and the narrowed
//! fixtures under `fixtures/ui/m5-presence-avatar-stack-primitive/`. Every M5
//! collaboration surface (the collaboration strip, the shared terminal header, the
//! shared debug pane, the review / session header, the presenter HUD, the follow-mode
//! banner, the session roster panel, the activity-center presence entry, and the shared
//! preview header) reads this primitive so who is present, who is presenting, who is
//! followed, and who held control stay consistent, and so the support export
//! reconstructs presence from one shared model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presence_avatar_stack_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presence_avatar_stack_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presence_avatar_stack_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presence_avatar_stack_primitive -- fixture-shared-debug-pane-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presence_avatar_stack_primitive -- fixture-review-session-header-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presence_avatar_stack_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_presence_avatar_stack_and_role_or_follow_badge_recording_state_and_local_fallback_continuity_primitive::{
    seeded_m5_presence_avatar_stack_primitive_packet,
    seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed,
    seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed,
    M5PresenceAvatarStackPrimitivePacket,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("support-export") | None => {
            let packet = seeded_m5_presence_avatar_stack_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_presence_avatar_stack_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_presence_avatar_stack_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-shared-debug-pane-beta-narrowed") => {
            let packet =
                seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-session-header-preview-narrowed") => {
            let packet =
                seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_presence_avatar_stack_primitive_packet(),
                seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed(),
                seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(
    packet: &M5PresenceAvatarStackPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
