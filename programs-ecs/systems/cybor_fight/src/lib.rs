use bolt_lang::*;
use attack::Attack;
use enemy::Enemy;
use cybor::Cybor;

declare_id!("Ez1aqXdP6YG9JnYvMp8rHuzBAysGzXmhwaE55x7VoGNw");

#[system]
pub mod cybor_fight {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let cybor = &mut ctx.accounts.cybor;
        cybor.hp -= ctx.accounts.attack.damage;
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub cybor: Cybor,
        pub attack: Attack,
        pub enemy: Enemy,
    }

}
