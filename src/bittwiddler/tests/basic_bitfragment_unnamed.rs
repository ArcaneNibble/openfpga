use bittwiddler::*;

#[bitpattern]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum {
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
struct MyStruct1 (
    #[pat_bits("0" = 1, "1" = 2)]
    MyEnum,
    #[pat_bits("0" = 0)]
    bool,
);

#[test]
fn basic_bitfragment_unnamed_encode() {
    let mut out = [false; 3];

    let x = MyStruct1(
        MyEnum::Choice2,
        false,
    );
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [false, false, true]);

    let x = MyStruct1(
        MyEnum::Choice3,
        true,
    );
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [true, true, false]);

    // offset
    let mut out = [true; 5];

    let x = MyStruct1(
        MyEnum::Choice2,
        false,
    );
    x.encode(&mut out[..], [1], [false]);
    assert_eq!(out, [true, false, false, true, true]);

    let x = MyStruct1(
        MyEnum::Choice3,
        true,
    );
    x.encode(&mut out[..], [1], [false]);
    assert_eq!(out, [true, true, true, false, true]);

    // mirroring
    let mut out = [false; 3];
    let x = MyStruct1(
        MyEnum::Choice2,
        false,
    );
    x.encode(&mut out[..], [2], [true]);
    assert_eq!(out, [true, false, false]);

    let mut out = [true; 5];
    let x = MyStruct1(
        MyEnum::Choice3,
        true,
    );
    x.encode(&mut out[..], [3], [true]);
    assert_eq!(out, [true, false, true, true, true]);
}

#[test]
fn basic_bitfragment_unnamed_decode() {
    let x = [true, false, false];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 (
        MyEnum::Choice1,
        true,
    ));

    let x = [false, true, true];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 (
        MyEnum::Choice4,
        false,
    ));

    // offset
    let x = [false, false, false, true, false, false];
    let out = MyStruct1::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyStruct1 (
        MyEnum::Choice1,
        true,
    ));

    let x = [true, true, true, false, true, true];
    let out = MyStruct1::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyStruct1 (
        MyEnum::Choice4,
        false,
    ));

    // mirroring
    let x = [true, false, false];
    let out = MyStruct1::decode(&x[..], [2], [true]).unwrap();
    assert_eq!(out, MyStruct1 (
        MyEnum::Choice2,
        false,
    ));

    let x = [true, true, true, false, true, true];
    let out = MyStruct1::decode(&x[..], [5], [true]).unwrap();
    assert_eq!(out, MyStruct1 (
        MyEnum::Choice3,
        true,
    ));
}
