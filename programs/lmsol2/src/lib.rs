use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_lang::solana_program::program::*;
use anchor_lang::solana_program::instruction::*;
use anchor_lang::solana_program::system_instruction::create_account;
use fixed::types::I80F48;
use marinade::*;
// use mango::instruction::MangoInstruction;
// use alloc::collections::btree_map::BTreeMap;

mod ext;
mod state;
mod marinade;

use ext::*;
use state::*;
use mango::state::{MangoAccount, MangoGroup, MangoCache};

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
        return ctx.accounts.process(
            *ctx.bumps.get("lmsol_state").unwrap(),
            *ctx.bumps.get("lmsol_mint").unwrap(),
        );
        // Err(error!(Errors::NoError))
    }

    pub fn kill_state(ctx: Context<KillState>) -> Result<()> {
        return ctx.accounts.process();
    }

    pub fn read_mango_account(ctx: Context<ReadMangoAccount>) -> Result<()> {
        return ctx.accounts.process();
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        return ctx.accounts.process(amount);
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
    marinade_state: Box<Account<'info, State>>,
    #[account(
        init, payer = signer, space = LmSolState::LEN,
        seeds = [LmSolState::SEED], bump
    )]
    lmsol_state: Box<Account<'info, LmSolState>>,
    sysvar_rent: Sysvar<'info, Rent>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_group: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut)]
    mango_account: AccountInfo<'info>,
    /// CHECK: we are going to init the mint ourselves
    #[account(mut, seeds = [LmSolState::MINT_SEED], bump)]
    lmsol_mint: AccountInfo<'info>,
}

impl <'info>Initialize<'info> {
    pub fn process(&mut self, state_bump: u8, mint_bump: u8) -> Result<()>{
        // init state and set accounts
        self.lmsol_state.mango_group = *self.mango_group.key;
        self.lmsol_state.mango_program = *self.mango_program.key;
        self.lmsol_state.marinade_state = self.marinade_state.key();
        self.lmsol_state.owner = self.signer.key();
        self.lmsol_state.mango_account = *self.mango_account.key;
        self.lmsol_state.lmsol_mint = self.lmsol_mint.key();
        self.lmsol_state.bump = state_bump;
        self.lmsol_state.mint_bump = mint_bump;

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
        msg!("allocate token mint");
        let allocate_mint_ix = create_account(
            &self.signer.key(),
            self.lmsol_mint.key,
            self.sysvar_rent.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            &self.token_program.key(),
        );
        let allocate_result = invoke_signed(&allocate_mint_ix, &[
            self.signer.to_account_infos()[0].clone(),
            self.lmsol_mint.to_account_infos()[0].clone(),
            self.token_program.to_account_infos()[0].clone(),
        ], &[&[LmSolState::MINT_SEED, &[mint_bump]]]);
        match allocate_result  {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }
        msg!("create token mint");
        let create_mint_ix = spl_token::instruction::initialize_mint(
            &self.token_program.key(),
            self.lmsol_mint.key,
            &self.lmsol_state.key(),
            None, 9)?;
        let init_mint_result = invoke(&create_mint_ix, &[
            // self.token_program.to_account_infos()[0].clone(),
            self.lmsol_mint.clone(),
            // self.lmsol_state.to_account_infos()[0].clone(),
            self.sysvar_rent.to_account_infos()[0].clone(),
        ]);
        match init_mint_result  {
            Ok(_) => {},
            Err(e) => panic!("{}", e)
        }
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
        // disallow touching mango accounts not owned by this state
        if self.lmsol_state.mango_account != *self.mango_account.key {
            return Err(error!(Errors::IncorrectMangoAccountOwner))
        }
        // disallow wrong mango programs
        if self.lmsol_state.mango_program != self.mango_program.key() {
            return Err(error!(Errors::IncorrectProgram))
        }
        // disallow wrong mango groups
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

#[derive(Accounts)]
pub struct ReadMangoAccount<'info> {
    marinade_state: Box<Account<'info, State>>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_account: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_group: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_cache: AccountInfo<'info>,
}

impl <'info>ReadMangoAccount<'info> {
    pub fn process(&self) -> Result<()> {
        let mango_ac_result = MangoAccount::load_checked(&self.mango_account, &MANGO_V3_ID, &self.mango_group.key);
        let mango_group_result = MangoGroup::load_checked(&self.mango_group, &MANGO_V3_ID);
        match mango_ac_result {
            Ok(_) => {}
            Err(e) => {panic!("{}", e)}
        }
        match mango_group_result {
            Ok(_) => {}
            Err(e) => {panic!("{}", e)}
        }
        let ac = mango_ac_result.unwrap();
        let group = mango_group_result.unwrap();
        let cache_result = MangoCache::load_checked(&self.mango_cache, &MANGO_V3_ID, &group);
        match cache_result {
            Ok(_) => {}
            Err(e) => {panic!("{}", e)}
        }
        let cache = cache_result.unwrap();

        let msol_index = group.find_token_index(&MSOL_MINT).unwrap();
        let sol_index = group.find_token_index(&SOL_MINT).unwrap();
        msg!("mango group {}", ac.mango_group);
        msg!("mango deposits {:?} mSOL", ac.deposits[msol_index]);
        msg!("mango borrows {:?} SOL", ac.borrows[sol_index]);
        msg!("msol deposit idx {}", cache.root_bank_cache[msol_index].deposit_index);
        msg!("sol borrow idx {}", cache.root_bank_cache[sol_index].borrow_index);
        let msol = ac.get_native_deposit(&cache.root_bank_cache[msol_index], msol_index).unwrap();
        let sol = ac.get_native_deposit(&cache.root_bank_cache[sol_index], sol_index).unwrap();
        let msol_price = I80F48::from_bits((self.marinade_state.msol_price << 16) as i128);
        msg!("msol balance {:?}", msol);
        msg!("sol balance {:?}", sol);
        msg!("net balance {:?}", msol.checked_mul(msol_price).unwrap().checked_sub(sol).unwrap());
        Ok(())
    }
}

/// deposit an amount of tokens into mango
#[derive(Accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mango_program: Program<'info, MangoV3>,
    token_program: Program<'info, Token>,
    marinade_state: Box<Account<'info, State>>,
    #[account(
        seeds = [LmSolState::SEED], bump
    )]
    lmsol_state: Box<Account<'info, LmSolState>>,
    // the mint is init'd at this point
    #[account(seeds = [LmSolState::MINT_SEED], bump)]
    lmsol_mint: Account<'info, Mint>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(owner = MANGO_V3_ID)]
    mango_group: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_account: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_cache: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_root: AccountInfo<'info>,
    /// CHECK: mango accounts are not anchor accounts
    #[account(mut, owner = MANGO_V3_ID)]
    mango_node: AccountInfo<'info>,
    #[account(mut)]
    bank_msol_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    source_msol_ata: Account<'info, TokenAccount>,
}

