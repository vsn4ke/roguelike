use super::{
    action::{WantsToDropItem, WantsToRemoveItem, WantsToUseItem},
    camera::{render_camera, render_debug_map},
    effect::Ranged,
    gui::{
        cheat::{show_cheat_menu, CheatMenuResult},
        inventory::*,
        menu::*,
        target::ranged_target,
        tooltips::draw_tooltips,
        ui::draw_ui,
    },
    item::Item,
    map::{
        master::level_transition,
        master::{freeze_level_entities, unfreeze_level_entities, MasterMap},
        Map,
    },
    player_action::input,
    raws::{items::spawn_named_item, SpawnType, RAWS},
    spawner::build_player_entity,
    systems::{
        ai::{
            adjacent::AdjacentAI, approaching::ApproachAI, default::DefaultMoveAI,
            encumbrance::EncumbranceSystem, fleeing::FleeAI, initiative::InitiativeSystem,
            quipping::QuipSystem, turn_status::TurnStatusSystem, visible::VisibleAI,
        },
        damage::{delete_the_deads, DamageSystem},
        inventory::*,
        lighting::LightingSystem,
        map_indexing::MapIndexingSystem,
        melee_combat::MeleeCombatSystem,
        particle::{cull_dead_particles, ParticleSpawnSystem},
        trigger::TriggerSystem,
        visibility::VisibilitySystem,
    },
    unit::{Pools, VendorMode},
    First, Log, FIRST_LEVEL, SHOW_MAPGEN_VISUALIZER,
};
mod sub;
use sub::*;

use bracket_lib::terminal::{BTerm, GameState};
use specs::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity },
    MainMenu { menu_selection: MainMenuSelection },
    NextLevel,
    GameOver,
    ShowRemoveItem,
    MapGeneration,
    PreviousLevel,
    ShowCheatMenu,
    Ticking,
    ShowVendor { vendor: Entity, mode: VendorMode },
}

pub fn new_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        .with(MapIndexingSystem {}, "map_index", &[])
        .with(VisibilitySystem {}, "visibility", &[])
        .with(InitiativeSystem {}, "initiative", &[])
        .with(TurnStatusSystem {}, "turn_status", &[])
        .with(AdjacentAI {}, "adjacent", &[])
        .with(VisibleAI {}, "visible", &[])
        .with(ApproachAI {}, "approach", &[])
        .with(FleeAI {}, "flee", &[])
        .with(DefaultMoveAI {}, "default_move", &[])
        .with(MeleeCombatSystem {}, "melee", &[])
        .with(ItemCollectionSystem {}, "pickup", &[])
        .with(ItemUseSystem {}, "use", &[])
        .with(ItemRemoveSystem {}, "remove", &[])
        .with(ItemDropSystem {}, "drop", &[])
        .with(ItemUseSystem {}, "item", &[])
        .with(TriggerSystem {}, "triggers", &[])
        .with(QuipSystem {}, "quipping", &[])
        .with(DamageSystem {}, "damage", &[])
        .with(EncumbranceSystem {}, "encumbrance", &[])
        .with(LightingSystem {}, "lighting", &[])
        .with(ParticleSpawnSystem {}, "particles", &[])
        .build()
}

pub struct State {
    pub ecs: World,
    pub gen: MapGen,
    pub dispatcher: Dispatcher<'static, 'static>,
}

pub struct MapGen {
    pub index: usize,
    pub timer: f32,
    pub history: Vec<Map>,
    pub next_state: Option<RunState>,
}

impl State {
    fn run_systems(&mut self) {
        self.dispatcher.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn cleanup(&mut self) {
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }

        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Deletion failed");
        }

        *self.ecs.write_resource::<Entity>() = build_player_entity(&mut self.ecs, 0, 0);

