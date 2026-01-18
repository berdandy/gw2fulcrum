use gw2fulcrum::BalanceUpdate;

#[test]
fn parse_update_notes() {
    let update_str = include_str!("2025-06.txt");
    let update = BalanceUpdate::parse_notes(&update_str);

    assert_eq!(update.changes.len(), 192);
}
