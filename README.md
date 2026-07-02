# GalaxyFryerGUI

The graphical client for **GalaxyFryer**, built with [Bevy](https://bevyengine.org) 0.17. It renders the galaxy, the seven planets, and the two explorers, and drives the simulation running in the [`galaxy_fryer`](https://github.com/Advanced-Panic-Programming/GalaxyFryer) backend crate — the game logic itself (Orchestrator, Explorers, Planets) lives entirely in that crate; this repository is presentation and input only.

This is to respect the Bevy constraint that the application needs to run in the main thread.

---

## Architecture

The GUI never touches game state directly. On StartUp it spawns the backend's `Orchestrator` on its own OS thread and talks to it exclusively through two unbounded `crossbeam-channel`s wrapped as Bevy resources:

```
GUI (Bevy, main thread)  <── FromOrchestrator (Receiver<OrchestratorToGUI>) ──  Orchestrator (background thread)
GUI (Bevy, main thread)  ──   ToOrchestrator (Sender<GUIToOrchestrator>)   ──>  Orchestrator (background thread)
```

- `setup_orchestrator::systems::setup_orchestrator` spawns the thread, constructs the two channels, and inserts `ToOrchestrator`/`FromOrchestrator` as resources.
- `communication::systems::receive_from_orchestrator` drains `FromOrchestrator` every frame and turns each `OrchestratorToGUI` variant into a typed Bevy `Message` (`ReceivedPlanetState`, `ReceivedExplorerBag`, `ReceivedPlanetDestroyed`, `ReceivedManualModeAck`, ...). Every other plugin reacts to *these* messages — nothing downstream ever touches the raw channel to avoid race conditions.
- Anything the GUI wants to *do* (send a sunray, move an explorer, switch to automatic mode, ask a planet's state) is expressed as a `GUIToOrchestrator` value sent on `ToOrchestrator`.

---

### Communication
Hybrid communication system:
1) Orchestrator to GUI communication process:
   - Orchestrator sends message on channel
   - Communication module receives it, reads it and generates the corresponding event using Bevy Messages
   - The relative GUI module will listen for that particular event and react to it in the GUI world

2) GUI to Orchestrator communication process:\
   Since the communication is simple and doesn't impact the GUI, I decided to opt for a simpler approach.\
   The GUI element that would cause the event that we need to notify to the orchestrator will be responsible
   for sending it in the channel right away, without going through the communication module.

```The event messages definitions are inside galaxy_view/messages.rs```

---

### App states

The whole UI is one Bevy state machine (`AppState`, in `src/app_states.rs`), driven by `app_state_manager`:

The state flow is:
```
SetupSimulation (default)
        │  SetupSimulationCompleted
        ▼
SetupOrchestrator                 spawns the Orchestrator thread here
        │  SetupOrchestratorCompleted
        ▼
  ──PauseMenu  ◄──────────────────────────────┐
  │     │  PlayPressed                        │ PausePressed (Esc)
  │     ▼                                     │
  │ GalaxyView  ─────────────────────► PlanetView
  │     ▲  GalaxyViewPressed (G)   PlanetViewPressed (1-7, E) │
  │     └──────────────────────────────────────┘
ExitPressed
    ▼
SimulationEnd → AppExit
```

`GalaxyView` and `PlanetView` are the two "gameplay" states — most gameplay plugins (`manual_mode`, `legend`, `log::ui`, `cutscene`) run in both, entering/exiting their UI via `OnEnter`/`OnExit` for each.

Two related but distinct mode flags exist:
- `app_state_manager::CurrentMode` (`Manual`/`Automatic`) — the GUI's own intent, toggled locally by the M/A keys, used to decide what to send when resuming from the pause menu.
- `setup_orchestrator::CurrentOrchestratorMode` — only updated once the orchestrator actually **acknowledges** `ManualModeAck`/`AutomaticModeAck`; this is what the legend and manual-mode panel visibility key off, so the UI reflects reality rather than an optimistic guess.

---

## Modules

