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

const DIM1: usize = 2;
const DIM2: usize = 3;

#[bitfragment(dimensions = 1)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct1 {
    #[pat_bits("0" = 1, "1" = 2)]
    field_enum: [[MyEnum; DIM1]; DIM2],
    #[pat_bits("0" = 0)]
    field_bool: bool,
}

#[test]
fn pattern_array_encode() {
    let mut out = [false; 13];

    let x = MyStruct1 {
        field_enum: [
            [MyEnum::Choice1, MyEnum::Choice2],
            [MyEnum::Choice3, MyEnum::Choice4],
            [MyEnum::Choice1, MyEnum::Choice4],
        ],
        field_bool: true,
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
            [MyEnum::Choice2, MyEnum::Choice3],
            [MyEnum::Choice4, MyEnum::Choice1],
            [MyEnum::Choice1, MyEnum::Choice3],
        ],
        field_bool: true,
    };
    x.encode(&mut out[..], [0], [false]);
    assert_eq!(out, [true,
        false, true,
        true, false,
        true, true,
        false, false,
        false, false,
        true, false]);
}

// #[test]
// fn pattern_array_decode() {
//     let x = [true, false, false];
//     let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
//     assert_eq!(out, MyStruct1 {
//         field_enum: MyEnum::Choice1,
//         field_bool: true,
//     });

//     let x = [false, true, true];
//     let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
//     assert_eq!(out, MyStruct1 {
//         field_enum: MyEnum::Choice4,
//         field_bool: false,
//     });

//     // offset
//     let x = [false, false, false, true, false, false];
//     let out = MyStruct1::decode(&x[..], [3], [false]).unwrap();
//     assert_eq!(out, MyStruct1 {
//         field_enum: MyEnum::Choice1,
//         field_bool: true,
//     });

//     let x = [true, true, true, false, true, true];
//     let out = MyStruct1::decode(&x[..], [3], [false]).unwrap();
//     assert_eq!(out, MyStruct1 {
//         field_enum: MyEnum::Choice4,
//         field_bool: false,
//     });

//     // mirroring
//     let x = [true, false, false];
//     let out = MyStruct1::decode(&x[..], [2], [true]).unwrap();
//     assert_eq!(out, MyStruct1 {
//         field_enum: MyEnum::Choice2,
//         field_bool: false,
//     });

//     let x = [true, true, true, false, true, true];
//     let out = MyStruct1::decode(&x[..], [5], [true]).unwrap();
//     assert_eq!(out, MyStruct1 {
//         field_enum: MyEnum::Choice3,
//         field_bool: true,
//     });
// }
