const express = require('express');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = 8080;
const SCORES_FILE = path.join(__dirname, 'scores.json');

app.use(cors());
app.use(express.json());

// Helper to read scores
function readScores() {
    try {
        if (!fs.existsSync(SCORES_FILE)) {
            return [];
        }
        const data = fs.readFileSync(SCORES_FILE, 'utf8');
        return JSON.parse(data);
    } catch (err) {
        console.error('Error reading scores:', err);
        return [];
    }
}

// Helper to save scores
function saveScores(scores) {
    try {
        fs.writeFileSync(SCORES_FILE, JSON.stringify(scores, null, 2));
    } catch (err) {
        console.error('Error saving scores:', err);
    }
}

// GET /api/scores
app.get('/api/scores', (req, res) => {
    let scores = readScores();
    // Sort descending by score
    scores.sort((a, b) => b.score - a.score);
    // Return top 10
    res.json(scores.slice(0, 10));
});

// POST /api/scores
app.post('/api/scores', (req, res) => {
    const { name, score } = req.body;
    if (!name || score === undefined) {
        return res.status(400).json({ error: 'Name and score are required' });
    }

    const newScore = {
        name: String(name),
        score: Number(score)
    };

    const scores = readScores();
    scores.push(newScore);
    saveScores(scores);

    res.json('Score saved');
});

// Serve static files from the parent directory (repo root)
const rootDir = path.join(__dirname, '..');
app.use(express.static(rootDir));

// Catch-all for other requests (optional, mostly for SPA routing if needed, but here we just serve files)
// If we want to replicate Actix's behavior of serving index.html for unknown routes or just 404
// The Actix code did: .service(Files::new("/", "../").index_file("index.html"))
// express.static automatically serves index.html if found in root.
// If a file is not found, we can send index.html or 404.
// For now, let's keep it simple.

app.listen(PORT, '0.0.0.0', () => {
    console.log(`Server running at http://0.0.0.0:${PORT}/`);
});