impl <'info>DepositTokens<'info> {
    pub fn process(&self, amount: u64) -> Result<()>{
        msg!("depositing {}", amount);
        // get mango account net lamports
        // build approve ix - we need to approve lmsol state to transfer on the user's behalf
        let approve_ctx = CpiContext::new(
            self.token_program.to_account_infos()[0].clone(),
            Approve {
                authority: self.signer.to_account_infos()[0].clone(),
                delegate: self.lmsol_state.to_account_infos()[0].clone(),
                to: self.source_msol_ata.to_account_infos()[0].clone(),
            },
        );
        let approve_result = approve(approve_ctx, amount);
        match approve_result {
            Ok(_) => {}
            Err(e) => {panic!("{}", e)}
        }
        // build deposit ix
        let deposit_ix = mango::instruction::deposit(
            &self.mango_program.key(),
            self.mango_group.key,
            self.mango_account.key,
            &self.lmsol_state.key(),
            self.mango_cache.key,
            self.mango_root.key,
            self.mango_node.key,
            &self.bank_msol_ata.key(),
            &self.source_msol_ata.key(),
            amount
        )?;
        let deposit_result = invoke_signed(
            &deposit_ix,
            &[
                self.mango_program.to_account_infos()[0].clone(),
                self.mango_group.clone(),
                self.mango_account.clone(),
                self.lmsol_state.to_account_infos()[0].clone(),
                self.mango_cache.clone(),
                self.mango_root.clone(),
                self.mango_node.clone(),
                self.bank_msol_ata.to_account_infos()[0].clone(),
                self.source_msol_ata.to_account_infos()[0].clone(),
            ],
            &[&[LmSolState::SEED, &[self.lmsol_state.bump]]]
        );
        match deposit_result {
            Ok(_) => {}
            Err(e) => {panic!("{}", e)}
        }
        // build revoke ix - the state no longer need access to the user's funds
        let revoke_ctx = CpiContext::new(
            self.token_program.to_account_infos()[0].clone(),
            Revoke {
                authority: self.signer.to_account_infos()[0].clone(),
                source: self.source_msol_ata.to_account_infos()[0].clone(),
            },
        );
        let revoke_result = revoke(revoke_ctx);
        match revoke_result {
            Ok(_) => {}
            Err(e) => {panic!("{}", e)}
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