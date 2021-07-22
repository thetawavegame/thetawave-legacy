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

All of the components besides the `ItemComponent` and optional `AnimationComponent`
are consistent throughout all of the items, so they do not need to be specified
by you when adding a new item.

The `ItemComponent` and optional `AnimationComponent` need to be specified in
`assets/data/items.ron`. For the "Frequency Augmentor" example we won't be adding
an animation at this point, so we don't need to worry about adding
`AnimationComponent` data to the file. For right now we just need to add the
`ItemComponent` data. This data is all of the information listed in the design section:

- Name
- Sprite
- Price
- Effects

Here is what the full entry for the new "Frequency Augmentor" item should like
when appended to the end `assets/data/items.ron`:

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

The price is set to 8, because we want the item to cost $8 from the item shop.
The name is "Frequency Augmentor" in snakecase which would be "frequency_augmentor"
(this is also the key in the hashmap). The `sprite_index` is set to 17 because
the art for the item is at index 17 in the spritesheet data file
(assets/texture/item_spritesheet.ron).
If you did not add art for your item to the spritesheet you can set the
`sprite_index` to 0 to use the placeholder art.

The most involved data to add is the "stat_effects" property. This is a hashmap
that maps names of attributes to float values. You can find a list of all these
attributes in [Appendix A](./stat_effects.md). Map the name of the effect you
want the item to make to a float value. For the "Frequency Augmentor" item,
I add  `-0.10` to `blast_fire_speed`, to decrease the delay between blasts. I
also decrease the damage per blast by 4 points of damage by setting
`blast_damage` to `-4`.

If you run the game after following the sections up to this point in [Adding Items](./add_item.md),
your item should work! Here is the "Frequency Augmentor" item in action:

![frequency_augmentor_gif](./assets/frequency_augmentor.gif)

This is the minimum amount of work that needs to be done to in order to add an item
to the game. However, you should continue reading this section if...

- you want your item to have an animation.
- you want to add an item that affects stats not listed in [Appendix A](./item_effects.md).
- you want to add an item with a bool effect.
