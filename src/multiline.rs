#[test]
fn multiline_interpolation_test() {
    let owner = "monadplus";
    let name = "Arnau Abella";
    let query = format!(
        r#"
    query {{
      repository(owner: "{o}", name: "{n}")
    }}"#,
        o = owner,
        n = name
    );
    println!("{query}");
}
