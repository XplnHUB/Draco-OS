use clap::{Parser, Subcommand};
use draco_voice::VoicePipeline;
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "draco-voice")]
#[command(about = "Draco OS Voice Command Interface")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Process a voice command from text input")]
    Text {
        #[arg(help = "Command text to process")]
        command: String,
    },
    #[command(about = "Process audio file (requires whisper feature)")]
    Audio {
        #[arg(help = "Path to audio file")]
        path: String,
    },
    #[command(about = "Show recent command history")]
    History {
        #[arg(short, long, default_value = "10", help = "Number of recent commands to show")]
        count: usize,
    },
    #[command(about = "Clear command history")]
    ClearHistory,
    #[command(about = "Test the 5 required commands")]
    Test,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();
    let mut pipeline = VoicePipeline::new()?;

    match cli.command {
        Commands::Text { command } => {
            let result = pipeline.process_text(&command)?;
            println!("{}", result);
        }
        Commands::Audio { path } => {
            let result = pipeline.process_audio(&path)?;
            println!("{}", result);
        }
        Commands::History { count } => {
            let recent = pipeline.recent_commands(count);
            println!("Recent commands:");
            for (i, entry) in recent.iter().enumerate() {
                println!("{}. [{}] {} -> {}", 
                    i + 1, 
                    entry.timestamp.format("%H:%M:%S"),
                    entry.input,
                    entry.result
                );
            }
        }
        Commands::ClearHistory => {
            pipeline.clear_history()?;
            println!("Command history cleared");
        }
        Commands::Test => {
            run_tests(&mut pipeline)?;
        }
    }

    Ok(())
}

fn run_tests(pipeline: &mut VoicePipeline) -> Result<()> {
    println!("🧪 Testing Draco OS Voice Pipeline\n");
    
    let test_commands = vec![
        "open firefox",
        "open terminal", 
        "close this",
        "check ram",
        "shutdown",
    ];

    for (i, cmd) in test_commands.iter().enumerate() {
        println!("{}. Testing: {}", i + 1, cmd);
        let result = pipeline.process_text(cmd)?;
        println!("   Result: {}\n", result);
    }

    println!("✅ All required commands tested!");
    Ok(())
}
