use super::{build_test, Felt};
use std::ops::{Add, Mul, Sub};
use vm_core::StarkField;

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
