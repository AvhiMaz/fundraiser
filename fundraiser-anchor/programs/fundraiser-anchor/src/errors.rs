use anchor_lang::error_code;

#[error_code]

pub enum FundraiserError {
    #[msg("Target not Completed")]
    FundraisingNotCompleted,
}
