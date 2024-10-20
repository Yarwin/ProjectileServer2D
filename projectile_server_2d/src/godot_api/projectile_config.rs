use godot::classes::{Area2D, Material, Shape2D, Texture2D};
use godot::global::PropertyUsageFlags;
use godot::meta::{ClassName, PropertyInfo};
use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Default, Copy, Clone, PartialEq, Eq, Debug)]
#[godot(via = i32)]
pub enum ProjectileVisualType {
    #[default]
    Texture = 0,
    Rect,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct ProjectileTextureDisplay {
    #[export]
    pub texture: Option<Gd<Texture2D>>,
    #[export]
    pub material: Option<Gd<Material>>,
    #[init(val = Color::WHITE)]
    pub modulate: Color,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct ProjectileRectDisplay {
    #[export]
    #[init(val = 2.)]
    pub width: f64,
    #[export]
    #[init(val = 2.)]
    pub height: f64,
    #[export]
    #[init(val = Color::WHITE)]
    pub modulate: Color,
    #[export]
    #[init(val = Color::WHITE)]
    pub color: Color,
    #[export]
    pub material: Option<Gd<Material>>,
}

#[derive(GodotConvert, Var, Export, Default, Copy, Clone, PartialEq, Eq, Debug)]
#[godot(via = i32)]
pub enum PropagationType {
    CastArea = 0,
    #[default]
    RayCast = 1,
    // RigidBody = 2,
}

#[derive(GodotConvert, Var, Export, Default, Copy, Clone, PartialEq, Eq, Debug)]
#[godot(via = i32)]
pub enum ProjectileTrajectory {
    #[default]
    Straight = 0,
    Sinusoid = 1,
    Curved = 2,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct SinusoidTrajectoryConfig {
    #[export]
    pub(crate) modulator: f64,
    #[export]
    pub(crate) offset: f64,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct CurvedTrajectoryConfig {
    #[export]
    pub(crate) angular_speed: f32,
}

/// A resource that specifies the behaviour of Projectile and controls its callbacks.
///
/// In order to control behaviour and effects of given projectile extend this resource and specify callbacks in [method _on_projectile_removed], [method _on_area_collided] and [method _on_body_collided].
#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub(crate) struct ProjectileConfig {
    // dumb hack (empty docs for group) because I forget to fix this bug in gdext. Oops.
    ///
    // dumb hack before proper groups will be implemented
    #[var(usage_flags = [GROUP, EDITOR, READ_ONLY])]
    trajectory: u32,
    /// Initial speed of the projectile.
    #[export]
    pub speed: f32,
    /// Max speed of the projectile.
    #[export]
    pub max_speed: f32,
    /// Minimal speed of the projectile.
    #[export]
    pub min_speed: f32,
    /// Projectile acceleration, in units/second
    #[export]
    pub acceleration: f32,
    /// Time in seconds after which the projectile will be removed.
    #[export]
    pub lifetime: f64,
    /// Specify behaviour of the projectile.
    /// **Straight** – projectile flies in straight line. Requires no additional config. [br]**Sinusoid** - Projectile will travel in sinusoid path described in [SinusoidTrajectoryConfig]. [br]**Curved** - Projectile will travel in curved path described in [CurvedTrajectoryConfig].
    #[export]
    #[var(get, set = set_trajectory_type)]
    pub trajectory_type: ProjectileTrajectory,
    ///
    #[var(usage_flags = [GROUP, EDITOR, READ_ONLY])]
    physics: u32,
    /// The physics layers this Projectile scans. Projectile can scan one or more of 32 different layers. See also [member CollisionObject2D.collision_layer].
    ///     **Note**: Projectile can detect a contact with [CollisionObject2D] only if given [CollisionObject2D] is in any of the layers that Projectile scans. See Collision layers and masks in the documentation for more information.
    #[export(flags_2d_physics)]
    pub collision_mask: u32,
    /// If `true`, the Projectile will collide with [PhysicsBody2D]s.
    #[export]
    pub collide_with_bodies: bool,
    /// If `true`, the Projectile will collide with [Area2D]s.
    #[export]
    pub collide_with_areas: bool,
    /// Specifies how given projectile will check for collisions.
    /// **CastArea** - Collisions will be detected by casting [member collision_shape] every frame along the path of the projectile. Recommended for slow and big projectiles [br]**Raycast** - collisions will be detected by casting RayCast along the projectile path every frame [br]**RigidBody** - not yet implemented (coming in version 1.1).
    #[export]
    pub propagation_type: PropagationType,
    /// [Shape2D] required for `CastArea` and `RigidBody` Projectile types.
    #[export]
    pub collision_shape: Option<Gd<Shape2D>>,
    ///

    #[var(usage_flags = [GROUP, EDITOR, READ_ONLY])]
    display: u32,
    /// Enables physics interpolation for given projectile. See [member ProjectSettings.physics/common/physics_interpolation] and [member SceneTree.physics_interpolation] for the global setting.
    /// [b]Note:[/b] Interpolation will only be active if both the flag is set [b]and[/b] physics interpolation is enabled within the [SceneTree]
    #[export]
    #[init(val = true)]
    pub physics_interpolation: bool,
    /// If true the projectile will face its target.
    #[export]
    #[init(val = true)]
    pub face_direction: bool,
    /// The projectile's scale. Unscaled value: [code](1, 1)[/code].
    #[export(range = (0.0, 10.0, or_greater))]
    #[init(val = Vector2::ONE)]
    pub scale: Vector2,
    /// Defines how given projectile is being drawn.
    /// **Texture** - draws sprite according to settings in [ProjectileTextureDisplay]. [br]**Rect** - draws white rect according to settings in [ProjectileRectDisplay]
    #[export]
    #[var(get, set = set_display_type)]
    pub display_type: ProjectileVisualType,
    /// Controls the order in which the projectile render. A projectile with a higher Z index will display in front of others canvas items. Must be between [constant RenderingServer.CANVAS_ITEM_Z_MIN] and [constant RenderingServer.CANVAS_ITEM_Z_MAX] (inclusive).
    #[export]
    #[init(val = 0)]
    pub z_index: i32,
    pub texture_display: Option<Gd<ProjectileTextureDisplay>>,
    pub rect_display: Option<Gd<ProjectileRectDisplay>>,
    pub sinusoid_trajectory_config: Option<Gd<SinusoidTrajectoryConfig>>,
    pub curved_trajectory_config: Option<Gd<CurvedTrajectoryConfig>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ProjectileConfig {
    fn get_property(&self, property: StringName) -> Option<Variant> {
        match property.to_string().as_ref() {
            "rect_display" => self.rect_display.clone().map(|v| v.to_variant()),
            "texture_display" => self.texture_display.clone().map(|v| v.to_variant()),
            "sinusoid_trajectory_config" => self
                .sinusoid_trajectory_config
                .clone()
                .map(|v| v.to_variant()),
            "curved_trajectory_config" => self
                .curved_trajectory_config
                .clone()
                .map(|v| v.to_variant()),
            _ => None,
        }
    }

    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        match property.to_string().as_ref() {
            "rect_display" => {
                self.rect_display = Some(value.to());
                true
            }
            "texture_display" => {
                self.texture_display = Some(value.to());
                true
            }
            "sinusoid_trajectory_config" => {
                self.sinusoid_trajectory_config = Some(value.to());
                true
            }
            "curved_trajectory_config" => {
                self.curved_trajectory_config = Some(value.to());
                true
            }
            _ => false,
        }
    }

    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        let mut property_list = Vec::new();
        self.extend_with_display(&mut property_list);
        self.extend_with_trajectory(&mut property_list);
        property_list
    }
}

#[godot_api]
impl ProjectileConfig {
    ///
    #[func(gd_self)]
    fn set_trajectory_type(mut this: Gd<Self>, value: ProjectileTrajectory) {
        let current = this.bind().trajectory_type;

        if current != value {
            this.bind_mut().trajectory_type = value;
            this.notify_property_list_changed();
        }
    }
    ///
    #[func(gd_self)]
    fn set_display_type(mut this: Gd<Self>, value: ProjectileVisualType) {
        let current = this.bind().display_type;
        if current != value {
            this.bind_mut().display_type = value;
            this.notify_property_list_changed();
        }
    }

    /// Called when projectile is being removed.
    #[func(virtual, gd_self)]
    #[allow(unused_variables)]
    pub fn on_projectile_removed(
        this: Gd<Self>,
        caster: Option<Gd<Node2D>>,
        projectile_transform: Transform2D,
    ) {
    }
    /// Called when the received area collides with this projectile. The method must return a boolean value. `true` removes the projectile, while `false` lets it proceed to the next frame.
    /// The structure of collision data depends on selected propagation node. [br] For `RayCast` they are identical to [method PhysicsDirectSpaceState2D.intersect_ray], while `CastArea` is identical to [method PhysicsDirectSpaceState2D.intersect_shape].
    #[func(virtual, gd_self)]
    #[allow(unused_variables)]
    pub fn on_area_collided(
        this: Gd<Self>,
        area: Gd<Area2D>,
        caster: Option<Gd<Node2D>>,
        collision_data: Dictionary,
    ) -> bool {
        true
    }
    /// Called when the received body collides with this projectile. `true` removes the projectile, while `false` lets it proceed to the next frame.
    /// The structure of collision data depends on selected propagation node. [br] For `RayCast` it is identical to [method PhysicsDirectSpaceState2D.intersect_ray], while `CastArea` is identical to [method PhysicsDirectSpaceState2D.intersect_shape].
    #[func(virtual, gd_self)]
    #[allow(unused_variables)]
    pub fn on_body_collided(
        this: Gd<Self>,
        body: Gd<Node2D>,
        caster: Option<Gd<Node2D>>,
        collision_data: Dictionary,
    ) -> bool {
        true
    }
}

impl ProjectileConfig {
    fn create_prop_info<T: Export>(&self, prop: &str, variant_type: VariantType) -> PropertyInfo {
        PropertyInfo {
            variant_type,
            class_name: ClassName::none(),
            property_name: StringName::from(prop),
            hint_info: T::export_hint(),
            usage: PropertyUsageFlags::EDITOR | PropertyUsageFlags::STORAGE,
        }
    }

    fn extend_with_display(&self, property_list: &mut Vec<PropertyInfo>) {
        match self.display_type {
            ProjectileVisualType::Texture => {
                property_list.push(self.create_prop_info::<Gd<ProjectileTextureDisplay>>(
                    "texture_display",
                    VariantType::OBJECT,
                ))
            }
            ProjectileVisualType::Rect => {
                property_list.push(self.create_prop_info::<Gd<ProjectileRectDisplay>>(
                    "rect_display",
                    VariantType::OBJECT,
                ))
            }
        }
    }

    fn extend_with_trajectory(&self, property_list: &mut Vec<PropertyInfo>) {
        match self.trajectory_type {
            // no need for additional config.
            ProjectileTrajectory::Straight => (),
            ProjectileTrajectory::Sinusoid => {
                property_list.push(PropertyInfo::new_group("trajectory config", ""));
                property_list.push(self.create_prop_info::<Gd<SinusoidTrajectoryConfig>>(
                    "sinusoid_trajectory_config",
                    VariantType::OBJECT,
                ));
            }
            ProjectileTrajectory::Curved => {
                property_list.push(PropertyInfo::new_group("trajectory config", ""));
                property_list.push(self.create_prop_info::<Gd<CurvedTrajectoryConfig>>(
                    "curved_trajectory_config",
                    VariantType::OBJECT,
                ));
            }
        }
    }
}
