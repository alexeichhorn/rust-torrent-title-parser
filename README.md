# Torrent Title Parser

A Rust library for parsing and extracting information from torrent titles. This is a Rust port of the Python library [PTT](https://github.com/dreulavelle/PTT).

## Features

- Parse torrent titles into structured data
- Extract common information like:
  - Resolution (1080p, 720p, etc.)
  - Quality (BluRay, WebDL, etc.)
  - Codecs (x264, x265, etc.)
  - Audio formats (AAC, AC3, etc.)
  - Channel information (2.0, 5.1, etc.)
  - Season and episode numbers
  - Languages
  - Release groups
  - And more!

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
torrent-title-parser = "0.1.0"
```

## Usage

### Basic Usage

```rust
use torrent_title_parser::parse_title;

fn main() {
    let title = "The Simpsons S01E01 1080p BluRay x265 HEVC 10bit AAC 5.1 Tigole";
    let result = parse_title(title).unwrap();
    
    println!("Title: {}", result.title);
    println!("Resolution: {:?}", result.resolution);
    println!("Quality: {:?}", result.quality);
    println!("Codec: {:?}", result.codec);
    println!("Audio: {:?}", result.audio);
    println!("Channels: {:?}", result.channels);
    println!("Season: {:?}", result.seasons);
    println!("Episode: {:?}", result.episodes);
    println!("Group: {:?}", result.group);
}
```

### Example Output

```rust
Title: The Simpsons
Resolution: Some("1080p")
Quality: Some("bluray")
Codec: Some("x265")
Audio: ["aac"]
Channels: ["5.1"]
Season: [1]
Episode: [1]
Group: Some("Tigole")
```

### Custom Parsing

You can also create your own parser instance and add custom handlers:

```rust
use torrent_title_parser::{Parser, ParsedTitle};
use regex::Regex;

fn main() {
    let mut parser = Parser::new();
    
    // Add a custom handler
    parser.add_handler(|title| {
        let custom_pattern = Regex::new(r"Custom-(\w+)").unwrap();
        custom_pattern.captures(title)
            .map(|caps| ("custom_field".to_string(), caps[1].to_string()))
    });
    
    let result = parser.parse("My Title Custom-Value").unwrap();
    // Process result...
}
```

## Supported Fields

The parser can extract the following information from torrent titles:

- `title`: The main title
- `resolution`: Video resolution (e.g., "1080p", "720p")
- `quality`: Source quality (e.g., "BluRay", "WebDL")
- `codec`: Video codec (e.g., "x264", "x265")
- `audio`: Audio formats (e.g., "AAC", "AC3")
- `channels`: Audio channels (e.g., "2.0", "5.1")
- `seasons`: Season numbers
- `episodes`: Episode numbers
- `languages`: Detected languages
- `group`: Release group name
- And many more fields (see `ParsedTitle` struct documentation)
