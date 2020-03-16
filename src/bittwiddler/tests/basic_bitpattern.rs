use bittwiddler::*;

#[bitpattern]
enum MyEnum {
    #[bits("00")]
    /// docstring1
    Choice1,
    #[bits("01")]
    ChoiceTwo,
    #[bits("10")]
    /// docstring2
    /// docstring3
    Choice3,
    #[bits("11")]
    ChoiceFour,
}

#[test]
fn basic_bitpattern_encode() {
    let x = MyEnum::Choice1;
    assert_eq!(x.encode(), [false, false]);

    let x = MyEnum::ChoiceTwo;
    assert_eq!(x.encode(), [false, true]);

    let x = MyEnum::Choice3;
    assert_eq!(x.encode(), [true, false]);

    let x = MyEnum::ChoiceFour;
    assert_eq!(x.encode(), [true, true]);
}

#[test]
fn basic_bitpattern_docs() {
    let x = docs_as_ascii_table::<MyEnum>();
    assert_eq!(x, r#"01 |            |
---+------------+----------------------
00 | Choice1    | docstring1
01 | ChoiceTwo  | 
10 | Choice3    | docstring2 docstring3
11 | ChoiceFour | 
"#);
}
