use anchor_lang::prelude::*;

declare_id!("HHPEy5pyj5Lz99eeRgdEq6mBv3Hw653kGBK9RHoPy6uu");

#[program]
pub mod game_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
