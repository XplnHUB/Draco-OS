use crate::Intent;

pub struct IntentParser {
    app_aliases: Vec<(Vec<String>, String)>,
}

impl IntentParser {
    pub fn new() -> Self {
        let app_aliases = vec![
            (vec!["firefox".into(), "browser".into()], "firefox".into()),
            (vec!["terminal".into(), "console".into()], "orbterm".into()),
            (vec!["files".into(), "file manager".into()], "filemanager".into()),
            (vec!["code".into(), "editor".into(), "vscode".into()], "code".into()),
            (vec!["minecraft".into(), "game".into()], "minecraft".into()),
            (vec!["settings".into(), "preferences".into()], "settings".into()),
            (vec!["calculator".into(), "calc".into()], "calculator".into()),
        ];
        Self { app_aliases }
    }

    pub fn parse(&self, input: &str) -> Intent {
        let text = input.to_lowercase().trim().to_string();
        if text.is_empty() { return Intent::Unknown("(empty)".into()); }

        self.try_open(&text)
            .or_else(|| self.try_close(&text))
            .or_else(|| self.try_info(&text))
            .or_else(|| self.try_power(&text))
            .or_else(|| self.try_switch(&text))
            .or_else(|| self.try_volume(&text))
            .or_else(|| self.try_lock(&text))
            .unwrap_or(Intent::Unknown(input.into()))
    }

    fn resolve_app(&self, name: &str) -> String {
        for (aliases, canonical) in &self.app_aliases {
            if aliases.iter().any(|a| a == name) { return canonical.clone(); }
        }
        name.to_string()
    }

    fn match_app(&self, text: &str) -> Option<String> {
        for (aliases, canonical) in &self.app_aliases {
            if aliases.iter().any(|a| text.contains(a.as_str())) {
                return Some(canonical.clone());
            }
        }
        None
    }

    fn try_open(&self, t: &str) -> Option<Intent> {
        for kw in &["open","launch","start","run","execute"] {
            if t.starts_with(kw) {
                let rem = t[kw.len()..].trim();
                if !rem.is_empty() { return Some(Intent::OpenApp(self.resolve_app(rem))); }
            }
        }
        self.match_app(t).map(Intent::OpenApp)
    }

    fn try_close(&self, t: &str) -> Option<Intent> {
        let pats = ["close this","close app","close window","quit this","quit app","kill this","exit app"];
        if pats.iter().any(|p| t.contains(p)) { Some(Intent::CloseActiveApp) } else { None }
    }

    fn try_info(&self, t: &str) -> Option<Intent> {
        let kws = ["check","show","what is","tell me","how much","status"];
        let targets = [
            (&["ram","memory","mem"][..], "ram"),
            (&["cpu","processor"][..], "cpu"),
            (&["battery","power","charge"][..], "battery"),
            (&["disk","storage","space"][..], "disk"),
            (&["network","internet","wifi"][..], "network"),
            (&["temperature","temp"][..], "temperature"),
        ];
        let has_kw = kws.iter().any(|k| t.contains(k));
        for (ts,name) in &targets {
            if ts.iter().any(|x| t.contains(x)) && (has_kw || t == *name) {
                return Some(Intent::GetSystemInfo((*name).into()));
            }
        }
        None
    }

    fn try_power(&self, t: &str) -> Option<Intent> {
        if t.contains("shutdown") || t.contains("shut down") || t.contains("power off") { return Some(Intent::Shutdown); }
        if t.contains("reboot") || t.contains("restart") { return Some(Intent::Reboot); }
        if t.contains("sleep") || t.contains("suspend") { return Some(Intent::Sleep); }
        None
    }

    fn try_switch(&self, t: &str) -> Option<Intent> {
        for kw in &["switch to","go to","focus"] {
            if t.starts_with(kw) {
                let rem = t[kw.len()..].trim();
                if !rem.is_empty() { return Some(Intent::SwitchApp(self.resolve_app(rem))); }
            }
        }
        None
    }

    fn try_volume(&self, t: &str) -> Option<Intent> {
        if t.contains("volume up") || t.contains("louder") { return Some(Intent::VolumeUp); }
        if t.contains("volume down") || t.contains("quieter") { return Some(Intent::VolumeDown); }
        if t.contains("mute") || t.contains("unmute") { return Some(Intent::ToggleMute); }
        None
    }

    fn try_lock(&self, t: &str) -> Option<Intent> {
        if t.contains("lock screen") || t.contains("lock computer") { return Some(Intent::LockScreen); }
        None
    }
}

impl Default for IntentParser {
    fn default() -> Self { Self::new() }
}
