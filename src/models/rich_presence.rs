#![cfg(feature = "rich_presence")]

use std::default::Default;
use super::shared::PartialUser;
use crate::utils;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SetActivityArgs {
    pid: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<Activity>,
}

impl SetActivityArgs {
    pub fn new<F>(f: F) -> Self
        where F: FnOnce(Activity) -> Activity
    {
        Self { pid: utils::pid(), activity: Some(f(Activity::new())) }
    }
}

impl Default for SetActivityArgs {
    fn default() -> Self {
        Self { pid: utils::pid(), activity: None }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SendActivityJoinInviteArgs {
    pub user_id: String,
}

pub type CloseActivityRequestArgs = SendActivityJoinInviteArgs;

impl SendActivityJoinInviteArgs {
    pub fn new(user_id: u64) -> Self {
        Self { user_id: user_id.to_string() }
    }
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ActivityButtons(Vec<ActivityButton>);

impl ActivityButtons {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_button<S>(mut self, label: S, url: S) -> Self 
    where S: Into<String>
    {
        if self.0.len() >= 2 {
            panic!("Cannot add more than 2 buttons");
        }
        self.0.push(ActivityButton::new(label, url));
        self
    }
}

impl Default for ActivityButtons {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct ActivityButton {
    pub label: String,
    pub url: String
}

impl ActivityButton {
    fn new<S>(label: S, url: S) -> Self 
    where S: Into<String> 
    {
        Self { label: label.into(), url: url.into() }
    }
}

builder!{ActivityJoinEvent
    secret: String,
}

builder!{ActivitySpectateEvent
    secret: String,
}

builder!{ActivityJoinRequestEvent
    user: PartialUser,
}


builder!{Activity
    state: String,
    details: String,
    instance: bool,
    timestamps: ActivityTimestamps func,
    assets: ActivityAssets func,
    party: ActivityParty func,
    buttons: ActivityButtons func,
    secrets: ActivitySecrets func,
}

builder!{ActivityTimestamps
    start: u64,
    end: u64,
}

builder!{ActivityAssets
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
}

builder!{ActivityParty
    id: String,
    size: (u32, u32),
}

builder!{ActivitySecrets
    join: String,
    spectate: String,
    game: String alias = "match",
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    #[should_panic]
    fn check_buttons_panic() {
        ActivityButtons::new()
        .add_button("Rusty", "https://example.com/")
        .add_button("Rusty 2", "https://example.com/")
        .add_button("Rusty 3", "https://example.com/");
    }

    #[test]
    fn can_serialize_full_activity() {
        let expected = include_str!("../../tests/fixtures/activity_full.json");

        let activity = Activity::new()
            .state("rusting")
            .details("detailed")
            .instance(true)
            .timestamps(|t| t
                .start(1000)
                .end(2000))
            .assets(|a| a
                .large_image("ferris")
                .large_text("Ferris")
                .small_image("rusting")
                .small_text("Rusting..."))
            .party(|p| p
                .id("1")
                .size((3, 6)))
            .secrets(|s| s
                .join("025ed05c71f639de8bfaa0d679d7c94b2fdce12f")
                .spectate("e7eb30d2ee025ed05c71ea495f770b76454ee4e0")
                .game("4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"))
            .buttons(|b| b.add_button("Rusty", "https://example.com/"));

        let json = serde_json::to_string_pretty(&activity).unwrap();
        assert_eq!(expected.replace("\r", ""), json);
    }

    #[test]
    fn can_serialize_empty_activity() {
        let activity = Activity::new();
        let json = serde_json::to_string(&activity).unwrap();
        assert_eq![json, "{}"];
    }
}
