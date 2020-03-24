use bittwiddler::*;

enum Var1 {}

#[bitpattern]
#[bitfragment(variant = Var1, dimensions = 1)]
#[pat_pict(frag_variant = Var1, "0 . 1 .")]
#[bitfragment(dimensions = 1)]
#[pat_pict(".  0 . 1")]
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

#[bitfragment(variant = Var1, dimensions = 1)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct1 {
    #[frag(outer_frag_variant = Var1, inner_frag_variant = Var1)]
    field1: MyEnum,
    field2: MyEnum,
}

#[test]
fn subfragment_variant_encode() {
    let mut out = [false; 4];

    let x = MyStruct1 {
        field1: MyEnum::Choice2,
        field2: MyEnum::Choice3,
    };
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [false, true, true, false]);

    let x = MyStruct1 {
        field1: MyEnum::Choice4,
        field2: MyEnum::Choice1,
    };
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [true, false, true, false]);
}

#[test]
fn subfragment_variant_decode() {
    let x = [true, false, false, true];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field1: MyEnum::Choice3,
        field2: MyEnum::Choice2,
    });

    let x = [false, true, false, true];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field1: MyEnum::Choice1,
        field2: MyEnum::Choice4,
    });
}
