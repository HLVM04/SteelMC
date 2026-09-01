use steel_macros::{ClientPacket, WriteTo};
use steel_registry::packets::play::C_SET_TITLE_TEXT;
use text_components::TextComponent;

#[derive(ClientPacket, WriteTo, Clone, Debug)]
#[packet_id(Play = C_SET_TITLE_TEXT)]
pub struct CSetTitleText {
    pub text: TextComponent,
}