        Log::clear();
        self.ecs.insert(MasterMap::new());
        self.generate_world_map(FIRST_LEVEL, 0);
    }

    fn goto_level(&mut self, offset: i32) {
        freeze_level_entities(&mut self.ecs);
        let depth = self.ecs.write_resource::<Map>().depth + offset;
        self.generate_world_map(depth, offset);
    }

    fn generate_world_map(&mut self, depth: i32, offset: i32) {
        self.gen.index = 0;
        self.gen.timer = 0.0;
        self.gen.history.clear();

        if let Some(history) = level_transition(&mut self.ecs, depth, offset) {
            self.gen.history = history;
        } else {
            unfreeze_level_entities(&mut self.ecs)
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut newrunstate = *self.ecs.fetch::<RunState>();
        ctx.cls();
        cull_dead_particles(&mut self.ecs, ctx);

        match newrunstate {
            RunState::GameOver => {}
            RunState::MainMenu { .. } => {}
            _ => {
                self.ecs.write_resource::<First>().run = false;
                render_camera(&self.ecs, ctx);
                draw_ui(&self.ecs, ctx);
                draw_tooltips(&self.ecs, ctx);
            }
        }

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = input(self, ctx);
            }
            RunState::Ticking => {
                while newrunstate == RunState::Ticking {
                    self.run_systems();
                    self.ecs.maintain();
                    newrunstate = match *self.ecs.fetch::<RunState>() {
                        RunState::AwaitingInput => RunState::AwaitingInput,
                        _ => RunState::Ticking,
                    }
                }
            }
            RunState::ShowInventory => newrunstate = show_inv_use(self, ctx),
            RunState::ShowDropItem => newrunstate = show_inv_drop(self, ctx),
            RunState::ShowRemoveItem => newrunstate = show_inv_remove(self, ctx),
            RunState::ShowTargeting { range, item } => {
                newrunstate = show_targeting(self, ctx, range, item)
            }
            RunState::MainMenu { .. } => match main_menu(self, ctx) {
                MainMenuResult::NoSelection { selected } => {
                    newrunstate = RunState::MainMenu {
                        menu_selection: selected,
                    }
                }
                MainMenuResult::Selected { selected } => match selected {
                    MainMenuSelection::NewGame => {
                        self.cleanup();
                        newrunstate = RunState::MapGeneration
                    }
                    MainMenuSelection::Continue => {
                        newrunstate = RunState::AwaitingInput;
                    }
                    MainMenuSelection::Quit => {
                        ::std::process::exit(0);
                    }
                },
            },
            RunState::GameOver => match game_over(ctx) {
                GameOverResult::NoSelection => {}
                GameOverResult::QuitToMenu => {
                    self.cleanup();
                    self.ecs.write_resource::<First>().run = true;
                    newrunstate = RunState::MainMenu {
                        menu_selection: MainMenuSelection::NewGame,
                    }
                }
            },
            RunState::MapGeneration => newrunstate = map_generation(&mut self.gen, ctx),
            RunState::NextLevel => {
                self.goto_level(1);
                newrunstate = RunState::PreRun;
            }
            RunState::PreviousLevel => {
                self.goto_level(-1);
                self.gen.next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            }
            RunState::ShowCheatMenu => match show_cheat_menu(ctx) {
                CheatMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                CheatMenuResult::NoResponse => {}
                CheatMenuResult::TeleportToExit => {
                    self.goto_level(1);
                    self.gen.next_state = Some(RunState::PreRun);
                    newrunstate = RunState::MapGeneration;
                }
                CheatMenuResult::Heal => {
                    let mut pools = self.ecs.write_storage::<Pools>();
                    let mut player_pools = pools.get_mut(*self.ecs.fetch::<Entity>()).unwrap();
                    player_pools.hit_points.current = player_pools.hit_points.max;
                    newrunstate = RunState::AwaitingInput;
                }
                CheatMenuResult::RevealMap => {
                    for v in self.ecs.fetch_mut::<Map>().tiles.iter_mut() {
                        v.revealed = true;
                    }
                    newrunstate = RunState::AwaitingInput;
                }
                CheatMenuResult::GodMode => {
                    let mut pools = self.ecs.write_storage::<Pools>();
                    let mut player_pools = pools.get_mut(*self.ecs.fetch::<Entity>()).unwrap();
                    player_pools.god_mode = !player_pools.god_mode;
                    newrunstate = RunState::AwaitingInput;
                }
            },
            RunState::ShowVendor { vendor, mode } => {
                let (result, entity, name, value) = show_vendor_menu(self, ctx, vendor, mode);
                match result {
                    VendorResult::Cancel => newrunstate = RunState::AwaitingInput,
                    VendorResult::NoResponse => {}
                    VendorResult::Sell => sell(&mut self.ecs, entity.unwrap()),
                    VendorResult::Buy => buy(&mut self.ecs, value.unwrap(), &name.unwrap()),
                    VendorResult::BuyMode => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: VendorMode::Buy,
                        }
                    }
                    VendorResult::SellMode => {
                        newrunstate = RunState::ShowVendor {
                            vendor,
                            mode: VendorMode::Sell,
                        }
                    }
                }
            }
        }

        *self.ecs.write_resource::<RunState>() = newrunstate;
        delete_the_deads(&mut self.ecs);
    }
}
