use super::{build_test, Felt};
use ::air::FieldElement;
use std::ops::{Add, Div, Mul, Sub};
use vm_core::StarkField;

// Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, this routine raises
// it to the power 2^n, by means of n successive squarings
//
// See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L461-L469
fn msquare(v: Felt, n: usize) -> Felt {
    let mut v_ = v;
    for _ in 0..n {
        v_ = v_.square();
    }
    v_
}

// Given an element v ∈ Z_q | q = 2^64 - 2^32 + 1, this routine raises
// it to the power (p - 1) / 2
//
// See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L448-L459
fn legendre(v: Felt) -> Felt {
    let v0 = msquare(v, 31);
    let v1 = msquare(v0, 32);
    let v2 = v1 / v0;
    v2
}

fn is_zero(a: Felt) -> Felt {
    Felt::new((a == Felt::ZERO) as u64)
}

fn is_one(a: Felt) -> Felt {
    Felt::new((a == Felt::ONE) as u64)
}

fn bv_or(a: Felt, b: Felt) -> Felt {
    let flg_a = (a == Felt::ZERO) | (a == Felt::ONE);
    let flg_b = (b == Felt::ZERO) | (b == Felt::ONE);

    assert_eq!(flg_a & flg_b, true);

    let c = a.as_int() | b.as_int();
    Felt::new(c)
}

fn sqrt(x: Felt) -> (Felt, Felt) {
    const GG: [u64; 32] = [
        1753635133440165772,
        4614640910117430873,
        9123114210336311365,
        16116352524544190054,
        6414415596519834757,
        1213594585890690845,
        17096174751763063430,
        5456943929260765144,
        9713644485405565297,
        16905767614792059275,
        5416168637041100469,
        17654865857378133588,
        3511170319078647661,
        18146160046829613826,
        9306717745644682924,
        12380578893860276750,
        6115771955107415310,
        17776499369601055404,
        16207902636198568418,
        1532612707718625687,
        17492915097719143606,
        455906449640507599,
        11353340290879379826,
        1803076106186727246,
        13797081185216407910,
        17870292113338400769,
        549755813888,
        70368744161280,
        17293822564807737345,
        18446744069397807105,
        281474976710656,
        18446744069414584320,
    ];

    let mut u = msquare(x, 31);
    let mut v = u.square() / (x + is_zero(x));

    const N: usize = 32;
    for j in 1..N {
        let i = N - j;
        let w = msquare(v, i - 1);
        let cc = w == Felt::new(Felt::MODULUS - 1);

        v = if !cc { v } else { v * Felt::new(GG[N - i]) };
        u = if !cc { u } else { u * Felt::new(GG[N - i - 1]) };
    }

    let cc = bv_or(is_zero(v), is_one(v));
    (u * cc, cc)
}

#[derive(Copy, Clone, Debug)]
struct GFp5 {
    pub a0: Felt,
    pub a1: Felt,
    pub a2: Felt,
    pub a3: Felt,
    pub a4: Felt,
}

