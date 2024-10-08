use bolt_lang::*;
use cybor::Cybor;

declare_id!("J8AV28cNjdovGC9NDpPmuFmSgkr9XQQvEzz8KivBFwcA");

#[system]
pub mod cybor_upgrade {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let cybor = &mut ctx.accounts.cybor ;
        cybor.template = CyborTemplate::Rodriguez;
        if cybor.experience >= 100 {
            cybor.level += 1;
            cybor.experience -= 100;
        } else {
            panic!("Not enough experience");
        }

        if cybor.template == CyborTemplate::Rodriguez {
            cybor.damage += 10;
            cybor.hp += 100;
            cybor.move_speed += 1;
            cybor.knockdown_hit += 1;
            cybor.score_per_block += 1;
        }
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub cybor: Cybor,
    }

}
