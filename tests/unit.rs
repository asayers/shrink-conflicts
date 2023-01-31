macro_rules! assert_inout {
    ($input: literal, $output: literal) => {
        assert_eq!(shrink_conflicts::run($input.into()).unwrap(), $output);
    };
}
macro_rules! assert_unchanged {
    ($input: literal) => {
        assert_eq!(shrink_conflicts::run($input.into()).unwrap(), $input);
    };
}

#[test]
fn test_empty() {
    assert_unchanged!("");
}
#[test]
fn test_trailing_newline() {
    assert_unchanged!("Hello world!\n");
    assert_unchanged!("Foo\nbar\nqux\n");
}
#[test]
fn test_no_trailing_newline() {
    assert_unchanged!("Hello world!");
    assert_unchanged!("Foo\nbar\nqux");
}
#[test]
fn test_nothing_to_do() {
    assert_unchanged!("One\n<<<<<<<\nTwo\n|||||||\nThree\n=======\nFour\n>>>>>>>\nFive");
}
#[test]
fn test_empty_conflict() {
    assert_inout!("One\n<<<<<<<\n|||||||\n=======\n>>>>>>>\nFive", "One\nFive");
}
#[test]
fn test_same_on_left() {
    assert_inout!(
        "One\n<<<<<<<\nTwo\n|||||||\nTwo\n=======\nThree\n>>>>>>>\nFive",
        "One\nThree\nFive"
    );
}
#[test]
fn test_same_on_right() {
    assert_inout!(
        "One\n<<<<<<<\nTwo\n|||||||\nThree\n=======\nThree\n>>>>>>>\nFive",
        "One\nTwo\nFive"
    );
}
#[test]
fn test_same_on_sides() {
    assert_unchanged!("One\n<<<<<<<\nTwo\n|||||||\nThree\n=======\nTwo\n>>>>>>>\nFive");
}
#[test]
fn test_common_prefix() {
    assert_inout!(
        "One\n<<<<<<<\nTwo\nThree\n|||||||\nTwo\nFour\n=======\nTwo\nFive\n>>>>>>>\nSix",
        "One\nTwo\n<<<<<<<\nThree\n|||||||\nFour\n=======\nFive\n>>>>>>>\nSix"
    );
}
#[test]
fn test_common_suffix() {
    assert_inout!(
        "One\n<<<<<<<\nTwo\nFive\n|||||||\nThree\nFive\n=======\nFour\nFive\n>>>>>>>\nSix",
        "One\n<<<<<<<\nTwo\n|||||||\nThree\n=======\nFour\n>>>>>>>\nFive\nSix"
    );
}
#[test]
fn test_common_prefix_same_on_left() {
    assert_inout!(
        "One\n<<<<<<<\nTwo\nThree\n|||||||\nTwo\nThree\n=======\nTwo\n>>>>>>>\nFive",
        "One\nTwo\nFive"
    );
}