impl GFp5 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            a0: Felt::new(0),
            a1: Felt::new(0),
            a2: Felt::new(0),
            a3: Felt::new(0),
            a4: Felt::new(0),
        }
    }

    pub fn rand() -> Self {
        Self {
            a0: rand_utils::rand_value::<Felt>(),
            a1: rand_utils::rand_value::<Felt>(),
            a2: rand_utils::rand_value::<Felt>(),
            a3: rand_utils::rand_value::<Felt>(),
            a4: rand_utils::rand_value::<Felt>(),
        }
    }

    pub fn square(self) -> Self {
        let two = Felt::new(2);
        let three = Felt::new(3);
        let six = two * three;

        Self {
            a0: self.a0 * self.a0 + six * (self.a1 * self.a4 + self.a2 * self.a3),
            a1: two * self.a0 * self.a1 + three * (self.a3 * self.a3 + two * self.a2 * self.a4),
            a2: self.a1 * self.a1 + two * self.a0 * self.a2 + six * self.a3 * self.a4,
            a3: two * (self.a0 * self.a3 + self.a1 * self.a2) + three * self.a4 * self.a4,
            a4: self.a2 * self.a2 + two * (self.a0 * self.a4 + self.a1 * self.a3),
        }
    }

    fn frobenius_once(self) -> Self {
        Self {
            a0: self.a0,
            a1: self.a1 * Felt::new(1041288259238279555),
            a2: self.a2 * Felt::new(15820824984080659046),
            a3: self.a3 * Felt::new(211587555138949697),
            a4: self.a4 * Felt::new(1373043270956696022),
        }
    }

    fn frobenius_twice(self) -> Self {
        Self {
            a0: self.a0,
            a1: self.a1 * Felt::new(15820824984080659046),
            a2: self.a2 * Felt::new(1373043270956696022),
            a3: self.a3 * Felt::new(1041288259238279555),
            a4: self.a4 * Felt::new(211587555138949697),
        }
    }

    pub fn inv(self) -> Self {
        let t0 = self.frobenius_once();
        let t1 = t0 * t0.frobenius_once();
        let t2 = t1 * t1.frobenius_twice();

        let t3 = self.a0 * t2.a0
            + Felt::new(3)
                * (self.a1 * t2.a4 + self.a2 * t2.a3 + self.a3 * t2.a2 + self.a4 * t2.a1);

        let flg = t3 == Felt::new(0);
        let t3 = t3 + Felt::new(flg as u64);
        let t4 = Felt::new(1) / t3;

        Self {
            a0: t4 * t2.a0,
            a1: t4 * t2.a1,
            a2: t4 * t2.a2,
            a3: t4 * t2.a3,
            a4: t4 * t2.a4,
        }
    }

    pub fn legendre(self) -> Felt {
        let t0 = self.frobenius_once();
        let t1 = t0 * t0.frobenius_once();
        let t2 = t1 * t1.frobenius_twice();

        let t3 = self.a0 * t2.a0
            + Felt::new(3)
                * (self.a1 * t2.a4 + self.a2 * t2.a3 + self.a3 * t2.a2 + self.a4 * t2.a1);

        legendre(t3)
    }
}

impl Add for GFp5 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a0: self.a0 + rhs.a0,
            a1: self.a1 + rhs.a1,
            a2: self.a2 + rhs.a2,
            a3: self.a3 + rhs.a3,
            a4: self.a4 + rhs.a4,
        }
    }
}

impl Sub for GFp5 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a0: self.a0 - rhs.a0,
            a1: self.a1 - rhs.a1,
            a2: self.a2 - rhs.a2,
            a3: self.a3 - rhs.a3,
            a4: self.a4 - rhs.a4,
        }
    }
}

impl Mul for GFp5 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            a0: self.a0 * rhs.a0
                + Felt::new(3)
                    * (self.a1 * rhs.a4 + self.a2 * rhs.a3 + self.a3 * rhs.a2 + self.a4 * rhs.a1),
            a1: self.a0 * rhs.a1
                + self.a1 * rhs.a0
                + Felt::new(3) * (self.a2 * rhs.a4 + self.a3 * rhs.a3 + self.a4 * rhs.a2),
            a2: self.a0 * rhs.a2
                + self.a1 * rhs.a1
                + self.a2 * rhs.a0
                + Felt::new(3) * (self.a3 * rhs.a4 + self.a4 * rhs.a3),
            a3: self.a0 * rhs.a3
                + self.a1 * rhs.a2
                + self.a2 * rhs.a1
                + self.a3 * rhs.a0
                + Felt::new(3) * (self.a4 * rhs.a4),
            a4: self.a0 * rhs.a4
                + self.a1 * rhs.a3
                + self.a2 * rhs.a2
                + self.a3 * rhs.a1
                + self.a4 * rhs.a0,
        }
    }
}

impl Div for GFp5 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

#[test]
fn test_gfp5_add() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::add
    end";

    let a = GFp5::rand();
    let b = GFp5::rand();
    let c = a + b;

    let mut stack = [
        a.a0.as_int(),
        a.a1.as_int(),
        a.a2.as_int(),
        a.a3.as_int(),
        a.a4.as_int(),
        b.a0.as_int(),
        b.a1.as_int(),
        b.a2.as_int(),
        b.a3.as_int(),
        b.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], c.a0);
    assert_eq!(strace[1], c.a1);
    assert_eq!(strace[2], c.a2);
    assert_eq!(strace[3], c.a3);
    assert_eq!(strace[4], c.a4);
}

