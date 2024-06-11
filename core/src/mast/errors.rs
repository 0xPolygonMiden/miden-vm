#[derive(Clone, Debug, thiserror::Error)]
pub enum ProgramError {
    #[error("tried to create a program from a MAST forest with no entrypoint")]
    NoEntrypoint,
}
