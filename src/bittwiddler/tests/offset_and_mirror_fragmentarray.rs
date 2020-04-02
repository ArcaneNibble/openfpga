use bittwiddler::*;

#[bitpattern]
#[bitfragment(dimensions = 1)]
#[pat_bits("0" = 300, "1" = 299)]
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

const DIM1: usize = 2;
const DIM2: usize = 3;

#[bitfragment(dimensions = 1)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct1 {
    #[offset([301usize])]
    #[mirror([true])]
    #[arr_off(|i| if i == 0 { [3] } else if i == 5 { [13] } else { [i * 2] })]
    #[arr_mirror(|i| [i == 0 || i == 5])]
    field_enum: [[[MyEnum; DIM1]; 1]; DIM2],
    #[pat_bits("0" = 0)]
    #[arr_off(|_| [0])]
    field_bool: [bool; 1],
}

#[test]
fn offset_and_mirror_fragmentarray_encode() {
    let mut out = [false; 13];

    let x = MyStruct1 {
        field_enum: [
            [[MyEnum::Choice1, MyEnum::Choice2]],
            [[MyEnum::Choice3, MyEnum::Choice4]],
            [[MyEnum::Choice1, MyEnum::Choice4]],
        ],
        field_bool: [true],
    };
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [true,
        false, false,
        false, true,
        true, false,
        true, true,
        false, false,
        true, true]);

    let x = MyStruct1 {
        field_enum: [
            [[MyEnum::Choice2, MyEnum::Choice3]],
            [[MyEnum::Choice4, MyEnum::Choice1]],
            [[MyEnum::Choice1, MyEnum::Choice3]],
        ],
        field_bool: [true],
    };
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [true,
        true, false,
        true, false,
        true, true,
        false, false,
        false, false,
        false, true]);

    let x = MyStruct1 {
        field_enum: [
            [[MyEnum::Choice1, MyEnum::Choice2]],
            [[MyEnum::Choice3, MyEnum::Choice4]],
            [[MyEnum::Choice1, MyEnum::Choice4]],
        ],
        field_bool: [true],
    };
    x.encode(&mut out[..], [12], [true]);
    assert_eq!(out, [
        true, true,
        false, false,
        true, true,
        false, true,
        true, false,
        false, false,
        true]);

    let x = MyStruct1 {
        field_enum: [
            [[MyEnum::Choice2, MyEnum::Choice3]],
            [[MyEnum::Choice4, MyEnum::Choice1]],
            [[MyEnum::Choice1, MyEnum::Choice3]],
        ],
        field_bool: [true],
    };
    x.encode(&mut out[..], [12], [true]);
    assert_eq!(out, [
        true, false,
        false, false,
        false, false,
        true, true,
        false, true,
        false, true,
        true]);
}

#[test]
fn offset_and_mirror_fragmentarray_decode() {
    let x = [false,
        true, true,
        false, true,
        true, false,
        false, false,
        false, true,
        false, true];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: [
            [[MyEnum::Choice4, MyEnum::Choice2]],
            [[MyEnum::Choice3, MyEnum::Choice1]],
            [[MyEnum::Choice2, MyEnum::Choice3]],
        ],
        field_bool: [false],
    });

    let x = [false,
        false, true,
        false, true,
        true, true,
        false, false,
        false, false,
        false, true];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: [
            [[MyEnum::Choice3, MyEnum::Choice2]],
            [[MyEnum::Choice4, MyEnum::Choice1]],
            [[MyEnum::Choice1, MyEnum::Choice3]],
        ],
        field_bool: [false],
    });

    let x = [
        true, false,
        true, false,
        false, false,
        false, true,
        true, false,
        true, true,
        false];
    let out = MyStruct1::decode(&x[..], [12], [true]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: [
            [[MyEnum::Choice4, MyEnum::Choice2]],
            [[MyEnum::Choice3, MyEnum::Choice1]],
            [[MyEnum::Choice2, MyEnum::Choice3]],
        ],
        field_bool: [false],
    });

    let x = [
        true, false,
        false, false,
        false, false,
        true, true,
        true, false,
        true, false,
        false];
    let out = MyStruct1::decode(&x[..], [12], [true]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: [
            [[MyEnum::Choice3, MyEnum::Choice2]],
            [[MyEnum::Choice4, MyEnum::Choice1]],
            [[MyEnum::Choice1, MyEnum::Choice3]],
        ],
        field_bool: [false],
    });
}
