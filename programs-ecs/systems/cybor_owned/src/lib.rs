use bolt_lang::*;
use position::Position;

declare_id!("BmnWUykh35Z99Jz4dSpMBNEF69dsc3rWNSYanJQ59BuZ");

#[system]
pub mod cybor_owned {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let cybors = &mut ctx.accounts.cybors;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub cybor: Cybor,
    }

}
