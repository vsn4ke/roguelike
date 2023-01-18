use super::{ParticleLifetime, Position, Renderable};
use bracket_lib::terminal::{BTerm, RGB};
use bracket_terminal::FontCharType;
use specs::prelude::*;

pub fn cull_dead_particles(ecs: &mut World, ctx: &BTerm) {
    let mut dead_particles: Vec<Entity> = Vec::new();
    {
        let mut particles = ecs.write_storage::<ParticleLifetime>();
        let entities = ecs.entities();
        for (entity, mut particle) in (&entities, &mut particles).join() {
            particle.lifetime_ms -= ctx.frame_time_ms;
            if particle.lifetime_ms < 0.0 {
                dead_particles.push(entity);
            }
        }
    }
    for dead in dead_particles.iter() {
        ecs.delete_entity(*dead).expect("Particle will not die");
    }
}

struct ParticleRequest {
    x: i32,
    y: i32,
    fg: RGB,
    bg: RGB,
    glyph: FontCharType,
    lifetime: f32,
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {
            requests: Vec::new(),
        }
    }

    pub fn request(
        &mut self,
        x: i32,
        y: i32,
        fg: RGB,
        bg: RGB,
        glyph: FontCharType,
        lifetime: f32,
    ) {
        self.requests.push(ParticleRequest {
            x,
            y,
            fg,
            bg,
            glyph,
            lifetime,
        });
    }
}

pub struct ParticleSpawnSystem {}

impl<'a> System<'a> for ParticleSpawnSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, ParticleLifetime>,
        WriteExpect<'a, ParticleBuilder>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut renderables, mut particles, mut particle_builder) = data;
        for new in particle_builder.requests.iter() {
            let p = entities.create();
            positions
                .insert(p, Position { x: new.x, y: new.y })
                .expect("Unable to insert position");
            renderables
                .insert(
                    p,
                    Renderable {
                        glyph: new.glyph,
                        fg: new.fg,
                        bg: new.bg,
                        render_order: 0,
                    },
                )
                .expect("Unable to insert renderable");
            particles
                .insert(
                    p,
                    ParticleLifetime {
                        lifetime_ms: new.lifetime,
                    },
                )
                .expect("Unable to instert lifetime");
        }

        particle_builder.requests.clear();
    }
}
