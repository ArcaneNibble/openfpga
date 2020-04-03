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
    #[offset([301usize])]
    #[mirror([true])]
    #[pat_bits("0" = 300, "1" = 299)]
    field_enum: MyEnum,
    #[pat_bits("0" = 0)]
    field_bool: bool,
}

#[test]
fn offset_and_mirror_pattern_encode() {
    let mut out = [false; 3];

    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    x.encode(&mut out[..], [0], [false], ());
    assert_eq!(out, [false, false, true]);

    let x = MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    };
    x.encode(&mut out[..], [0], [false], ());
    assert_eq!(out, [true, true, false]);

    // offset
    let mut out = [true; 5];

    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    x.encode(&mut out[..], [1], [false], ());
    assert_eq!(out, [true, false, false, true, true]);

    let x = MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    };
    x.encode(&mut out[..], [1], [false], ());
    assert_eq!(out, [true, true, true, false, true]);

    // mirroring
    let mut out = [false; 3];
    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    x.encode(&mut out[..], [2], [true], ());
    assert_eq!(out, [true, false, false]);

    let mut out = [true; 5];
    let x = MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    };
    x.encode(&mut out[..], [3], [true], ());
    assert_eq!(out, [true, false, true, true, true]);
}

#[test]
fn offset_and_mirror_pattern_decode() {
    let x = [true, false, false];
    let out = MyStruct1::decode(&x[..], [0], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice1,
        field_bool: true,
    });

    let x = [false, true, true];
    let out = MyStruct1::decode(&x[..], [0], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice4,
        field_bool: false,
    });

    // offset
    let x = [false, false, false, true, false, false];
    let out = MyStruct1::decode(&x[..], [3], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice1,
        field_bool: true,
    });

    let x = [true, true, true, false, true, true];
    let out = MyStruct1::decode(&x[..], [3], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice4,
        field_bool: false,
    });

    // mirroring
    let x = [true, false, false];
    let out = MyStruct1::decode(&x[..], [2], [true], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    });

    let x = [true, true, true, false, true, true];
    let out = MyStruct1::decode(&x[..], [5], [true], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    });
}
