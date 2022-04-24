use anchor_lang::prelude::*;
use std::{
    fmt::Display,
};

pub static MARINADE_ID: Pubkey = Pubkey::new_from_array([
    5, 69, 227, 101, 190, 242, 113, 173, 117, 53, 3, 103, 86, 93, 164, 13, 163, 54, 220, 28, 135,
    155, 177, 84, 138, 122, 252, 197, 90, 169, 57, 30,
]); // "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"

#[account("account")]
#[derive(Debug)]
pub struct State {
    pub msol_mint: Pubkey,

    pub admin_authority: Pubkey,

    // Target for withdrawing rent reserve SOLs. Save bot wallet account here
    pub operational_sol_account: Pubkey,
    // treasury - external accounts managed by marinade DAO
    // pub treasury_sol_account: Pubkey,
    pub treasury_msol_account: Pubkey,

    // Bump seeds:
    pub reserve_bump_seed: u8,
    pub msol_mint_authority_bump_seed: u8,

    pub rent_exempt_for_token_acc: u64, // Token-Account For rent exempt

    // fee applied on rewards
    pub reward_fee: Fee,

    pub stake_system: StakeSystem,
    pub validator_system: ValidatorSystem, //includes total_balance = total stake under management

    // sum of all the orders received in this epoch
    // must not be used for stake-unstake amount calculation
    // only for reference
    // epoch_stake_orders: u64,
    // epoch_unstake_orders: u64,
    pub liq_pool: LiqPool,
    pub available_reserve_balance: u64, // reserve_pda.lamports() - self.rent_exempt_for_token_acc. Virtual value (real may be > because of transfers into reserve). Use Update* to align
    pub msol_supply: u64, // Virtual value (may be < because of token burn). Use Update* to align
    // For FE. Don't use it for token amount calculation
    pub msol_price: u64,

    ///count tickets for delayed-unstake
    pub circulating_ticket_count: u64,
    ///total lamports amount of generated and not claimed yet tickets
    pub circulating_ticket_balance: u64,
    pub lent_from_reserve: u64,
    pub min_deposit: u64,
    pub min_withdraw: u64,
    pub staking_sol_cap: u64,

    pub emergency_cooling_down: u64,
}

impl anchor_lang::Owner for State {
    fn owner() -> Pubkey {
        // pub use spl_token::ID is used at the top of the file
        MARINADE_ID
    }
}

// impl anchor_lang::Discriminator for State {
//     fn discriminator() -> [u8; 8] {
//         return [0xd8, 0x92, 0x6b, 0x5e, 0x68, 0x4b, 0xb6, 0xb1];
//     }
// }

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct LiqPool {
    pub lp_mint: Pubkey,
    pub lp_mint_authority_bump_seed: u8,
    pub sol_leg_bump_seed: u8,
    pub msol_leg_authority_bump_seed: u8,
    pub msol_leg: Pubkey,

    //The next 3 values define the SOL/mSOL Liquidity pool fee curve params
    // We assume this pool is always UNBALANCED, there should be more SOL than mSOL 99% of the time
    ///Liquidity target. If the Liquidity reach this amount, the fee reaches lp_min_discount_fee
    pub lp_liquidity_target: u64, // 10_000 SOL initially
    /// Liquidity pool max fee
    pub lp_max_fee: Fee, //3% initially
    /// SOL/mSOL Liquidity pool min fee
    pub lp_min_fee: Fee, //0.3% initially
    /// Treasury cut
    pub treasury_cut: Fee, //2500 => 25% how much of the Liquid unstake fee goes to treasury_msol_account

    pub lp_supply: u64, // virtual lp token supply. May be > real supply because of burning tokens. Use UpdateLiqPool to align it with real value
    pub lent_from_sol_leg: u64,
    pub liquidity_sol_cap: u64,
}


#[derive(Clone, Copy, Debug, Default, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct StakeRecord {
    pub stake_account: Pubkey,
    pub last_update_delegated_lamports: u64,
    pub last_update_epoch: u64,
    pub is_emergency_unstaking: u8, // 1 for cooling down after emergency unstake, 0 otherwise
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct StakeSystem {
    pub stake_list: List,
    //pub last_update_epoch: u64,
    //pub updated_during_last_epoch: u32,
    pub delayed_unstake_cooling_down: u64,
    pub stake_deposit_bump_seed: u8,
    pub stake_withdraw_bump_seed: u8,

    /// set by admin, how much slots before the end of the epoch, stake-delta can start
    pub slots_for_stake_delta: u64,
    /// Marks the start of stake-delta operations, meaning that if somebody starts a delayed-unstake ticket
    /// after this var is set with epoch_num the ticket will have epoch_created = current_epoch+1
    /// (the user must wait one more epoch, because their unstake-delta will be execute in this epoch)
    pub last_stake_delta_epoch: u64,
    pub min_stake: u64, // Minimal stake account delegation
    /// can be set by validator-manager-auth to allow a second run of stake-delta to stake late stakers in the last minute of the epoch
    /// so we maximize user's rewards
    pub extra_stake_delta_runs: u32,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct List {
    pub account: Pubkey,
    pub item_size: u32,
    pub count: u32,
    // For chunked change account
    pub new_account: Pubkey,
    pub copied_count: u32,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct ValidatorSystem {
    pub validator_list: List,
    pub manager_authority: Pubkey,
    pub total_validator_score: u32,
    /// sum of all active lamports staked
    pub total_active_balance: u64,
    /// allow & auto-add validator when a user deposits a stake-account of a non-listed validator
    pub auto_add_validator_enabled: u8,
}

impl ValidatorSystem {
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct Fee {
    pub basis_points: u32,
}

impl Display for Fee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.basis_points as f32 / 100.0)
    }
}
