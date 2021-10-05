use super::{hash_op, hash_seq, BaseElement, FieldElement, OpCode, OpHint, BASE_CYCLE_LENGTH};
use core::fmt;
use winter_utils::collections::BTreeMap;

//#[cfg(test)]
//mod tests;

// CONSTANTS
// ================================================================================================
const BLOCK_SUFFIX: [u8; 1] = [OpCode::Noop as u8];
const BLOCK_SUFFIX_OFFSET: usize = BASE_CYCLE_LENGTH - 1;

const LOOP_SKIP_BLOCK: [OpCode; 15] = [
    OpCode::Not,
    OpCode::Assert,
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
];

const LOOP_BLOCK_SUFFIX: [u8; 16] = [
    OpCode::Not as u8,
    OpCode::Assert as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
    OpCode::Noop as u8,
];

// TYPES AND INTERFACES
// ================================================================================================

#[derive(Clone)]
pub enum ProgramBlock {
    Span(Span),
    Group(Group),
    Switch(Switch),
    Loop(Loop),
}

#[derive(Clone)]
pub struct Span {
    op_codes: Vec<OpCode>,
    op_hints: BTreeMap<usize, OpHint>,
}

#[derive(Clone)]
pub struct Group {
    body: Vec<ProgramBlock>,
}

#[derive(Clone)]
pub struct Switch {
    t_branch: Vec<ProgramBlock>,
    f_branch: Vec<ProgramBlock>,
}

#[derive(Clone)]
pub struct Loop {
    body: Vec<ProgramBlock>,
    skip: Vec<ProgramBlock>,
}

// PROGRAM BLOCK IMPLEMENTATION
// ================================================================================================

impl ProgramBlock {
    pub fn is_span(&self) -> bool {
        matches!(self, ProgramBlock::Span(_))
    }
}

impl fmt::Debug for ProgramBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramBlock::Span(block) => write!(f, "{:?}", block)?,
            ProgramBlock::Group(block) => write!(f, "{:?}", block)?,
            ProgramBlock::Switch(block) => write!(f, "{:?}", block)?,
            ProgramBlock::Loop(block) => write!(f, "{:?}", block)?,
        }
        Ok(())
    }
}

// SPAN IMPLEMENTATION
// ================================================================================================
impl Span {
    pub fn new(instructions: Vec<OpCode>, hints: BTreeMap<usize, OpHint>) -> Span {
        let alignment = instructions.len() % BASE_CYCLE_LENGTH;
        assert!(
            alignment == BASE_CYCLE_LENGTH - 1,
            "invalid number of instructions: expected one less than a multiple of {}, but was {}",
            BASE_CYCLE_LENGTH,
            instructions.len()
        );

        // make sure all instructions are valid
        for (i, &op_code) in instructions.iter().enumerate() {
            if op_code == OpCode::Push {
                assert!(
                    i % 8 == 0,
                    "PUSH is not allowed on step {}, must be on step which is a multiple of 8",
                    i
                );
                let hint = hints.get(&i);
                assert!(
                    hint.is_some(),
                    "invalid PUSH operation on step {}: operation value is missing",
                    i
                );
                match hint.unwrap() {
                    OpHint::PushValue(_) => (),
                    _ => panic!(
                        "invalid PUSH operation on step {}: operation value is of wrong type",
                        i
                    ),
                }
            }
        }

        // make sure all hints are within bounds
        for &step in hints.keys() {
            assert!(
                step < instructions.len(),
                "hint out of bounds: step must be smaller than {} but is {}",
                instructions.len(),
                step
            );
        }

        Span {
            op_codes: instructions,
            op_hints: hints,
        }
    }

    pub fn new_block(instructions: Vec<OpCode>) -> ProgramBlock {
        ProgramBlock::Span(Span::new(instructions, BTreeMap::new()))
    }

    pub fn from_instructions(instructions: Vec<OpCode>) -> Span {
        Span::new(instructions, BTreeMap::new())
    }

    pub fn length(&self) -> usize {
        self.op_codes.len()
    }

    pub fn starts_with(&self, instructions: &[OpCode]) -> bool {
        self.op_codes.starts_with(instructions)
    }

    pub fn get_op(&self, step: usize) -> (OpCode, OpHint) {
        (self.op_codes[step], self.get_hint(step))
    }

    pub fn get_hint(&self, op_index: usize) -> OpHint {
        match self.op_hints.get(&op_index) {
            Some(&hint) => hint,
            None => OpHint::None,
        }
    }

    pub fn hash(&self, mut state: [BaseElement; 4]) -> [BaseElement; 4] {
        for (i, &op_code) in self.op_codes.iter().enumerate() {
            let op_value = if op_code == OpCode::Push {
                match self.get_hint(i) {
                    OpHint::PushValue(op_value) => op_value,
                    _ => panic!("value for PUSH operation is missing"),
                }
            } else {
                BaseElement::ZERO
            };
            hash_op(&mut state, op_code as u8, op_value, i)
        }
        state
    }

    pub fn merge(span1: &Span, span2: &Span) -> Span {
        // merge op codes
        let mut new_op_codes = span1.op_codes.clone();
        new_op_codes.push(OpCode::Noop);
        new_op_codes.extend_from_slice(&span2.op_codes);

        // merge hints
        let offset = span1.length() + 1;
        let mut new_hints = span1.op_hints.clone();
        for (step, &hint) in &span2.op_hints {
            new_hints.insert(step + offset, hint);
        }

        // build and return a new Span
        Span::new(new_op_codes, new_hints)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (op_code, op_hint) = self.get_op(0);
        write!(f, "{}{}", op_code, op_hint)?;

        for i in 1..self.length() {
            let (op_code, op_hint) = self.get_op(i);
            write!(f, " {}{}", op_code, op_hint)?;
        }
        Ok(())
    }
}

