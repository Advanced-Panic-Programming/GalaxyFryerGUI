mod app_states;
mod setup_simulation;
mod galaxy_view;
mod resources_manager;
mod legend;
mod planet_view;
mod common_systems;
mod input_handler;

use std::thread;
use bevy::prelude::*;
use crate::galaxy_view::plugin::GalaxyViewPlugin;
use crate::input_handler::plugin::InputHandlerPlugin;
use crate::setup_simulation::plugin::SetupSimulationPlugin;
use crate::planet_view::plugin::PlanetViewPlugin;


fn main() {

    let handle = thread::spawn(|| {
       // orchestrator_code()
        /*
            Todo:
             - terminare thread orchestrator dall'app
             - terminare l'app dal thread orchestrator

             2 varianti di terminazione:

             - Se l'orchestrator vuole terminare, killa i sotto-thread (planet, explorer, ...) e manda messaggio all'app nel main
             L'app termina ed essendo l'orchestrator un thread, viene anch'esso terminato

             - Se decido di terminare dall'app (quit), mando un messaggio al thread dell'orchestrator di terminare, lui uccide i sotto-thread
             e poi termina.
             Quando termina, l'app termina a sua volta

        */
    });

    App::new()
        // ===== Plugins =====
        .add_plugins(SetupSimulationPlugin)
        .add_plugins(GalaxyViewPlugin)
        .add_plugins(PlanetViewPlugin)
        .add_plugins(InputHandlerPlugin)
        // ===== Resources =====
        // .insert_resource(rx)
        // ===== Message Systems =====
        // .add_systems(Update, core_messages_system) // This system handles messages from core and updates shared resources
        .run();
}
