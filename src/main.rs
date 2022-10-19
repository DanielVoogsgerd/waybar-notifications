use std::process::Command;
use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
struct MakoNotificationField<T> {
    data: T
}

#[derive(Serialize, Deserialize, Debug)]
struct MakoNotification {
    #[serde(rename = "app-name")]
    app_name: MakoNotificationField<String>,
    #[serde(rename = "app-icon")]
    app_icon: MakoNotificationField<String>,
    category: MakoNotificationField<String>,
    summary: MakoNotificationField<String>,
    body: MakoNotificationField<String>,
    id: MakoNotificationField<u32>,
    actions: MakoNotificationField<Value>,
}

#[derive(Deserialize, Debug)]
struct MakoctlResponse {
    data: Vec<Vec<MakoNotification>>
}

#[derive(Serialize)]
struct WaybarResponse {
    text: String,
    tooltip: String,
    class: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let command_output = Command::new("makoctl").arg("list").output()?;
    let response: MakoctlResponse = serde_json::from_slice(&command_output.stdout)?;

    let notifications = response.data.into_iter().next().expect("Data format has changed");

    let waybar_response = WaybarResponse {
	text: notifications.len().to_string(),
	tooltip: notifications.iter().map(|y| {y.summary.data.clone()}).collect::<Vec<_>>().join("\n"),
	class: vec![]
    };

    let output = serde_json::to_string(&waybar_response).expect("Cannot create output from notifications");

    println!("{}", output);

    Ok(())
}
