use anchor_lang::prelude::*;

#[account]
pub struct LmSolState {
    pub owner: Pubkey,
    pub mango_program: Pubkey,
    pub mango_group: Pubkey,
    pub marinade_state: Pubkey,
    pub mango_account: Pubkey,
    pub bump: u8,
}

impl LmSolState {
    pub const SEED: &'static [u8] = b"lmsol";
    pub const LEN: usize = 8 + 32 * 5 + 1;
}