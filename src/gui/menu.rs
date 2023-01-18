use bracket_lib::terminal::{BTerm, VirtualKeyCode};

use super::super::{
    colors::*,
    rex_assets::RexAssets,
    state::{RunState, State},
    First,
};

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    Continue,
    Quit,
}

pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub enum GameOverResult {
    NoSelection,
    QuitToMenu,
}

pub fn main_menu(gs: &mut State, ctx: &mut BTerm) -> MainMenuResult {
    let runstate = gs.ecs.fetch::<RunState>();
    let assets = gs.ecs.fetch::<RexAssets>();
    let first = gs.ecs.fetch::<First>();
    ctx.render_xp_sprite(&assets.menu, 0, 0);

    ctx.draw_box_double(24, 18, 31, 10, c(GRAY5), c(GRAY1));
    ctx.print_color_centered(20, c(YELLOW1), c(BLACK), "Rusty Roguelike");

    let mut y = 24;
    if let RunState::MainMenu {
        menu_selection: selection,
    } = *runstate
    {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(y, c(GREEN5), c(BLACK), "> Begin New Game <");
        } else {
            ctx.print_color_centered(y, c(GRAY2), c(BLACK), "Begin New Game");
        }
        y += 1;
        if !first.run {
            if selection == MainMenuSelection::Continue {
                ctx.print_color_centered(y, c(GREEN5), c(BLACK), "> Continue <");
            } else {
                ctx.print_color_centered(y, c(GRAY2), c(BLACK), "Continue");
            }
            y += 1;
        }
        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(y, c(GREEN5), c(BLACK), "> Quit <");
        } else {
            ctx.print_color_centered(y, c(GRAY2), c(BLACK), "Quit");
        }

        match ctx.key {
            None => {
                return MainMenuResult::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    return MainMenuResult::NoSelection {
                        selected: MainMenuSelection::Quit,
                    }
                }
                VirtualKeyCode::Up => {
                    return MainMenuResult::NoSelection {
                        selected: {
                            let mut new = match selection {
                                MainMenuSelection::NewGame => MainMenuSelection::Quit,
                                MainMenuSelection::Continue => MainMenuSelection::NewGame,
                                MainMenuSelection::Quit => MainMenuSelection::Continue,
                            };

                            if new == MainMenuSelection::Continue && first.run {
                                new = MainMenuSelection::NewGame
                            }

                            new
                        },
                    };
                }
                VirtualKeyCode::Down => {
                    return MainMenuResult::NoSelection {
                        selected: {
                            let mut new = match selection {
                                MainMenuSelection::NewGame => MainMenuSelection::Continue,
                                MainMenuSelection::Continue => MainMenuSelection::Quit,
                                MainMenuSelection::Quit => MainMenuSelection::NewGame,
                            };

                            if new == MainMenuSelection::Continue && first.run {
                                new = MainMenuSelection::Quit
                            }

                            new
                        },
                    };
                }
                VirtualKeyCode::Return => {
                    return MainMenuResult::Selected {
                        selected: selection,
                    }
                }
                _ => {
                    return MainMenuResult::NoSelection {
                        selected: selection,
                    }
                }
            },
        }
    }

    MainMenuResult::NoSelection {
        selected: MainMenuSelection::NewGame,
    }
}

pub fn game_over(ctx: &mut BTerm) -> GameOverResult {
    ctx.print_color_centered(15, c(YELLOW1), c(BLACK), "Your journey has ended!");
    ctx.print_color_centered(
        17,
        c(WHITE),
        c(BLACK),
        "One day, we'll tell you all about how you did.",
    );
    ctx.print_color_centered(
        18,
        c(WHITE),
        c(BLACK),
        "That day, sadly, is not in this chapter..",
    );

    ctx.print_color_centered(
        20,
        c(SHALLOWWATERS1),
        c(BLACK),
        "Press any key to return to the menu.",
    );

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu,
    }
}
