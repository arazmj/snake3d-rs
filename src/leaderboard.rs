
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardEntry {
    name: String,
    score: u32,
}

fn get_leaderboard() -> Vec<LeaderboardEntry> {
    let window = web_sys::window().unwrap();
    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(json)) = storage.get_item("snake3d_leaderboard") {
            if let Ok(entries) = serde_json::from_str::<Vec<LeaderboardEntry>>(&json) {
                return entries;
            }
        }
    }
    Vec::new()
}

pub fn save_score(name: &str, score: u32) {
    let mut entries = get_leaderboard();
    entries.push(LeaderboardEntry {
        name: name.to_string(),
        score,
    });
    // Sort descending
    entries.sort_by(|a, b| b.score.cmp(&a.score));
    // Keep top 10
    entries.truncate(10);

    if let Ok(json) = serde_json::to_string(&entries) {
        let window = web_sys::window().unwrap();
        if let Ok(Some(storage)) = window.local_storage() {
             storage.set_item("snake3d_leaderboard", &json).unwrap_or(());
        }
    }
}

pub fn update_leaderboard_ui() {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Some(list) = document.get_element_by_id("leaderboard-list") {
        list.set_inner_html(""); // Clear
        let entries = get_leaderboard();

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
    }
}
