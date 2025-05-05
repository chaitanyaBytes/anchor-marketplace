use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    /// The public key of the user who created the listing (the seller)
    pub maker: Pubkey,
    /// The mint address of the token being listed
    pub maker_mint: Pubkey,
    /// The price at which the item is being listed (in lamports or smallest unit)
    pub price: u64,
    /// The bump seed used for the generation of this state account
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 * 2 + 8 + 1;
}
