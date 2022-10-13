use anchor_lang::prelude::*;

declare_id!("3K6e8zTsXtBatEJf9tDmXNm91PHpEZmHGcNxhrMEiB4F");

#[program]
pub mod onchain_voting_anchor_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, description: String) -> Result<()> {
        ctx.accounts.vote_account.is_open_to_vote = true;
        ctx.accounts.vote_account.description = description;
        Ok(())
    }

    pub fn give_vote(ctx: Context<GiveVote>, vote: VoteType) -> Result<()> {
        match vote {
            VoteType::YES => {
                ctx.accounts.vote_account.yes += 1;
            }
            VoteType::NO => {
                ctx.accounts.vote_account.no += 1;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 566)]
    pub vote_account: Account<'info, Proposal>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Proposal {
    is_open_to_vote: bool,
    description: String,
    yes: u64,
    no: u64,
}

#[derive(Accounts)]
pub struct GiveVote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, Proposal>,
    pub signer: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    YES,
    NO,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid vote type")]
    InvalidVoteType,
}
