use anchor_lang::prelude::*;

declare_id!("8nsznYpJoCjvtHBB7E1tYrssTXj79zYrKfFGZydZ23FJ");

#[program]
pub mod cybor_contract_bolt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
