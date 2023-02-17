/// debug commands supported by the debugger
pub enum DebugCommand {
    PlayAll,
    Play(usize),
    RewindAll,
    Rewind(usize),
    PrintState,
    PrintStack,
    PrintStackItem(usize),
    PrintMem,
    PrintMemAddress(u64),
    Clock,
    Quit,
    Help,
}

impl DebugCommand {
    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new DebugCommand created specified command string.
    ///
    /// # Errors
    /// Returns an error if the command cannot be parsed.
    pub fn parse(command: &str) -> Result<Self, String> {
        match command {
            "!next" => Ok(Self::Play(1)),
            "!play" => Ok(Self::PlayAll),
            "!prev" => Ok(Self::Rewind(1)),
            "!rewind" => Ok(Self::RewindAll),
            "!print" => Ok(Self::PrintState),
            "!mem" => Ok(Self::PrintMem),
            "!stack" => Ok(Self::PrintStack),
            "!clock" => Ok(Self::Clock),
            "!quit" => Ok(Self::Quit),
            "!help" => Ok(Self::Help),
            x if x.starts_with("!rewind.") => Self::parse_rewind(x),
            x if x.starts_with("!play.") => Self::parse_play(command),
            x if x.starts_with("!stack[") && x.ends_with("]") => Self::parse_print_stack(x),
            x if x.starts_with("!mem[") && x.ends_with(']') => Self::parse_print_memory(x),
            _ => {
                Err(format!("malformed command - does not match any known command: `{}`", command))
            }
        }
    }

    // HELPERS
    // --------------------------------------------------------------------------------------------

    /// parse play command - !play.num_cycles
    fn parse_play(command: &str) -> Result<Self, String> {
        // parse number of cycles
        let num_cycles = command[6..].parse::<usize>().map_err(|err| {
            format!("malformed command - failed to parse number of cycles: `{}` {}", command, err)
        })?;

        Ok(Self::Play(num_cycles))
    }

    /// parse rewind command - !rewind.num_cycles
    fn parse_rewind(command: &str) -> Result<Self, String> {
        // parse number of cycles
        let num_cycles = command[8..].parse::<usize>().map_err(|err| {
            format!("malformed command - failed to parse number of cycles: `{}` {}", command, err)
        })?;

        Ok(Self::Rewind(num_cycles))
    }

    /// parse print memory command - !mem[address]
    fn parse_print_memory(command: &str) -> Result<Self, String> {
        // parse address
        let address = command[5..command.len() - 1].parse::<u64>().map_err(|err| {
            format!("malformed command - failed to parse address parameter: `{}`  {}", command, err)
        })?;

        Ok(Self::PrintMemAddress(address))
    }

    /// parse print stack command - !stack[index]
    fn parse_print_stack(command: &str) -> Result<Self, String> {
        // parse stack index
        let index = command[7..command.len() - 1].parse::<usize>().map_err(|err| {
            format!("malformed command - failed to parse stack index: `{}` {}", command, err)
        })?;

        Ok(Self::PrintStackItem(index))
    }
}
