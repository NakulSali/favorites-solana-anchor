use anchor_lang::prelude::*;

declare_id!("76YQS4BwiMKXFWrb9NHdV4XTGFgmPGnwT5N9HxS3dwcB");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        colour: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greeting from {}", context.program_id);

        let user_public_key = context.accounts.user.key(); // ❗Fixed: `accounts` not `account`

        msg!(
            "User {}'s favorite number is {}, favorite color is {}, and their hobbies are {:?}",
            user_public_key,
            number,
            colour,
            hobbies
        );

        context.accounts.favorites.set_inner(Favorites {
            number,
            colour,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub colour: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
