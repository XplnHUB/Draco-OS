use draco_intent::{Intent, IntentParser};
use draco_executor::{CommandExecutor, executor::ExecutionResult};
use crate::{SpeechToText, history::{CommandEntry, CommandHistory}};
use anyhow::{Result, Context};
use tracing::{info, warn, error};

/// Main voice command pipeline: Audio → Text → Intent → Execution
pub struct VoicePipeline {
    stt: SpeechToText,
    parser: IntentParser,
    executor: CommandExecutor,
    history: CommandHistory,
}

impl VoicePipeline {
    pub fn new() -> Result<Self> {
        Ok(Self {
            stt: SpeechToText::new(),
            parser: IntentParser::new(),
            executor: CommandExecutor::new(),
            history: CommandHistory::new()?,
        })
    }

    pub fn with_ipc_addr(addr: &str) -> Result<Self> {
        Ok(Self {
            stt: SpeechToText::new(),
            parser: IntentParser::new(),
            executor: CommandExecutor::with_ipc_addr(addr),
            history: CommandHistory::new()?,
        })
    }

    /// Process audio file through the full pipeline
    pub fn process_audio(&mut self, audio_path: &str) -> Result<String> {
        info!("Processing audio: {}", audio_path);
        
        // STT: Audio → Text
        let text = self.stt.transcribe_file(audio_path)
            .context("Speech-to-text failed")?;
        info!("Transcribed: {}", text);
        
        // Process the text through intent → execution pipeline
        self.process_text(&text)
    }

    /// Process raw text input (for CLI testing)
    pub fn process_text(&mut self, text: &str) -> Result<String> {
        info!("Processing text: {}", text);
        
        // Intent Parsing: Text → Intent
        let intent = self.parser.parse(text);
        info!("Parsed intent: {}", intent);
        
        // Execution: Intent → Action
        let result = self.executor.execute(intent.clone());
        info!("Execution result: {}", result);
        
        // History logging
        let success = matches!(result, ExecutionResult::Success(_));
        if let Err(e) = self.history.add(text, &intent.to_string(), &result.to_string(), success) {
            warn!("Failed to log command: {}", e);
        }
        
        Ok(result.to_string())
    }

    /// Get recent command history
    pub fn recent_commands(&self, count: usize) -> &[CommandEntry] {
        self.history.recent(count)
    }

    /// Clear command history
    pub fn clear_history(&mut self) -> Result<()> {
        self.history.clear().context("Failed to clear history")
    }

    /// Initialize whisper model if available
    #[cfg(feature = "whisper")]
    pub fn load_model(&mut self, model_path: &str) -> Result<()> {
        self.stt = SpeechToText::with_model(model_path)?;
        info!("Loaded whisper model: {}", model_path);
        Ok(())
    }
}

impl Default for VoicePipeline {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            error!("Failed to create VoicePipeline: {}", e);
            panic!("VoicePipeline initialization failed");
        })
    }
}
