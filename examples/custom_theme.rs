use anyhow::Result;
use pingtest::ui::{ThemeManager, Theme, ThemeColors};
use ratatui::style::Color;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎨 PingTest Custom Theme Example");
    println!("=================================\n");

    let mut theme_manager = ThemeManager::new();

    // Show available themes
    println!("Available themes:");
    let themes = theme_manager.get_available_themes();
    for theme in &themes {
        println!("  • {}", theme);
    }
    println!();

    // Create a custom theme
    println!("Creating custom 'ocean' theme...");
    
    let ocean_theme = Theme {
        name: "ocean".to_string(),
        colors: ThemeColors {
            background: Color::Rgb(0, 20, 40),      // Deep ocean blue
            foreground: Color::Rgb(173, 216, 230),  // Light blue
            primary: Color::Rgb(0, 191, 255),       // Deep sky blue
            secondary: Color::Rgb(72, 209, 204),    // Medium turquoise
            success: Color::Rgb(0, 255, 127),       // Spring green
            warning: Color::Rgb(255, 215, 0),       // Gold
            error: Color::Rgb(255, 69, 0),          // Red orange
            info: Color::Rgb(30, 144, 255),          // Dodger blue
            accent: Color::Rgb(138, 43, 226),       // Blue violet
            border: Color::Rgb(25, 25, 112),         // Midnight blue
            text_primary: Color::Rgb(173, 216, 230), // Light blue
            text_secondary: Color::Rgb(0, 191, 255), // Deep sky blue
            text_muted: Color::Rgb(70, 130, 180),    // Steel blue
        },
    };

    // Add the custom theme to the theme manager
    // Note: In a real implementation, you'd need to modify the ThemeManager
    // to support adding custom themes at runtime
    println!("Custom theme created:");
    println!("  Name: {}", ocean_theme.name);
    println!("  Background: Deep ocean blue");
    println!("  Primary: Deep sky blue");
    println!("  Secondary: Medium turquoise");
    println!("  Accent: Blue violet");
    println!();

    // Demonstrate theme switching
    println!("Demonstrating theme switching...");
    for theme in &themes[..3] { // Show first 3 themes
        theme_manager.set_theme(theme)?;
        println!("  ✓ Switched to '{}' theme", theme);
        
        // In a real TUI application, this would change the visual appearance
        // For this example, we'll just show the theme name
        let current_theme = theme_manager.get_current_theme();
        println!("    Current theme: {}", current_theme);
    }
    println!();

    // Show theme color information
    println!("Theme color information:");
    for theme_name in &themes[..3] {
        theme_manager.set_theme(theme_name)?;
        let theme = theme_manager.get_theme(theme_name).unwrap();
        
        println!("  Theme: {}", theme.name);
        println!("    Primary: {:?}", theme.colors.primary);
        println!("    Secondary: {:?}", theme.colors.secondary);
        println!("    Success: {:?}", theme.colors.success);
        println!("    Warning: {:?}", theme.colors.warning);
        println!("    Error: {:?}", theme.colors.error);
        println!();
    }

    // Demonstrate theme validation
    println!("Testing theme validation...");
    
    // Valid theme
    match theme_manager.set_theme("dracula") {
        Ok(_) => println!("  ✓ 'dracula' theme is valid"),
        Err(e) => println!("  ✗ Error setting 'dracula' theme: {}", e),
    }
    
    // Invalid theme
    match theme_manager.set_theme("invalid_theme") {
        Ok(_) => println!("  ✓ 'invalid_theme' theme is valid"),
        Err(e) => println!("  ✗ Error setting 'invalid_theme' theme: {}", e),
    }
    println!();

    // Show how to create theme variations
    println!("Creating theme variations...");
    
    // Create a dark variation of the ocean theme
    let ocean_dark = Theme {
        name: "ocean-dark".to_string(),
        colors: ThemeColors {
            background: Color::Rgb(0, 10, 20),       // Even darker ocean
            foreground: Color::Rgb(135, 206, 235),   // Sky blue
            primary: Color::Rgb(0, 150, 200),        // Darker sky blue
            secondary: Color::Rgb(50, 150, 150),     // Darker turquoise
            success: Color::Rgb(0, 200, 100),        // Darker spring green
            warning: Color::Rgb(200, 150, 0),        // Darker gold
            error: Color::Rgb(200, 50, 0),           // Darker red orange
            info: Color::Rgb(20, 100, 200),          // Darker dodger blue
            accent: Color::Rgb(100, 30, 180),        // Darker blue violet
            border: Color::Rgb(15, 15, 80),          // Darker midnight blue
            text_primary: Color::Rgb(135, 206, 235), // Sky blue
            text_secondary: Color::Rgb(0, 150, 200), // Darker sky blue
            text_muted: Color::Rgb(50, 100, 150),    // Darker steel blue
        },
    };

    println!("Created 'ocean-dark' variation:");
    println!("  Background: Even darker ocean blue");
    println!("  Text: Sky blue for better contrast");
    println!("  Accents: Darker variants for subtle appearance");
    println!();

    // Show theme accessibility considerations
    println!("Theme accessibility considerations:");
    println!("  • High contrast between text and background");
    println!("  • Color-blind friendly color choices");
    println!("  • Consistent color meanings across themes");
    println!("  • Sufficient color differentiation");
    println!("  • Fallback to terminal default colors when needed");
    println!();

    println!("✅ Custom theme example completed successfully!");
    println!();
    println!("💡 Tips for creating custom themes:");
    println!("  1. Choose colors that work well together");
    println!("  2. Ensure good contrast for readability");
    println!("  3. Test with different terminal backgrounds");
    println!("  4. Consider color-blind users");
    println!("  5. Use semantic color names (success, warning, error)");
    println!("  6. Provide both light and dark variants");

    Ok(())
}