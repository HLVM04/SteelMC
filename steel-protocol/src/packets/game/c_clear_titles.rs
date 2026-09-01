use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_CLEAR_TITLES;

#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_CLEAR_TITLES)]
pub struct CClearTitles {
    pub reset_times: bool,
}
