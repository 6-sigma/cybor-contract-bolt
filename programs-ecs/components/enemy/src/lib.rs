use bolt_lang::*;

declare_id!("AyGTcPj9nJZP7McB16S7Uuz5qc14ApBQSUb5q7QHGaoj");

#[component]
#[derive(Default)]
pub struct Enemy {
    pub hp: i64,
    pub damage: i64,
    pub def: i64,
    pub move_speed: i64,
    pub knockdown_hit: i64,
    #[max_len(20)]
    pub description: String,
}
