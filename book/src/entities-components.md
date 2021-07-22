# Entities and their components

The key to understanding ECS is knowing that an entity is a collection of
components. These components provide bits of functionality to the entity that,
when combined together, make it easier to conceptualize the expected behaviour
of the entity.

There are several
[entities](https://github.com/amethyst/theta-wave/tree/master/src/entities)
and
[components](https://github.com/amethyst/theta-wave/tree/master/src/components)
in Theta Wave. This might make the game's code seem
intimidating and needlessly complex, but being able to break down functionality
of an entity into smaller pieces of data makes it much easier to expand upon
as the game grows.

We will look at three entities present in Theta Wave: `Spaceship`
(refers to the player), `Enemy`
and `Item`. For each one, we can define a set of requirements these entities
should have:

|Component|Description|Entities attached to|
|:---:|:---:|:---:|
|Animation|Is animated|Item, Enemy, Spaceship|
|Blaster|Can fire projectiles (blasts)|Enemy, Spaceship|
|Enemy|Data specific to `Enemy` entities|Enemy|
|Health|Can receive damage|Enemy, Spaceship|
|Hitbox2D|Can collide in 2D space|Item, Enemy, Spaceship|
|Item|Data specific to `Item` entities|Item|
|Motion2D|Can move in 2D space|Item, Enemy, Spaceship|
|Spaceship|Data specific to `Spaceship` entities|Spaceship|
|SpriteRender|Is rendered to screen|Item, Enemy, Spaceship|
|Transform|Has a position, scale, and rotation in the game|Item, Enemy, Spaceship|

Again, this seems like a lot of components for only three entities. But if we keep
in mind that every component is a small piece of data that represents a single
piece of functionality for an entity, then it's easier to have a holistic view of
the entity's behavior.

![Drawing grouping together components per entity](assets/entity-component-drawing.png)
*Drawing that groups together components relevant to each entity: Spaceship,
Enemy, and Item. Made with Excalidraw.*

You might notice that each entity in this example has a component with the same
name. In Theta Wave, functionality that is specific to an entity should
be contained by its own specific component. For example, the `Item` component
has properties that are specific to an `Item` entity:

```rust
pub struct ItemComponent {
    pub price: usize,
    pub stat_effects: HashMap<String, f32>,
    pub bool_effects: HashMap<String, bool>,
    pub sprite_index: usize,
}
```

- `price`: the monetary value required to purchase an `Item` entity
from the store.

- `stat_effects`: numerical properties affected when this item is applied
to another entity (i.e: increasing damage output).

- `bool_effects`: boolean properties affected when this item is applied to another
entity (i.e: grants damage immunity)

- `sprite_index`: location of the sprite to be rendered to the screen.

## Conventions for Components in Space Shooter

When defining a new entity, it is always preferable to reuse existing components.
However, as described above, there will be cases where pieces of functionality
will be specific to an entity. So it's okay to add a new one when necessary.

### Custom Components

If you need to add a new component, make sure to add "Component" to the end of
its name. For example (and as shown above) the "Item" component should be named
`ItemComponent` in its struct definition. This makes it easier to distinguish
between entities, components, and systems that have the same name.

### Tag Components

These are components whose sole purpose is to act as an identifier for entity.
This is particularly useful when filtering for a component that is attached to
a specific entity. For example, if an enemy entity reaches the bottom of the
game window, this causes damage to the player's defense status bar. We only
want to affect the `HealthComponent` associated with the entity with a
`DefenseTag` attached to it, not the `HealthComponent` associated with the
spaceship entity.

If you need to create a tag component, then "Tag" should be added to the end of
its name. For example, `DefenseTag` is a component meant as an identifier and
nothing else. These tag components should be added to `src/components/tags.rs`.

## Conclusion

Having entities and components alone doesn't actually add functionality to the game.
This is where the "Systems" in ECS comes in. In the next sections of this chapter,
we will go over a few of the systems present in Theta Wave.