| Module               | Responsibility                                                                                                                                                                                                                                                                                                       |
|----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `app_states`         | Defines the `AppState` enum shared by every plugin.                                                                                                                                                                                                                                                                  |
| `app_state_manager`  | Central state-transition hub: turns UI messages (`PlayPressed`, `PausePressed`, `GalaxyViewPressed`, ...) and orchestrator acks into `AppState` transitions. Owns `CurrentMode`.                                                                                                                                     |
| `setup_simulation`   | First state. Spawns the main 2D camera, background, galaxy orbit, and loads/initializes every sprite/resource table (planets, explorers, terrain, rockets, energy cells) used everywhere else.                                                                                                                       |
| `setup_orchestrator` | Spawns the backend `Orchestrator` on a background thread and wires up the two communication channels as resources.                                                                                                                                                                                                   |
| `communication`      | Bridges `FromOrchestrator` into typed Bevy messages consumed by the rest of the app.                                                                                                                                                                                                                                 |
| `input_handler`      | Keyboard shortcuts: `G` galaxy view, `1`-`7` planet view, `E` cycle to the explorer's current planet, `M`/`A` toggle manual/automatic, `P`/`R` pause/resume, `Esc` pause.                                                                                                                                            |
| `galaxy_view`        | The overview screen: orbiting planet sprites, explorer position arrows, galaxy map icon, and the resources (`PlanetsData`, `ExplorersData`, `GalaxyOrbit`) that the rest of the app reads. `update_planets_data`/`update_explorer_data` run in *every* state so data stays fresh even while viewing a single planet. |
| `planet_view`        | The zoomed-in single-planet screen: terrain, rocket, energy cells, and any explorer currently standing there. Full respawn on planet change, targeted patch systems otherwise.                                                                                                                                       |
| `manual_mode`        | The bottom-left control panel (tabs: Galaxy / Explorer 1 / Explorer 2) for manual play — move, generate, combine, start/stop AI, send sunray/asteroid. Hidden while in automatic mode.                                                                                                                               |
| `legend`             | Small always-on-top key-hint panel (top right) reminding the player of the current shortcuts, including whichever of M/A currently applies.                                                                                                                                                                          |
| `log` / `log::ui`    | In-memory ring buffer (`LogStore`, 2000 entries) of game events (acks, deaths, destruction, ...) plus a scrollable on-screen panel to browse them.                                                                                                                                                                   |
| `cutscene`           | Full-screen animated cutscenes for a planet being destroyed or an asteroid being shot down: fade-in, impact, explosion, caption, auto-despawn after ~3s. Queues cutscenes if more than one fires close together.                                                                                                     |
| `pause_menu`         | Also doubles as the main/title menu (animated logo, Play/Exit buttons) shown on `PauseMenu`.                                                                                                                                                                                                                         |
| `simulation_end`     | Terminates the app (`AppExit`) once the orchestrator confirms simulation end.                                                                                                                                                                                                                                        |

---

## Rendering scale: one camera, one design resolution

Every world-space sprite (planets, explorers, terrain, rockets, corner planet, the galaxy map icon, ...) is authored against a fixed **1920×1080 design canvas** (`setup_simulation::utils::DESIGN_WIDTH`/`DESIGN_HEIGHT`) using plain constants — no per-sprite window-size math.

The single 2D camera (`setup_simulation::systems::spawn_camera`) uses `ScalingMode::AutoMin { min_width: DESIGN_WIDTH, min_height: DESIGN_HEIGHT }`: it always shows *at least* that canvas, scaled uniformly (no stretching/distortion) to whatever the real window/monitor is — including native OS fullscreen. The two full-bleed sprites (space background, planet terrain) are the only ones that need to know the real window size, since `AutoMin` can reveal more than 1920×1080 on one axis depending on the real aspect ratio; `visible_world_size()` computes exactly how much they need to grow to cover that extra area, kept in sync on resize by `update_background_size_on_resize` / `update_terrain_size_on_resize`.

`bevy_ui` (the manual-mode panel, legend, log, pause menu) is a separate render layer the camera projection doesn't touch, so it's kept legible on a large screen via Bevy's `UiScale` resource, computed the same way (`ui_scale_for_size`) and kept in sync by `update_ui_scale_on_resize`.

This is the *only* place resize-handling code exists in the whole project — every other system just uses the fixed design-canvas constants.

---

## Assets

`assets/` is organized by category: `backgrounds/`, `planets/` + `destroyedPlanets/`, `explorers/` + `explorer_arrows/`, `rocket/`, `energy_cell/`, `galaxy/`, `effects/`, `objects/`, `fonts/`, `music/`.

Contains all the sprites/sfx of the application.

---

## Dependencies

```toml
bevy = { version = "0.17.2", features = ["mp3"] }
galaxy_fryer = { git = "...GalaxyFryer.git" }   # game logic (Orchestrator/Explorer/Planet)
common-game = { git = "...unitn-ap-2025/common" } # shared protocol types
crossbeam-channel = "0.5.15"                     # GUI <-> Orchestrator channels
```

---

## Running

```
cargo run
```

The window starts windowed, then switches to borderless fullscreen in `PostStartup` (done there rather than at window creation to work around a Bevy/init startup timing issue). From the title screen, press **Play** to start; the orchestrator thread and its channels are created when entering `SetupOrchestrator`, before the title screen's Play button is even shown.
