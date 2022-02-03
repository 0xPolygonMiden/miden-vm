use crate::DebugOptions;
use crate::Operation::Debug;

#[test]
fn display() {
    // Test debug fmt works
    let display = format!("{}", Debug(DebugOptions::All));
    assert_eq!(display, "debug(all)");
}
