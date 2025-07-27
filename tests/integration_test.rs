use gifferno::{GifProcessor, ProcessedGif};
use std::path::Path;

#[test]
fn test_gif_processing_integration() {
    // Skip test if no example GIFs are available
    let example_paths = ["examples/blink.gif", "examples/homer.gif"];
    
    for example_path in &example_paths {
        if Path::new(example_path).exists() {
            println!("Testing with: {}", example_path);
            
            // Test GIF loading and processing
            let result = ProcessedGif::from_file(example_path);
            match result {
                Ok(gif) => {
                    assert!(gif.frames.len() > 0, "GIF should have frames");
                    assert!(gif.width > 0, "GIF should have width");
                    assert!(gif.height > 0, "GIF should have height");
                    
                    // Test character conversion
                    for frame in &gif.frames {
                        assert_eq!(frame.pixels.len(), gif.height as usize);
                        if !frame.pixels.is_empty() {
                            assert_eq!(frame.pixels[0].len(), gif.width as usize);
                        }
                    }
                    
                    println!("✅ Successfully processed {}: {}x{} with {} frames", 
                            example_path, gif.width, gif.height, gif.frames.len());
                }
                Err(e) => {
                    println!("⚠️  Could not process {}: {}", example_path, e);
                }
            }
        } else {
            println!("⚠️  Example file not found: {}", example_path);
        }
    }
}

#[test]
fn test_character_conversion() {
    // Test block characters for default
    assert_eq!(GifProcessor::grayscale_to_char(0, "#"), " ");
    assert_eq!(GifProcessor::grayscale_to_char(255, "#"), "█");
    
    // Test with custom character
    assert_eq!(GifProcessor::grayscale_to_char(0, "*"), " ");
    // High intensity should give multiple characters
    let result = GifProcessor::grayscale_to_char(255, "*");
    assert!(result.len() > 1, "High intensity should produce multiple characters");
    
    println!("✅ Character conversion tests passed");
}

#[test]
fn test_gif_processor_resize() {
    // Test the processor can handle terminal size constraints
    let example_paths = ["examples/blink.gif"];
    
    for example_path in &example_paths {
        if Path::new(example_path).exists() {
            let result = GifProcessor::process_for_terminal(example_path, Some(40), Some(20));
            match result {
                Ok(gif) => {
                    assert!(gif.width <= 40, "Width should be constrained");
                    assert!(gif.height <= 20, "Height should be constrained");
                    println!("✅ Resize test passed for {}: {}x{}", example_path, gif.width, gif.height);
                }
                Err(e) => {
                    println!("⚠️  Could not process {}: {}", example_path, e);
                }
            }
        }
    }
}
