pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

#[test]
fn test_defang_i_paddr() {
    assert_eq!(
        defang_i_paddr("1.1.1.1".into()),
        "1[.]1[.]1[.]1".to_string()
    );
    assert_eq!(
        defang_i_paddr("255.100.50.0".into()),
        "255[.]100[.]50[.]0".to_string()
    );
}
