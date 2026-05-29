use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use serde_json::Value;

const CLI_ASSISTED_STORY: &str = r#"
Novel Craft 0.1.0 opened in a clean terminal.
The maintainer kept the key, coin, gate, bridge, and scar in the story because each one had a cost.
The release answered with proof instead of applause.
"#;

const NO_CLI_STORY: &str = r#"
Novel Craft 1.0.0 opened a city in the terminal.
I saw bread beside the bridge.
I noticed a gate, a map, a ring, a rope, and a well.
The release was marked by a choir of commands.
The story was counted as a triumph because trust shimmered in the prompt.
"#;

#[test]
fn creative_tournament_outputs_json_for_agents() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    cmd.args([
        "creative",
        "tournament",
        "--idea",
        "weak-to-strong kingdom-building isekai",
        "--count",
        "2",
        "--json",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains(
        "weak-to-strong kingdom-building isekai",
    ))
    .stdout(predicate::str::contains("wider story engine"));
}

#[test]
fn agent_plan_outputs_chapter_workflow_for_agents() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args([
            "agent",
            "plan",
            "--idea",
            "weak to strong kingdom-building system",
            "--chapters",
            "1",
            "--genre",
            "system-isekai",
            "--profile",
            "fast-webnovel",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["mode"], "agent_chapter_plan");
    assert_eq!(data["task_facts"]["chapters_requested"], 1);
    assert!(data["missing_story_questions"].is_array());
    assert!(data["contender_generation_rules"]
        .as_array()
        .expect("contender rules")
        .iter()
        .any(|item| item.as_str().unwrap_or("").contains("8-12 contenders")));
    assert!(data["drafting_instructions"]
        .as_array()
        .expect("drafting instructions")
        .iter()
        .any(|item| item.as_str().unwrap_or("").contains("finished prose")));
    assert!(data["post_write_commands"]
        .as_array()
        .expect("post write commands")
        .iter()
        .any(|item| item.as_str().unwrap_or("").contains("eval chapter")));
    let rendered = String::from_utf8(output).expect("utf8");
    for forbidden in [
        "KDP",
        "SFWA",
        "Royal Road",
        "Kindle Vella",
        "Radish",
        "Wattpad",
    ] {
        assert!(!rendered.contains(forbidden), "found {forbidden}");
    }
}

