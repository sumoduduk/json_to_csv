pub fn get_category_and_type(name: &str) -> (&str, &str, &str) {
    let name_split: Vec<&str> = name.split('-').collect();

    (name, name_split[0], name_split[1])
}

#[test]
fn test_hash() {
    let name = "bambang-sri-jatmoko";

    let tup = get_category_and_type(name);
    dbg!(&tup);

    assert_eq!(("bambang-sri-jatmoko", "bambang", "sri",), tup);
}
