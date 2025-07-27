use clap::Parser;
use crossterm::terminal::size;
use anyhow::Result;

mod gif;
mod terminal;

use crate::gif::processor::VisualPreset;
use crate::gif::GifProcessor;
use crate::terminal::TerminalDisplay;

#[derive(Parser, Debug)]
#[command(version, about = "Display GIFs in your terminal", long_about = None)]
struct Args {
    /// Path to the GIF file
    file_path: String,
    
    /// Character to use for display (default: block characters for best quality)
    #[arg(short, long, default_value = "#")]
    character: String,
    
    /// Use high resolution mode with half-blocks (doubles vertical resolution)
    #[arg(long, help = "Use standard resolution mode instead of high-resolution")]
    standard_res: bool,

    /// Visual preset for different styles
    #[arg(long, value_enum, default_value_t = VisualPreset::Balanced, help = "Visual preset for different contrast and brightness styles")]
    preset: VisualPreset,

    /// Enable looping of the GIF
    #[arg(short, long, default_value_t = true)]
    loop_gif: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🎬 Loading GIF: {}", args.file_path);
    
    // Initialize terminal display
    let mut display = TerminalDisplay::new()?;
    
    // High-res is now default, use standard_res flag to disable
    let high_res = !args.standard_res;
    
    // Calculate optimal display size with maximum width utilization
    let (width, height) = size()?;
    let display_width = width as usize; // Use full terminal width
    let display_height = if high_res {
        (height as f32 * 0.9) as usize // 90% height for high-res
    } else {
        (height as f32 * 0.9) as usize // Same for standard
    };
    
    // Load and process the GIF with terminal dimensions and visual preset
    let processed_gif = GifProcessor::process_for_terminal_with_preset(
        &args.file_path, 
        Some(display_width as u32), 
        Some(display_height as u32),
        &args.preset
    )?;
    
    println!("✅ GIF loaded successfully: {}x{} pixels, {} frames", 
             processed_gif.width, processed_gif.height, processed_gif.frames.len());
    
    println!("✨ Playing GIF... (Press ESC, 'q', or Ctrl+C to quit)");
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // Play the GIF with high-res as default
    display.play_gif(processed_gif, &args.character, args.loop_gif, high_res).await?;
    
    Ok(())
}
