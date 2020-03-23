use bittwiddler::*;

enum Var1 {}
enum Var2 {}

#[bitpattern(variant = Var1)]
#[bitpattern(variant = Var2)]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum {
    #[bits("00", variant = Var1)]
    #[bits("11", variant = Var2)]
    /// docstring1
    Choice1,
    #[bits("01", variant = Var1)]
    #[bits("01", variant = Var2)]
    ChoiceTwo,
    #[bits("10", variant = Var1)]
    #[bits("10", variant = Var2)]
    /// docstring2
    /// docstring3
    Choice3,
    #[bits("11", variant = Var1)]
    #[bits("00", variant = Var2)]
    ChoiceFour,
}

#[test]
fn two_variants_var1_bitpattern_encode() {
    let x = MyEnum::Choice1;
    assert_eq!(BitPattern::<Var1>::encode(&x), [false, false]);

    let x = MyEnum::ChoiceTwo;
    assert_eq!(BitPattern::<Var1>::encode(&x), [false, true]);

    let x = MyEnum::Choice3;
    assert_eq!(BitPattern::<Var1>::encode(&x), [true, false]);

    let x = MyEnum::ChoiceFour;
    assert_eq!(BitPattern::<Var1>::encode(&x), [true, true]);
}

#[test]
fn two_variants_var1_bitpattern_decode() {
    let x = [false, false];
    assert_eq!(<MyEnum as BitPattern<Var1>>::decode(&x).unwrap(), MyEnum::Choice1);

    let x = [false, true];
    assert_eq!(<MyEnum as BitPattern<Var1>>::decode(&x).unwrap(), MyEnum::ChoiceTwo);

    let x = [true, false];
    assert_eq!(<MyEnum as BitPattern<Var1>>::decode(&x).unwrap(), MyEnum::Choice3);

    let x = [true, true];
    assert_eq!(<MyEnum as BitPattern<Var1>>::decode(&x).unwrap(), MyEnum::ChoiceFour);
}

#[test]
fn two_variants_var1_bitpattern_docs() {
    let x = <MyEnum as BitPattern<Var1>>::docs_as_ascii_table();
    assert_eq!(x, r#"01 |            |
---+------------+----------------------
00 | Choice1    | docstring1
01 | ChoiceTwo  | 
10 | Choice3    | docstring2 docstring3
11 | ChoiceFour | 
"#);
}

#[test]
fn two_variants_var2_bitpattern_encode() {
    let x = MyEnum::Choice1;
    assert_eq!(BitPattern::<Var2>::encode(&x), [true, true]);

    let x = MyEnum::ChoiceTwo;
    assert_eq!(BitPattern::<Var2>::encode(&x), [false, true]);

    let x = MyEnum::Choice3;
    assert_eq!(BitPattern::<Var2>::encode(&x), [true, false]);

    let x = MyEnum::ChoiceFour;
    assert_eq!(BitPattern::<Var2>::encode(&x), [false, false]);
}

#[test]
fn two_variants_var2_bitpattern_decode() {
    let x = [true, true];
    assert_eq!(<MyEnum as BitPattern<Var2>>::decode(&x).unwrap(), MyEnum::Choice1);

    let x = [false, true];
    assert_eq!(<MyEnum as BitPattern<Var2>>::decode(&x).unwrap(), MyEnum::ChoiceTwo);

    let x = [true, false];
    assert_eq!(<MyEnum as BitPattern<Var2>>::decode(&x).unwrap(), MyEnum::Choice3);

    let x = [false, false];
    assert_eq!(<MyEnum as BitPattern<Var2>>::decode(&x).unwrap(), MyEnum::ChoiceFour);
}

#[test]
fn two_variants_var2_bitpattern_docs() {
    let x = <MyEnum as BitPattern<Var2>>::docs_as_ascii_table();
    assert_eq!(x, r#"01 |            |
---+------------+----------------------
11 | Choice1    | docstring1
01 | ChoiceTwo  | 
10 | Choice3    | docstring2 docstring3
00 | ChoiceFour | 
"#);
}

