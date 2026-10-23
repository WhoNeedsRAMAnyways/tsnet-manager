
use tsnet_gui::shell;


#[test]
fn parse_two_profiles() {
    let text = "ID    Tailnet               Account\n\
                82ac  testNumStr@gmail.com  testNumStr@gmail.com\n\
                9982  testPureNum@gmail.com  testPureNum@gmail.com*\n";
    
    let profiles = shell::parse_profiles(text);

    assert_eq!(profiles.len(), 2);
    assert!(!profiles[0].active);
    assert!(profiles[1].active);
    assert_eq!(profiles[1].account, "testPureNum@gmail.com");
}

