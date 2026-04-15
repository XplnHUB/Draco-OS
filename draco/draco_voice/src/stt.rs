use anyhow::{Result, Context};

/// Speech-to-text abstraction. Supports both whisper-rs (when compiled with
/// the `whisper` feature) and a text passthrough for CLI testing.
pub struct SpeechToText {
    #[cfg(feature = "whisper")]
    model: Option<whisper_rs::WhisperContext>,
}

impl SpeechToText {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "whisper")]
            model: None,
        }
    }

    #[cfg(feature = "whisper")]
    pub fn with_model(path: &str) -> Result<Self> {
        let params = whisper_rs::WhisperContextParameters::default();
        let ctx = whisper_rs::WhisperContext::new_with_params(path, params)
            .context("Failed to load whisper model")?;
        Ok(Self { model: Some(ctx) })
    }

    /// Transcribe audio file to text. Falls back to passthrough if no model loaded.
    pub fn transcribe_file(&self, path: &str) -> Result<String> {
        #[cfg(feature = "whisper")]
        if let Some(ref ctx) = self.model {
            return self.whisper_transcribe(ctx, path);
        }
        // Fallback: read file as raw text for testing
        std::fs::read_to_string(path)
            .map(|s| s.trim().to_string())
            .context("No whisper model loaded and file read failed")
    }

    /// Transcribe from raw text input (CLI passthrough mode).
    pub fn transcribe_text(&self, text: &str) -> Result<String> {
        Ok(text.trim().to_string())
    }

    #[cfg(feature = "whisper")]
    fn whisper_transcribe(&self, ctx: &whisper_rs::WhisperContext, path: &str) -> Result<String> {
        let audio = whisper_rs::WhisperAudio::from_wav_file(path)
            .context("Failed to read WAV file")?;
        let mut state = ctx.create_state()?;
        let params = whisper_rs::WhisperSegmentCallbackData::default();
        state.full(params, &audio)?;
        let n_segments = state.full_n_segments()?;
        let mut text = String::new();
        for i in 0..n_segments {
            text.push_str(&state.full_get_segment_text(i)?);
            text.push(' ');
        }
        Ok(text.trim().to_string())
    }
}

impl Default for SpeechToText {
    fn default() -> Self { Self::new() }
}
