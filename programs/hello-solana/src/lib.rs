use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

declare_id!("DQ8H7EGgWpxqv6rV9rj2DVWyyRZvGwj8PvRYjyfQG8Qo");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Hello Solana Program",
    project_url: "https://github.com/bobur1/hello-solana",
    contacts: "email:derbibme@gmail.com",
    policy: "https://github.com/bobur1/hello-solana/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/bobur1/hello-solana"
}

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
