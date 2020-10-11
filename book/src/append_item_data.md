# Append Data to items.ron

An item entity requires seven components:

- SpriteRender
- ItemComponent
- Hitbox2DComponent
- Motion2DComponent
- Transform
- Transparent
- Name
- AnimationComponent (Optional)

All of the components besides the ItemComponent and optional AnimationComponent are consistent throughtout all of the items, so they do not need to be specified by you when adding an item.

The ItemComponent and optional AnimationComponent do need to be specified in assets/data/items.ron. In the "Frequency Augmentor" example we won't be adding an animation at this point, so we don't need to worry about adding AnimationComponent data to the file. For right now we just need to add the ItemComponent data. This data is all of the information listed in the design section:

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
    ),
}
```

The price is set to 8, because we want the item to cost $8 from the item shop. The name is in snakecase which would be "frequency_augmentor" (this is also the key value in the hashmap). The sprite_index is set to 17 because the art for the item is the 17th sprite to be added to the spritesheet and ron file.

The most involved data to add is the "stat_effects" property. This is a hashmap that maps names of attributes to float values. You can find a list of all of the attributes in the Appendix.
