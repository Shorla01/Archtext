# ArchText – Architecture Writing Assistant

**Turn rough architectural notes into organised, structured project documentation.**

ArchText is a command-line writing assistant designed for architecture and interior design students, graduates, and young professionals.

It helps users organise architectural project notes, analyse the completeness of project information, count text content, and export structured documentation in Markdown format.

## The Problem

Architecture students often collect project information in scattered notes, making it difficult to organise their ideas into clear and professional project descriptions.

ArchText addresses this problem by providing a simple command-line tool that automatically organises architectural notes into meaningful categories and checks whether important project information is included.

## Features

- **Word Counter:** Counts words, characters, lines, and paragraphs.
- **Note Organiser:** Sorts architectural notes into predefined categories.
- **Markdown Export:** Exports organised notes into a Markdown document.
- **Quality Checker:** Checks for important architectural project information and reports missing categories.

## Organisation Categories

ArchText organises notes into five categories:

1. Site and Context
2. Design Concept
3. Design Objectives
4. Architectural Features
5. General Notes

## Quality Checker

The quality checker looks for eight information categories:

- Project location
- Site area
- Design concept
- Design objectives
- Building features
- Sustainability
- Materials
- Project challenges

It provides a completeness score based on keyword detection. This is a basic rule-based check, not an AI evaluation.

## Installation

ArchText is written in Rust.

To run the project locally, install Rust and Cargo, then clone the repository:

```bash
git clone https://github.com/Shorla01/Archtext.git
cd Archtext
```

Build and run:

```bash
cargo build
cargo run -- --help
```

## Usage

Place your architectural project notes in a text file.

Example using the included Oja Moferere project:

**Count text:**

```bash
cargo run -- count oja-moferere.txt
```

**Organise notes:**

```bash
cargo run -- organize oja-moferere.txt
```

**Export organised notes:**

```bash
cargo run -- export oja-moferere.txt
```

This creates an organised Markdown file.

**Check project completeness:**

```bash
cargo run -- check oja-moferere.txt
```

**Display help:**

```bash
cargo run -- --help
```

## Demonstration Project

ArchText includes an example based on the Oja Moferere Market Renovation project in Ondo Town, Nigeria.

The example demonstrates how architectural information about site context, design concepts, building features, accessibility, and sustainability can be organised into structured documentation.

## Technology

- Rust
- Cargo
- Command-line interface
- Markdown

## Target Users

- Architecture students
- Interior design students
- Architecture graduates
- Young architects and designers

## Project Status

ArchText is an educational software project currently under development.

Future improvements may include more advanced text classification, additional export formats, and an improved user interface.

## Author

**Sola Adu**

GitHub: [Shorla01](https://github.com/Shorla01)

---

*ArchText – Making architectural documentation more organised.*
