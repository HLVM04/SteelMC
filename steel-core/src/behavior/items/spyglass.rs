//! Spyglass item behavior.

use std::sync::Arc;

use steel_macros::item_behavior;
use steel_registry::item_stack::ItemStack;
use steel_registry::sound_events;

use crate::behavior::{InteractionResult, ItemBehavior, ItemUseAnimation, UseItemContext};
use crate::entity::{Entity, LivingEntity};
use crate::world::World;

const USE_DURATION: i32 = 1200;

/// Vanilla spyglass active-use behavior.
#[item_behavior]
pub struct SpyglassItem;

impl ItemBehavior for SpyglassItem {
    fn use_item(&self, context: &mut UseItemContext) -> InteractionResult {
        context.world.play_sound_at(
            &sound_events::ITEM_SPYGLASS_USE,
            context.player.sound_source(),
            context.player.position(),
            1.0,
            1.0,
            Some(context.player.id()),
        );
        context.player.start_using_item(context.hand);
        InteractionResult::Consume
    }

    fn get_use_animation(&self, _stack: &ItemStack) -> ItemUseAnimation {
        ItemUseAnimation::Spyglass
    }

    fn get_use_duration(&self, _stack: &ItemStack, _user: &dyn LivingEntity) -> i32 {
        USE_DURATION
    }

    fn finish_using(
        &self,
        stack: &mut ItemStack,
        world: &Arc<World>,
        user: &dyn LivingEntity,
    ) -> ItemStack {
        stop_using(world, user);
        stack.copy_with_count(stack.count())
    }

    fn release_using(
        &self,
        _stack: &mut ItemStack,
        world: &Arc<World>,
        user: &dyn LivingEntity,
        _time_left: i32,
    ) -> bool {
        stop_using(world, user);
        true
    }
}

fn stop_using(world: &World, user: &dyn LivingEntity) {
    // Vanilla Player.playSound excludes the source player; Entity.playSound does not.
    if let Some(player) = user.as_player() {
        world.play_sound_at(
            &sound_events::ITEM_SPYGLASS_STOP_USING,
            player.sound_source(),
            player.position(),
            1.0,
            1.0,
            Some(player.id()),
        );
    } else {
        user.play_sound(&sound_events::ITEM_SPYGLASS_STOP_USING, 1.0, 1.0);
    }
}
