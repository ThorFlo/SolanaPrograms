use anchor_lang::prelude::*;

declare_id!("FpK5U9MEoSxNjJXUn9XFWVMYfc2AVw4KMTUaU4yKB5js");

#[program]
pub mod optimisation {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u64, extra_value: u64, status: u8, counter: u8) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = initial_value.to_be_bytes();
        my_account.sum = 0;
        my_account.extra = extra_value;
        my_account.status = status;
        my_account.counter = counter;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_value: [u8; 8], increment: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        //let mut sum = 0;
        /*for _ in 0..increment {
            sum += 1;
        }*/
        //sum += increment;
        my_account.data = new_value;
        my_account.sum += increment * 2;
        if my_account.status < 10 {
            my_account.status += 1;
        } else {
            my_account.status = 0;
        }
        my_account.counter += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + MyAccount::INIT_SPACE)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct MyAccount {
    pub data: [u8; 8],
    pub sum: u64,
    pub extra: u64,
    pub status: u8,
    pub counter: u8,
    pub authority: Pubkey,
}