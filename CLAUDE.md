# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with the SystemTactics tactical RPG project.

## Project Overview

SystemTactics is a tactical RPG inspired by Final Fantasy Tactics, featuring unique god evolution mechanics and system apocalypse elements. Built in Rust using the Bevy game engine, it combines hex-grid tactical combat with deep character customization and dynamic divine power systems.

**Repository**: https://github.com/eren/system-tactics
**Live Demo**: Will be hosted at [erik-engelhardt.com/demos/system-tactics](https://erik-engelhardt.com/demos/system-tactics)

## Documentation Architecture and Workflow

### Primary Documentation Location: Obsidian Vault

**CRITICAL**: The main game design documentation lives in the Obsidian vault, NOT in this code repository.

**Primary Documentation Path**: `/Users/eren/SynologyDrive/Zettelkasten/1 - Notes/1 - Projects/Development/SystemTactics/`

**Key Design Documents**:
- `System Tactics.md` - Main project overview and planning document (ProjectDevelopment class)
- `202509241800 - E - SystemTactics Game Design.md` - Core game mechanics (Evidence class)
- `202509241801 - E - SystemTactics Technical Architecture.md` - Technical implementation details (Evidence class)
- `202509241802 - E - SystemTactics God System.md` - Divine power mechanics (Evidence class)
- `202509241803 - E - SystemTactics Combat System.md` - Tactical combat rules (Evidence class)
- `202509241804 - E - SystemTactics Development Roadmap.md` - Implementation timeline (Evidence class)

### Obsidian Workflow Protocol

**MANDATORY**: Before making ANY changes to the Obsidian vault, you MUST read the vault's CLAUDE.md file:

**Required Reading**: `/Users/eren/SynologyDrive/Zettelkasten/CLAUDE.md`

This file contains:
- Complete metadata structure and class definitions
- YAML frontmatter requirements for all note types
- Proper field formatting (File vs MultiFile vs Multi vs Date)
- Relationship linking protocols
- Naming conventions and folder organization

### Documentation Workflow Rules

1. **Design Changes Priority**: Game design changes go in Obsidian FIRST, code follows design
2. **Evidence Class Usage**: All game design documents use the `Evidence` class with proper YAML frontmatter
3. **Source Attribution**: All Evidence notes must include `source: '[[Erik Engelhardt]]'` in frontmatter
4. **Cross-Reference Protocol**: Always check existing Evidence notes before creating new design documents
5. **Metadata Compliance**: Follow strict YAML frontmatter rules defined in vault's CLAUDE.md
6. **Bidirectional Linking**: Maintain proper `[[Note Title]]` links between related design documents

### Repository Integration Links

**Example Project Template**: `/Users/eren/Projects/example_project/`
- Reference for workspace structure, build system patterns, and AI-assisted development workflow
- Contains proven justfile commands, WASM build configuration, and project organization

**Deployment Target**: `/Users/eren/Projects/rust-website/`
- WASM builds will be deployed here for web hosting at erik-engelhardt.com
- Contains established deployment pipeline and website integration patterns
- Use existing `/static/wasm/` directory structure for game deployment

## Repository Structure

```
system-tactics/
├── Cargo.toml                # Workspace configuration with game-focused dependencies
├── LICENSE                   # MIT license
├── .gitignore               # Comprehensive development environment coverage
├── .cargo/config.toml       # WASM build configuration for game deployment
├── justfile                 # Game development commands and build automation
├── docs/                    # AI-assisted development documentation (code-focused)
│   ├── architecture.md     # Technical architecture and implementation decisions
│   ├── README.md           # Documentation overview and usage guidelines
│   └── projects/           # Project-based development documentation
│       ├── archive/        # Completed implementation specifications and plans
│       └── template/       # Reusable specification and plan templates
├── shared/
│   ├── Cargo.toml          # Common game logic library
│   └── src/
│       ├── lib.rs          # Common exports
│       └── hex_grid.rs     # Hexagonal grid coordinate system and pathfinding
├── game/
│   ├── Cargo.toml          # Main Bevy game binary (WASM target)
│   └── src/
│       └── main.rs         # Bevy application entry point with basic setup
├── level-editor/
│   ├── Cargo.toml          # Level editor binary (native target)
│   └── src/
│       └── main.rs         # Bevy-based hex map creation and scenario design tool
└── models/                 # Generated content files from level editor
```

## AI-Assisted Development Workflow

This project follows a systematic three-phase development process specifically adapted for game development with comprehensive Obsidian integration.

### Documentation Hierarchy

**Strategic Design**: Lives in Obsidian vault (game mechanics, narrative, architecture, god system)
**Implementation Details**: Lives in code repository (technical specs, API docs, system implementations)
**Cross-Integration**: This CLAUDE.md ensures seamless workflow between both systems

### Three-Phase Development Process

#### Phase 1: Specification (Obsidian-First)
1. **Design Documentation**: Create/update Evidence notes in Obsidian vault following proper metadata structure
2. **Game Mechanics Definition**: Define tactical systems, god evolution, character progression in Evidence notes
3. **Cross-Reference Validation**: Check existing design documents and ensure proper linking
4. **Design Approval**: Ensure game design coherence before moving to implementation

Requirements for specification completion:
- All game mechanics documented in Obsidian Evidence notes
- Proper YAML frontmatter with source attribution to `'[[Erik Engelhardt]]'`
- Cross-links between related design concepts
- Technical constraints identified and documented

#### Phase 2: Planning (Repository-Based)
1. **Task Breakdown**: Decompose features into discrete, implementable todos in code repository
2. **Technical Architecture**: Plan code structure based on Obsidian design specifications
3. **Dependency Mapping**: Identify implementation dependencies and optimal sequencing
4. **WASM Considerations**: Plan for web deployment and performance requirements

Plans should contain:
- Numbered, sequential tasks referencing specific Obsidian design documents
- Clear technical implementation approach for each game system
- Commit points that represent stable, testable game states
- Integration strategy with existing rust-website deployment

#### Phase 3: Execution (Code Implementation)
1. **Systematic Implementation**: Work through planned todos with frequent reference to Obsidian design
2. **Design-Code Sync**: Update Obsidian project status as implementation progresses
3. **Continuous Validation**: Test against design specifications in Obsidian
4. **Documentation Updates**: Keep both code docs and Obsidian project tracking current

### Commit Strategy

#### Commit Message Conventions

Follow this format for SystemTactics development commits:

```
[PHASE] Brief description of change

- Specific changes made
- Reference to Obsidian design docs
- Any deviations from original design

Refs: Obsidian vault design documents, docs/projects/ files
```

**IMPORTANT**: Do NOT include "Generated with Claude Code" footers or "Co-Authored-By: Claude" lines in commit messages.

Examples:
```
[SPEC] Define hex grid tactical combat mechanics

- Updated SystemTactics Combat System Evidence note
- Added movement rules, facing mechanics, and terrain effects
- Defined action point system and turn order resolution

Refs: 202509241803 - E - SystemTactics Combat System.md
```

```
[PLAN] Break down hex grid implementation

- Created technical implementation plan for hex coordinate system
- Identified hexx crate integration requirements
- Planned pathfinding and movement validation systems

Refs: 202509241801 - E - SystemTactics Technical Architecture.md, docs/projects/hex-grid/plan.md
```

```
[IMPL] Add basic hex grid coordinate system

- Implemented HexPosition with distance calculation
- Added hex grid configuration and layout
- Created placeholder for pathfinding algorithms
- Tests pass for coordinate math operations

Refs: shared/src/hex_grid.rs, 202509241803 - E - SystemTactics Combat System.md
```

## Development Commands

The project uses a `justfile` adapted for tactical RPG development. All commands should be run from the repository root.

### Setup and Prerequisites

```bash
# One-time setup: Install WASM target and development tools
just setup

# Install just command runner if not already installed
cargo install just

# See all available commands
just --list
```

### Game Development Workflow

```bash
# Run the game natively during development (fastest iteration)
just run-game

# Run the level editor for hex map creation and scenario design
just run-level-editor

# Test WASM build locally with development server
just wasm

# Test WASM release build locally
just wasm-release
```

### Build Commands

```bash
# Build all workspace components (debug)
just build

# Build all workspace components (release)
just build-release

# Build complete web package for deployment to rust-website
just build-web

# Deploy to rust-website repository (copies WASM files)
just deploy-web
```

### Code Quality

```bash
# Format all code
just fmt

# Run clippy linting
just lint

# Run test suite
just test

# Run all quality checks (format + lint + test)
just check
```

## Technology Stack

### Game Binary (`game/`)

- **Framework**: Bevy 0.16.1 game engine compiled to WASM
- **Hex Grid**: hexx 0.21 crate for coordinate system and pathfinding
- **Target**: WebAssembly with wasm-bindgen integration for web deployment
- **Asset System**: Bevy's embedded asset system for WASM builds
- **Text Rendering**: Modern Bevy component-based text system (Text2d, TextFont, TextColor)
- **Dependencies**: bevy, hexx, wasm-bindgen, shared crate

### Level Editor (`level-editor/`)

- **Framework**: Bevy 0.16.1 native application
- **Purpose**: Hex map creation, scenario design, and tactical encounter building
- **Usage**: Native binary for content creation during development
- **UI**: Bevy-based editor interface with hex grid visualization
- **Dependencies**: bevy, hexx, shared crate, serde, anyhow

### Shared Library (`shared/`)

- **Purpose**: Common game logic shared between game and level editor
- **Contents**: Hex grid coordinate system and pathfinding algorithms
- **Usage**: Imported by both game and level-editor binaries
- **Dependencies**: hexx (with serde features), serde, anyhow

### Build System

- **Command Runner**: just (Justfile-based workflow)
- **WASM Compilation**: wasm-bindgen with web target
- **Package Management**: Cargo workspace with shared dependencies
- **Rust Edition**: 2024 edition with workspace metadata
- **Deployment**: Integration with rust-website repository

## Game Development Guidelines

### Design-First Development

**Golden Rule**: Game design changes must be made in Obsidian FIRST, then implemented in code.

1. **Mechanical Changes**: Update relevant Evidence notes in Obsidian vault
2. **Technical Planning**: Plan implementation approach in code repository
3. **Implementation**: Code the mechanics following the design specification
4. **Validation**: Test implementation against Obsidian design requirements

### Code Organization

- **Hex Grid System**: Core coordinate math and pathfinding in `shared/src/hex_grid.rs`
- **Game Application**: Main Bevy tactical RPG app in `game/src/main.rs`
- **Level Editor**: Bevy-based hex map creation tool in `level-editor/src/main.rs`
- **Shared Library**: Common game logic in `shared/src/lib.rs`

**Current Implementation Status**:
- ✅ Basic hex grid coordinate system with hexx 0.21
- ✅ Bevy 0.16.1 applications with modern text rendering
- ⏳ Combat mechanics, entities, and god system (to be developed)

### Tactical RPG Design Patterns

Key design principles for SystemTactics development:

- **Hex Grid Foundation**: All spatial logic built on hexagonal coordinates
- **Turn-Based Combat**: Clear action phases with predictable turn order
- **Deep Customization**: Extensive character progression and equipment systems
- **God Evolution**: Player choices directly influence divine power development
- **Territory Management**: Strategic base-building supports tactical combat
- **System Apocalypse**: RPG mechanics integrated into world narrative

### Tracing and Debugging Guidelines

**Strategic Tracing**: Add meaningful tracing at key system lifecycle points to aid debugging and development.

**Tracing Levels and Usage**:
- **info!**: System initialization, level loading, major state changes, resource creation
- **warn!**: Missing resources, fallback behaviors, configuration issues
- **debug!**: Detailed state information, frequent but important events (use sparingly)
- **trace!**: Very verbose, fine-grained debugging (avoid in Update systems)

**Best Practices**:
- Add tracing to system startup functions (spawn, initialization)
- Log resource state changes and entity creation with entity IDs
- Use warn! for missing entities that should exist (easier to spot than debug!)
- Include relevant context: entity IDs, coordinates, names, counts
- Avoid tracing in high-frequency Update systems unless using trace! level
- Always include the system/component name in the message for context
- **Always use direct variable interpolation in format strings to avoid linting errors**

**String Formatting Convention**:
Always use direct variable interpolation in format strings. This applies to all macros including `info!`, `warn!`, `debug!`, `trace!`, `format!`, `panic!`, etc.

**Examples**:
```rust
// ✅ CORRECT: Direct variable interpolation
info!("LevelPlugin: Created default level '{level_name}'", level_name = level.name);
info!("Entity spawned: {entity:?} at position {pos:?}", pos = position);
warn!("Failed to load {file_name}", file_name = path.display());
let message = format!("FPS: {fps:.1}", fps = current_fps);

// ✅ CORRECT: Multiple variables with clear naming
info!("Spawning hex grid for level '{level_name}' ({width}x{height})",
      level_name = level.name, width = level.width, height = level.height);

// ✅ CORRECT: Format specifiers with direct interpolation
debug!("Camera rotation: {angle:.2}°", angle = rotation.to_degrees());
info!("Loaded {count} assets in {time:?}", count = assets.len(), time = elapsed);

// ❌ WRONG: Separate positional arguments (causes clippy::uninlined_format_args)
info!("Created level '{}'", level.name);
warn!("Entity count: {}", entities.len());
let text = format!("Score: {}", player.score);

// ❌ WRONG: Mixed interpolation styles
info!("Level '{}' has {entity_count} entities", level.name, entity_count = count);

// ✅ CORRECT: Simple static messages (no interpolation needed)
warn!("No camera found for positioning level name UI");
info!("System initialization completed");
```

**Key Rules**:
1. **All variables must be explicitly named**: Use `{variable_name}` with `variable_name = value`
2. **No positional arguments**: Never use `{}` with separate arguments
3. **Consistent naming**: Variable names in format string should match parameter names
4. **Format specifiers**: Use `{var:.2}`, `{var:?}`, etc. with named parameters

### WASM Optimization

- Use `just build-web` for production deployment (includes release optimizations)
- Size optimization profile enabled: `opt-level = 'z'`, `lto = true`, `strip = "symbols"`
- Minimize unnecessary Bevy features in Cargo.toml
- Embedded assets eliminate network requests for game content
- Target deployment to rust-website for hosting at erik-engelhardt.com

### Error Handling

- Graceful degradation for game state loading failures
- Clear error messages for unsupported browsers or devices
- Fallback UI states when game systems fail
- Loading indicators for asset loading and game initialization

## Integration with Existing Infrastructure

### Rust Website Deployment

The game will be deployed as part of the existing rust-website infrastructure:

**Deployment Path**: `/Users/eren/Projects/rust-website/static/wasm/system-tactics/`
**Web URL**: `https://erik-engelhardt.com/demos/system-tactics`
**Integration**: Uses existing website's responsive iframe system

### Example Project Reference

Reference the example project for established patterns:

**Template Path**: `/Users/eren/Projects/example_project/`
**Key References**:
- Justfile command patterns and WASM build workflow
- Workspace structure and dependency management
- AI-assisted development documentation organization
- WASM optimization and deployment strategies

## Testing Strategy

### Game System Testing

- Verify hex grid coordinate math and distance calculations
- Test combat calculations against design specifications in Obsidian
- Validate god system evolution mechanics
- Test character progression and equipment systems

### WASM Deployment Testing

- Test WASM compilation and loading performance
- Verify game functionality in target browsers
- Test responsive behavior in iframe deployment
- Validate asset loading and game initialization

### Design Validation Testing

- Test implemented mechanics against Obsidian Evidence specifications
- Verify tactical combat feels engaging and strategic
- Validate god system provides meaningful player choices
- Test that territory management supports combat objectives

## Common Issues and Solutions

### Obsidian Integration Issues

- **Missing metadata**: Always check `/Users/eren/SynologyDrive/Zettelkasten/CLAUDE.md` before creating notes
- **Broken cross-links**: Verify `[[Note Title]]` syntax and ensure target notes exist
- **Wrong field formats**: Use correct format for File/MultiFile/Multi/Date fields as specified in vault CLAUDE.md

### Game Development Issues

- **Hex coordinate bugs**: Test distance calculations with known coordinate pairs
- **Combat balance**: Reference design specifications in Obsidian Evidence notes
- **WASM build failures**: Ensure all dependencies support WASM compilation
- **Performance issues**: Profile with Bevy's built-in performance tools

### WASM Build Issues

- Ensure wasm-server-runner and wasm-bindgen-cli are installed (`just setup`)
- Use `just wasm` for development testing instead of manual compilation
- Verify all dependencies support WASM compilation (check Cargo.toml features)
- Long compile times (5+ minutes) are normal for Bevy WASM builds

## Contributing Guidelines

This is a personal learning project focused on tactical RPG development and AI-assisted game design.

### Development Standards

- **Design First**: All game mechanics must be documented in Obsidian before implementation
- **Code Quality**: Use `just check` (format + lint + test) before committing
- **Documentation**: Keep both Obsidian design docs and code documentation current
- **Testing**: Validate implementations against design specifications

### AI-Assisted Development Best Practices

- **Context Awareness**: Reference both Obsidian design docs and code repository state
- **Incremental Development**: Work on discrete, well-defined tasks
- **Cross-System Validation**: Ensure code implementations match Obsidian specifications
- **Documentation Sync**: Update project status in Obsidian as implementation progresses

This comprehensive documentation structure enables effective AI-assisted development while maintaining clear separation between strategic game design (Obsidian) and technical implementation (code repository), with seamless integration to existing deployment infrastructure.