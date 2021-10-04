use super::{HashMap, OpCode, OpHint, Span, BaseElement, FieldElement};

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
            BaseElement::new(21281585952089818632667426088457220169),
            BaseElement::new(74371079575379061369490632324778122819),
            BaseElement::new(307843531863530356619580434289168636552),
            BaseElement::new(70180866509186194377223664970003982850),
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
            BaseElement::new(120312439524824049007173098751258097313),
            BaseElement::new(9063583005632364127455674159119118212),
            BaseElement::new(217782017725949502969611551785928467521),
            BaseElement::new(42776518812589542572619464996530538072),
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
            BaseElement::new(21548381523225837058194789987159148415),
            BaseElement::new(279198440899867644154340390529802814983),
            BaseElement::new(139665667881959253181285846812987917034),
            BaseElement::new(332966755258441265531266460444426875939),
        ],
        hash
    );
}
