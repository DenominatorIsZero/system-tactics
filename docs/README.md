# SystemTactics Documentation

Technical documentation for AI-assisted development of the SystemTactics tactical RPG.

## Documentation Structure

### `architecture.md`
Comprehensive technical architecture covering:
- Workspace design and crate organization
- Core system architecture (hex grid, combat, entities, gods)
- WASM deployment and performance considerations
- Development tool integration and workflow

### `projects/`
Project-based documentation organization following AI-assisted development workflow:

#### `projects/template/`
Reusable templates for systematic feature development:
- `spec-template.md` - Specification template for new features
- `plan-template.md` - Implementation plan template

#### `projects/archive/`
Completed project specifications and implementation plans (to be populated as features are developed)

## AI-Assisted Development Workflow

This documentation structure supports a three-phase development process:

1. **Specification Phase**: Use spec template to define requirements and design
2. **Planning Phase**: Use plan template to break down implementation tasks
3. **Execution Phase**: Implement according to plan with continuous validation

## Integration with Obsidian Vault

**Important**: Strategic game design documentation lives in the Obsidian vault at:
`/Users/eren/SynologyDrive/Zettelkasten/1 - Notes/1 - Projects/Development/SystemTactics/`

This repository's documentation focuses on technical implementation details, while the Obsidian vault contains:
- Game mechanics and design specifications
- Narrative and world-building elements
- God system evolution mechanics
- Combat system tactical design
- Character progression and territory management

## Usage Guidelines

### For New Features

1. **Check Obsidian vault** for existing game design documentation
2. **Create specification** using `projects/template/spec-template.md`
3. **Create implementation plan** using `projects/template/plan-template.md`
4. **Execute implementation** following the plan with frequent validation
5. **Archive completed work** in `projects/archive/[feature-name]/`

### Documentation Maintenance

- Keep `architecture.md` updated with significant architectural changes
- Update templates based on lessons learned from feature development
- Maintain clear separation between game design (Obsidian) and technical implementation (this repository)

## Cross-Reference Guidelines

Always reference relevant Obsidian vault documentation when creating technical specifications:
- Link to Evidence notes for game mechanics
- Reference design decisions and rationale
- Ensure technical implementation aligns with strategic design

This documentation structure enables effective AI-assisted development while maintaining clear boundaries between strategic design and technical implementation.