use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardEntry {
    name: String,
    score: u32,
}

fn get_leaderboard() -> Result<Vec<LeaderboardEntry>, Box<dyn std::error::Error>> {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    if let Ok(Some(json)) = storage.get_item("snake3d_scores") {
        let entries: Vec<LeaderboardEntry> = serde_json::from_str(&json)?;
        Ok(entries)
    } else {
        Ok(Vec::new())
    }
}

pub fn save_score(name: &str, score: u32) {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();

    let mut entries = get_leaderboard().unwrap_or_default();
    entries.push(LeaderboardEntry { name: name.to_string(), score });
    // Sort by score descending
    entries.sort_by(|a, b| b.score.cmp(&a.score));
    // Keep top 10
    if entries.len() > 10 {
        entries.truncate(10);
    }

    if let Ok(json) = serde_json::to_string(&entries) {
        let _ = storage.set_item("snake3d_scores", &json);
    }

    update_leaderboard_ui();
}

pub fn update_leaderboard_ui() {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Some(list) = document.get_element_by_id("leaderboard-list") {
        list.set_inner_html("");

        match get_leaderboard() {
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
}
