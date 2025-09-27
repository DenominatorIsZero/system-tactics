# SystemTactics

A tactical RPG inspired by Final Fantasy Tactics, featuring unique god evolution mechanics and system apocalypse elements. Built in Rust using the Bevy game engine, SystemTactics combines hex-grid tactical combat with deep character customization and dynamic divine power systems.

**Live Demo**: Coming soon at [erik-engelhardt.com/demos/system-tactics](https://erik-engelhardt.com/demos/system-tactics)
**Repository**: [github.com/eren/system-tactics](https://github.com/eren/system-tactics)

## Overview

SystemTactics is a Final Fantasy Tactics-inspired tactical RPG set in a world where reality undergoes dimensional merge, introducing RPG "System" mechanics to Earth. Players lead a guild through survival, territory expansion, and ultimate defense against cosmic threats, with a dynamic god system that evolves based on player choices.

### Key Features

- **Hexagonal Grid Combat**: FFT-inspired tactical battles on hex grids with facing mechanics
- **God Evolution System**: Divine powers that change based on player decisions and worship patterns
- **System Apocalypse Setting**: Earth transformed by interdimensional RPG mechanics
- **Territory Management**: Strategic base-building that supports tactical objectives
- **Deep Character Progression**: Job/class system with race-specific advancement trees
- **Web Deployment**: Playable in browsers via WebAssembly

## Technology Stack

**Game Engine**: Bevy 0.16 (Rust game engine)
**Hex Grid**: hexx crate for coordinate system and pathfinding
**Target Platform**: WebAssembly for browser deployment
**Build System**: Cargo workspace with just command runner
**Documentation**: Integrated with Obsidian knowledge management system

## Quick Start

### Prerequisites

- Rust 1.70+ (MSRV)
- [just](https://github.com/casey/just) command runner: `cargo install just`

### Development Workflow

```bash
# One-time setup (installs WASM target and tools)
just setup

# Run the game natively during development
just run-game

# Run the level editor for map creation
just run-level-editor

# Test WASM build locally
just wasm

# Build web package for deployment
just build-web

# Code quality checks
just check        # Format, lint, and test all code

# See all available commands
just --list
```

## Project Structure

SystemTactics uses a three-crate workspace structure optimized for game development:

```
├── Cargo.toml              # Workspace configuration and dependencies
├── justfile               # Development task automation (setup, build, test)
├── docs/                  # Architecture and project documentation
├── shared/                # Common library for game logic and calculations
├── game/                  # Main Bevy game binary (WASM target)
│   ├── src/main.rs        # 🎯 Game application entry point
├── level-editor/          # Level editor tool (native binary)
│   └── src/main.rs        # Hex map creation and scenario design tool
└── models/                # Generated content files from tools
```

**Key Components:**

- **`shared/`**: Game logic shared between main game and development tools
- **`game/`**: Bevy-based tactical RPG compiled to WASM for web deployment
- **`level-editor/`**: Bevy-based level editor for hex map creation and tactical scenario design
- **`docs/`**: Technical documentation for AI-assisted development

## Game Design Philosophy

### Core Design Pillars

1. **Tactical Depth**: Every decision matters in hex-grid combat with facing, terrain, and positioning
2. **Meaningful Choices**: God evolution reflects player values and strategic decisions
3. **Progressive Complexity**: Simple mechanics that deepen through interaction and mastery
4. **Emergent Narrative**: System apocalypse setting reacts to player choices and god development

### Unique Mechanics

**God Evolution System**: Unlike traditional tactical RPGs, your divine patron evolves based on your decisions:

- Combat choices influence god domains (War, Magic, Nature, Death, Order, Chaos)
- Moral decisions shift god alignment (Good, Neutral, Evil)
- God powers provide unique battlefield abilities and strategic options

**System Apocalypse Integration**: The world transformation provides narrative justification for RPG mechanics:

- Levels, stats, and skills exist in-world as quantified reality
- Territory management represents claiming dimensional stability
- Monster encounters are interdimensional incursions

## Documentation and Design

### AI-Assisted Development

This project follows a systematic three-phase development approach:

1. **Specification Phase**: Game design documented in Obsidian vault
2. **Planning Phase**: Technical implementation planned in code repository
3. **Execution Phase**: Step-by-step implementation with continuous validation

For complete AI assistance guidelines, see `CLAUDE.md` in this repository.

## Web Deployment

SystemTactics is designed for web deployment with responsive iframe embedding:

```html
<!-- Responsive iframe for website integration -->
<iframe src="/demos/system-tactics" width="800" height="600"></iframe>
```

### Deployment Features

- **Responsive Design**: Automatically fits parent container
- **Self-Contained**: All assets embedded in WASM binary
- **Fast Loading**: Optimized bundle size for web performance
- **Cross-Platform**: Runs in modern browsers without plugins

## Development Status

**Current Phase**: Foundation setup and basic structure creation
**MVP Timeline**: 6-12 months for playable tactical combat
**Full Version**: 1-2+ years for complete god evolution and territory systems

### Milestone Roadmap

- **M1**: Playable hex grid combat prototype
- **M2**: Character progression and basic job system
- **M3**: God system integration with battlefield effects
- **M4**: Territory management functionality
- **M5**: End-to-end gameplay loop
- **M6**: MVP feature complete with web deployment

## Contributing

This is primarily a personal learning project for exploring tactical RPG development and AI-assisted game design. However, feedback and suggestions are welcome!

### Development Guidelines

- **Code Quality**: Use `just check` before committing (format + lint + test)
- **Testing**: Validate implementations against design specifications
- **Documentation**: Keep both technical docs and design docs current

## Inspiration and References

**Primary Inspiration**: Final Fantasy Tactics (job system, tactical depth, narrative approach)
**Modern References**: Triangle Strategy, XCOM series, Divinity: Original Sin 2
**Literary Influence**: System apocalypse and LitRPG genres
**Technical Reference**: Rust gamedev community and Bevy examples

## License

MIT License - see `LICENSE` file for details.
