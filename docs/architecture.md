# SystemTactics Architecture

Technical architecture documentation for the SystemTactics tactical RPG project.

## Project Architecture Overview

SystemTactics follows a three-crate workspace design optimized for tactical RPG development with WASM deployment capabilities.

### Workspace Structure

```
system-tactics/
├── shared/          # Common game logic library
├── game/           # Main Bevy game binary (WASM target)
├── tools/          # Development utilities (native binary)
└── docs/           # Technical documentation
```

### Design Rationale

**Shared Library Pattern**: Common game logic (hex grid, combat math, entity definitions) is shared between the main game and development tools, ensuring consistency and reducing code duplication.

**Bevy Game Engine**: Selected for excellent WASM support, ECS architecture suitable for tactical RPGs, and active Rust gamedev community.

**WASM Deployment**: Game compiles to WebAssembly for browser deployment, enabling cross-platform play without native installation.

## Core Systems Architecture

### Hex Grid System (`shared/src/hex_grid.rs`)

**Purpose**: Foundation for all spatial gameplay mechanics
**Key Components**:
- `HexPosition`: Coordinate wrapper around hexx::Hex
- `HexGridConfig`: Layout and rendering configuration
- Distance calculation and neighbor finding
- Future: Pathfinding and movement validation

**Dependencies**: hexx crate for proven hex coordinate mathematics

### Combat System (`shared/src/combat.rs`)

**Purpose**: Tactical combat calculations and mechanics
**Key Components**:
- Damage calculation with critical hits
- Status effects and condition tracking
- Combat resolution and turn order
- Integration with hex grid for positioning

**Design**: Turn-based tactical combat inspired by Final Fantasy Tactics

### Entity System (`shared/src/entities.rs`)

**Purpose**: Game object definitions and character systems
**Key Components**:
- Character definitions with classes and progression
- Item and equipment systems
- Environmental objects and terrain
- Integration with Bevy ECS for game runtime

### God System (`shared/src/gods.rs`)

**Purpose**: Unique divine evolution mechanics
**Key Components**:
- God entities with domains and alignments
- Power level progression and ability trees
- Player choice tracking for evolution
- Divine intervention in tactical combat

## Technology Integration

### Bevy ECS Integration

The shared library provides data structures that integrate seamlessly with Bevy's Entity-Component-System architecture:

- Entities: Characters, items, environmental objects
- Components: Position, health, equipment, divine favor
- Systems: Combat resolution, movement, god evolution

### WASM Optimization

**Build Configuration**:
- Size-optimized release profile (`opt-level = 'z'`)
- Link-time optimization enabled
- Symbol stripping for reduced bundle size
- Panic handling optimized for WASM

**Asset Strategy**:
- Embedded assets using Bevy's asset system
- No external asset loading for WASM builds
- Progressive loading for larger content

### Cross-Platform Considerations

**Native Development**: Full feature set available for rapid iteration
**WASM Deployment**: Optimized subset focusing on core gameplay
**Responsive Design**: Game scales to container size for iframe embedding

## Development Tool Architecture

### Tools Binary (`tools/src/main.rs`)

**Purpose**: Development utilities and content generation
**Key Functions**:
- Map generation and validation
- Character creation and balancing tools
- Combat simulation and testing
- Content export for game binary

**Integration**: Shares core logic with game binary through shared library

## Deployment Architecture

### Web Integration

**Target Platform**: erik-engelhardt.com demos section
**Deployment Path**: `/static/wasm/system-tactics/`
**Integration**: Responsive iframe embedding in website
**Build Output**: WASM binary, JavaScript bindings, HTML wrapper

### Development Workflow

**Local Development**: Native builds for rapid iteration
**Testing**: WASM builds with local server for deployment testing
**Production**: Optimized WASM builds deployed to website

## Documentation Integration

### Obsidian Vault Integration

**Design Documentation**: Strategic game design lives in Obsidian vault
**Technical Documentation**: Implementation details in code repository
**Cross-Reference**: CLAUDE.md maintains links between systems

### AI-Assisted Development

**Phase-Based Development**: Specification → Planning → Execution
**Documentation-Driven**: Design specifications guide implementation
**Continuous Validation**: Code tested against design requirements

## Performance Considerations

### WASM Performance

**Bundle Size**: Target <10MB for reasonable download times
**Runtime Performance**: 60fps target for tactical combat
**Memory Usage**: Efficient data structures for browser constraints
**Loading Time**: Progressive asset loading with meaningful loading states

### Scalability

**Hex Grid**: Optimized for maps up to 50x50 hexes
**Combat Units**: Support for 20+ active units in combat
**God System**: Efficient tracking of player choice history
**Content**: Modular asset system for expandable content

## Security Considerations

**WASM Sandbox**: Game runs in browser security sandbox
**Asset Security**: No external asset loading eliminates injection risks
**Save Data**: Client-side save system with validation
**Network**: No network requirements for core gameplay

## Future Architecture Considerations

### Potential Expansions

**Multiplayer**: Architecture supports future multiplayer additions
**Mobile**: Responsive design enables mobile browser play
**Native**: Shared library enables future native builds
**Modding**: Plugin architecture considerations for content expansion

### Technical Debt Management

**Refactoring**: Modular design enables incremental improvements
**Testing**: Comprehensive test coverage for core systems
**Documentation**: Maintained architecture documentation for future development
**Performance**: Regular profiling and optimization