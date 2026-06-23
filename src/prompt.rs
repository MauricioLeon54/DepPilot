use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

pub struct Prompt;

impl Prompt {
    pub fn confirm(message: &str) -> Result<bool> {
        Ok(Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(message)
            .default(true)
            .interact()?)
    }

    /// Display the default commit message and let the user edit it.
    /// Returns None when the user clears the field, signalling "skip this commit".
    pub fn edit_commit_message(default_msg: &str) -> Result<Option<String>> {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Commit message (Enter to accept, clear to skip)")
            .default(default_msg.to_string())
            .allow_empty(true)
            .interact_text()?;
        if input.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(input))
        }
    }

    /// Present a list and return the selected index.
    pub fn select(prompt: &str, items: &[&str]) -> Result<usize> {
        Ok(Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(items)
            .default(0)
            .interact()?)
    }
}
