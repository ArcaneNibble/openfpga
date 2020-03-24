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
struct MyStruct1 {
    #[pat_bits("0" = 1, "1" = 2)]
    field_enum: MyEnum,
    #[pat_bits("0" = 0)]
    field_bool: bool,
}

#[test]
fn basic_bitfragment_encode() {
    let mut out = [false; 3];

    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [false, false, true]);

    let x = MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    };
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [true, true, false]);
}
