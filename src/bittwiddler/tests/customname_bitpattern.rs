use bittwiddler::*;

#[bitpattern(default = MyEnum1::Choice1, bitnames="ABCD")]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum1 {
    #[bits("0000")]
    /// docstring1
    Choice1,
    #[bits("0001")]
    ChoiceTwo,
}

#[test]
fn customname_nospaces_bitpattern_docs() {
    let x = MyEnum1::docs_as_ascii_table();
    assert_eq!(x, r#"ABCD |           |
-----+-----------+-----------
0000 | Choice1   | docstring1
0001 | ChoiceTwo | 
"#);
}

#[bitpattern(default = MyEnum2::Choice1, bitnames="A B   C 	D")]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum2 {
    #[bits("0000")]
    /// docstring1
    Choice1,
    #[bits("0001")]
    ChoiceTwo,
}

#[test]
fn customname_spaces_bitpattern_docs() {
    let x = MyEnum2::docs_as_ascii_table();
    assert_eq!(x, r#"ABCD |           |
-----+-----------+-----------
0000 | Choice1   | docstring1
0001 | ChoiceTwo | 
"#);
}

