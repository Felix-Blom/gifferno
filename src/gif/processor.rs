use anyhow::Result;
use crate::gif::ProcessedGif;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum VisualPreset {
    /// Balanced contrast and brightness
    Balanced,
    /// High contrast with deep shadows and bright highlights
    Bright,
    /// Low contrast with muted tones
    Dark,
    /// Maximum brightness and contrast
    SuperBright,
    /// Retro terminal look with heavy contrast
    Retro,
    /// Soft, low contrast appearance
    Soft,
}

/// High-performance GIF processor with optimization for terminal display
pub struct GifProcessor;

impl GifProcessor {
    /// Process a GIF file optimized for terminal display with visual preset
    pub fn process_for_terminal_with_preset(path: &str, max_width: Option<u32>, max_height: Option<u32>, preset: &VisualPreset) -> Result<ProcessedGif> {
        let mut gif = ProcessedGif::from_file(path)?;
        
        // Resize if needed to fit terminal constraints
        if let (Some(max_w), Some(max_h)) = (max_width, max_height) {
            if gif.width > max_w || gif.height > max_h {
                gif = Self::resize_gif(gif, max_w, max_h)?;
            }
        }
        
        // Apply preset-specific processing
        gif = Self::apply_visual_preset(gif, preset)?;
        
        Ok(gif)
    }

    /// Process a GIF file optimized for terminal display
    pub fn process_for_terminal(path: &str, max_width: Option<u32>, max_height: Option<u32>) -> Result<ProcessedGif> {
        let mut gif = ProcessedGif::from_file(path)?;
        
        // Resize if needed to fit terminal constraints
        if let (Some(max_w), Some(max_h)) = (max_width, max_height) {
            if gif.width > max_w || gif.height > max_h {
                gif = Self::resize_gif(gif, max_w, max_h)?;
            }
        }
        
        Ok(gif)
    }
    
    /// Resize GIF frames to fit within specified dimensions while maintaining aspect ratio
    fn resize_gif(mut gif: ProcessedGif, max_width: u32, max_height: u32) -> Result<ProcessedGif> {
        // Calculate new dimensions maintaining aspect ratio
        let (new_width, new_height) = if gif.width > max_width || gif.height > max_height {
            let scale_w = max_width as f32 / gif.width as f32;
            let scale_h = max_height as f32 / gif.height as f32;
            let scale = scale_w.min(scale_h);
            
            ((gif.width as f32 * scale) as u32, (gif.height as f32 * scale) as u32)
        } else {
            (gif.width, gif.height)
        };
        
        // Only resize if dimensions actually changed
        if new_width != gif.width || new_height != gif.height {
            for frame in &mut gif.frames {
                frame.pixels = Self::resize_frame_simple(&frame.pixels, gif.width, gif.height, new_width, new_height);
                frame.width = new_width;
                frame.height = new_height;
            }
            
            gif.width = new_width;
            gif.height = new_height;
        }
        
        Ok(gif)
    }
    
    /// Simple and fast nearest-neighbor resize for grayscale frames
    fn resize_frame_simple(
        pixels: &[Vec<u8>], 
        old_width: u32, 
        old_height: u32, 
        new_width: u32, 
        new_height: u32
    ) -> Vec<Vec<u8>> {
        let mut resized = vec![vec![0u8; new_width as usize]; new_height as usize];
        
        let x_scale = old_width as f32 / new_width as f32;
        let y_scale = old_height as f32 / new_height as f32;
        
        for new_y in 0..new_height {
            for new_x in 0..new_width {
                let old_x = (new_x as f32 * x_scale) as usize;
                let old_y = (new_y as f32 * y_scale) as usize;
                
                // Bounds checking
                let old_x = old_x.min(old_width as usize - 1);
                let old_y = old_y.min(old_height as usize - 1);
                
                resized[new_y as usize][new_x as usize] = pixels[old_y][old_x];
            }
        }
        
        resized
    }
    
