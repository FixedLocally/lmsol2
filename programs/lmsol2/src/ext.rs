use anchor_lang::prelude::*;

pub static MANGO_V3_ID: Pubkey = MANGO_V3_ID_MAINNET;
pub static MANGO_V3_ID_MAINNET: Pubkey = Pubkey::new_from_array([
    11, 129, 136, 217, 110, 11, 207, 49, 238, 37, 70, 198, 50, 87, 144, 157, 211,
    141, 129, 216, 200, 164, 178, 213, 174, 41, 177, 146, 223, 8, 83, 37
]); // "mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68"
pub static MANGO_V3_ID_DEVNET: Pubkey = Pubkey::new_from_array([
    57, 147, 14, 38, 221, 20, 229, 173, 141, 84, 213, 48, 232, 168, 241, 233, 135,
    132, 144, 165, 97, 224, 193, 78, 77, 12, 236, 124, 165, 181, 46, 49
]); // "4skJ85cdxQAFVKbcGgfun8iZPL7BadVYXG3kGEGkufqA"
pub static MSOL_MINT: Pubkey = Pubkey::new_from_array([
    11, 98, 186, 7, 79, 114, 44, 157, 65, 20, 242, 216, 247, 10, 0, 198, 96, 2, 51,
    123, 155, 249, 12, 135, 54, 87, 166, 210, 1, 219, 76, 128
]); // "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"
pub static SOL_MINT: Pubkey = Pubkey::new_from_array([
    6, 155, 136, 87, 254, 171, 129, 132, 251, 104, 127, 99, 70, 24, 192, 53, 218,
    196, 57, 220, 26, 235, 59, 85, 152, 160, 240, 0, 0, 0, 0, 1
]); // "So11111111111111111111111111111111111111112"

#[derive(Clone)]
pub struct MangoV3;

impl anchor_lang::Id for MangoV3 {
    fn id() -> Pubkey {
        MANGO_V3_ID
    }
}
