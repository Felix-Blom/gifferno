use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

use crate::gif::ProcessedGif;

/// Advanced display utilities for terminal rendering
pub struct DisplayUtils;

impl DisplayUtils {
    /// Calculate optimal GIF dimensions for terminal display
    pub fn calculate_display_size(
        gif_width: u32,
        gif_height: u32,
        terminal_width: u16,
        terminal_height: u16,
        preserve_aspect_ratio: bool,
    ) -> (u32, u32) {
        if !preserve_aspect_ratio {
            return (terminal_width as u32, terminal_height as u32);
        }
        
        let aspect_ratio = gif_width as f32 / gif_height as f32;
        
        // Account for character height being roughly 2x width in most terminals
        let adjusted_terminal_height = (terminal_height as f32 * 0.5) as u16;
        let adjusted_aspect = terminal_width as f32 / adjusted_terminal_height as f32;
        
        if aspect_ratio > adjusted_aspect {
            // GIF is wider, fit to width
            let new_width = terminal_width as u32;
            let new_height = (new_width as f32 / aspect_ratio) as u32;
            (new_width, new_height.min(adjusted_terminal_height as u32))
        } else {
            // GIF is taller, fit to height
            let new_height = adjusted_terminal_height as u32;
            let new_width = (new_height as f32 * aspect_ratio) as u32;
            (new_width.min(terminal_width as u32), new_height)
        }
    }
    
    /// Create a progress bar for loading operations
    pub fn show_progress_bar(message: &str, progress: f32) -> Result<()> {
        let width = 50;
        let filled = (progress * width as f32) as usize;
        
        let bar = format!(
            "\r{} [{}{}] {:.1}%",
            message,
            "█".repeat(filled),
            "░".repeat(width - filled),
            progress * 100.0
        );
        
        print!("{}", bar);
        stdout().flush()?;
        Ok(())
    }
    
    /// Clear the current line
    pub fn clear_line() -> Result<()> {
        execute!(stdout(), Clear(ClearType::CurrentLine), MoveTo(0, 0))?;
        Ok(())
    }
    
    /// Show loading animation
    pub fn show_loading_spinner(message: &str, frame: usize) -> Result<()> {
        let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let spinner = spinners[frame % spinners.len()];
        
        print!("\r{} {}", spinner, message);
        stdout().flush()?;
        Ok(())
    }
    
    /// Display GIF metadata information
    pub fn display_gif_info(gif: &ProcessedGif) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        
        let info = format!(
            "GIF Information:\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            📐 Dimensions: {}x{}\n\
            🎬 Frames: {}\n\
            ⏱️  Total Duration: {:.2}s\n\
            🔄 Loop Count: {}\n\
            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            \n\
            Press any key to continue...",
            gif.width,
            gif.height,
            gif.frames.len(),
            gif.frames.iter().map(|f| f.delay.as_secs_f32()).sum::<f32>(),
            gif.loop_count.map_or("infinite".to_string(), |c| c.to_string())
        );
        
        execute!(stdout(), Print(info))?;
        stdout().flush()?;
        Ok(())
    }
    
    /// Display help information
    pub fn display_help() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        
        let help = r#"
🎬 Gifferno - Terminal GIF Player
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

COMMANDS:
  run <file>              Play a GIF file directly
  save <file> <name>      Save a GIF with a memorable name
  get <name>              Play a saved GIF
  pick                    Interactive browser for saved GIFs
  delete <name>           Delete a saved GIF  
  clear                   Clear all saved GIFs

OPTIONS:
  -p, --print-character   Character to use for display (default: #)

CONTROLS (during playback):
  ESC, q, Ctrl+C         Quit playback

EXAMPLES:
  gifferno run animation.gif
  gifferno run -p "*" animation.gif  
  gifferno save cool_gif.gif "my-cool-gif"
  gifferno get "my-cool-gif"
  gifferno pick

Press any key to continue...
"#;
        
        execute!(stdout(), Print(help))?;
        stdout().flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_display_size() {
        // Test landscape GIF
        let (w, h) = DisplayUtils::calculate_display_size(200, 100, 80, 24, true);
        assert_eq!(w, 80);
        assert!(h <= 12); // Accounting for character aspect ratio
        
        // Test portrait GIF  
        let (w, h) = DisplayUtils::calculate_display_size(100, 200, 80, 24, true);
        assert!(w <= 80);
        assert_eq!(h, 12); // Half terminal height due to character aspect ratio
        
        // Test without aspect ratio preservation
        let (w, h) = DisplayUtils::calculate_display_size(100, 100, 80, 24, false);
        assert_eq!(w, 80);
        assert_eq!(h, 24);
    }
}
