
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardEntry {
    name: String,
    score: u32,
}

async fn get_leaderboard() -> Result<Vec<LeaderboardEntry>, Box<dyn std::error::Error>> {
    // We construct the absolute URL to avoid issues if base URL is somehow weird in tests,
    // but relative should work if served from same origin.
    // However, let's just use relative "/api/scores".
    let client = reqwest::Client::new();
    let resp = client.get("/api/scores").send().await?;

    if !resp.status().is_success() {
        return Err(format!("Request failed with status: {}", resp.status()).into());
    }

    let entries = resp.json::<Vec<LeaderboardEntry>>().await?;
    Ok(entries)
}

pub fn save_score(name: &str, score: u32) {
    let name = name.to_string();
    spawn_local(async move {
        let client = reqwest::Client::new();
        let entry = LeaderboardEntry { name, score };
        let _ = client.post("/api/scores").json(&entry).send().await;
        // After save, update UI
        update_leaderboard_ui();
    });
}

pub fn update_leaderboard_ui() {
    let document = web_sys::window().unwrap().document().unwrap();

    // Show loading state?
    if let Some(list) = document.get_element_by_id("leaderboard-list") {
        list.set_inner_html("<li>Loading...</li>");
    }

    spawn_local(async move {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(list) = document.get_element_by_id("leaderboard-list") {
            list.set_inner_html(""); // Clear

            match get_leaderboard().await {
                Ok(entries) => {
                    if entries.is_empty() {
                         let li = document.create_element("li").unwrap();
                         li.set_text_content(Some("No scores yet!"));
                         li.set_attribute("style", "justify-content: center; color: #888;").unwrap_or(());
                         list.append_child(&li).unwrap();
                    } else {
                        for (i, entry) in entries.iter().enumerate() {
                            let li = document.create_element("li").unwrap();

                            let name_span = document.create_element("span").unwrap();
                            name_span.set_text_content(Some(&format!("{}. {}", i + 1, entry.name)));

                            let score_span = document.create_element("span").unwrap();
                            score_span.set_text_content(Some(&entry.score.to_string()));
                            score_span.set_attribute("style", "color: #ffeb3b;").unwrap_or(());

                            li.append_child(&name_span).unwrap();
                            li.append_child(&score_span).unwrap();
                            list.append_child(&li).unwrap();
                        }
                    }
                },
                Err(_) => {
                    let li = document.create_element("li").unwrap();
                    li.set_text_content(Some("Failed to load scores."));
                    li.set_attribute("style", "justify-content: center; color: #f44;").unwrap_or(());
                    list.append_child(&li).unwrap();
                }
            }
        }
    });
}
