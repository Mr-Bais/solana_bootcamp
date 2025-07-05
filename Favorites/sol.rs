use anchor_lang::prelude::*; // Import commonly used Anchor macros and types

// Declare the unique program ID on the Solana blockchain
declare_id!("4rUFbRpp4g8kNL8k6XHgn65C9gEttNtf3HpnLR6SmeLM");

// Anchor accounts require 8-byte discriminators to identify their type
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;
    // Entry point to the program: saves the user's favorite number, color, and hobbies
    pub fn set_favorites(
        context: Context<SetFavorites>, // Context includes all accounts used in this instruction
        number: u64, // User's favorite number
        color: String, // Favorite color
        hobbies: Vec<String> // List of hobbies
    ) -> Result<()> {
        msg!("Greetings from {} ", context.program_id); // Print the program ID for debug
        let user_public_key = context.accounts.user.key(); // Get the user's public key
        // Log the user's input for debugging
        msg!(
            "User { }'s favorite number is { }, color is { }, and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );
        // Save the data into the 'Favorites' account using set_inner
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)] // Automatically calculates required storage space
pub struct Favorites {
    pub number: u64, // Favorite number

    #[max_len(50)]
    pub color: String, // Favorite color (max 50 chars)

    #[max_len(5, 50)]
    pub hobbies: Vec<String>, // Up to 5 hobbies, each up to 50 chars
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // The user calling the instruction

    #[account(
        init_if_needed, // Create if doesn't exist
        payer = user, // User pays the account rent
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, // Allocate correct space
        seeds = [b"Favorites", user.key().as_ref()], // PDA seed
        bump // Required for PDA
    )]
    pub favorites: Account<'info, Favorites>, // Account to store the favorites

    pub system_program: Program<'info, System>, // System program is needed for account creation
}
