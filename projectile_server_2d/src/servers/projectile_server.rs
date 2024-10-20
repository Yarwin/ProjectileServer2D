use crate::constants::CONNECT_ONE_SHOT;
use crate::godot_api::projectile_config::{ProjectileConfig, PropagationType};
use crate::servers::projectile::Projectile;
use godot::classes::{
    Area2D, PhysicsRayQueryParameters2D, PhysicsServer2D, PhysicsShapeQueryParameters2D,
    RenderingServer,
};
use godot::prelude::*;
use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
enum LiveProjectile {
    CastArea(Projectile),
    RayCast(Projectile),
    RigidBody(Projectile),
}

impl LiveProjectile {
    fn projectile(&self) -> &Projectile {
        match self {
            LiveProjectile::CastArea(p)
            | LiveProjectile::RayCast(p)
            | LiveProjectile::RigidBody(p) => p,
        }
    }

    fn projectile_mut(&mut self) -> &mut Projectile {
        match self {
            LiveProjectile::CastArea(p)
            | LiveProjectile::RayCast(p)
            | LiveProjectile::RigidBody(p) => p,
        }
    }

    fn update_cast_area(projectile: &mut Projectile, delta: f64, idx: usize) {
        let Some(mut space_state) =
            PhysicsServer2D::singleton().space_get_direct_state(projectile.space)
        else {
            godot_error!("couldn't access Projectile Space State!");
            return;
        };
        projectile.increment_time(delta);
        projectile.update_current_transform(delta);
        projectile.update(delta);
        let collisions = {
            let projectile_config = &projectile.projectile_config.bind();
            let mut params = PhysicsShapeQueryParameters2D::new_gd();
            params.set_shape_rid(
                projectile_config
                    .collision_shape
                    .as_ref()
                    .expect("CastArea propagation mode requires specifying its shape!")
                    .get_rid(),
            );
            params.set_transform(projectile.transform);
            params.set_collision_mask(projectile_config.collision_mask);
            params.set_collide_with_areas(projectile_config.collide_with_areas);
            params.set_collide_with_bodies(projectile_config.collide_with_bodies);
            space_state.intersect_shape(params)
        };
        for collision in collisions.iter_shared() {
            Self::resolve_collision(projectile, collision);
        }
        Self::draw_projectile(projectile, idx);
    }

    fn update_raycast(projectile: &mut Projectile, delta: f64, idx: usize) {
        let Some(mut space_state) =
            PhysicsServer2D::singleton().space_get_direct_state(projectile.space)
        else {
            godot_error!("couldn't access Projectile Space State!");
            return;
        };
        projectile.increment_time(delta);
        projectile.update_current_transform(delta);
        let vel = projectile.get_current_velocity(delta);
        let cast: Dictionary = {
            let projectile_config_bind = &projectile.projectile_config.bind();
            let mut params = PhysicsRayQueryParameters2D::create(
                projectile.transform.origin,
                projectile.transform.origin + vel,
            )
            .expect("couldn't create PhysicsRayQueryParameters2D!");
            params.set_collision_mask(projectile_config_bind.collision_mask);
            params.set_collide_with_areas(projectile_config_bind.collide_with_areas);
            params.set_collide_with_bodies(projectile_config_bind.collide_with_bodies);
            params.set_exclude(&projectile.exclude);
            space_state.intersect_ray(params)
        };
        projectile.update(delta);
        if !cast.is_empty() {
            Self::resolve_collision(projectile, cast);
        }
        Self::draw_projectile(projectile, idx);
    }

    fn resolve_collision(projectile: &mut Projectile, collision_info: Dictionary) {
        let projectile_config = projectile.projectile_config.clone();
        let caster = projectile.owner.clone();
        if let Some(Ok(area)) = collision_info
            .get("collider")
            .map(|v| v.try_to::<Gd<Area2D>>())
        {
            // note: Dictionaries are RefCounted, so cloning them is fairly cheap
            let should_be_removed = ProjectileConfig::on_area_collided(
                projectile_config,
                area,
                caster,
                collision_info.clone(),
            );
            projectile.is_queued_for_removal = should_be_removed;
        } else if let Some(Ok(body)) = collision_info
            .get("collider")
            .map(|v| v.try_to::<Gd<Node2D>>())
        {
            let should_be_removed = ProjectileConfig::on_body_collided(
                projectile_config,
                body,
                caster,
                collision_info.clone(),
            );
            projectile.is_queued_for_removal = should_be_removed;
        }
    }

