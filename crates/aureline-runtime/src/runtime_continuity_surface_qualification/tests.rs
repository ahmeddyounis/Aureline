use super::*;

#[test]
fn seeded_packet_is_stable_for_desktop_profiles_and_narrow_for_browser_handoff() {
    let packet = seeded_runtime_continuity_surface_qualification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let local = packet
        .profile_rows
        .iter()
        .find(|row| row.profile == RuntimeContinuityProfile::LocalOnly)
        .expect("local row");
    assert_eq!(local.displayed_label, RuntimeContinuityLabel::Stable);
    assert!(local.narrow_reasons.is_empty());

    let browser = packet
        .profile_rows
        .iter()
        .find(|row| row.profile == RuntimeContinuityProfile::BrowserHandoff)
        .expect("browser row");
    assert_eq!(browser.displayed_label, RuntimeContinuityLabel::Preview);
    assert_eq!(
        browser.narrow_reasons,
        vec![RuntimeContinuityNarrowReason::BrowserHandoffContinuityUnqualified]
    );
}