    /// Apply visual preset transformations to the GIF
    fn apply_visual_preset(mut gif: ProcessedGif, preset: &VisualPreset) -> Result<ProcessedGif> {
        // Store the preset for use in character conversion
        for frame in &mut gif.frames {
            frame.pixels = Self::apply_preset_to_frame(&frame.pixels, preset);
        }
        Ok(gif)
    }
    
    /// Apply preset-specific transformations to a single frame
    fn apply_preset_to_frame(pixels: &[Vec<u8>], preset: &VisualPreset) -> Vec<Vec<u8>> {
        let mut transformed = pixels.to_vec();
        
        for row in &mut transformed {
            for pixel in row {
                *pixel = Self::apply_preset_to_pixel(*pixel, preset);
            }
        }
        
        transformed
    }
    
    /// Apply preset transformation to a single pixel
    fn apply_preset_to_pixel(value: u8, preset: &VisualPreset) -> u8 {
        let normalized = value as f32 / 255.0;
        
        let transformed = match preset {
            VisualPreset::Balanced => {
                // Standard S-curve for balanced contrast
                if normalized < 0.5 {
                    2.0 * normalized * normalized
                } else {
                    1.0 - 2.0 * (1.0 - normalized) * (1.0 - normalized)
                }
            },
            VisualPreset::Bright => {
                // High contrast with boosted highlights
                let contrast = normalized.powf(0.6) * 1.2;
                contrast.min(1.0)
            },
            VisualPreset::Dark => {
                // Muted tones with reduced highlights
                normalized.powf(1.4) * 0.8
            },
            VisualPreset::SuperBright => {
                // Maximum contrast and brightness
                let extreme = normalized.powf(0.4) * 1.4;
                extreme.min(1.0)
            },
            VisualPreset::Retro => {
                // Heavy contrast with sharp transitions
                if normalized < 0.3 {
                    normalized * 0.5
                } else if normalized > 0.7 {
                    0.5 + (normalized - 0.7) * 1.67
                } else {
                    0.15 + (normalized - 0.3) * 0.875
                }
            },
            VisualPreset::Soft => {
                // Gentle contrast with smooth gradients
                normalized.powf(1.2) * 0.9
            },
        };
        
        (transformed * 255.0) as u8
    }
    
    /// Convert grayscale value to block character with enhanced contrast and highlights
    pub fn grayscale_to_char(value: u8, character: &str) -> String {
        if character == "#" {
            // Enhanced Unicode block characters with more dynamic contrast and highlights
            let blocks = [
                " ",        // 0-12% - deep shadows
                "░",        // 13-25% - light shadows  
                "▒",        // 26-40% - medium tones
                "▓",        // 41-60% - darker tones
                "█",        // 61-80% - highlights
                "█",        // 81-100% - bright highlights (ensure they stay bright)
            ];
            
            // Enhanced contrast mapping with more dramatic transitions
            let enhanced_value = ((value as f32 / 255.0).powf(0.8) * 255.0) as u8;
            let index = (enhanced_value as usize * (blocks.len() - 1)) / 255;
            blocks[index].to_string()
        } else {
            // For custom characters, use enhanced intensity-based mapping
            let enhanced_intensity = (value as f32 / 255.0).powf(0.7);
            if enhanced_intensity < 0.15 {
                " ".to_string()
            } else if enhanced_intensity < 0.35 {
                character.to_string()
            } else if enhanced_intensity < 0.6 {
                format!("{}{}", character, character)
            } else if enhanced_intensity < 0.8 {
                format!("{}{}{}", character, character, character)
            } else {
                format!("{}{}{}{}", character, character, character, character)
            }
        }
    }
    
