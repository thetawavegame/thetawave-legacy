# Space Shooter

# Project Introduction

This game was made with the [Amethyst](https://amethyst.rs/) engine. It is inspired by games like Raiden and The Binding of Isaac. Checkout my development journey on my [youtube](https://www.youtube.com/watch?v=OtN5vL80u4Q&list=PL4Dmmk1VXA5p0tVmxNUr8j0ly5ERbL1hN) channel and my write-up about using Amethyst on my [website](https://micronote.tech/2019/06/Space-Shooter-Game/).

# Game Introduction

In this game you control a spaceship. Your goal is to defend an objective from the incoming invasion of enemy spaceships while not running out of health in the process. You can shoot the enemies with your spaceships blasters or ram into them with your barrel roll dealing damage to yourself in the process. How long can you last?

![Alt text](https://giant.gfycat.com/GreenWindingAltiplanochinchillamouse.gif)

# To Run

Clone this repository and enter `cargo run` in the root of its directory. There is currently no metal integration, so it must be run on Windows or Linux.

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

# MVP Specification

**Inspirations**: Raiden, Binding of Isaac

**Genres**: Space shooter, Fantasy

**Plot**: Under discussion

## Setting

The universe starting at Earth.

## Camera

Stationary, focused on the play arena.

## Player Interaction

Player can move, shoot, barrel roll and use other special abilities and items.

## Game Mechanics

- The goal of the game is to unlock all of the characters and beat all of the bosses for all characters.
- The goal of any given run is to beat a final boss
- The goal of any given level is to prevent enemies from getting to the bottom of the screen, depleting the "defense" bar while not having the player's "health" bar depleted.
- The player loses health if their ship collides with an enemy blast or an enemy spaceship.
- Each enemy type has a set amount of health that must be depleted to destroy them.
- Each level has several phases where each phase has some degree of random generation. (See Level Phases)
- Each level increases in difficulty.
- Meeting certain objectives will lead to unlocks of other characters and items.
- Enemies drop pickups and a currency which will need to be manually picked up by the player.

## Level Phases
A level features multiple phases. See example phase-map below.

Enemies->Mini-game->Shop/Pitstop->Enemies->Boss

In the "Enemies" phase an appropriately themed and appropriately difficult selection of enemies is chosen and total enemy count is determined. Enemies are then spawned at random from the selection and the player must deal with them until the number of enemies spawned reaches the total number of enemies for the level.

The "Mini-game" phase will feature a challenge apart from shooting eneimes. This could be something like a large amount of asteroids spawning which the player will need to dodge.

The "Shop/Pitstop" phase will be a point in the level where the player can spend their currency to buy items and pickups.

The "Boss" phase will feature a super tough enemy that will serve as the toughest challenge in the level. Defeating the boss ends the level.


