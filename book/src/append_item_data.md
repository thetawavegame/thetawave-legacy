# Append Data to items.ron

An item entity requires three components:

- ItemComponent
- Hitbox2DComponent
- Motion2DComponent

These components are listed for each item and are stored in assets/data/items.ron. Motion2DComponent and Hitbox2DComponent should remain the same across all items. Respectively, here is what they should look like in items.ron:

```rust
...

motion2d_component: (
    velocity: [0.0, 70.0],
    acceleration: [0.0, 0.0],
    deceleration: [0.0, 0.0],
    max_speed: [0.0, 70.0],
    knockback_max_speed: [0.0, 70.0],
    angular_velocity: 0.0,
    angular_acceleration: 0.0,
    angular_deceleration: 0.0,
),
hitbox_component: (
    width: 14.0,
    height: 14.0,
    offset_x: 0.0,
    offset_y: 0.0,
    offset_rotation: 0.0,
),

...
```

The Motion2DComponent makes the item entity to travel down at a speed of 70. The Hitbox2DComponent makes the item entity have a 14 by 14 pixel hitbox.

The final component, called ItemComponent defines all of the attributes listed in the design section:

- Name
- Sprite
- Price
- Effects

Here is what the full entry for the new "Frequency Augmentor" item should like when appended to the end assets/data/items.ron:

```rust
...

    "frequency_augmentor": (
        item_component: (
            stat_effects: {
                "blast_fire_speed": -0.12,
                "blast_damage": -4,
            },
            price: 8,
            sprite_index: 17,
            name: "frequency_augmentor",
        ),
        motion2d_component: (
            velocity: [0.0, 70.0],
            acceleration: [0.0, 0.0],
            deceleration: [0.0, 0.0],
            max_speed: [0.0, 70.0],
            knockback_max_speed: [0.0, 70.0],
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
        ),
        hitbox_component: (
            width: 14.0,
            height: 14.0,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_rotation: 0.0,
        ),
    ),
}
```
