use bolt_lang::*;

declare_id!("g1SjuZEWGGihALDnvJSNJ7E5tY8tZWcEzSJv82mS2Bk");


#[component]
#[derive(Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    #[max_len(20)]
    pub description: String,
}
