# Space Shooter

This game was made with the [amethyst](https://amethyst.rs/) engine. It is inspired by games like Raiden and The Binding of Isaac. Checkout my development journey on my [youtube](https://www.youtube.com/watch?v=OtN5vL80u4Q&list=PL4Dmmk1VXA5p0tVmxNUr8j0ly5ERbL1hN) channel and my write-up about using Amethyst on my [website](https://micronote.tech/2019/06/Space-Shooter-Game/).

![Alt text](https://giant.gfycat.com/GreenWindingAltiplanochinchillamouse.gif)

# To Run

Clone this repository and enter `cargo run` in the root of its directory.

# Controls

- W: accelerate up
- S: accelerate down
- A: accelerate left
- D: accelerate right
- Space: fire blast
- Left: barrel roll left
- Right: barrel roll right
- Escape: pause

# Items

- Steel Barrel: gives player collision damage immunity while barrel rolling
- Plasma Blast: increases fire rate and damage
- Hazardous Reactor: increases maximum speed
- Warp Thruster: increases acceleration and deceleration

# TODOS

## Core Mechanics
- [x] player health and damage
- [x] enemy pool
- [x] enemy variety
- [x] enemy blasts
- [x] barrel roll blast immunity
- [x] spaceship to enemy collisions
- [ ] enemy to enemy collisions
## Visual and Audio
- [x] health bar
- [x] defense bar
- [x] barrel roll cooldown bar
- [ ] pause menu
- [ ] display resolutions options
- [ ] player movement animation
- [ ] player barrel roll animation
- [ ] damage feedback
## Gameplay
- [x] penalty for letting enemies past you
- [x] health pickups
- [ ] levels with end bosses
- [ ] specific item spawn times
- [ ] mini challenges for rewards or penalties (example: certain ships to let past you unharmed)
- [ ] defence objective
## Items
### Passive 
- [ ] reduce barrel roll cooldown
- [ ] increase max health
- [ ] increase max defense
- [ ] trail behind player that does damage
- [ ] increase knockback of enemies
### Active
- [ ] gravity gun effect with enemies
- [ ] shield
- [ ] speed boost
