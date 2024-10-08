use bolt_lang::*;
use borsh::{BorshSerialize, BorshDeserialize};

declare_id!("DBeSFRbEdviC1VM4dRWMsugypV8D2PoytgSAodKWKKa8");

#[derive(Default, Clone, BorshSerialize, BorshDeserialize)]
pub enum CyborRace {
    #[default]
    Rodriguez,
    Nguyen,
}

impl Space for CyborRace {
    const INIT_SPACE: usize = 1; // Adjust this value as needed
}

#[derive(Default, Clone, BorshSerialize, BorshDeserialize)]
pub struct CyborTemplate {
    pub race_name: String, // Changed from &'static str to String
    pub price: u128,
    pub basic_damage: u32,
    pub basic_hp: u32,
    pub basic_move_speed: u8,
    pub basic_knockdown_hit: u8,
    pub score_per_block: u64,
}

// Implement Space trait for CyborTemplate
impl Space for CyborTemplate {
    const INIT_SPACE: usize = 10; // Adjust this value as needed
}

impl CyborTemplate {
    pub fn get_template(cybor_type: CyborRace) -> Self {
        match cybor_type {
            CyborRace::Rodriguez => Self {
                race_name: "rodriguez".to_string(), // Changed to String
                price: 0,
                basic_damage: 15,
                basic_hp: 1000,
                basic_move_speed: 6,
                basic_knockdown_hit: 4,
                score_per_block: 1,
            },
            CyborRace::Nguyen => Self {
                race_name: "nguyen".to_string(), // Changed to String
                price: 2e+12 as u128,
                basic_damage: 23,
                basic_hp: 1500,
                basic_move_speed: 7,
                basic_knockdown_hit: 1,
                score_per_block: 3,
            },
        }
    }
}

#[component]
#[derive(Default)]
pub struct CyborData {
    pub race: CyborRace,
    pub cybor_template: CyborTemplate,
    pub is_have_finishing_skill: bool,
    pub mint_at: u32,
}