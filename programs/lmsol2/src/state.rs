use anchor_lang::prelude::*;

#[account]
pub struct LmSolState {
    pub owner: Pubkey,
    pub mango_program: Pubkey,
    pub mango_group: Pubkey,
    pub marinade_state: Pubkey,
    pub mango_account: Pubkey,
    pub lmsol_mint: Pubkey,
    pub bump: u8,
    pub mint_bump: u8,
}

impl LmSolState {
    pub const SEED: &'static [u8] = b"lmsol";
    pub const MINT_SEED: &'static [u8] = b"lmsol_mint1";
    pub const LEN: usize = 8 + 32 * 6 + 1 * 2;
}