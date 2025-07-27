use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use std::time::Duration;
use tokio::time::sleep;

use crate::gif::{GifProcessor, ProcessedGif};

pub mod display;

/// Terminal renderer for GIF animation
pub struct TerminalDisplay {
    width: u16,
    height: u16,
}

impl TerminalDisplay {
    /// Create a new terminal display
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(TerminalDisplay { width, height })
    }
    
    /// Play a GIF in the terminal with the specified character
    pub async fn play_gif(&mut self, gif: ProcessedGif, character: &str, loop_forever: bool, high_res: bool) -> Result<()> {
        // Enter alternate screen mode
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        terminal::enable_raw_mode()?;
        
        let result = self.play_gif_loop(gif, character, loop_forever, high_res).await;
        
        // Cleanup
        terminal::disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen, Show)?;
        
        result
    }
    
    async fn play_gif_loop(&mut self, gif: ProcessedGif, character: &str, loop_forever: bool, high_res: bool) -> Result<()> {
        let mut stdout = stdout();
        let loop_count = if loop_forever { None } else { Some(1) };
        let mut loops_played = 0;
        
        loop {
            for frame in gif.frames.iter() {
                // Check for user input (ESC or Ctrl+C to quit)
                if self.should_quit()? {
                    return Ok(());
                }
                
                // Clear screen and move cursor to top-left for left-aligned display
                execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                
                // Render frame from top-left corner (no centering for maximum pixel density)
                if high_res && character == "#" {
                    // High-resolution mode using half-blocks
                    let display_height = if frame.height % 2 == 0 {
                        frame.height / 2
                    } else {
                        (frame.height + 1) / 2
                    };
                    
                    for display_y in 0..display_height as usize {
                        if display_y >= self.height as usize {
                            break;
                        }
                        
                        execute!(stdout, MoveTo(0, display_y as u16))?;
                        
                        let mut line = String::new();
                        for x in 0..frame.width as usize {
                            if x >= self.width as usize {
                                break;
                            }
                            
                            let top_y = display_y * 2;
                            let bottom_y = (display_y * 2 + 1).min(frame.height as usize - 1);
                            
                            let top_pixel = frame.pixels[top_y][x];
                            let bottom_pixel = if bottom_y < frame.height as usize {
                                frame.pixels[bottom_y][x]
                            } else {
                                0 // Use black for missing bottom pixel
                            };
                            
                            line.push_str(&GifProcessor::grayscale_to_half_block(top_pixel, bottom_pixel));
                        }
                        
                        execute!(stdout, Print(line))?;
                    }
                } else {
                    // Standard resolution mode
                    for (y, row) in frame.pixels.iter().enumerate() {
                        if y >= self.height as usize {
                            break;
                        }
                        
                        execute!(stdout, MoveTo(0, y as u16))?;
                        
                        let mut line = String::new();
                        for (x, &pixel) in row.iter().enumerate() {
                            if x >= self.width as usize {
                                break;
                            }
                            line.push_str(&GifProcessor::grayscale_to_char(pixel, character));
                        }
                        
                        execute!(stdout, Print(line))?;
                    }
                }
                
                stdout.flush()?;
                
                // Wait for frame delay
                sleep(frame.delay).await;
            }
            
            loops_played += 1;
            if let Some(max_loops) = loop_count {
                if loops_played >= max_loops {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if user wants to quit (non-blocking)
    fn should_quit(&self) -> Result<bool> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Esc => return Ok(true),
                    KeyCode::Char('c') | KeyCode::Char('C') if modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(true),
                    _ => {}
                }
            }
        }
        Ok(false)
    }
    
    /// Get terminal dimensions
    pub fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    
    /// Display a message and wait for user input
    pub fn display_message(&mut self, message: &str) -> Result<()> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        execute!(stdout(), Print(message))?;
        stdout().flush()?;
        Ok(())
    }
    
    /// Wait for any key press
    pub fn wait_for_keypress(&self) -> Result<()> {
        loop {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
        Ok(())
    }
}

impl Default for TerminalDisplay {
    fn default() -> Self {
        Self::new().unwrap_or(TerminalDisplay { width: 80, height: 24 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_terminal_display_creation() {
        let display = TerminalDisplay::new();
        // This might fail in test environments without a proper terminal
        // So we just check that it doesn't panic
        match display {
            Ok(d) => {
                assert!(d.width > 0);
                assert!(d.height > 0);
            }
            Err(_) => {
                // Expected in some test environments
            }
        }
    }
}
