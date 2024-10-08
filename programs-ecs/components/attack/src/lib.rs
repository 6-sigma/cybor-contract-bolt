use bolt_lang::*;

declare_id!("68LB1xwpQKPDp3pYKYdMhLYjAbTqap6cuzwxUfVhGDro");

#[component]
#[derive(Default)]
pub struct Attack {
    pub damage: i64,
    pub skill_id: i8,
    pub is_round: bool,
}
