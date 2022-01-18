use super::{Felt, RangeChecker, StarkField};

#[test]
fn temp_test() {
    let mut checker = RangeChecker::new();

    let value1 = Felt::new(1);
    let value2 = Felt::new(100);
    let value3 = Felt::new(360);

    checker.check(value1);
    checker.check(value2);
    checker.check(value3);

    let trace = checker.build_trace(1024);
    print_trace(&trace);
}

fn print_trace(trace: &[Vec<Felt>]) {
    for i in 0..trace[0].len() {
        println!(
            "{}\t{}\t{}\t{}",
            trace[0][i].as_int(),
            trace[1][i].as_int(),
            trace[2][i].as_int(),
            trace[3][i].as_int(),
        );
    }
}
