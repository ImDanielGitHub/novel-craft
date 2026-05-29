use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

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
    ));
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
        .stdout(predicate::str::contains("novel-next-chapter"));
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
