use super::{build_test, Felt};
use std::ops::{Add, Sub};
use vm_core::StarkField;

#[derive(Copy, Clone)]
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
