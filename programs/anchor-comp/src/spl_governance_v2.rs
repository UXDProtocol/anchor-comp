use anchor_lang::prelude::*;
use spl_governance::state::vote_record::Vote;

mod spl_governance_program_id {
    #[cfg(feature = "development")]
    solana_program::declare_id!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");
    #[cfg(feature = "production")]
    solana_program::declare_id!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");
}

#[derive(Clone)]
pub struct SplGovernanceV2;

impl anchor_lang::Id for SplGovernanceV2 {
    fn id() -> Pubkey {
        spl_governance_program_id::ID
    }
}

// Utility function
fn get_pubkey_from_opt_account_info(opt_account_info: Option<&AccountInfo>) -> Option<Pubkey> {
    return match opt_account_info {
        Some(account_info) => Some(*account_info.key),
        None => None,
    };
}

pub fn set_governance_delegate<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SetGovernanceDelegate<'info>>,
) -> Result<()> {
    check_program_account(ctx.program.key)?;

    let mut remaining_accounts_iter = ctx.remaining_accounts.iter();

    let new_governance_delegate = get_pubkey_from_opt_account_info(remaining_accounts_iter.next());

    let ix = spl_governance::instruction::set_governance_delegate(
        &spl_governance_program_id::ID,
        ctx.accounts.governance_authority.key,
        ctx.accounts.governance_realm_account.key,
        ctx.accounts.governing_token_mint.key,
        ctx.accounts.governing_token_owner.key,
        &new_governance_delegate,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn deposit_governing_tokens<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositGoverningTokens<'info>>,
    amount: u64,
) -> Result<()> {
    check_program_account(ctx.program.key)?;

    let ix = spl_governance::instruction::deposit_governing_tokens(
        &spl_governance_program_id::ID,
        ctx.accounts.governance_realm_account.key,
        ctx.accounts.governing_token_source_account.key,
        ctx.accounts.governing_token_owner_account.key,
        ctx.accounts.governing_token_transfer_authority.key,
        ctx.accounts.payer.key,
        amount,
        ctx.accounts.governing_token_mint.key,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn withdraw_governing_tokens<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawGoverningTokens<'info>>,
) -> Result<()> {
    check_program_account(ctx.program.key)?;

    let ix = spl_governance::instruction::withdraw_governing_tokens(
        &spl_governance_program_id::ID,
        ctx.accounts.governance_realm_account.key,
        ctx.accounts.governing_token_destination_account.key,
        ctx.accounts.governing_token_owner_account.key,
        ctx.accounts.governing_token_mint.key,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn cast_vote<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CastVote<'info>>,
    vote: Vote,
) -> Result<()> {
    check_program_account(ctx.program.key)?;

    let mut remaining_accounts_iter = ctx.remaining_accounts.iter();

    let voter_weight_record = get_pubkey_from_opt_account_info(remaining_accounts_iter.next());
    let max_voter_weight_record = get_pubkey_from_opt_account_info(remaining_accounts_iter.next());

    let ix = spl_governance::instruction::cast_vote(
        &spl_governance_program_id::ID,
        ctx.accounts.governance_realm_account.key,
        ctx.accounts.governance.key,
        ctx.accounts.proposal.key,
        ctx.accounts.proposal_owner_record.key,
        ctx.accounts.voter_token_owner_record.key,
        ctx.accounts.governance_authority.key,
        ctx.accounts.governing_token_mint.key,
        ctx.accounts.payer.key,
        voter_weight_record,
        max_voter_weight_record,
        vote,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn relinquish_vote<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RelinquishVote<'info>>,
) -> Result<()> {
    check_program_account(ctx.program.key)?;

    let mut remaining_accounts_iter = ctx.remaining_accounts.iter();

    let governance_authority = get_pubkey_from_opt_account_info(remaining_accounts_iter.next());
    let beneficiary = get_pubkey_from_opt_account_info(remaining_accounts_iter.next());

    let ix = spl_governance::instruction::relinquish_vote(
        &spl_governance_program_id::ID,
        ctx.accounts.governance.key,
        ctx.accounts.proposal.key,
        ctx.accounts.voter_token_owner_record.key,
        ctx.accounts.governing_token_mint.key,
        governance_authority,
        beneficiary,
    );

    solana_program::program::invoke_signed(
        &ix,
        &ToAccountInfos::to_account_infos(&ctx),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct SetGovernanceDelegate<'info> {
    /// CHECK: Spl Governance CPI
    pub governance_authority: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governance_realm_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_mint: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositGoverningTokens<'info> {
    /// CHECK: Spl Governance CPI
    pub governance_realm_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_mint: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_source_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_owner_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_transfer_authority: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub payer: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_holding_address: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawGoverningTokens<'info> {
    /// CHECK: Spl Governance CPI
    pub governance_realm_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_destination_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_owner_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    /// CHECK: Spl Governance CPI
    pub governance: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governance_realm_account: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governance_authority: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub proposal: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub proposal_owner_record: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub voter_token_owner_record: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_authority: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_mint: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RelinquishVote<'info> {
    /// CHECK: Spl Governance CPI
    pub governance: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub proposal: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub voter_token_owner_record: AccountInfo<'info>,
    /// CHECK: Spl Governance CPI
    pub governing_token_mint: AccountInfo<'info>,
}

/// Checks that the supplied program ID is the correct one
pub fn check_program_account(spl_governance_program_id: &Pubkey) -> Result<()> {
    if !spl_governance_program_id.eq(&spl_governance_program_id::ID) {
        return Err(error!(ErrorCode::InvalidProgramId));
    }

    return Ok(());
}
