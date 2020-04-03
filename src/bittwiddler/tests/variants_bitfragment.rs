use bittwiddler::*;

enum EnumVar1{}
enum EnumVar2{}

#[bitpattern(variant = EnumVar1)]
#[bitpattern(variant = EnumVar2)]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum {
    #[bits(variant = EnumVar1, "00")]
    #[bits(variant = EnumVar2, "11")]
    Choice1,
    #[bits(variant = EnumVar1, "01")]
    #[bits(variant = EnumVar2, "10")]
    Choice2,
    #[bits(variant = EnumVar1, "10")]
    #[bits(variant = EnumVar2, "01")]
    Choice3,
    #[bits(variant = EnumVar1, "11")]
    #[bits(variant = EnumVar2, "00")]
    Choice4,
}

enum FragVar1{}
enum FragVar2{}
enum FragVar3{}

#[bitfragment(variant = FragVar1, dimensions = 1)]
#[bitfragment(variant = FragVar2, dimensions = 1)]
#[bitfragment(variant = FragVar3, dimensions = 1)]
#[derive(Debug, PartialEq, Eq)]
struct MyStruct1 {
    #[pat_bits(frag_variant = FragVar1, pat_variant = EnumVar1, "0" = 1, "1" = 2)]
    #[pat_bits(frag_variant = FragVar2, pat_variant = EnumVar1, "0" = 2, "1" = 3)]
    #[pat_bits(frag_variant = FragVar3, pat_variant = EnumVar2, "0" = 3, "1" = 4)]
    field_enum: MyEnum,
    #[pat_bits(frag_variant = FragVar1, "0" = 0)]
    #[pat_bits(frag_variant = FragVar2, "0" = 1)]
    #[pat_bits(frag_variant = FragVar3, "0" = 2)]
    field_bool: bool,
}

#[test]
fn frag_variants_encode() {
    let mut out = [false; 5];
    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    BitFragment::<FragVar1>::encode(&x, &mut out[..], [0], [false], ());
    assert_eq!(out, [false, false, true, false, false]);

    let mut out = [false; 5];
    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    BitFragment::<FragVar2>::encode(&x, &mut out[..], [0], [false], ());
    assert_eq!(out, [false, false, false, true, false]);

    let mut out = [false; 5];
    let x = MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: false,
    };
    BitFragment::<FragVar3>::encode(&x, &mut out[..], [0], [false], ());
    assert_eq!(out, [false, false, false, true, false]);
}

#[test]
fn frag_variants_decode() {
    let x = [true, true, false, true, true];
    let out = <MyStruct1 as BitFragment<FragVar1>>::decode(&x[..], [0], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    });

    let x = [true, true, true, false, true];
    let out = <MyStruct1 as BitFragment<FragVar2>>::decode(&x[..], [0], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice3,
        field_bool: true,
    });

    let x = [true, true, true, true, false];
    let out = <MyStruct1 as BitFragment<FragVar3>>::decode(&x[..], [0], [false], ()).unwrap();
    assert_eq!(out, MyStruct1 {
        field_enum: MyEnum::Choice2,
        field_bool: true,
    });
}
