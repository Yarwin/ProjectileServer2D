use crate::godot_api::projectile_config::{
    ProjectileConfig, ProjectileTrajectory, ProjectileVisualType,
};
use godot::classes::RenderingServer;
use godot::global::sin;
use godot::prelude::*;

pub(crate) struct Projectile {
    time: f64,
    /// `true` if given bullet is set for removal.
    pub(crate) is_queued_for_removal: bool,
    pub(crate) projectile_config: Gd<ProjectileConfig>,
    pub(crate) canvas: Rid,
    pub(crate) space: Rid,
    pub(crate) owner: Option<Gd<Node2D>>,
    pub transform: Transform2D,
    pub exclude: Array<Rid>,
    pub rid: Rid,
}

impl Projectile {
    pub fn new(
        projectile_config: Gd<ProjectileConfig>,
        canvas: Rid,
        space: Rid,
        transform2d: Transform2D,
        exclude: Array<Rid>,
        owner: Option<Gd<Node2D>>,
    ) -> Self {
        let rid = RenderingServer::singleton().canvas_item_create();
        let transform = transform2d.scaled_local(projectile_config.bind().scale);

        Self {
            time: 0.,
            is_queued_for_removal: false,
            projectile_config,
            canvas,
            space,
            owner,
            transform,
            exclude,
            rid,
        }
    }

    pub fn spawn(&self) {
        let physics_interpolation = self.projectile_config.bind().physics_interpolation;
        let z_index = self.projectile_config.bind().z_index;
        RenderingServer::singleton().canvas_item_set_parent(self.rid, self.canvas);
        RenderingServer::singleton().canvas_item_set_transform(self.rid, self.transform);
        RenderingServer::singleton().canvas_item_set_z_index(self.rid, z_index);
        RenderingServer::singleton().canvas_item_set_interpolated(self.rid, physics_interpolation);
    }

    pub fn configure_display(&self) {
        let config_bind = self.projectile_config.bind();

        match config_bind.display_type {
            ProjectileVisualType::Texture => {
                let Some(texture_display_bind) =
                    config_bind.texture_display.as_ref().map(|c| c.bind())
                else {
                    godot_error!("Given projectile doesn't have any texture display!");
                    return;
                };
                let Some(texture) = texture_display_bind.texture.as_ref() else {
                    return;
                };
                let texture_rect = Rect2::new(-texture.get_size() / 2.0, texture.get_size());
                RenderingServer::singleton().canvas_item_add_texture_rect(
                    self.rid,
                    texture_rect,
                    texture.get_rid(),
                );
                RenderingServer::singleton()
                    .canvas_item_set_modulate(self.rid, texture_display_bind.modulate);
                if let Some(mat) = texture_display_bind.material.as_ref() {
                    RenderingServer::singleton().canvas_item_set_material(self.rid, mat.get_rid());
                }
            }

            ProjectileVisualType::Rect => {
                let Some(rect_display_bind) = config_bind.rect_display.as_ref().map(|c| c.bind())
                else {
                    godot_error!("Given projectile doesn't have any Rect display!");
                    return;
                };
                let rect_size = Vector2::new(
                    rect_display_bind.width as real,
                    rect_display_bind.height as real,
                );
                let rect = Rect2::new(-rect_size / 2., rect_size);
                RenderingServer::singleton().canvas_item_add_rect(
                    self.rid,
                    rect,
                    rect_display_bind.color,
                );
                if let Some(mat) = rect_display_bind.material.as_ref() {
                    RenderingServer::singleton().canvas_item_set_material(self.rid, mat.get_rid());
                }
                RenderingServer::singleton()
                    .canvas_item_set_modulate(self.rid, rect_display_bind.modulate);
            }
        }
    }

    pub fn remove(&mut self) {
        self.is_queued_for_removal = true;
        RenderingServer::singleton().canvas_item_clear(self.rid);
        RenderingServer::singleton().free_rid(self.rid);
    }

    pub fn update_current_transform(&mut self, delta: f64) {
        let config_bind = self.projectile_config.bind();
        match config_bind.trajectory_type {
            ProjectileTrajectory::Straight => (),
            ProjectileTrajectory::Sinusoid => {
                let modulator = config_bind
                    .sinusoid_trajectory_config
                    .as_ref()
                    .map(|c| c.bind().modulator)
                    .unwrap_or(0.);
                let offset = config_bind
                    .sinusoid_trajectory_config
                    .as_ref()
                    .map(|c| c.bind().offset)
                    .unwrap_or(0.);
                let current_angle = sin(self.time * modulator + offset);
                let previous_angle = sin((self.time - delta) * modulator + offset);
                let angle_difference = (current_angle - previous_angle) as f32;
                self.transform = self.transform.rotated_local(angle_difference);
            }
            ProjectileTrajectory::Curved => {
                let angular_speed = config_bind
                    .curved_trajectory_config
                    .as_ref()
                    .map(|c| c.bind().angular_speed)
                    .unwrap_or(1.);
                let current_angle = self.time as f32 * angular_speed;
                let previous_angle = (self.time - delta) as f32 * angular_speed;
                let angle_difference = current_angle - previous_angle;
                self.transform = self.transform.rotated_local(angle_difference);
            }
        }
    }

    pub fn get_current_velocity(&self, delta: f64) -> Vector2 {
        let config_bind = self.projectile_config.bind();
        let current_speed = (config_bind.speed + config_bind.acceleration * self.time as f32)
            .min(config_bind.max_speed)
            .max(config_bind.min_speed);
        self.transform.a * current_speed * delta as f32
    }

    pub fn increment_time(&mut self, delta: f64) {
        self.time += delta;
    }

    /// Checks projectile lifetime and updates its transform
    pub fn update(&mut self, delta: f64) {
        self.transform.origin += self.get_current_velocity(delta);
        if self.projectile_config.bind().lifetime - self.time <= 0. {
            self.is_queued_for_removal = true;
        }
    }
}