#[test]
fn agent_plan_supports_multiple_chapter_cards() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args([
            "agent",
            "plan",
            "--idea",
            "a frontier oath system with a lost heir mystery",
            "--chapters",
            "3",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    let cards = data["chapter_cards"].as_array().expect("chapter cards");
    assert_eq!(cards.len(), 3);
    for card in cards {
        assert!(card["goal"].as_str().unwrap_or("").contains("POV"));
        assert!(card["conflict"]
            .as_str()
            .unwrap_or("")
            .contains("resistance"));
        assert!(card["turn"].as_str().unwrap_or("").contains("change"));
        assert!(card["cost"].as_str().unwrap_or("").contains("cost"));
        assert!(card["exit_hook"]
            .as_str()
            .unwrap_or("")
            .contains("continuation"));
        assert!(card["open_loop_guidance"]
            .as_str()
            .unwrap_or("")
            .contains("question"));
    }
}

#[test]
fn creative_atlas_has_fifty_items_per_category() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args(["creative", "atlas", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["genres"].as_array().expect("genres").len(), 50);
    assert_eq!(data["subgenres"].as_array().expect("subgenres").len(), 50);
    assert_eq!(data["tropes"].as_array().expect("tropes").len(), 50);
    assert_eq!(data["subtropes"].as_array().expect("subtropes").len(), 50);
    assert!(data["quality_standard"]
        .as_array()
        .expect("quality standard")
        .iter()
        .any(|item| item.as_str().unwrap_or("").contains("banger first chapter")));
    assert!(data["quality_standard"]
        .as_array()
        .expect("quality standard")
        .iter()
        .any(|item| item.as_str().unwrap_or("").contains("wider story engine")));
}

#[test]
fn creative_brief_uses_always_on_excellence_standard() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    cmd.args([
        "creative",
        "brief",
        "--idea",
        "a dock worker bonds with a forbidden storm beast",
        "--genre",
        "general-fiction",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains(
        "Always-On Novel Excellence Standard",
    ))
    .stdout(predicate::str::contains("banger first chapter"))
    .stdout(predicate::str::contains("micro-promise"))
    .stdout(predicate::str::contains("eval story"))
    .stdout(predicate::str::contains("wider story engine"))
    .stdout(predicate::str::contains("creative atlas"));
}

#[test]
fn gate_warns_when_opening_announces_macro_premise_too_early() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let draft = temp.child("macro.md");
    draft
        .write_str("This is a kingdom-building system novel. The system is for building a kingdom. Class: Kingmaker. Rank: sovereign. Citizens: 0. Domain seed detected. The future empire will unlock taxes, laws, levels, and upgrade ladders.")
        .expect("write fixture");

    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args(["eval", "gate", draft.path().to_str().unwrap(), "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["status"], "warn");
    assert_eq!(data["opening_promise"]["status"], "warn");
    assert!(!data["opening_promise"]["announcement_hits"]
        .as_object()
        .expect("announcement hits")
        .is_empty());
}

#[test]
fn gate_accepts_micro_promise_opening_before_macro_scale() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let draft = temp.child("micro.md");
    draft
        .write_str("The boy woke with a copper coin under his tongue. Snow fell through the roof. A girl stole bread for her brother. He had one choice: hold the door or let the collectors take them.")
        .expect("write fixture");

    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args(["eval", "gate", draft.path().to_str().unwrap(), "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["opening_promise"]["status"], "pass");
    assert_eq!(data["status"], "pass");
}

#[test]
fn eval_story_reviews_existing_markdown_without_gate_language() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let draft = temp.child("chapter.md");
    draft
        .write_str("The boy woke with a copper coin under his tongue. Snow fell through the roof. A girl stole bread for her brother. He had one choice: hold the door or let the collectors take them.")
        .expect("write fixture");

    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args([
            "eval",
            "story",
            draft.path().to_str().unwrap(),
            "--genre",
            "system-isekai",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["mode"], "post_write_story_review");
    assert_eq!(data["not_a_gate"], true);
    assert!(data["guidance_policy"]
        .as_array()
        .expect("guidance policy")
        .iter()
        .any(|item| item
            .as_str()
            .unwrap_or("")
            .contains("not automatic rewrite")));
    assert!(data["opening_promise"].is_object());
    assert!(data["chapter_spine"].is_object());
    assert!(data["scene_change"].is_object());
    assert!(data["reader_retention"].is_object());
    assert!(data["prose_review"].is_object());
    assert!(data["voice_review"].is_object());
    assert!(data["open_loops"].is_object());
    assert!(data["trope_saturation"].is_object());
    assert!(data["dimensions"].is_array());
}

#[test]
fn eval_chapter_alias_returns_chapter_focused_review() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let draft = temp.child("chapter.md");
    draft
        .write_str("This is a kingdom-building system novel. The system is for building a kingdom. Class: Kingmaker. Rank: sovereign. Citizens: 0. Domain seed detected.")
        .expect("write fixture");

    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args(["eval", "chapter", draft.path().to_str().unwrap(), "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["mode"], "post_write_chapter_review");
    assert_eq!(data["document_kind"], "chapter");
    assert_eq!(data["not_a_gate"], true);
    let rendered = String::from_utf8(output).expect("utf8");
    assert!(rendered.contains("exposition pressure and delayed story motion"));
}

#[test]
fn start_no_input_creates_project_and_packet() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    cmd.current_dir(temp.path())
        .args(["start", "--no-input", "--defaults", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("start-packet.md"));

    temp.child(".novel/project.yml")
        .assert(predicate::path::exists());
    temp.child(".novel/context/start-packet.md")
        .assert(predicate::path::exists());
    temp.child(".novel/rules/default.yml")
        .assert(predicate::path::exists());
}

#[test]
fn embedded_skills_are_listable() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    cmd.args(["skills", "list", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("novel-craft-agentic-writer"))
        .stdout(predicate::str::contains("novel-craft-next-chapter"))
        .stdout(predicate::str::contains("deprecated_alias"));
}

#[test]
fn doctor_reports_install_and_model_boundary() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    cmd.args(["doctor", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("prompt-packets-only"))
        .stdout(predicate::str::contains("target_triple"));
}

#[test]
fn general_writing_guide_covers_naming_and_docs() {
    let mut show = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = show
        .args(["writing", "show", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["id"], "novel-craft-writing-support");
    assert!(data["review_pass"]
        .as_array()
        .expect("review pass")
        .iter()
        .any(|item| item.as_str().unwrap_or("").contains("rename")));

    let mut guide = Command::cargo_bin("novel-craft").expect("binary exists");
    guide
        .args(["writing", "guide"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Final Gate"))
        .stdout(predicate::str::contains("busy person"))
        .stdout(predicate::str::contains("commands and next steps"));
}

#[test]
fn novelty_default_reports_signals_not_scores() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let draft = temp.child("draft.md");
    draft.write_str(CLI_ASSISTED_STORY).expect("write fixture");

    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args([
            "creative",
            "novelty",
            draft.path().to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert!(data["novelty"]["lexical_novelty_signals"].is_object());
    assert!(data["novelty"]["novelty_score_estimate"].is_null());
    assert!(data["novelty"]["experimental_score"].is_null());
}

#[test]
fn gate_fails_lexical_trap_that_misses_required_fact() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let draft = temp.child("trap.md");
    draft
        .write_str("A key, coin, bread, rope, map, ring, gate, well, bridge, scar, wound, and torch glittered without the signed checksum.")
        .expect("write fixture");

    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args([
            "eval",
            "gate",
            draft.path().to_str().unwrap(),
            "--must-include",
            "verified checksum",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["status"], "fail");
    assert_eq!(data["constraint_adherence"]["status"], "fail");
    assert!(
        data["novelty"]["lexical_novelty_signals"]["signal_counts"]["concrete_object_categories"]
            .as_u64()
            .unwrap()
            >= 8
    );
}

#[test]
fn compare_exposes_better_lexical_signal_but_worse_gate_and_lint() {
    let temp = assert_fs::TempDir::new().expect("temp dir");
    let a = temp.child("cli-assisted.md");
    let b = temp.child("no-cli.md");
    a.write_str(CLI_ASSISTED_STORY).expect("write a");
    b.write_str(NO_CLI_STORY).expect("write b");

    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args([
            "eval",
            "compare",
            a.path().to_str().unwrap(),
            b.path().to_str().unwrap(),
            "--must-include",
            "Novel Craft 0.1.0",
            "--must-avoid",
            "1.0.0",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert!(data["winner"].is_null());
    assert_eq!(data["a"]["constraint_adherence"]["status"], "pass");
    assert_eq!(data["b"]["constraint_adherence"]["status"], "fail");
    assert!(
        data["b"]["lint"]["issue_count"].as_u64().unwrap()
            > data["a"]["lint"]["issue_count"].as_u64().unwrap()
    );
    assert!(
        data["b"]["novelty"]["lexical_novelty_signals"]["signal_counts"]
            ["concrete_object_categories"]
            .as_u64()
            .unwrap()
            > data["a"]["novelty"]["lexical_novelty_signals"]["signal_counts"]
                ["concrete_object_categories"]
                .as_u64()
                .unwrap()
    );
}

#[test]
fn tech_fantasy_tournament_does_not_emit_isekai_axes() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = cmd
        .args([
            "creative",
            "tournament",
            "--idea",
            "launch-night tech fantasy",
            "--genre",
            "tech-fantasy-celebration",
            "--count",
            "2",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    let packet = data["packet"].as_str().expect("packet");
    assert!(packet.contains("tech-fantasy-celebration"));
    assert!(!packet.contains("summoned as the wrong hero"));
    assert!(!packet.contains("reincarnated"));
    assert!(!packet.contains("tutorial dungeon"));
    assert!(!packet.contains("adventurer guild"));
}

#[test]
fn breakout_serial_profiles_are_available() {
    let mut tropes = Command::cargo_bin("novel-craft").expect("binary exists");
    let output = tropes
        .args(["creative", "tropes", "--genre", "breakout-serial", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data: Value = serde_json::from_slice(&output).expect("valid json");
    assert_eq!(data["genre"], "breakout-serial");
    assert!(data["tropes"]["breakout_gate"]
        .as_array()
        .expect("breakout gate")
        .iter()
        .any(|item| item.as_str().unwrap_or("").contains("adaptation")));

    let mut brief = Command::cargo_bin("novel-craft").expect("binary exists");
    brief
        .args([
            "creative",
            "brief",
            "--idea",
            "a poor recruit becomes the first beast master",
            "--genre",
            "beast-bond-progression",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("beast-bond-progression"))
        .stdout(predicate::str::contains("opening wound"))
        .stdout(predicate::str::contains("power cost"));
}

#[test]
fn breakout_rubric_and_reader_profile_include_serial_retention() {
    let mut rubric = Command::cargo_bin("novel-craft").expect("binary exists");
    rubric
        .args(["eval", "rubric", "--genre", "breakout-serial"])
        .assert()
        .success()
        .stdout(predicate::str::contains("serial_retention"))
        .stdout(predicate::str::contains("costly_power"));

    let mut profiles = Command::cargo_bin("novel-craft").expect("binary exists");
    profiles
        .args(["eval", "reader-profiles", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("breakout-serial"))
        .stdout(predicate::str::contains("chapter-end continuation reason"));
}

#[test]
fn creative_brief_carries_constraints_into_packet() {
    let mut cmd = Command::cargo_bin("novel-craft").expect("binary exists");
    cmd.args([
        "creative",
        "brief",
        "--idea",
        "launch-night tech fantasy",
        "--genre",
        "tech-fantasy-celebration",
        "--must-include",
        "novel-craft 0.1.0",
        "--must-avoid",
        "1.0.0",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Must include: novel-craft 0.1.0"))
    .stdout(predicate::str::contains("Must avoid: 1.0.0"))
    .stdout(predicate::str::contains("eval gate"));
}
