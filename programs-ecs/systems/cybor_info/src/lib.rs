use bolt_lang::*;
use cybor::Cybor;

declare_id!("4GgsADCpvpBZvJkTLnqsmSvPbcKirzeRUvw7zGJLk3Gm");

#[system]
pub mod cybor_info {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let cybor = &mut ctx.accounts.cybor;
        Ok(cybor)
    }

    #[system_input]
    pub struct Components {
        pub cybor: Cybor,
    }

}
