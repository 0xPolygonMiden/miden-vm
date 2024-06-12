use miette::Diagnostic;

#[derive(Clone, Debug, thiserror::Error, Diagnostic)]
pub enum ProgramError {
    #[error("tried to create a program from a MAST forest with no entrypoint")]
    #[diagnostic()]
    NoEntrypoint,
}