// GROUP IMPLEMENTATION
// ================================================================================================
impl Group {
    pub fn new(body: Vec<ProgramBlock>) -> Group {
        validate_block_list(&body, &[]);
        Group { body }
    }

    pub fn new_block(body: Vec<ProgramBlock>) -> ProgramBlock {
        ProgramBlock::Group(Group::new(body))
    }

    pub fn body(&self) -> &[ProgramBlock] {
        &self.body
    }

    pub fn body_hash(&self) -> BaseElement {
        hash_seq(&self.body, &BLOCK_SUFFIX, BLOCK_SUFFIX_OFFSET)
    }

    pub fn get_hash(&self) -> (BaseElement, BaseElement) {
        let v0 = self.body_hash();
        (v0, BaseElement::ZERO)
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block ")?;
        for block in self.body.iter() {
            write!(f, "{:?} ", block)?;
        }
        write!(f, "end")
    }
}

// SWITCH IMPLEMENTATION
// ================================================================================================
impl Switch {
    pub fn new(true_branch: Vec<ProgramBlock>, false_branch: Vec<ProgramBlock>) -> Switch {
        validate_block_list(&true_branch, &[OpCode::Assert]);
        validate_block_list(&false_branch, &[OpCode::Not, OpCode::Assert]);
        Switch {
            t_branch: true_branch,
            f_branch: false_branch,
        }
    }

    pub fn new_block(
        true_branch: Vec<ProgramBlock>,
        false_branch: Vec<ProgramBlock>,
    ) -> ProgramBlock {
        ProgramBlock::Switch(Switch::new(true_branch, false_branch))
    }

    pub fn true_branch(&self) -> &[ProgramBlock] {
        &self.t_branch
    }

    pub fn true_branch_hash(&self) -> BaseElement {
        hash_seq(&self.t_branch, &BLOCK_SUFFIX, BLOCK_SUFFIX_OFFSET)
    }

    pub fn false_branch(&self) -> &[ProgramBlock] {
        &self.f_branch
    }

    pub fn false_branch_hash(&self) -> BaseElement {
        hash_seq(&self.f_branch, &BLOCK_SUFFIX, BLOCK_SUFFIX_OFFSET)
    }

    pub fn get_hash(&self) -> (BaseElement, BaseElement) {
        let v0 = self.true_branch_hash();
        let v1 = self.false_branch_hash();
        (v0, v1)
    }
}

impl fmt::Debug for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if ")?;
        for block in self.t_branch.iter() {
            write!(f, "{:?} ", block)?;
        }
        write!(f, "else ")?;
        for block in self.f_branch.iter() {
            write!(f, "{:?} ", block)?;
        }
        write!(f, "end")
    }
}

// LOOP IMPLEMENTATION
// ================================================================================================
impl Loop {
    pub fn new(body: Vec<ProgramBlock>) -> Loop {
        validate_block_list(&body, &[OpCode::Assert]);

        let skip_block = Span::from_instructions(LOOP_SKIP_BLOCK.to_vec());
        let skip = vec![ProgramBlock::Span(skip_block)];

        Loop { body, skip }
    }

    pub fn new_block(body: Vec<ProgramBlock>) -> ProgramBlock {
        ProgramBlock::Loop(Loop::new(body))
    }

    pub fn body(&self) -> &[ProgramBlock] {
        &self.body
    }

    pub fn image(&self) -> BaseElement {
        hash_seq(&self.body, &[], 0)
    }

    pub fn body_hash(&self) -> BaseElement {
        hash_seq(&self.body, &LOOP_BLOCK_SUFFIX, 0)
    }

    pub fn skip(&self) -> &[ProgramBlock] {
        &self.skip
    }

    pub fn skip_hash(&self) -> BaseElement {
        hash_seq(&self.skip, &BLOCK_SUFFIX, BLOCK_SUFFIX_OFFSET)
    }

    pub fn get_hash(&self) -> (BaseElement, BaseElement) {
        let v0 = self.body_hash();
        let v1 = self.skip_hash();
        (v0, v1)
    }
}

impl fmt::Debug for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while ")?;
        for block in self.body.iter() {
            write!(f, "{:?} ", block)?;
        }
        write!(f, "end")
    }
}

// HELPER FUNCTIONS
// ================================================================================================
fn validate_block_list(blocks: &[ProgramBlock], starts_with: &[OpCode]) {
    assert!(
        !blocks.is_empty(),
        "a sequence of blocks must contain at least one block"
    );

    // first block must be a span block
    match &blocks[0] {
        ProgramBlock::Span(block) => {
            // if the block must start with a specific sequence of instructions, make sure it does
            if !starts_with.is_empty() {
                assert!(
                    block.starts_with(starts_with),
                    "the first block does not start with a valid sequence of instructions"
                );
            }
        }
        _ => panic!("a sequence of blocks must start with a Span block"),
    };

    // span block cannot be followed by another span block
    let mut was_span = true;
    for block in blocks.iter().skip(1) {
        match block {
            ProgramBlock::Span(_) => {
                assert!(
                    !was_span,
                    "a Span block cannot be followed by another Span block"
                );
            }
            _ => was_span = false,
        }
    }
}
