use super::{HashMap, OpCode, OpHint, Span};
use winterfell::math::{fields::f128::BaseElement, FieldElement};

#[test]
fn span_hash() {
    // hash noop operations
    let block = Span::from_instructions(vec![
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
        OpCode::Noop,
    ]);

    let hash = block.hash([BaseElement::ZERO; 4]);
    assert_eq!(
        [
            BaseElement::new(283855050660402859567809346597024356257),
            BaseElement::new(290430270201175202384178252750741838599),
            BaseElement::new(33642161455895506272337605785278290375),
            BaseElement::new(114906032113415280284656928780040029722),
        ],
        hash
    );

    // hash noops and a push operation
    let mut hints = HashMap::new();
    hints.insert(8, OpHint::PushValue(BaseElement::new(1)));
    let block = Span::new(
        vec![
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Push,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
        ],
        hints,
    );

    let hash = block.hash([BaseElement::ZERO; 4]);
    assert_eq!(
        [
            BaseElement::new(309939768290184920181146334415666126639),
            BaseElement::new(189522128575407709345588553132211127638),
            BaseElement::new(300449513105356487315600679523377528535),
            BaseElement::new(201241536410685268433124688525928056833),
        ],
        hash
    );

    // hash noops and a push operation with a different value
    let mut hints = HashMap::new();
    hints.insert(8, OpHint::PushValue(BaseElement::new(2)));
    let block = Span::new(
        vec![
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Push,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
            OpCode::Noop,
        ],
        hints,
    );

    let hash = block.hash([BaseElement::ZERO; 4]);
    assert_eq!(
        [
            BaseElement::new(238085520613464573032580920836572617149),
            BaseElement::new(98362585914038709664139524327351111560),
            BaseElement::new(159064915881679512167348007665307977960),
            BaseElement::new(152057468867502483682425300737565245134),
        ],
        hash
    );
}
