use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Read};

#[derive(Deserialize)]
struct SessionInput {
    #[serde(rename = "topicsCovered")]
    topics_covered: Vec<String>,
    #[serde(rename = "problemsAttempted")]
    problems_attempted: u32,
    #[serde(rename = "problemsCorrect")]
    problems_correct: u32,
    summary: String,
    #[serde(rename = "weakPointsIdentified")]
    weak_points_identified: Vec<String>,
    /// トピックごとの理解度 (0-100)
    #[serde(rename = "topicUnderstandings")]
    topic_understandings: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize)]
struct SessionFile {
    id: String,
    date: String,
    #[serde(rename = "topicsCovered")]
    topics_covered: Vec<String>,
    #[serde(rename = "problemsAttempted")]
    problems_attempted: u32,
    #[serde(rename = "problemsCorrect")]
    problems_correct: u32,
    summary: String,
    #[serde(rename = "weakPointsIdentified")]
    weak_points_identified: Vec<String>,
}

fn repo_root() -> String {
    // このバイナリは scripts/save_progress/target/release/ に置かれる想定。
    // ただし実行時のカレントディレクトリがリポジトリルートであることを前提にする。
    ".".to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read failed");

    let session_input: SessionInput =
        serde_json::from_str(&input).expect("invalid JSON input");

    let now_utc = Utc::now();
    let now_local = Local::now();

    let session_id = now_local.format("%Y-%m-%dT%H%M%S").to_string();
    let iso_date = now_utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let today = now_local.format("%Y-%m-%d").to_string();

    let root = repo_root();
    let sessions_dir = format!("{}/data/sessions", root);
    let progress_path = format!("{}/data/progress.json", root);

    // --- セッションファイルを作成 ---
    let session_file = SessionFile {
        id: session_id.clone(),
        date: iso_date.clone(),
        topics_covered: session_input.topics_covered.clone(),
        problems_attempted: session_input.problems_attempted,
        problems_correct: session_input.problems_correct,
        summary: session_input.summary.clone(),
        weak_points_identified: session_input.weak_points_identified.clone(),
    };

    fs::create_dir_all(&sessions_dir).expect("failed to create sessions dir");
    let session_path = format!("{}/{}.json", sessions_dir, session_id);
    let session_json = serde_json::to_string_pretty(&session_file).unwrap();
    fs::write(&session_path, &session_json).expect("failed to write session file");
    println!("セッションファイル作成: {}", session_path);

    // --- progress.json を更新 ---
    let progress_raw = fs::read_to_string(&progress_path).expect("failed to read progress.json");
    let mut progress: Value = serde_json::from_str(&progress_raw).expect("invalid progress.json");

    let topics_obj = progress["topics"].as_object_mut().expect("topics is not object");

    for topic in &session_input.topics_covered {
        let new_understanding = session_input
            .topic_understandings
            .get(topic)
            .copied()
            .unwrap_or(60);

        let entry = topics_obj.entry(topic).or_insert_with(|| {
            serde_json::json!({
                "understanding": new_understanding,
                "attempts": 0,
                "correct": 0,
                "weakPoints": [],
                "lastPracticed": today
            })
        });

        // understanding: weighted average (初回以外)
        if let Some(prev) = entry["understanding"].as_u64() {
            let updated = (prev as f64 * 0.6 + new_understanding as f64 * 0.4).round() as u64;
            entry["understanding"] = Value::from(updated);
        }

        // attempts / correct: 累積加算
        // トピックごとの問題数は session 全体を均等分配（最善の近似）
        let n = session_input.topics_covered.len() as u32;
        let topic_attempted = session_input.problems_attempted / n;
        let topic_correct = session_input.problems_correct / n;

        if let Some(v) = entry["attempts"].as_u64() {
            entry["attempts"] = Value::from(v + topic_attempted as u64);
        }
        if let Some(v) = entry["correct"].as_u64() {
            entry["correct"] = Value::from(v + topic_correct as u64);
        }

        // weakPoints: union（重複除去）
        let existing: HashSet<String> = entry["weakPoints"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        let mut merged: Vec<String> = existing.into_iter().collect();
        for wp in &session_input.weak_points_identified {
            if !merged.contains(wp) {
                merged.push(wp.clone());
            }
        }
        entry["weakPoints"] = Value::Array(merged.into_iter().map(Value::String).collect());

        entry["lastPracticed"] = Value::String(today.clone());

        println!("トピック更新: {} (理解度→{})", topic, entry["understanding"]);
    }

    // overallStats を更新
    let stats = progress["overallStats"].as_object_mut().expect("overallStats is not object");
    if let Some(v) = stats["totalSessions"].as_u64() {
        stats["totalSessions"] = Value::from(v + 1);
    }
    if let Some(v) = stats["totalProblems"].as_u64() {
        stats["totalProblems"] = Value::from(v + session_input.problems_attempted as u64);
    }
    if let Some(v) = stats["totalCorrect"].as_u64() {
        stats["totalCorrect"] = Value::from(v + session_input.problems_correct as u64);
    }

    progress["lastUpdated"] = Value::String(iso_date);

    let progress_json = serde_json::to_string_pretty(&progress).unwrap();
    fs::write(&progress_path, progress_json).expect("failed to write progress.json");
    println!("progress.json 更新完了");
}
