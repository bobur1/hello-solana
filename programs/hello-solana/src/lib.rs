use anchor_lang::prelude::*;

declare_id!("8FLVYw42CfMjcbYuvB6tKz2Ms4Pe6QXoaReqZEoY1NkQ");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, Solana!");
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
