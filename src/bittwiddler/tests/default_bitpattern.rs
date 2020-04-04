use bittwiddler::*;

#[bitpattern(default = Self::Choice3)]
#[derive(Debug, PartialEq, Eq)]
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
}

#[test]
fn default_bitpattern_encode() {
    let x = MyEnum::Choice1;
    assert_eq!(x.encode(()), [false, false]);

    let x = MyEnum::ChoiceTwo;
    assert_eq!(x.encode(()), [false, true]);

    let x = MyEnum::Choice3;
    assert_eq!(x.encode(()), [true, false]);
}

#[test]
fn default_bitpattern_decode() {
    let x = [false, false];
    assert_eq!(MyEnum::decode(&x, ()).unwrap(), MyEnum::Choice1);

    let x = [false, true];
    assert_eq!(MyEnum::decode(&x, ()).unwrap(), MyEnum::ChoiceTwo);

    let x = [true, false];
    assert_eq!(MyEnum::decode(&x, ()).unwrap(), MyEnum::Choice3);

    let x = [true, true];
    assert_eq!(MyEnum::decode(&x, ()).unwrap(), MyEnum::Choice3);
}

#[test]
fn default_bitpattern_docs() {
    let x = MyEnum::docs_as_ascii_table();
    assert_eq!(x, r#"01 |           |
---+-----------+----------------------
00 | Choice1   | docstring1
01 | ChoiceTwo | 
10 | Choice3   | docstring2 docstring3
"#);
}
