[![build](https://github.com/simon0302010/falling/actions/workflows/rust.yml/badge.svg)](https://github.com/simon0302010/falling/actions/workflows/rust.yml)
![Hackatime](https://hackatime-badge.hackclub.com/U08HC7N4JJW/falling)
![Crates.io Total Downloads](https://img.shields.io/crates/d/falling)
![Crates.io Version](https://img.shields.io/crates/v/falling)

# falling

This is a simple 2D game about falling. The user controls a falling player ragdoll and must avoid obstacles. The game is built using Bevy and Rapier2D.

## Demo

https://github.com/user-attachments/assets/0e8d140e-0641-4a03-8fd5-ab57840883ea

## Installation

The game can be installed using cargo:

```bash
cargo install falling
```

You can also download a precompiled binary from [Releases](https://github.com/simon0302010/falling/releases).

## Starting

To run the game execute the following command in your terminal:

```bash
falling
```
> Run the binary if you downloaded it from the releases page.

## Gameplay

The goal of the game is to survive as long as possible while avoiding the obstacles.
If the player collides with an obstacle, the broken body part will turn yellow.
If the same body part collides again, the game is over.
If you hit your head, you lose immediately.
The score increases with the distance fallen.

## Controls
- **Space**: Start the game
- **Left Arrow**: Move left
- **Right Arrow**: Move right
- **R**: Restart the game (Space if game is over)

## License
This project is licensed under the GNU General Public License Version 3. See the [LICENSE](LICENSE) file for details.
