use super::{
    action::{WantsToDropItem, WantsToRemoveItem, WantsToUseItem},
    camera::{render_camera, render_debug_map},
    effect::Ranged,
    gui::{inventory::*, menu::*, target::ranged_target, tooltips::draw_tooltips, ui::draw_ui},
    map::{
        level_transition,
        master::{freeze_level_entities, unfreeze_level_entities, MasterMap},
        Map,
    },
    player_action::input,
    spawner::build_player_entity,
    systems::{
        ai::{animal::AnimalAI, bystander::BystanderAI, monster::MonsterAI},
        damage::{delete_the_deads, DamageSystem},
        inventory::*,
        map_indexing::MapIndexingSystem,
        melee_combat::MeleeCombatSystem,
        particle::{cull_dead_particles, ParticleSpawnSystem},
        trigger::TriggerSystem,
        visibility::VisibilitySystem,
    },
    Log, SHOW_MAPGEN_VISUALIZER, First
};

use bracket_lib::terminal::{BTerm, GameState};
use specs::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity },
    MainMenu { menu_selection: MainMenuSelection },
    NextLevel,
    GameOver,
    ShowRemoveItem,
    MapGeneration,
    PreviousLevel,
}

pub fn new_dispatcher() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        .with(MapIndexingSystem {}, "map_index", &[])
        .with(VisibilitySystem {}, "visibility", &[])
        .with(MonsterAI {}, "mobs", &[])
        .with(AnimalAI {}, "animals", &[])
        .with(BystanderAI {}, "bystanders", &[])
        .with(MeleeCombatSystem {}, "melee", &[])
        .with(ItemCollectionSystem {}, "pickup", &[])
        .with(ItemUseSystem {}, "use", &[])
        .with(ItemRemoveSystem {}, "remove", &[])
        .with(ItemDropSystem {}, "drop", &[])
        .with(ItemUseSystem {}, "item", &[])
        .with(TriggerSystem {}, "triggers", &[])
        .with(DamageSystem {}, "damage", &[])
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
        self.generate_world_map(0, 0);
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
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let result = show_inventory(self, ctx);
                match result.0 {
                    ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        if let Some(is_ranged) = self.ecs.read_storage::<Ranged>().get(item_entity)
                        {
                            newrunstate = RunState::ShowTargeting {
                                range: is_ranged.range,
                                item: item_entity,
                            };
                        } else {
                            self.ecs
                                .write_storage::<WantsToUseItem>()
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: item_entity,
                                        target: None,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = drop_item_menu(self, ctx);
                match result.0 {
                    ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        self.ecs
                            .write_storage::<WantsToDropItem>()
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToDropItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowRemoveItem => {
                let result = remove_item_menu(self, ctx);
                match result.0 {
                    ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        self.ecs
                            .write_storage::<WantsToRemoveItem>()
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToRemoveItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting { range, item } => {
                let result = ranged_target(self, ctx, range);
                match result.0 {
                    ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Selected => {
                        self.ecs
                            .write_storage::<WantsToUseItem>()
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToUseItem {
                                    item,
                                    target: result.1,
                                },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
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
            RunState::MapGeneration => {
                if !SHOW_MAPGEN_VISUALIZER {
                    newrunstate = self.gen.next_state.unwrap();
                } else {
                    ctx.cls();
                    if self.gen.index < self.gen.history.len() {
                        render_debug_map(&self.gen.history[self.gen.index], ctx);
                    }

                    self.gen.timer += ctx.frame_time_ms;
                    if self.gen.timer > 300.0 {
                        self.gen.timer = 0.0;
                        self.gen.index += 1;
                        if self.gen.index >= self.gen.history.len() {
                            newrunstate = self.gen.next_state.unwrap();
                        }
                    }
                }
            }
            RunState::NextLevel => {
                self.goto_level(1);
                newrunstate = RunState::PreRun;
            }
            RunState::PreviousLevel => {
                self.goto_level(-1);
                self.gen.next_state = Some(RunState::PreRun);
                newrunstate = RunState::MapGeneration;
            }
        }
        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        delete_the_deads(&mut self.ecs);
    }
}
