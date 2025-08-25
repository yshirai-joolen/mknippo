use mknippo::{construct_mention_name, construct_mention_raw};

#[test]
fn construct_mention_name_basic() {
    assert_eq!(construct_mention_name("abc").unwrap(), "@abc");
    assert_eq!(construct_mention_name("試験山").unwrap(), "@試験山");
    assert_eq!(construct_mention_name("123").unwrap(), "@123");
    assert_eq!(construct_mention_name("").unwrap(), "@");
}

#[test]
fn construct_mention_raw_single() {
    assert_eq!(construct_mention_raw(&["abc"]).unwrap(), "@abc");
    assert_eq!(construct_mention_raw(&[""]).unwrap(), "@");
}

#[test]
fn construct_mention_raw_multiple() {
    assert_eq!(construct_mention_raw(&["abc", "cbd"]).unwrap(), "@abc @cbd");
    assert_eq!(construct_mention_raw(&["試験山", "abc", "cbd"]).unwrap(), "@試験山 @abc @cbd");
    assert_eq!(construct_mention_raw(&["a", "b", "c", "d"]).unwrap(), "@a @b @c @d");
}

#[test]
fn construct_mention_raw_empty() {
    assert_eq!(construct_mention_raw(&[]).unwrap(), "");
}

#[test]
fn construct_mention_name_unicode() {
    assert_eq!(construct_mention_name("山田太郎").unwrap(), "@山田太郎");
    assert_eq!(construct_mention_name("😀").unwrap(), "@😀");
}