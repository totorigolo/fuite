use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::DispatcherBuilder,
    assets::{
        PrefabLoaderSystem,
//        HotReloadStrategy,
//        HotReloadSystem,
    },
};

use crate::{
    systems::*,
    states::GamePrefabData,
};

/// Bundle of all the game Systems, to avoid polluting main.rs
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {

//        builder.add(System::default(), "system_name", &[
//            "dependency_1",
//            "dependency_2",
//            "dependency_3",
//        ]);

        builder.add(PrefabLoaderSystem::<GamePrefabData>::default(), "", &[]);

//        builder.add(HotReloadSystem::new(HotReloadStrategy::every(2)), "", &[]);


        builder.add(Fps::default(), "fps_system", &[]);
        builder.add(Text::default(), "text_system", &[]);

        builder.add(PlayerInput::default(), "player_input_system", &[]);
        builder.add(CameraMoveSystem::default(), "camera_system", &[]);

        builder.add(BotMouseAction::default(), "bots_mouse_system", &[
            "player_input_system"
        ]);

        builder.add(BotsRandomHops::default(), "bots_hop_system", &[]);
        builder.add(BadBotsMover::default(), "bots_bm_system", &[]);
        builder.add(BotsActionExecutor::default(), "bots_ae_system", &[
            "bots_bm_system"
        ]);
        builder.add(BotUndertaker::default(), "bots_ut_system", &[]);
        builder.add(RocketProximityDoorway::default(), "rocket_proximity_system", &[]);
        builder.add(RocketTakeOffSystem::default(), "rocket_takeoff_system", &[]);

        builder.add(Physics::default(), "physics_system", &[
            "bots_hop_system",
            "bots_ae_system",
            "bots_ut_system",
            "bots_bm_system",
        ]);

//        use log::*;
//        debug!("{:?}", builder);
        Ok(())
    }
}
