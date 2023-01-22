use bracket_lib::prelude::*;
use specs::prelude::*;

pub mod camera;
pub mod colors;
pub mod components;
pub mod gui;
pub mod logger;
pub mod map;
pub mod map_builders;
pub mod pathfinding;
pub mod player_action;
pub mod raws;
pub mod rex_assets;
pub mod rng;
pub mod spawner;
pub mod state;
pub mod systems;

use components::*;
use gui::menu::MainMenuSelection;
use logger::builder::Log;
use map::Map;
use state::{new_dispatcher, MapGen, RunState, State};

const RESOURCES_PATH: &str = "resources/";
const FONT_NAME: &str = "Kjammer_16x16.png";
const FONT_SIZE: usize = 16;
const TITLE: &str = "The Last Samouraï";
pub const CONSOLE_WIDTH: usize = 80;
pub const CONSOLE_HEIGHT: usize = 60;
const FPS_CAP: f32 = 30.0;
const SHOW_MAPGEN_VISUALIZER: bool = true;
const FIRST_LEVEL: i32 = 0;

pub struct First {
    run: bool,
}

fn main() {
    let ctx = BTermBuilder::new()
        .with_tile_dimensions(FONT_SIZE, FONT_SIZE)
        .with_dimensions(CONSOLE_WIDTH, CONSOLE_HEIGHT)
        .with_resource_path(RESOURCES_PATH)
        .with_font(FONT_NAME, FONT_SIZE, FONT_SIZE)
        .with_title(TITLE)
        .with_fps_cap(FPS_CAP)
        .with_simple_console(CONSOLE_WIDTH, CONSOLE_HEIGHT, FONT_NAME)
        .build()
        .expect("Context not build.");

    let mut gs = State {
        ecs: World::new(),
        gen: MapGen {
            index: 0,
            timer: 0.0,
            history: Vec::new(),
            next_state: Some(RunState::PreRun),
        },
        dispatcher: new_dispatcher(),
    };

    gs.ecs.register::<Renderable>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<OtherLevelPosition>();
    gs.ecs.register::<ParticleLifetime>();
    gs.ecs.register::<Hidden>();
    gs.ecs.register::<BlocksVisibility>();
    gs.ecs.register::<unit::Player>();
    gs.ecs.register::<unit::Viewshed>();
    gs.ecs.register::<unit::Quips>();
    gs.ecs.register::<unit::EntityMoved>();
    gs.ecs.register::<unit::SufferDamage>();
    gs.ecs.register::<unit::Attribute>();
    gs.ecs.register::<unit::Attributes>();
    gs.ecs.register::<unit::Skills>();
    gs.ecs.register::<unit::Pool>();
    gs.ecs.register::<unit::Pools>();
    gs.ecs.register::<unit::NaturalAttack>();
    gs.ecs.register::<unit::NaturalProperty>();
    gs.ecs.register::<unit::LootTable>();
    gs.ecs.register::<unit::Initiative>();
    gs.ecs.register::<unit::MyTurn>();
    gs.ecs.register::<unit::Faction>();
    gs.ecs.register::<unit::MovementMode>();
    gs.ecs.register::<unit::Chasing>();
    gs.ecs.register::<unit::Vendor>();
    gs.ecs.register::<action::WantsToMelee>();
    gs.ecs.register::<action::WantsToPickupItem>();
    gs.ecs.register::<action::WantsToUseItem>();
    gs.ecs.register::<action::WantsToDropItem>();
    gs.ecs.register::<action::WantsToRemoveItem>();
    gs.ecs.register::<action::WantsToApproach>();
    gs.ecs.register::<action::WantsToFlee>();
    gs.ecs.register::<item::Item>();
    gs.ecs.register::<item::InBackpack>();
    gs.ecs.register::<item::Consumable>();
    gs.ecs.register::<item::Equippable>();
    gs.ecs.register::<item::Equipped>();
    gs.ecs.register::<item::MeleeWeapon>();
    gs.ecs.register::<item::Wearable>();
    gs.ecs.register::<item::EquipmentChanged>();
    gs.ecs.register::<effect::ProvidesHealing>();
    gs.ecs.register::<effect::Ranged>();
    gs.ecs.register::<effect::EntryTrigger>();
    gs.ecs.register::<effect::InflictsDamage>();
    gs.ecs.register::<effect::AreaOfEffect>();
    gs.ecs.register::<effect::Confusion>();
    gs.ecs.register::<props::SingleActivation>();
    gs.ecs.register::<props::Door>();
    gs.ecs.register::<props::LightSource>();

    raws::load_raws();

    let player_entity = spawner::build_player_entity(&mut gs.ecs, 0, 0);

    gs.ecs.insert(rex_assets::RexAssets::new());
    gs.ecs.insert(systems::particle::ParticleBuilder::new());
    gs.ecs.insert(map::master::MasterMap::new());
    gs.ecs.insert(Map::new(1, 0, 0, ""));
    gs.ecs.insert(Point::new(0, 0));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::MainMenu {
        menu_selection: MainMenuSelection::NewGame,
    });
    gs.ecs.insert(First { run: true });

    Log::clear();
    Log::new()
        .append("Welcome to")
        .color(colors::YELLOW1)
        .append("to Rusty Roguelike")
        .build();

    main_loop(ctx, gs).expect("Main loop issue");
}
