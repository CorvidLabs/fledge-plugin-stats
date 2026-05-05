// fledge-plugin-stats: example plugin demonstrating the fledge-v1 protocol.
// Zero external dependencies — uses only stdlib for JSON handling via simple helpers.

use std::io::{self, BufRead, Write};

fn send(msg: &str) {
    println!("{}", msg);
    io::stdout().flush().unwrap();
}

fn recv() -> String {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    lines.next().unwrap().unwrap()
}

/// Extract a string value for a given key from a flat or nested JSON string.
/// This is a simple parser that finds `"key":"value"` patterns and handles
/// escaped characters in the value.
fn json_str(key: &str, json: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", key);
    let start = json.find(&pattern)? + pattern.len();
    let mut end = start;
    let bytes = json.as_bytes();
    while end < bytes.len() {
        if bytes[end] == b'"' && (end == start || bytes[end - 1] != b'\\') {
            break;
        }
        end += 1;
    }
    Some(json[start..end].replace("\\n", "\n").replace("\\\"", "\""))
}

/// Extract a nested JSON object/value for a given key.
/// Returns the raw JSON substring for the value (object, array, string, or primitive).
fn json_object(key: &str, json: &str) -> Option<String> {
    let pattern = format!("\"{}\":", key);
    let idx = json.find(&pattern)?;
    let after_key = idx + pattern.len();
    let rest = json[after_key..].trim_start();
    let first = rest.as_bytes().first()?;
    match first {
        b'{' => {
            let mut depth = 0;
            let mut i = 0;
            let bytes = rest.as_bytes();
            while i < bytes.len() {
                match bytes[i] {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(rest[..=i].to_string());
                        }
                    }
                    b'"' => {
                        i += 1;
                        while i < bytes.len() && bytes[i] != b'"' {
                            if bytes[i] == b'\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            None
        }
        _ => json_str(key, json).map(|s| s.to_string()),
    }
}

fn exec(id: &str, command: &str) -> String {
    send(&format!(
        "{{\"type\":\"exec\",\"id\":\"{}\",\"command\":\"{}\"}}",
        id,
        command.replace('\\', "\\\\").replace('"', "\\\"")
    ));
    recv()
}

fn exec_stdout(id: &str, command: &str) -> String {
    let resp = exec(id, command);
    // Response format: {"type":"response","id":"...","value":{"code":0,"stdout":"...","stderr":"..."}}
    // Extract the value object first, then get stdout from it
    if let Some(value_obj) = json_object("value", &resp) {
        json_str("stdout", &value_obj).unwrap_or_default()
    } else {
        json_str("stdout", &resp).unwrap_or_default()
    }
}

fn store(key: &str, value: &str) {
    send(&format!(
        "{{\"type\":\"store\",\"key\":\"{}\",\"value\":\"{}\"}}",
        key,
        value.replace('\\', "\\\\").replace('"', "\\\"")
    ));
    // store is fire-and-forget — no response from host
}

fn load(id: &str, key: &str) -> String {
    send(&format!(
        "{{\"type\":\"load\",\"id\":\"{}\",\"key\":\"{}\"}}",
        id, key
    ));
    let resp = recv();
    // Response format: {"type":"response","id":"...","value":"..."}
    // value may be a string or null
    json_str("value", &resp).unwrap_or_default()
}

fn log(level: &str, message: &str) {
    send(&format!(
        "{{\"type\":\"log\",\"level\":\"{}\",\"message\":\"{}\"}}",
        level,
        message.replace('\\', "\\\\").replace('"', "\\\"")
    ));
}

fn output(text: &str) {
    send(&format!(
        "{{\"type\":\"output\",\"text\":\"{}\"}}",
        text.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
    ));
}

fn progress(message: &str, current: u64, total: u64) {
    send(&format!(
        "{{\"type\":\"progress\",\"message\":\"{}\",\"current\":{},\"total\":{}}}",
        message, current, total
    ));
}

fn progress_done() {
    send("{\"type\":\"progress\",\"done\":true}");
}

fn metadata(id: &str, keys: &[&str]) -> String {
    let keys_json: Vec<String> = keys.iter().map(|k| format!("\"{}\"", k)).collect();
    send(&format!(
        "{{\"type\":\"metadata\",\"id\":\"{}\",\"keys\":[{}]}}",
        id,
        keys_json.join(",")
    ));
    recv()
}

fn main() {
    let init_line = recv();

    // The init message has structure: {"type":"init","protocol":"fledge-v1","project":{"name":"...","language":"...",...},...}
    // Extract the project object first, then get name/language from it.
    let project_json = json_object("project", &init_line).unwrap_or_default();
    let project_name = json_str("name", &project_json).unwrap_or_else(|| "unknown".into());
    let project_lang = json_str("language", &project_json).unwrap_or_else(|| "unknown".into());

    log("info", &format!("Analyzing project: {}", project_name));

    // Step 1: Count source files by extension
    progress("Counting files", 1, 5);

    let find_cmd = if cfg!(windows) {
        "dir /s /b /a-d"
    } else {
        "find . -type f -not -path './.git/*' -not -path './target/*' -not -path './node_modules/*'"
    };
    let files_output = exec_stdout("files", find_cmd);
    let all_files: Vec<&str> = files_output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    let mut rs_count = 0u64;
    let mut ts_count = 0u64;
    let mut js_count = 0u64;
    let mut py_count = 0u64;
    let mut go_count = 0u64;
    let mut md_count = 0u64;
    let mut toml_count = 0u64;
    let mut other_count = 0u64;

    for f in &all_files {
        if f.ends_with(".rs") {
            rs_count += 1;
        } else if f.ends_with(".ts") || f.ends_with(".tsx") {
            ts_count += 1;
        } else if f.ends_with(".js") || f.ends_with(".jsx") {
            js_count += 1;
        } else if f.ends_with(".py") {
            py_count += 1;
        } else if f.ends_with(".go") {
            go_count += 1;
        } else if f.ends_with(".md") {
            md_count += 1;
        } else if f.ends_with(".toml") {
            toml_count += 1;
        } else {
            other_count += 1;
        }
    }

    // Step 2: Lines of code (primary language)
    progress("Counting lines of code", 2, 5);

    let loc_cmd = match project_lang.as_str() {
        "rust" => "find . -name '*.rs' -not -path './target/*' -exec cat {} + | wc -l",
        "typescript" => "find . -name '*.ts' -o -name '*.tsx' | grep -v node_modules | xargs cat 2>/dev/null | wc -l",
        "python" => "find . -name '*.py' -exec cat {} + | wc -l",
        "go" => "find . -name '*.go' -exec cat {} + | wc -l",
        _ => "find . -type f \\( -name '*.rs' -o -name '*.ts' -o -name '*.py' -o -name '*.go' \\) -exec cat {} + 2>/dev/null | wc -l",
    };
    let loc_output = exec_stdout("loc", loc_cmd);
    let loc: u64 = loc_output
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    // Step 3: Git statistics
    progress("Gathering git info", 3, 5);

    let _meta_resp = metadata("meta1", &["git_status", "git_tags"]);

    let commit_count_output = exec_stdout("commits", "git rev-list --count HEAD 2>/dev/null");
    let commit_count: u64 = commit_count_output.trim().parse().unwrap_or(0);

    let authors_output = exec_stdout(
        "authors",
        "git shortlog -sn --no-merges HEAD 2>/dev/null | head -5",
    );
    let top_authors: Vec<String> = authors_output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .collect();

    let recent_output = exec_stdout("recent", "git log --oneline -5 --no-merges 2>/dev/null");

    // Step 4: Test file ratio
    progress("Analyzing test coverage", 4, 5);

    let test_files: u64 = all_files
        .iter()
        .filter(|f| f.contains("test") || f.contains("spec") || f.ends_with("_test.go"))
        .count() as u64;

    // Step 5: Load previous stats and compare
    progress("Comparing with previous run", 5, 5);

    let prev_loc = load("prev_loc", "loc");
    let prev_files = load("prev_files", "total_files");
    let prev_loc_num: i64 = prev_loc.parse().unwrap_or(0);
    let prev_files_num: i64 = prev_files.parse().unwrap_or(0);

    let loc_delta = loc as i64 - prev_loc_num;
    let files_delta = all_files.len() as i64 - prev_files_num;

    progress_done();

    // Store current stats for next run
    store("loc", &loc.to_string());
    store("total_files", &all_files.len().to_string());
    store("last_run", &timestamp_now());

    // Render output
    output("\n");
    output("  ╭─────────────────────────────────────────╮\n");
    output(&format!(
        "  │  {:<16} stats                 │\n",
        truncate(&project_name, 16)
    ));
    output("  ├─────────────────────────────────────────┤\n");
    output(&format!(
        "  │  Language    {:<12}              │\n",
        truncate(&project_lang, 12)
    ));
    output(&format!(
        "  │  Files       {:>6} {:<10}│\n",
        all_files.len(),
        delta_str(files_delta)
    ));
    output(&format!(
        "  │  LOC         {:>6} {:<10}│\n",
        loc,
        delta_str(loc_delta)
    ));
    output(&format!(
        "  │  Commits     {:>6}                    │\n",
        commit_count
    ));
    output(&format!(
        "  │  Test files  {:>6} ({:.0}%)               │\n",
        test_files,
        if all_files.is_empty() {
            0.0
        } else {
            test_files as f64 / all_files.len() as f64 * 100.0
        }
    ));
    output("  ├─────────────────────────────────────────┤\n");

    // File breakdown
    let counts = [
        ("Rust", rs_count),
        ("TypeScript", ts_count),
        ("JavaScript", js_count),
        ("Python", py_count),
        ("Go", go_count),
        ("Markdown", md_count),
        ("TOML", toml_count),
        ("Other", other_count),
    ];
    for (name, count) in &counts {
        if *count > 0 {
            output(&format!(
                "  │  {:<12} {:>5}                    │\n",
                name, count
            ));
        }
    }

    output("  ├─────────────────────────────────────────┤\n");

    // Top authors
    if !top_authors.is_empty() {
        output("  │  Top contributors                      │\n");
        for author in top_authors.iter().take(3) {
            output(&format!("  │    {:<36}│\n", truncate(author, 36)));
        }
        output("  ├─────────────────────────────────────────┤\n");
    }

    // Recent commits
    output("  │  Recent commits                        │\n");
    for line in recent_output.lines().take(5) {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            output(&format!("  │    {:<36}│\n", truncate(trimmed, 36)));
        }
    }

    output("  ╰─────────────────────────────────────────╯\n");
    output("\n");

    log("info", "Stats complete");
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max - 3])
    } else {
        s.to_string()
    }
}

fn delta_str(delta: i64) -> String {
    if delta == 0 {
        String::new()
    } else if delta > 0 {
        format!("(+{})", delta)
    } else {
        format!("({})", delta)
    }
}

fn timestamp_now() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    secs.to_string()
}
