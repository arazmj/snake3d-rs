use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_files::Files;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs::{self, File};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Score {
    name: String,
    score: u32,
}

struct AppState {
    scores: Mutex<Vec<Score>>,
    file_path: String,
}

impl AppState {
    fn load(file_path: &str) -> Self {
        let scores = if let Ok(data) = fs::read_to_string(file_path) {
            serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };
        Self {
            scores: Mutex::new(scores),
            file_path: file_path.to_string(),
        }
    }

    fn save(&self) {
        let scores = self.scores.lock().unwrap();
        if let Ok(json) = serde_json::to_string_pretty(&*scores) {
            let _ = fs::write(&self.file_path, json);
        }
    }
}

#[get("/api/scores")]
async fn get_scores(data: web::Data<AppState>) -> impl Responder {
    let mut scores = data.scores.lock().unwrap().clone();
    // Sort descending
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    // Return top 10
    scores.truncate(10);
    HttpResponse::Ok().json(scores)
}

#[post("/api/scores")]
async fn add_score(data: web::Data<AppState>, score: web::Json<Score>) -> impl Responder {
    let mut scores = data.scores.lock().unwrap();
    scores.push(score.into_inner());
    // Sort and keep top 10 (or keep all and filter on read? better to keep all for history, filter on read)
    // But user said "stored in server", "board sorted".
    // Let's keep all in file, but maybe cap it if it gets too big?
    // For now, just append.

    // Release lock before saving? No, need consistency.
    // Drop mutex guard?
    drop(scores);

    data.save();
    HttpResponse::Ok().json("Score saved")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::load("scores.json"));

    println!("Server running at http://127.0.0.1:8080/");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Cors::permissive()) // Allow CORS for dev if needed
            .service(get_scores)
            .service(add_score)
            // Serve static files from the root of the repo (parent of server directory)
            // We assume we run the binary from within `server/` or we point to `../`
            // Better to serve `../` as root.
            .service(Files::new("/", "../").index_file("index.html"))
            .default_service(web::route().to(|req: actix_web::HttpRequest| {
                println!("Request not found: {:?}", req);
                HttpResponse::NotFound()
            }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
