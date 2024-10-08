use bolt_lang::*;
use cybor::{Cybor, CyborRace};

declare_id!("5BeHWrGiqHNfTggNxRAJ6M7ecoDjJgjHAZ3YwArjBBDG");

#[system]
pub mod cybor_mint {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let cybor = &mut ctx.accounts.cybor;
        cybor.template = CyborTemplate::Rodriguez;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub cybor: Cybor,
    }

}
