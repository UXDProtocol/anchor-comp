use anchor_lang::prelude::error;

#[error]
enum ErrorCode {
    #[msg("The provided program does not match the expected program ID for the cluster")]
    WrongProgramId,
}
