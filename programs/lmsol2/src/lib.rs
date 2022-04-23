use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_lang::solana_program::program::*;
use anchor_lang::solana_program::instruction::*;
use marinade::*;
// use mango::instruction::MangoInstruction;
// use alloc::collections::btree_map::BTreeMap;

mod ext;
mod state;
mod marinade;

use ext::*;
use state::*;

declare_id!("DZV3G6oZw2Qebc7QdcgbC59bKMaaLk9pnBZSXWpG64fd");

pub static MARINADE_ID: Pubkey = Pubkey::new_from_array([
    5, 69, 227, 101, 190, 242, 113, 173, 117, 53, 3, 103, 86, 93, 164, 13, 163, 54, 220, 28, 135,
    155, 177, 84, 138, 122, 252, 197, 90, 169, 57, 30,
]); // "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"

#[program]
pub mod lmsol2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("{}", ctx.accounts.marinade_state.msol_price);
        return ctx.accounts.process(*ctx.bumps.get("lmsol_state").unwrap());
        // Err(error!(Errors::NoError))
    }

    pub fn kill_state(ctx: Context<KillState>) -> Result<()> {
        return ctx.accounts.process();
    }
}

#[derive(Accounts)]
// #[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
    mango_program: Program<'info, MangoV3>,
    token_program: Program<'info, Token>,
    marinade_state: Account<'info, State>,
    #[account(
        init, payer = signer, space = LmSolState::LEN,
        seeds = [LmSolState::SEED], bump
    )]
    lmsol_state: Box<Account<'info, LmSolState>>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID_DEVNET)]
    mango_group: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut)]
    mango_account: AccountInfo<'info>,
}

impl <'info>Initialize<'info> {
    pub fn process(&mut self, state_bump: u8) -> Result<()>{
        // init state and set accounts
        self.lmsol_state.mango_group = *self.mango_group.key;
        self.lmsol_state.mango_program = *self.mango_program.key;
        self.lmsol_state.marinade_state = self.marinade_state.key();
        self.lmsol_state.owner = self.signer.key();
        self.lmsol_state.mango_account = *self.mango_account.key;
        self.lmsol_state.bump = state_bump;
        // self.lmsol_state.ac;

        // create mango account
        let mango_program = &self.mango_program;
        let mut mango_data: Vec<u8> = Vec::new();
        mango_data.extend_from_slice(&Initialize::pack_u32(0x37)); // ix: CreateMangoAccount
        mango_data.extend_from_slice(&Initialize::pack_u64(0x00)); // account 0
        msg!("create mango account");
        let mango_ix = Instruction {
            program_id: mango_program.key(),
            accounts: vec![
                AccountMeta::new(*self.mango_group.key, false),
                AccountMeta::new(*self.mango_account.key, false),
                AccountMeta::new(self.lmsol_state.key(), true),
                AccountMeta::new_readonly(self.system_program.key(), false),
                AccountMeta::new(*self.signer.key, true),
            ],
            data: mango_data,
        };
        let mango_result = invoke_signed(&mango_ix, &[
            self.mango_group.clone(),
            self.mango_account.clone(),
            self.lmsol_state.to_account_infos()[0].clone(),
            self.system_program.to_account_infos()[0].clone(),
            self.signer.to_account_infos()[0].clone(),
        ], &[&[LmSolState::SEED, &[state_bump]]]);
        match mango_result  {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }

        // create token mint
        Ok(())
    }

    pub fn pack_u64(input: u64) -> [u8; 8] {
        let mut pack: [u8; 8] = [0; 8];
        for i in 0..8 {
            pack[i] = ((input & (0xff << (8 * i))) >> (8 * i)) as u8;
        }
        return pack;
    }

    pub fn pack_u32(input: u32) -> [u8; 4] {
        let mut pack: [u8; 4] = [0; 4];
        for i in 0..4 {
            pack[i] = ((input & (0xff << (8 * i))) >> (8 * i)) as u8;
        }
        return pack;
    }
}

#[derive(Accounts)]
pub struct KillState<'info> {
    signer: Signer<'info>,
    #[account(
        mut, seeds = [b"lmsol"], bump, close = signer
    )]
    lmsol_state: Box<Account<'info, LmSolState>>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut)]
    mango_account: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut)]
    mango_group: AccountInfo<'info>,
    mango_program: Program<'info, MangoV3>
}

impl <'info>KillState<'info> {
    pub fn process(&self) -> Result<()> {
        // disallow closing non owned states
        if self.lmsol_state.owner != self.signer.key() {
            return Err(error!(Errors::IncorrectStateOwner))
        }
        if self.lmsol_state.mango_account != *self.mango_account.key {
            return Err(error!(Errors::IncorrectMangoAccountOwner))
        }
        if self.lmsol_state.mango_program != self.mango_program.key() {
            return Err(error!(Errors::IncorrectProgram))
        }
        if self.lmsol_state.mango_group != self.mango_group.key() {
            return Err(error!(Errors::IncorrectProgram))
        }
        // close mango account
        let mut mango_data: Vec<u8> = Vec::new();
        mango_data.extend_from_slice(&Initialize::pack_u32(0x32)); // ix: CloseMangoAccount
        let mango_ix = Instruction {
            program_id: self.mango_program.key(),
            accounts: vec![
                AccountMeta::new(*self.mango_group.key, false),
                AccountMeta::new(*self.mango_account.key, false),
                AccountMeta::new(self.lmsol_state.key(), true),
            ],
            data: mango_data,
        };
        let mango_result = invoke_signed(&mango_ix, &[
            self.mango_group.clone(),
            self.mango_account.clone(),
            self.lmsol_state.to_account_infos()[0].clone(),
        ], &[&[LmSolState::SEED, &[self.lmsol_state.bump]]]);
        match mango_result  {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }
        Ok(())
    }
}

#[error_code]
pub enum Errors {
    #[msg("Lol")]
    NoError,
    #[msg("Incorrect State owner")]
    IncorrectStateOwner,
    #[msg("Incorrect Mango account owner")]
    IncorrectMangoAccountOwner,
    #[msg("Incorrect Program")]
    IncorrectProgram,
}