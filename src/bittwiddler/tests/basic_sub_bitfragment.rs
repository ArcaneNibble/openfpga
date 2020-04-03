use bittwiddler::*;

#[bitpattern]
#[bitfragment(dimensions = 1)]
#[pat_pict("0 . 1 .")]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum1 {
    #[bits("00")]
    Choice1,
    #[bits("01")]
    Choice2,
    #[bits("10")]
    Choice3,
    #[bits("11")]
    Choice4,
}

#[bitpattern]
#[bitfragment(dimensions = 1)]
#[pat_pict(". 0 . 1")]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum2 {
    #[bits("00")]
    Choice1,
    #[bits("01")]
    Choice2,
    #[bits("10")]
    Choice3,
    #[bits("11")]
    Choice4,
}

#[bitfragment(dimensions = 1)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct1 {
    field1: MyEnum1,
    field2: MyEnum2,
}

#[test]
fn basic_sub_bitfragment_encode() {
    let mut out = [false; 4];

    let x = MyStruct1 {
        field1: MyEnum1::Choice2,
        field2: MyEnum2::Choice3,
    };
    x.encode(&mut out[..], [0], [false], ());
    assert_eq!(out, [false, true, true, false]);

    let x = MyStruct1 {
        field1: MyEnum1::Choice4,
        field2: MyEnum2::Choice1,
    };
    x.encode(&mut out[..], [0], [false], ());
    assert_eq!(out, [true, false, true, false]);
}

#[test]
fn basic_sub_bitfragment_decode() {
    let x = [true, false, false, true];
    let out = MyStruct1::decode(&x[..], [0], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field1: MyEnum1::Choice3,
        field2: MyEnum2::Choice2,
    });

    let x = [false, true, false, true];
    let out = MyStruct1::decode(&x[..], [0], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field1: MyEnum1::Choice1,
        field2: MyEnum2::Choice4,
    });
}
