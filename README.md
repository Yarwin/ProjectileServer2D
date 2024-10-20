Projectile Server 2D
===================

Godot4 Gdextension plugin that allows one to instance thousands of projectiles using Godot's Rendering2DServer and Physics2DServer.

## Quickstart


1. Extend the `ProjectileConfig` to specify your callbacks.
```gdscript
extends ProjectileConfig

class_name UserExtendedProjectileConfig


func _on_area_collided(area: Area2D, caster: Node2D, collision_data: Dictionary) -> bool:
	print("Hello Area!")


func _on_body_collided(area: Area2D, caster: Node2D, collision_data: Dictionary) -> bool:
	print("Hello body!")


func _on_projectile_removed(caster: Node2D, projectile_transform: Transform2D) -> bool:
	print("Goodbay cruel world!")
```

2. Create your ProjectileConfig. Specify Trajectory type, Propagation Mode, Display Type and following required resources along all the other data.
3. Spawn your projectile!
```gdscript
    var canvas: RID = self.get_world_2d().canvas
    var space: RID = self.get_world_2d().space
    var projectile_transform: Transform2D = self.current_muzzle.get_global_transform()
    var exclude: Array[RID] = [self.get_rid()]

    ProjectileServer2D.spawn_new_projectile(
        projectile_config, # config
        canvas, # World2D canvas
        space, # World2D physics space
        projectile_transform, 
        exclude,
        self # (optional) caster (used only in callbacks).
    )
```

