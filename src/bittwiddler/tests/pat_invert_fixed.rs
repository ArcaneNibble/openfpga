use bittwiddler::*;

#[bitpattern(default = Self::Choice1)]
#[bitfragment(dimensions = 1)]
#[pat_bits("0" = 1, "1" = !2, "2" = true, "3" = false)]
#[derive(Debug, PartialEq, Eq)]
enum MyEnum {
    #[bits("0010")]
    Choice1,
    #[bits("0110")]
    Choice2,
    #[bits("1010")]
    Choice3,
    #[bits("1110")]
    Choice4,
}

#[test]
fn pat_invert_fixed_encode() {
    let mut out = [false; 3];

    let x = MyEnum::Choice2;
    BitFragment::encode(&x, &mut out[..], [0], [false]);
    assert_eq!(out, [false, false, false]);

    let x = MyEnum::Choice3;
    BitFragment::encode(&x, &mut out[..], [0], [false]);
    assert_eq!(out, [false, true, true]);

    // offset
    let mut out = [true; 5];

    let x = MyEnum::Choice2;
    BitFragment::encode(&x, &mut out[..], [1], [false]);
    assert_eq!(out, [true, true, false, false, true]);

    let x = MyEnum::Choice3;
    BitFragment::encode(&x, &mut out[..], [1], [false]);
    assert_eq!(out, [true, true, true, true, true]);

    // mirroring
    let mut out = [false; 3];
    let x = MyEnum::Choice2;
    BitFragment::encode(&x, &mut out[..], [2], [true]);
    assert_eq!(out, [false, false, false]);

    let mut out = [true; 5];
    let x = MyEnum::Choice3;
    BitFragment::encode(&x, &mut out[..], [3], [true]);
    assert_eq!(out, [true, true, true, true, true]);
}

#[test]
fn pat_invert_fixed_decode() {
    let x = [true, false, true];
    let out: MyEnum = BitFragment::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice1);

    let x = [false, true, false];
    let out: MyEnum = BitFragment::decode(&x[..], [0], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice4);

    // offset
    let x = [false, false, false, true, false, true];
    let out: MyEnum = BitFragment::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice1);

    let x = [true, true, true, false, true, false];
    let out: MyEnum = BitFragment::decode(&x[..], [3], [false]).unwrap();
    assert_eq!(out, MyEnum::Choice4);

    // mirroring
    let x = [false, false, false];
    let out: MyEnum = BitFragment::decode(&x[..], [2], [true]).unwrap();
    assert_eq!(out, MyEnum::Choice2);

    let x = [true, true, true, true, true, true];
    let out: MyEnum = BitFragment::decode(&x[..], [5], [true]).unwrap();
    assert_eq!(out, MyEnum::Choice3);
}
