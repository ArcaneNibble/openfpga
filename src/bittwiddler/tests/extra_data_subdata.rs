use bittwiddler::*;

#[bitpattern]
#[bitfragment(dimensions = 1)]
#[pat_bits("0" = 1, "1" = 2)]
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

#[bitfragment(dimensions = 1, encode_extra_type = u8, decode_extra_type = u16)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct1 {
    #[arr_off(|i| [i * (extra_data as usize)])]
    field_enum: [[[MyEnum; DIM1]; 1]; DIM2],
    #[pat_bits("0" = 0)]
    #[arr_off(|_| [0])]
    field_bool: [bool; 1],
}

#[bitfragment(dimensions = 1, encode_extra_type = u32, decode_extra_type = u32)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct2 {
    #[encode_sub_extra_data(extra_data as u8)]
    #[decode_sub_extra_data(extra_data as u16)]
    inner: MyStruct1,
}

#[test]
fn extra_data_subdata_encode() {
    let mut out = [false; 13];

    let x = MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice4, MyEnum::Choice3]],
                [[MyEnum::Choice2, MyEnum::Choice1]],
                [[MyEnum::Choice4, MyEnum::Choice1]],
            ],
            field_bool: [false],
        }
    };
    x.encode(&mut out[..], [0], [false], 2 as u32);
    assert_eq!(out, [false,
        true, true,
        true, false,
        false, true,
        false, false,
        true, true,
        false, false]);

    let x = MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice3, MyEnum::Choice2]],
                [[MyEnum::Choice1, MyEnum::Choice4]],
                [[MyEnum::Choice4, MyEnum::Choice2]],
            ],
            field_bool: [false],
        }
    };
    x.encode(&mut out[..], [0], [false], 2 as u32);
    assert_eq!(out, [false,
        true, false,
        false, true,
        false, false,
        true, true,
        true, true,
        false, true]);

    let x = MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice4, MyEnum::Choice3]],
                [[MyEnum::Choice2, MyEnum::Choice1]],
                [[MyEnum::Choice4, MyEnum::Choice1]],
            ],
            field_bool: [false],
        }
    };
    x.encode(&mut out[..], [12], [true], 2 as u32);
    assert_eq!(out, [
        false, false,
        true, true,
        false, false,
        true, false,
        false, true,
        true, true,
        false]);

    let x = MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice3, MyEnum::Choice2]],
                [[MyEnum::Choice1, MyEnum::Choice4]],
                [[MyEnum::Choice4, MyEnum::Choice2]],
            ],
            field_bool: [false],
        }
    };
    x.encode(&mut out[..], [12], [true], 2 as u32);
    assert_eq!(out, [
        true, false,
        true, true,
        true, true,
        false, false,
        true, false,
        false, true,
        false]);
}

#[test]
fn extra_data_subdata_decode() {
    let x = [true,
        false, false,
        true, false,
        false, true,
        true, true,
        true, false,
        false, true];
    let out = MyStruct2::decode(&x[..], [0], [false], 2 as u32).unwrap();
    assert_eq!(out, MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice1, MyEnum::Choice3]],
                [[MyEnum::Choice2, MyEnum::Choice4]],
                [[MyEnum::Choice3, MyEnum::Choice2]],
            ],
            field_bool: [true],
        }
    });

    let x = [true,
        false, true,
        true, false,
        false, false,
        true, true,
        true, true,
        false, true];
    let out = MyStruct2::decode(&x[..], [0], [false], 2 as u32).unwrap();
    assert_eq!(out, MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice2, MyEnum::Choice3]],
                [[MyEnum::Choice1, MyEnum::Choice4]],
                [[MyEnum::Choice4, MyEnum::Choice2]],
            ],
            field_bool: [true],
        }
    });

    let x = [
        true, false,
        false, true,
        true, true,
        true, false,
        false, true,
        false, false,
        true];
    let out = MyStruct2::decode(&x[..], [12], [true], 2 as u32).unwrap();
    assert_eq!(out, MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice1, MyEnum::Choice3]],
                [[MyEnum::Choice2, MyEnum::Choice4]],
                [[MyEnum::Choice3, MyEnum::Choice2]],
            ],
            field_bool: [true],
        }
    });

    let x = [
        true, false,
        true, true,
        true, true,
        false, false,
        false, true,
        true, false,
        true];
    let out = MyStruct2::decode(&x[..], [12], [true], 2 as u32).unwrap();
    assert_eq!(out, MyStruct2 {
        inner: MyStruct1 {
            field_enum: [
                [[MyEnum::Choice2, MyEnum::Choice3]],
                [[MyEnum::Choice1, MyEnum::Choice4]],
                [[MyEnum::Choice4, MyEnum::Choice2]],
            ],
            field_bool: [true],
        }
    });
}
