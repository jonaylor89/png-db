#[cfg(feature = "cli")]
mod cli {
    use clap::{Parser, Subcommand};
    use color_eyre::Result;
    use png_db::{PngDatabase, Schema};
    use serde_json::Value;
    use std::collections::HashMap;

    #[derive(Parser)]
    #[command(name = "png-db")]
    #[command(about = "A simple database that stores JSON data in PNG files")]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        Create {
            #[arg(short, long)]
            file: String,
            #[arg(short, long, default_value = "256")]
            width: u32,
            #[arg(long, default_value = "256")]
            height: u32,
            #[arg(short, long)]
            schema: String,
        },
        Insert {
            #[arg(short, long)]
            file: String,
            #[arg(short, long)]
            x: u32,
            #[arg(short, long)]
            y: u32,
            #[arg(short, long)]
            data: String,
        },
        Query {
            #[arg(short, long)]
            file: String,
            #[arg(short, long)]
            where_clause: String,
        },
        List {
            #[arg(short, long)]
            file: String,
        },
    }

    pub fn run() -> Result<()> {
        color_eyre::install()?;

        let cli = Cli::parse();

        match cli.command {
            Commands::Create { file, width, height, schema } => {
                let schema_map = parse_schema(&schema)?;
                let schema = Schema { fields: schema_map };
                PngDatabase::create_empty_png(width, height, schema, &file)?;
                println!("Created database: {}", file);
            }
            Commands::Insert { file, x, y, data } => {
                let mut db = PngDatabase::load_from_png(&file)?;
                let json_data: Value = serde_json::from_str(&data)?;
                db.insert(x, y, json_data)?;
                db.save_to_png(&file)?;
                println!("Inserted data at ({}, {})", x, y);
            }
            Commands::Query { file, where_clause } => {
                let db = PngDatabase::load_from_png(&file)?;
                let results = db.query(&where_clause)?;

                if results.is_empty() {
                    println!("No results found");
                } else {
                    println!("Found {} result(s):", results.len());
                    for row in results {
                        println!("  Position ({}, {}): {}", row.x, row.y, serde_json::to_string_pretty(&row.data)?);
                    }
                }
            }
            Commands::List { file } => {
                let db = PngDatabase::load_from_png(&file)?;
                println!("Database: {} ({}x{})", file, db.width, db.height);
                println!("Schema: {:?}", db.schema.fields);
                println!("Rows: {}", db.rows.len());

                for row in &db.rows {
                    println!("  Position ({}, {}): {}", row.x, row.y, serde_json::to_string(&row.data)?);
                }
            }
        }

        Ok(())
    }

    fn parse_schema(schema_str: &str) -> Result<HashMap<String, String>> {
        let mut schema = HashMap::new();

        for field_def in schema_str.split(',') {
            let parts: Vec<&str> = field_def.trim().split(':').collect();
            if parts.len() == 2 {
                schema.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
            }
        }

        Ok(schema)
    }
}

#[cfg(feature = "cli")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run().map_err(|e| e.into())
}

#[cfg(not(feature = "cli"))]
fn main() {
    // This binary is not meant to be run without the CLI feature
    eprintln!("This binary requires the 'cli' feature to be enabled.");
    std::process::exit(1);
}