    fn draw_projectile(projectile: &mut Projectile, idx: usize) {
        if !projectile.is_queued_for_removal {
            let draw_transform = if !projectile.projectile_config.bind().face_direction {
                let mut t = Transform2D::IDENTITY.scaled(projectile.projectile_config.bind().scale);
                t.origin = projectile.transform.origin;
                t
            } else {
                projectile.transform
            };
            RenderingServer::singleton().canvas_item_set_transform(projectile.rid, draw_transform);
            RenderingServer::singleton().canvas_item_set_draw_index(projectile.rid, idx as i32);
        } else {
            projectile.remove();
            ProjectileConfig::on_projectile_removed(
                projectile.projectile_config.clone(),
                projectile.owner.clone(),
                projectile.transform,
            );
        }
    }

    fn update(&mut self, delta: f64, idx: usize) {
        match self {
            LiveProjectile::CastArea(projectile) => {
                Self::update_cast_area(projectile, delta, idx);
            }
            LiveProjectile::RayCast(projectile) => {
                Self::update_raycast(projectile, delta, idx);
            }
            LiveProjectile::RigidBody(_projectile) => {
                unimplemented!()
            }
        }
    }
}

#[allow(unreachable_patterns)]
impl From<Projectile> for LiveProjectile {
    fn from(value: Projectile) -> Self {
        let prop_type = value.projectile_config.bind().propagation_type;
        match prop_type {
            PropagationType::CastArea => LiveProjectile::CastArea(value),
            PropagationType::RayCast => LiveProjectile::RayCast(value),
            // PropagationType::RigidBody => LiveProjectile::RigidBody(value),
            _ => {
                godot_error!("Given propagation mode has not been yet implemented!");
                unimplemented!()
            }
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ProjectileManager2D {
    /// New projectiles ready to be displayed
    new_projectiles: VecDeque<Projectile>,
    /// Active projectiles.
    // Might be split into two in the future.
    // One part would consist of Arena<Vec<T>> while another would be Hashmap of projectiles Rids and proper Indexes to entities kept in said Vec<T>.
    projectiles: HashMap<Rid, LiveProjectile>,
    base: Base<Node>,
}

#[godot_api]
impl INode for ProjectileManager2D {
    fn physics_process(&mut self, delta: f64) {
        let mut projectiles = std::mem::take(&mut self.projectiles);
        let mut new_projectiles = std::mem::take(&mut self.new_projectiles);
        for new_projectile in new_projectiles.drain(..) {
            new_projectile.configure_display();
            new_projectile.spawn();
            RenderingServer::singleton().canvas_item_transform_physics_interpolation(
                new_projectile.rid,
                new_projectile.transform,
            );
            projectiles.insert(new_projectile.rid, LiveProjectile::from(new_projectile));
        }

        for (idx, (_rid, projectile)) in projectiles.iter_mut().enumerate() {
            projectile.update(delta, idx);
        }
        projectiles.retain(|_r, p| !p.projectile().is_queued_for_removal);
        self.projectiles = projectiles;
    }
}

#[godot_api]
impl ProjectileManager2D {
    /// Spawns new projectile in given World2D's `canvas` and physics `space` at position declared in `transform2d.origin`. Returns [RID] of created projectile.
    #[func]
    fn spawn_new_projectile(
        &mut self,
        projectile_config: Gd<ProjectileConfig>,
        canvas: Rid,
        space: Rid,
        transform2d: Transform2D,
        exclude: Array<Rid>,
        mut caster: Option<Gd<Node2D>>,
    ) -> Rid {
        let projectile = Projectile::new(
            projectile_config,
            canvas,
            space,
            transform2d,
            exclude,
            caster.clone(),
        );
        if let Some(caster) = caster.as_mut() {
            let args = varray![self.to_gd(), projectile.rid];
            let internal_callable =
                Callable::from_fn("on_bullet_owner_removed", |args: &[&Variant]| {
                    let Some(mut this) = args.first().map(|v| v.to::<Gd<ProjectileManager2D>>())
                    else {
                        return Err(());
                    };
                    let mut this_bind = this.bind_mut();
                    let Some(projectile_rid) = args.get(1).map(|v| v.to::<Rid>()) else {
                        return Err(());
                    };
                    let Some(projectile) = this_bind.projectiles.get_mut(&projectile_rid) else {
                        return Err(());
                    };
                    projectile.projectile_mut().owner = None;
                    Ok(Variant::nil())
                })
                .bindv(&args);
            let tree_exiting = StringName::from("tree_exiting");
            caster
                .connect_ex(tree_exiting, internal_callable)
                .flags(CONNECT_ONE_SHOT)
                .done();
        }
        let projectile_rid = projectile.rid;
        self.new_projectiles.push_front(projectile);
        projectile_rid
    }
}