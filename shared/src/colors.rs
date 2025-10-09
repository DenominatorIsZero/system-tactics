//! Color constants for SystemTactics tactical RPG
//!
//! Defines a cohesive color palette for the tactical RPG interface and game world,
//! matching the website theme for consistency.

use bevy::prelude::*;

// =============================================================================
// TACTICAL RPG COLOR PALETTE
// =============================================================================

// Website-matching background colors
/// Main background color matching the website theme
#[allow(clippy::approx_constant)]
pub const BACKGROUND_COLOR: Color = Color::srgb(0.216, 0.255, 0.318); // rgb(55, 65, 81) - website background

/// Alternative light background for tactical grid view
pub const LIGHT_BACKGROUND: Color = Color::WHITE;

// Hex grid colors for tactical battlefield
/// Main hex column surface color - neutral gray for terrain
pub const HEX_SURFACE_GRAY: Color = Color::srgb(0.294, 0.333, 0.388); // Medium gray for hex surfaces

/// Hex edge highlighting color - tactical green
pub const HEX_EDGE_GREEN: Color = Color::srgb(0.133, 0.698, 0.298); // green-500: #22c55e

/// Secondary gray for UI elements and darker terrain
pub const GRAY_SECONDARY: Color = Color::srgb(0.294, 0.333, 0.388); // rgb(75, 85, 99) - website content

// Accent colors for tactical elements
/// Primary green for highlighting and selection
pub const GREEN_PRIMARY: Color = Color::srgb(0.133, 0.698, 0.298); // green-500: #22c55e

/// Hover/active green for interactive elements
pub const GREEN_HOVER: Color = Color::srgb(0.251, 0.831, 0.412); // green-400: #4ade80

/// Warning/attention yellow for important tactical info
pub const YELLOW_ACCENT: Color = Color::srgb(0.918, 0.784, 0.157); // yellow-500: #eab308

// Standard UI colors
/// Primary text color
pub const TEXT_COLOR: Color = Color::WHITE;

/// Standard white for high contrast elements
pub const WHITE: Color = Color::WHITE;

/// Pure black for borders and shadows
pub const BLACK: Color = Color::BLACK;

// =============================================================================
// TACTICAL GAME SPECIFIC COLORS
// =============================================================================

/// Unit selection highlight color
pub const UNIT_SELECTED: Color = GREEN_PRIMARY;

/// Movement range indicator color
pub const MOVEMENT_RANGE: Color = Color::srgb(0.4, 0.7, 1.0); // Light blue

/// Attack range indicator color
pub const ATTACK_RANGE: Color = Color::srgb(1.0, 0.4, 0.4); // Light red

/// Neutral terrain height gradient start (low)
pub const TERRAIN_LOW: Color = Color::srgb(0.4, 0.4, 0.4); // Dark gray

/// Neutral terrain height gradient end (high)
pub const TERRAIN_HIGH: Color = Color::srgb(0.7, 0.7, 0.7); // Light gray
