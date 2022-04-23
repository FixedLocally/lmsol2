use anchor_lang::prelude::*;

pub static MANGO_V3_ID: Pubkey = Pubkey::new_from_array([
    11, 129, 136, 217, 110, 11, 207, 49, 238, 37, 70, 198, 50, 87, 144, 157, 211,
    141, 129, 216, 200, 164, 178, 213, 174, 41, 177, 146, 223, 8, 83, 37
]); // "mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68"
pub static MANGO_V3_ID_DEVNET: Pubkey = Pubkey::new_from_array([
    57, 147, 14, 38, 221, 20, 229, 173, 141, 84, 213, 48, 232, 168, 241, 233, 135,
    132, 144, 165, 97, 224, 193, 78, 77, 12, 236, 124, 165, 181, 46, 49
]); // "4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA"

#[derive(Clone)]
pub struct MangoV3;

impl anchor_lang::Id for MangoV3 {
    fn id() -> Pubkey {
        MANGO_V3_ID_DEVNET
    }
}