use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    /// The public key of the marketplace admin (owner/manager)
    pub admin: Pubkey,
    /// The fee percentage (basis points, e.g., 100 = 1%) taken by the marketplace
    pub fee: u16,
    /// The bump seed for the marketplace PDA (Program Derived Address)
    pub bump: u8,
    /// The bump seed for the treasury vault PDA (stores collected fees)
    pub treasury_bump: u8,
    /// The bump seed for the rewards mint PDA (for user rewards)
    pub rewards_bump: u8,
    /// The name of the marketplace (used to uniquely identify it)
    pub name: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 1 * 3 + (4 + 32);
}
