use core::ops::{Index, IndexMut};
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
    #[pat_pict(". 0 1")]
    field_enum: MyEnum,
    #[pat_pict("0 . .")]
    field_bool: bool,
}

#[test]
fn pat_pict_encode() {
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

    // offset
    let mut out = [true; 5];

    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    x.encode(&mut out[..], [1], [false]);
    assert_eq!(out, [true, false, false, true, true]);

    let x = MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    };
    x.encode(&mut out[..], [1], [false]);
    assert_eq!(out, [true, true, true, false, true]);

    // mirroring
    let mut out = [false; 3];
    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    x.encode(&mut out[..], [2], [true]);
    assert_eq!(out, [true, false, false]);

    let mut out = [true; 5];
    let x = MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    };
    x.encode(&mut out[..], [3], [true]);
    assert_eq!(out, [true, false, true, true, true]);
}

#[test]
fn pat_pict_decode() {
    let x = [true, false, false];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice1,
        field_bool: true,
    });

    let x = [false, true, true];
    let out = MyStruct1::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice4,
        field_bool: false,
    });

    // offset
    let x = [false, false, false, true, false, false];
    let out = MyStruct1::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice1,
        field_bool: true,
    });

    let x = [true, true, true, false, true, true];
    let out = MyStruct1::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice4,
        field_bool: false,
    });

    // mirroring
    let x = [true, false, false];
    let out = MyStruct1::decode(&x[..], [2], [true]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    });

    let x = [true, true, true, false, true, true];
    let out = MyStruct1::decode(&x[..], [5], [true]).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    });
}

#[bitfragment(dimensions = 2)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct2 {
    #[pat_pict("0 .
                . 1")]
    field_enum: MyEnum,
    #[pat_pict(".
                .
                0")]
    field_bool: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct My2DArray([bool; 6]);

impl Index<[usize; 2]> for My2DArray {
    type Output = bool;

    fn index(&self, coords: [usize; 2]) -> &bool {
        &self.0[coords[1] * 2 + coords[0]]
    }
}

impl IndexMut<[usize; 2]> for My2DArray {
    fn index_mut(&mut self, coords: [usize; 2]) -> &mut bool {
        &mut self.0[coords[1] * 2 + coords[0]]
    }
}

#[test]
fn pat_pict_2d_encode() {
    let mut out = My2DArray([false; 6]);

    let x = MyStruct2 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    x.encode(&mut out, [0, 0], [false, false]);
    assert_eq!(out.0, [false, false,
                       false, true,
                       false, false]);

    let x = MyStruct2 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    };
    x.encode(&mut out, [0, 0], [false, false]);
    assert_eq!(out.0, [true, false,
                       false, false,
                       true, false]);
}

#[test]
fn pat_pict_2d_decode() {
    let x = My2DArray([false, false,
                       true, false,
                       true, false]);
    let out = MyStruct2::decode(&x, [0, 0], [false, false]).unwrap();
    assert_eq!(out, MyStruct2 {
        field_enum: MyEnum::Choice1,
        field_bool: true,
    });

    let x = My2DArray([true, true,
                       false, true,
                       false, true]);
    let out = MyStruct2::decode(&x, [0, 0], [false, false]).unwrap();
    assert_eq!(out, MyStruct2 {
        field_enum: MyEnum::Choice4,
        field_bool: false,
    });
}
