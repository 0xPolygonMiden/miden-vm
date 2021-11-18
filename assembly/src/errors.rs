use core::fmt;

// ASSEMBLY ERROR
// ================================================================================================

pub struct AssemblyError {
    message: String,
    step: usize,
    op: String,
}

impl AssemblyError {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn unexpected_eof(step: usize) -> AssemblyError {
        AssemblyError {
            message: "unexpected EOF".to_string(),
            step,
            op: String::from(""),
        }
    }

    pub fn empty_program() -> AssemblyError {
        AssemblyError {
            message: String::from("a program must contain at least one instruction"),
            step: 0,
            op: String::from("begin"),
        }
    }

    pub fn empty_block(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: String::from("a program block must contain at least one instruction"),
            step,
            op: op.join("."),
        }
    }

    pub fn invalid_program_start(op: &str) -> AssemblyError {
        AssemblyError {
            message: String::from("a program must start with a 'being' instruction"),
            step: 0,
            op: String::from(op),
        }
    }

    pub fn invalid_program_end(op: &str) -> AssemblyError {
        AssemblyError {
            message: String::from("a program must end with an 'end' instruction"),
            step: 0,
            op: String::from(op),
        }
    }

    pub fn invalid_op(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: format!("instruction {} is invalid", op.join(".")),
            step,
            op: op.join("."),
        }
    }

    pub fn missing_param(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: format!("malformed instruction {}: parameter is missing", op[0]),
            step,
            op: op.join("."),
        }
    }

    pub fn extra_param(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: format!(
                "malformed instruction {}: too many parameters provided",
                op[0]
            ),
            step,
            op: op.join("."),
        }
    }

    pub fn invalid_param(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: format!(
                "malformed instruction {}: parameter '{}' is invalid",
                op[0], op[1]
            ),
            step,
            op: op.join("."),
        }
    }

    pub fn invalid_param_reason(op: &[&str], step: usize, reason: String) -> AssemblyError {
        AssemblyError {
            message: format!("malformed instruction {}: {}", op[0], reason),
            step,
            op: op.join("."),
        }
    }

    pub fn invalid_block_head(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: format!("invalid block head '{}'", op.join(".")),
            step,
            op: op.join("."),
        }
    }

    pub fn invalid_num_iterations(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: format!(
                "invalid repeat statement '{}': 2 or more iterations must be specified",
                op.join(".")
            ),
            step,
            op: op.join("."),
        }
    }

    pub fn dangling_else(step: usize) -> AssemblyError {
        AssemblyError {
            message: "else without matching if".to_string(),
            step,
            op: String::from("else"),
        }
    }

    pub fn unmatched_block(step: usize) -> AssemblyError {
        AssemblyError {
            message: "block without matching end".to_string(),
            step,
            op: String::from("block"),
        }
    }

    pub fn unmatched_if(step: usize) -> AssemblyError {
        AssemblyError {
            message: "if without matching else/end".to_string(),
            step,
            op: String::from("if.true"),
        }
    }

    pub fn unmatched_while(step: usize) -> AssemblyError {
        AssemblyError {
            message: "while without matching end".to_string(),
            step,
            op: String::from("while.true"),
        }
    }

    pub fn unmatched_repeat(step: usize, op: &[&str]) -> AssemblyError {
        AssemblyError {
            message: "repeat without matching end".to_string(),
            step,
            op: op.join("."),
        }
    }

    pub fn unmatched_else(step: usize) -> AssemblyError {
        AssemblyError {
            message: "else without matching end".to_string(),
            step,
            op: String::from("else"),
        }
    }

    // SCRIPT
    // --------------------------------------------------------------------------------------------

    pub fn missing_begin(step: usize) -> AssemblyError {
        AssemblyError {
            message: "missing script body".to_string(),
            step,
            op: "begin".to_string(),
        }
    }

    pub fn unmatched_begin(step: usize) -> AssemblyError {
        AssemblyError {
            message: "begin without matching end".to_string(),
            step,
            op: "begin".to_string(),
        }
    }

    pub fn dangling_ops_after_script(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: "dangling instructions after script end".to_string(),
            step,
            op: op.join("."),
        }
    }

    // PROCEDURES
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_proc_label(step: usize, label: &str) -> AssemblyError {
        AssemblyError {
            message: format!("duplicate procedure label: {}", label),
            step,
            op: format!("proc.{}", label),
        }
    }

    pub fn invalid_proc_label(label: &str, op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: format!("invalid procedure label: {}", label),
            step,
            op: op.join("."),
        }
    }

    pub fn unmatched_proc(step: usize, label: &str) -> AssemblyError {
        AssemblyError {
            message: "proc without matching end".to_string(),
            step,
            op: format!("proc.{}", label),
        }
    }

    pub fn undefined_proc(step: usize, label: &str) -> AssemblyError {
        AssemblyError {
            message: format!("undefined procedure: {}", label),
            step,
            op: format!("exec.{}", label),
        }
    }

    pub fn dangling_ops_after_proc(op: &[&str], step: usize) -> AssemblyError {
        AssemblyError {
            message: "dangling instructions after procedure end".to_string(),
            step,
            op: op.join("."),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn operation(&self) -> &String {
        &self.op
    }

    pub fn step(&self) -> usize {
        self.step
    }
}

// COMMON TRAIT IMPLEMENTATIONS
// ================================================================================================

impl fmt::Debug for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}
