use bracket_lib::terminal::Point;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(r: i32) -> Viewshed {
        Viewshed {
            visible_tiles: Vec::new(),
            range: r,
            dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Quips {
    pub available: Vec<String>,
}

#[derive(Component)]
pub struct SufferDamage {
    pub amount: Vec<(i32, bool)>, // is the player doing the damage ?
}

impl SufferDamage {
    pub fn new_damage(
        store: &mut WriteStorage<SufferDamage>,
        victim: Entity,
        amount: i32,
        from_player: bool,
    ) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push((amount, from_player));
        } else {
            let damage = SufferDamage {
                amount: vec![(amount, from_player)],
            };
            store
                .insert(victim, damage)
                .expect("Unable to insert damage");
        }
    }
}

#[derive(Component)]
pub struct EntityMoved {}

#[derive(Component, Clone, Copy)]
pub struct Attribute {
    pub base: i32,
    pub modifiers: i32,
}

impl Attribute {
    pub fn new(base: i32) -> Self {
        Self { base, modifiers: 0 }
    }

    pub fn bonus(self) -> i32 {
        (self.total() - 10) / 2
    }

    pub fn total(self) -> i32 {
        self.base + self.modifiers
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            base: 11,
            modifiers: 0,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Attributes {
    pub might: Attribute,
    pub fitness: Attribute,
    pub quickness: Attribute,
    pub intelligence: Attribute,
    pub level: i32,

    pub total_weight: f32,
    pub total_initiative_penalty: f32,
}

impl Attributes {
    pub fn new(
        might: Attribute,
        fitness: Attribute,
        quickness: Attribute,
        intelligence: Attribute,
        level: i32,
    ) -> Self {
        Self {
            might,
            fitness,
            quickness,
            intelligence,
            level,
            total_weight: 0.0,
            total_initiative_penalty: 0.0,
        }
    }
    pub fn player_max_hp(self) -> i32 {
        10 + (10 + self.fitness.bonus()) * self.level
    }

    pub fn npc_max_hp(self) -> i32 {
        1 + i32::max(1, 8 + self.fitness.bonus()) * self.level
    }

    pub fn max_mana(self) -> i32 {
        i32::max(1, 4 + self.intelligence.bonus()) * self.level
    }

    pub fn max_weight(self) -> f32 {
        self.might.total() as f32 * 2.5
    }

    pub fn initiative_bonus(self) -> i32 {
        f32::floor(self.total_initiative_penalty) as i32 - self.quickness.bonus()
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            might: Attribute::default(),
            fitness: Attribute::default(),
            quickness: Attribute::default(),
            intelligence: Attribute::default(),
            level: 1,
            total_weight: 0.0,
            total_initiative_penalty: 0.0,
        }
    }
}

#[derive(Component, Default)]
pub struct Skills {
    pub melee: i32,
    pub magic: i32,
    pub defense: i32,
}

impl Skills {
    pub fn new(melee: i32, magic: i32, defense: i32) -> Self {
        Self {
            melee,
            magic,
            defense,
        }
    }
}

#[derive(Component)]
pub struct Pool {
    pub max: i32,
    pub current: i32,
}

impl Pool {
    pub fn new(value: i32) -> Self {
        Self {
            max: value,
            current: value,
        }
    }
}

#[derive(Component)]
pub struct Pools {
    pub hit_points: Pool,
    pub mana: Pool,
    pub xp: i32,
    pub money: i32,
    pub god_mode: bool,
}

impl Pools {
    pub fn new_npc(attr: Attributes) -> Self {
        Self {
            money: 0,
            xp: 0,
            hit_points: Pool::new(attr.npc_max_hp()),
            mana: Pool::new(attr.max_mana()),
            god_mode: false,
        }
    }

    pub fn new_player(attr: Attributes) -> Self {
        Self {
            money: 50,
            xp: 0,
            hit_points: Pool::new(attr.player_max_hp()),
            mana: Pool::new(attr.max_mana()),
            god_mode: false,
        }
    }
}

#[derive(Component)]
pub struct NaturalAttack {
    pub name: String,
    pub damage_n_dice: i32,
    pub damage_die_type: i32,
    pub damage_bonus: i32,
    pub hit_bonus: i32,
}

#[derive(Component)]
pub struct NaturalProperty {
    pub armor_class: Option<i32>,
    pub attacks: Vec<NaturalAttack>,
}

#[derive(Component)]
pub struct LootTable {
    pub table: String,
}

#[derive(Component)]
pub struct Initiative {
    pub current: i32,
}

#[derive(Component)]
pub struct MyTurn;

#[derive(Component)]
pub struct Faction {
    pub name: String,
}

#[derive(Clone)]
pub enum Movement {
    Static,
    Random,
    Waypoint { path: Option<Vec<usize>> },
}

#[derive(Component)]
pub struct MovementMode {
    pub mode: Movement,
}

#[derive(Component)]
pub struct Chasing {
    pub target: Entity,
}

#[derive(Component)]
pub struct Vendor {
    pub categories: Vec<String>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VendorMode {
    Buy,
    Sell,
}
