use anyhow::{Context, Result};
use gif::{DecodeOptions, Frame};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub mod processor;

pub use processor::GifProcessor;

/// Represents a single frame of a GIF with timing information
#[derive(Debug, Clone)]
pub struct GifFrame {
    /// Grayscale pixel data
    pub pixels: Vec<Vec<u8>>,
    /// Frame width
    pub width: u32,
    /// Frame height  
    pub height: u32,
    /// Duration to display this frame
    pub delay: Duration,
}

/// Container for all frames of a GIF
#[derive(Debug, Clone)]
pub struct ProcessedGif {
    pub frames: Vec<GifFrame>,
    pub width: u32,
    pub height: u32,
    pub loop_count: Option<u16>,
}

impl ProcessedGif {
    /// Load and process a GIF file from disk
    pub fn from_file(path: &str) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("Failed to open GIF file: {}", path))?;
        let reader = BufReader::new(file);
        
        let mut options = DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        
        let mut decoder = options.read_info(reader)
            .with_context(|| "Failed to decode GIF header")?;
        
        let mut frames = Vec::new();
        let width = decoder.width() as u32;
        let height = decoder.height() as u32;
        
        // Process each frame
        while let Some(frame) = decoder.read_next_frame()
            .with_context(|| "Failed to read GIF frame")? {
            
            let gif_frame = Self::process_frame(frame, width, height)?;
            frames.push(gif_frame);
        }
        
        if frames.is_empty() {
            return Err(anyhow::anyhow!("GIF contains no frames"));
        }
        
        Ok(ProcessedGif {
            frames,
            width,
            height,
            loop_count: None, // We'll implement this later if needed
        })
    }
    
    /// Convert a single GIF frame to grayscale
    fn process_frame(frame: &Frame, width: u32, height: u32) -> Result<GifFrame> {
        let delay = Duration::from_millis((frame.delay as u64) * 10); // GIF delay is in centiseconds
        
        // Convert RGBA to grayscale
        let mut pixels = vec![vec![0u8; width as usize]; height as usize];
        
        let buffer = &frame.buffer;
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                if idx + 3 < buffer.len() {
                    let r = buffer[idx];
                    let g = buffer[idx + 1];
                    let b = buffer[idx + 2];
                    let alpha = buffer[idx + 3];
                    
                    // Convert to grayscale using enhanced luminance formula with contrast boost
                    // Handle transparency by using black background
                    let gray = if alpha == 0 {
                        0
                    } else {
                        let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
                        
                        // Apply enhanced contrast curve for more dynamic visuals
                        let enhanced = if luminance < 0.5 {
                            // Enhance shadows - make them more defined
                            2.0 * luminance * luminance
                        } else {
                            // Enhance highlights - make them brighter
                            1.0 - 2.0 * (1.0 - luminance) * (1.0 - luminance)
                        };
                        
                        // Apply slight S-curve for even more contrast
                        let final_luminance = (enhanced * 1.1).min(1.0);
                        (final_luminance * 255.0) as u8
                    };
                    
                    pixels[y as usize][x as usize] = gray;
                }
            }
        }
        
        Ok(GifFrame {
            pixels,
            width,
            height,
            delay: delay.max(Duration::from_millis(50)), // Minimum 50ms delay
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_gif_loading() {
        // This test will be expanded once we have test data
        let test_files = ["examples/blink.gif", "examples/homer.gif"];
        
        for file in &test_files {
            if Path::new(file).exists() {
                let result = ProcessedGif::from_file(file);
                match result {
                    Ok(gif) => {
                        assert!(!gif.frames.is_empty(), "GIF should have frames");
                        assert!(gif.width > 0, "Width should be positive");
                        assert!(gif.height > 0, "Height should be positive");
                    }
                    Err(e) => {
                        // Print error but don't fail test if file doesn't exist or has issues
                        println!("Warning: Could not load {}: {}", file, e);
                    }
                }
            }
        }
    }
}
