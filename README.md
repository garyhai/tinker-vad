# Tinker VAD

Tinker VAD is a Rust library for Voice Activity Detection (VAD) that uses a pre-trained Silero VAD model to segment audio into speech and non-speech parts.

If the local model file path is not specified, the model will be downloaded from the Hugging Face Hub by default.

## Features

- Supports audio sample rates of 8kHz and 16kHz
- Utilizes a pre-trained Silero VAD model
- Capable of segmenting long audio recordings into speech segments for further processing

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
tinker-vad = { path = "path/to/your/tinker-vad" }
```

## Usage Example

Below is a basic example demonstrating how to use `tinker-vad` for voice activity detection:

```rust
// ...existing code...
use tinker_vad::{Vad, VadConfig};

fn main() {
    // Create VAD configuration with minimum silence duration of 500 milliseconds and sample rate of 16kHz
    let config = VadConfig::new(500, 16000);
    // Initialize VAD instance
    let mut vad = Vad::new(config);
    // Load the model
    vad.load("").expect("failed to load VAD model");

    // Load audio data (assuming you have a Vec<f32> named `audio`)
    let audio: Vec<f32> = // ...your audio data...

    // Perform voice activity detection
    vad.segment_audio(&audio).expect("failed to segment audio");

    // Retrieve detected speech segments
    for segment in vad.get_segments().iter() {
        println!("Detected segment from {} to {}", segment.0, segment.1);
    }

    // Process speech segments
    while let Some(segment) = vad.yield_segment() {
        println!("Processing segment from {} to {}", segment.0, segment.1);
        // Process the segment
    }

    // Flush and reset VAD state
    vad.flush().expect("failed to flush segments");
    vad.reset().expect("failed to reset VAD");
}
// ...existing code...
```

## License

This project is open-sourced under the MIT License.
