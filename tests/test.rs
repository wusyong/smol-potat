#[smol_potat::test]
async fn test() {
    assert_eq!(2 * 2, 4);
}

#[smol_potat::test]
async fn test_returns_ok() -> std::result::Result<(), Box<dyn std::error::Error>> {
    assert_eq!(2 * 2, 4);
    Ok(())
}