    /// Convert two grayscale values to half-block character with enhanced highlights and contrast
    pub fn grayscale_to_half_block(top_value: u8, bottom_value: u8) -> String {
        // Enhanced contrast curve for more dynamic visuals
        let enhance_contrast = |val: u8| -> u8 {
            ((val as f32 / 255.0).powf(0.75) * 255.0) as u8
        };
        
        let top_enhanced = enhance_contrast(top_value);
        let bottom_enhanced = enhance_contrast(bottom_value);
        
        // More dramatic intensity levels with enhanced contrast
        let top_intensity = (top_enhanced as f32 / 255.0 * 5.0) as u8;
        let bottom_intensity = (bottom_enhanced as f32 / 255.0 * 5.0) as u8;
        
        // Enhanced mapping with more highlight differentiation
        match (top_intensity, bottom_intensity) {
            // Deep shadows (0-1)
            (0, 0) => " ",          // Pure shadow
            (0, 1) => "░",          // Shadow with light bottom
            (0, 2..=3) => "▒",      // Shadow with medium bottom
            (0, 4..=5) => "▄",      // Shadow with bright bottom
            
            // Light areas (1)
            (1, 0) => "░",          // Light top, shadow bottom
            (1, 1) => "░",          // Both light
            (1, 2..=3) => "▒",      // Light top, medium bottom
            (1, 4..=5) => "▄",      // Light top, bright bottom
            
            // Medium tones (2-3)
            (2..=3, 0) => "▒",      // Medium top, shadow bottom
            (2..=3, 1) => "▒",      // Medium top, light bottom
            (2..=3, 2..=3) => "▓",  // Both medium (more contrast)
            (2..=3, 4..=5) => "▄",  // Medium top, bright bottom
            
            // Highlights (4-5)
            (4..=5, 0) => "▀",      // Bright top, shadow bottom
            (4..=5, 1) => "▀",      // Bright top, light bottom
            (4..=5, 2..=3) => "▀",  // Bright top, medium bottom
            (4..=5, 4..=5) => "█",  // Both bright (pure highlight)
            
            _ => "█",               // Fallback to bright
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grayscale_to_char() {
        // Test block characters for default
        assert_eq!(GifProcessor::grayscale_to_char(0, "#"), " ");
        assert_eq!(GifProcessor::grayscale_to_char(255, "#"), "█");
        // 127 maps to index 1 or 2, let's test the actual mapping
        let mid_char = GifProcessor::grayscale_to_char(127, "#");
        assert!(mid_char == "░" || mid_char == "▒", "Mid value should be a shade block");
        
        // Test custom character
        assert_eq!(GifProcessor::grayscale_to_char(0, "*"), " ");
        assert_eq!(GifProcessor::grayscale_to_char(255, "*"), "****");
        assert_eq!(GifProcessor::grayscale_to_char(100, "*"), "*");
    }
    
    #[test]
    fn test_grayscale_to_half_block() {
        // Test enhanced contrast mapping with highlights
        assert_eq!(GifProcessor::grayscale_to_half_block(0, 0), " ");      // Both deep shadow
        assert_eq!(GifProcessor::grayscale_to_half_block(255, 0), "▀");    // Top bright, bottom shadow
        assert_eq!(GifProcessor::grayscale_to_half_block(0, 255), "▄");    // Top shadow, bottom bright  
        assert_eq!(GifProcessor::grayscale_to_half_block(255, 255), "█");  // Both bright (highlights)
        
        // Test enhanced medium values with better contrast
        assert_eq!(GifProcessor::grayscale_to_half_block(64, 64), "░");    // Both light
        assert_eq!(GifProcessor::grayscale_to_half_block(128, 128), "▓");  // Both medium (enhanced contrast)
        
        // Test mixed intensities with enhanced mapping
        assert_eq!(GifProcessor::grayscale_to_half_block(64, 200), "▄");   // Light top, bright bottom
        assert_eq!(GifProcessor::grayscale_to_half_block(200, 64), "▀");   // Bright top, light bottom
    }
    
    #[test]
    fn test_resize_frame_simple() {
        // Test 2x2 to 4x4 resize
        let original = vec![
            vec![0, 255],
            vec![128, 64]
        ];
        
        let resized = GifProcessor::resize_frame_simple(&original, 2, 2, 4, 4);
        
        assert_eq!(resized.len(), 4);
        assert_eq!(resized[0].len(), 4);
        
        // Basic sanity check - corners should match
        assert_eq!(resized[0][0], 0);
        assert_eq!(resized[3][3], 64);
    }
}
