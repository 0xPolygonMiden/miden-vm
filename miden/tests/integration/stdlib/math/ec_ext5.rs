use super::ext5::{bv_or, Ext5};
use super::{build_test, Felt};
use ::air::FieldElement;
use test_case::test_case;
use vm_core::StarkField;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
struct ECExt5 {
    pub x: Ext5,
    pub y: Ext5,
    pub point_at_infinity: Felt,
}

impl ECExt5 {
    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L996
    pub fn a() -> Ext5 {
        Ext5::from_int(2)
    }

    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L997
    pub fn b() -> Ext5 {
        Ext5::new(0, 263, 0, 0, 0)
    }

    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L998
    pub fn bmul4_1() -> Felt {
        Self::b().a1 * Felt::new(4)
    }

    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L999
    #[allow(dead_code)]
    pub fn adiv3() -> Ext5 {
        Self::a() / Ext5::from_int(3)
    }

    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1000
    #[allow(dead_code)]
    pub fn a_prime() -> Ext5 {
        let three = Ext5::from_int(3);
        (three * Self::b() - Self::a().square()) / three
    }

    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1003
    #[allow(dead_code)]
    pub fn b_prime() -> Ext5 {
        let a = Self::a();
        let two = Ext5::from_int(2);
        let nine = Ext5::from_int(9);
        let twenty_seven = Ext5::from_int(27);

        a * (two * a.square() - nine * Self::b()) / twenty_seven
    }

    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1006
    #[allow(dead_code)]
    pub fn neutral() -> Self {
        Self {
            x: Ext5::zero(),
            y: Ext5::zero(),
            point_at_infinity: Felt::ONE,
        }
    }

    // Validates an encoded elliptic curve point, verifying whether it can be decoded successfully or not, denoted by boolean return value
    //
    // Taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1043-L1052
    pub fn validate(w: Ext5) -> Felt {
        let e = w.square() - Self::a();
        let delta = e.square().subk1(Self::bmul4_1());
        bv_or(
            Felt::new((delta.legendre() == Felt::ONE) as u64),
            w.is_zero(),
        )
    }
}

// Test vectors taken from https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1528-L1556
#[test_case(0, 0, 0, 0, 0, true; "[0] should validate")]
#[test_case(12539254003028696409, 15524144070600887654, 15092036948424041984, 11398871370327264211, 10958391180505708567, true; "[1] should validate")]
#[test_case(11001943240060308920, 17075173755187928434, 3940989555384655766, 15017795574860011099, 5548543797011402287, true; "[2] should validate")]
#[test_case(246872606398642312, 4900963247917836450, 7327006728177203977, 13945036888436667069, 3062018119121328861, true; "[3] should validate")]
#[test_case(8058035104653144162, 16041715455419993830, 7448530016070824199, 11253639182222911208, 6228757819849640866, true; "[4] should validate")]
#[test_case(10523134687509281194, 11148711503117769087, 9056499921957594891, 13016664454465495026, 16494247923890248266, true; "[5] should validate")]
#[test_case(12173306542237620, 6587231965341539782, 17027985748515888117, 17194831817613584995, 10056734072351459010, true; "[6] should validate")]
#[test_case(9420857400785992333, 4695934009314206363, 14471922162341187302, 13395190104221781928, 16359223219913018041, true; "[7] should validate")]
#[test_case(13557832913345268708, 15669280705791538619, 8534654657267986396, 12533218303838131749, 5058070698878426028, false; "[8] should not validate")]
#[test_case(135036726621282077, 17283229938160287622, 13113167081889323961, 1653240450380825271, 520025869628727862, false; "[9] should not validate")]
#[test_case(6727960962624180771, 17240764188796091916, 3954717247028503753, 1002781561619501488, 4295357288570643789, false; "[10] should not validate")]
#[test_case(4578929270179684956, 3866930513245945042, 7662265318638150701, 9503686272550423634, 12241691520798116285, false; "[11] should not validate")]
#[test_case(16890297404904119082, 6169724643582733633, 9725973298012340311, 5977049210035183790, 11379332130141664883, false; "[12] should not validate")]
#[test_case(13777379982711219130, 14715168412651470168, 17942199593791635585, 6188824164976547520, 15461469634034461986, false; "[13] should not validate")]
fn test_ec_ext5_point_validate(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, should_validate: bool) {
    let source = "
    use.std::math::ec_ext5

    begin
        exec.ec_ext5::validate
    end";

    let w = Ext5::new(a0, a1, a2, a3, a4);
    let flg = ECExt5::validate(w);

    let mut stack = [
        w.a0.as_int(),
        w.a1.as_int(),
        w.a2.as_int(),
        w.a3.as_int(),
        w.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], flg);
    assert_eq!(strace[0], Felt::new(should_validate as u64));
}
