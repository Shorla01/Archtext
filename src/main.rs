
use std::env;
use std::fs;

const APP_NAME: &str = "ArchText";
const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    println!();
    println!("========================================");
    println!("       ARCHTEXT - Writing Assistant");
    println!("========================================");

    if args.len() == 1 {
        show_help();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" | "help" => {
            show_help();
        }

        "--version" | "-v" | "version" => {
            println!("{} version {}", APP_NAME, VERSION);
        }

        "count" => {
            if args.len() < 3 {
                println!("Please provide a text file.");
                println!("Example: cargo run -- count project.txt");
                return;
            }

            count_file(&args[2..].join(" "));
        }

        "organize" => {
            if args.len() < 3 {
                println!("Please provide a text file.");
                println!("Example: cargo run -- organize project.txt");
                return;
            }

            organize_file(&args[2..].join(" "));
        }
"export" => {
    if args.len() < 3 {
        println!("Please provide a text file.");
        println!("Example: cargo run -- export project.txt");
        return;
    }

    export_file(&args[2..].join(" "));
}
"check" => {
    if args.len() < 3 {
        println!("Please provide a text file.");
        println!("Example: cargo run -- check project.txt");
        return;
    }

    quality_check(&args[2..].join(" "));
}
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Use 'archtext --help' to see available commands.");
        }
    }
}

fn show_help() {
    println!();
    println!("USAGE:");
    println!("    archtext [COMMAND] [FILE]");
    println!();
    println!("AVAILABLE COMMANDS:");
    println!("    help       Display available commands");
    println!("    version    Display application version");
    println!("    count      Count words and characters");
    println!("    organize   Organize architectural notes");
    println!("    export     Export organized notes to Markdown");
   println!("    check      Check architectural project completeness");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Show help information");
    println!("    -v, --version    Show version information");
    println!();
}

fn count_file(filename: &str) {
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(error) => {
            println!("Could not read file: {}", error);
            return;
        }
    };

    let words = content.split_whitespace().count();
    let characters = content.chars().count();
    let lines = content.lines().count();
    let paragraphs = content
        .split("\n\n")
        .filter(|paragraph| !paragraph.trim().is_empty())
        .count();

    println!();
    println!("TEXT ANALYSIS");
    println!("----------------------------------------");
    println!("File: {}", filename);
    println!("Words: {}", words);
    println!("Characters: {}", characters);
    println!("Lines: {}", lines);
    println!("Paragraphs: {}", paragraphs);
    println!("----------------------------------------");
}

fn organize_file(filename: &str) {
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(error) => {
            println!("Could not read file: {}", error);
            return;
        }
    };

    let headings = [
        "SITE AND CONTEXT",
        "DESIGN CONCEPT",
        "DESIGN OBJECTIVES",
        "ARCHITECTURAL FEATURES",
        "GENERAL NOTES",
    ];

    let mut sections: [Vec<String>; 5] =
        std::array::from_fn(|_| Vec::new());

    for line in content.lines() {
        let note = line.trim();

        if note.is_empty() {
            continue;
        }

        let category = categorize_note(note);
        sections[category].push(note.to_string());
    }

    println!();
    println!("ARCHITECTURAL PROJECT NOTES");
    println!("========================================");

    for (index, heading) in headings.iter().enumerate() {
        if !sections[index].is_empty() {
            println!();
            println!("{}", heading);
            println!("----------------------------------------");

            for note in &sections[index] {
                println!("- {}", note);
            }
        }
    }

    println!();
    println!("========================================");
    println!("Notes organized successfully!");
}

fn categorize_note(note: &str) -> usize {
    let text = note.to_lowercase();

    if [
        "site", "location", "ondo", "area", "market",
        "access", "surrounding", "boundary",
    ]
    .iter()
    .any(|word| text.contains(word))
    {
        0
    } else if [
        "concept", "canopy", "roof", "shell",
        "form", "material",
    ]
    .iter()
    .any(|word| text.contains(word))
    {
        1
    } else if [
        "aim", "objective", "improve", "provide",
        "create", "address", "reduce",
    ]
    .iter()
    .any(|word| text.contains(word))
    {
        2
    } else if [
        "building", "floor", "stall", "space",
        "ventilation", "daylight", "facade",
        "restaurant", "parking",
    ]
    .iter()
    .any(|word| text.contains(word))
    {
        3
    } else {
        4
    }
}
fn export_file(filename: &str) {
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(error) => {
            println!("Could not read file: {}", error);
            return;
        }
    };

    let headings = [
        "SITE AND CONTEXT",
        "DESIGN CONCEPT",
        "DESIGN OBJECTIVES",
        "ARCHITECTURAL FEATURES",
        "GENERAL NOTES",
    ];

    let mut sections: [Vec<String>; 5] =
        std::array::from_fn(|_| Vec::new());

    for line in content.lines() {
        let note = line.trim();

        if note.is_empty() {
            continue;
        }

        let category = categorize_note(note);
        sections[category].push(note.to_string());
    }

    let mut markdown = String::new();

    markdown.push_str("# Architectural Project Notes\n\n");

    for (index, heading) in headings.iter().enumerate() {
        if !sections[index].is_empty() {
            markdown.push_str(&format!("## {}\n\n", heading));

            for note in &sections[index] {
                markdown.push_str(&format!("- {}\n", note));
            }

            markdown.push('\n');
        }
    }

    let path = std::path::Path::new(filename);
    let stem = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("project");

    let output_filename = format!("{}-organized.md", stem);

    match fs::write(&output_filename, markdown) {
        Ok(()) => {
            println!("Markdown export successful!");
            println!("Created file: {}", output_filename);
        }
        Err(error) => {
            println!("Could not save Markdown file: {}", error);
        }
    }
}
fn quality_check(filename: &str) {
    let content = match fs::read_to_string(filename) {
        Ok(text) => text,
        Err(error) => {
            println!("Could not read file: {}", error);
            return;
        }
    };

    let text = content.to_lowercase();

    let checks = [
        ("Project location", ["location", "located", "town", "city"]),
        ("Site area", ["site area", "square metres", "square meters", "sqm"]),
        ("Design concept", ["concept", "canopy", "roof", "design idea"]),
        ("Design objectives", ["aim", "objective", "improve", "provide"]),
        ("Building features", ["building", "floor", "stall", "space"]),
        ("Sustainability", ["sustainable", "ventilation", "daylight", "energy"]),
        ("Materials", ["material", "concrete", "timber", "steel"]),
        ("Project challenges", ["problem", "challenge", "congestion", "inadequate"]),
    ];

    let mut found = 0;

    println!();
    println!("ARCHTEXT - QUALITY REPORT");
    println!("========================================");

    for (category, keywords) in checks.iter() {
        let is_found = keywords
            .iter()
            .any(|keyword| text.contains(keyword));

        if is_found {
            found += 1;
            println!("[FOUND]   {}", category);
        } else {
            println!("[MISSING] {}", category);
        }
    }

    println!("========================================");
    println!("Completeness: {}/{}", found, checks.len());

    let percentage = found * 100 / checks.len();

    println!("Score: {}%", percentage);

    if percentage == 100 {
        println!("Excellent! All information categories are covered.");
    } else {
        println!("Consider adding the missing information to your project notes.");
    }
}