#[test]
fn test_gfp5_sub() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::sub
    end";

    let a = GFp5::rand();
    let b = GFp5::rand();
    let c = a - b;

    let mut stack = [
        a.a0.as_int(),
        a.a1.as_int(),
        a.a2.as_int(),
        a.a3.as_int(),
        a.a4.as_int(),
        b.a0.as_int(),
        b.a1.as_int(),
        b.a2.as_int(),
        b.a3.as_int(),
        b.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], c.a0);
    assert_eq!(strace[1], c.a1);
    assert_eq!(strace[2], c.a2);
    assert_eq!(strace[3], c.a3);
    assert_eq!(strace[4], c.a4);
}

#[test]
fn test_gfp5_mul() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::mul
    end";

    let a = GFp5::rand();
    let b = GFp5::rand();
    let c = a * b;

    let mut stack = [
        a.a0.as_int(),
        a.a1.as_int(),
        a.a2.as_int(),
        a.a3.as_int(),
        a.a4.as_int(),
        b.a0.as_int(),
        b.a1.as_int(),
        b.a2.as_int(),
        b.a3.as_int(),
        b.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], c.a0);
    assert_eq!(strace[1], c.a1);
    assert_eq!(strace[2], c.a2);
    assert_eq!(strace[3], c.a3);
    assert_eq!(strace[4], c.a4);
}

#[test]
fn test_gfp5_square() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::square
    end";

    let a = GFp5::rand();
    let b = a.square();

    let mut stack = [
        a.a0.as_int(),
        a.a1.as_int(),
        a.a2.as_int(),
        a.a3.as_int(),
        a.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], b.a0);
    assert_eq!(strace[1], b.a1);
    assert_eq!(strace[2], b.a2);
    assert_eq!(strace[3], b.a3);
    assert_eq!(strace[4], b.a4);
}

#[test]
fn test_gfp5_inv() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::inv
    end";

    let a = GFp5::rand();
    let b = a.inv();

    let mut stack = [
        a.a0.as_int(),
        a.a1.as_int(),
        a.a2.as_int(),
        a.a3.as_int(),
        a.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], b.a0);
    assert_eq!(strace[1], b.a1);
    assert_eq!(strace[2], b.a2);
    assert_eq!(strace[3], b.a3);
    assert_eq!(strace[4], b.a4);
}

#[test]
fn test_gfp5_div() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::div
    end";

    let a = GFp5::rand();
    let b = GFp5::rand();
    let c = a / b;

    let mut stack = [
        a.a0.as_int(),
        a.a1.as_int(),
        a.a2.as_int(),
        a.a3.as_int(),
        a.a4.as_int(),
        b.a0.as_int(),
        b.a1.as_int(),
        b.a2.as_int(),
        b.a3.as_int(),
        b.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], c.a0);
    assert_eq!(strace[1], c.a1);
    assert_eq!(strace[2], c.a2);
    assert_eq!(strace[3], c.a3);
    assert_eq!(strace[4], c.a4);
}

#[test]
fn test_gfp5_legendre() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::legendre
    end";

    let a = GFp5::rand();
    let b = a.legendre();

    let mut stack = [
        a.a0.as_int(),
        a.a1.as_int(),
        a.a2.as_int(),
        a.a3.as_int(),
        a.a4.as_int(),
    ];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], b);
}

#[test]
fn test_gf_sqrt() {
    let source = "
    use.std::math::gfp5

    begin
        exec.gfp5::gf_sqrt
    end";

    let a = Felt::new(31);
    let (b, c) = sqrt(a);

    let mut stack = [a.as_int()];
    stack.reverse();

    let test = build_test!(source, &stack);
    let strace = test.get_last_stack_state();

    assert_eq!(strace[0], b);
    assert_eq!(strace[1], c);
}
