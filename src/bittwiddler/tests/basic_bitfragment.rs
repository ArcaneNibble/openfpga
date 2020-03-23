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

#[bitfragment]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct1 {
    field_enum: MyEnum,
    field_bool: bool,
}

#[test]
fn basic_bitfragment_encode() {
    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };

    let mut out = [false; 3];

    x.encode(&mut out[..], [0], [false]);
}
