use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_SET_TITLES_ANIMATION;

#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_SET_TITLES_ANIMATION)]
pub struct CSetTitlesAnimation {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}
