use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("triz").unwrap()
}

#[test]
fn help_shows_usage() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("triz"));
}

#[test]
fn version_shows_version() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn ping_human_mode() {
    cmd()
        .arg("ping")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pong! triz is alive"));
}

#[test]
fn ping_json_mode_has_success_status() {
    cmd()
        .args(["--output", "json", "ping"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"success\""));
}

#[test]
fn ping_json_mode_has_command_name() {
    cmd()
        .args(["--output", "json", "ping"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"command\": \"ping\""));
}

#[test]
fn ping_json_mode_has_data() {
    cmd()
        .args(["--output", "json", "ping"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"message\": \"triz is alive\""));
}

#[test]
fn ping_json_mode_no_stderr_noise() {
    cmd()
        .args(["--output", "json", "ping"])
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

#[test]
fn no_subcommand_shows_error() {
    cmd().assert().failure();
}

#[test]
fn unknown_subcommand_fails() {
    cmd().arg("nonexistent").assert().failure();
}

// ── Error path tests (robustness) ─────────────────────────────

#[test]
fn json_mode_bad_config_produces_json_error_on_stdout() {
    // CKSPEC-OUT-002: errors in JSON mode MUST be JSON envelopes on stdout
    cmd()
        .args([
            "--output",
            "json",
            "--config",
            "/nonexistent/config.toml",
            "ping",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"status\": \"error\""))
        .stdout(predicate::str::contains("\"error\""));
}

#[test]
fn json_mode_error_envelope_identifies_failing_subcommand() {
    // CKSPEC-OUT-003: the envelope's `command` field MUST identify
    // the failing subcommand so downstream consumers can correlate
    // envelopes to commands. A hardcoded placeholder (e.g. "init")
    // violates the spirit of this requirement even though the envelope
    // is structurally valid.
    cmd()
        .args([
            "--output",
            "json",
            "--config",
            "/nonexistent/config.toml",
            "ping",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("\"status\": \"error\""))
        .stdout(predicate::str::contains("\"command\": \"ping\""));
}

#[test]
fn json_mode_error_has_no_stderr() {
    // JSON mode: stderr must be clean even on errors
    cmd()
        .args([
            "--output",
            "json",
            "--config",
            "/nonexistent/config.toml",
            "ping",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::is_empty());
}

#[test]
fn human_mode_error_goes_to_stderr() {
    // Human mode: errors go to stderr, not stdout
    cmd()
        .args(["--config", "/nonexistent/config.toml", "ping"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn json_verbose_no_stderr_leak() {
    // --json + --verbose: verbose must not leak debug logs to stderr
    cmd()
        .args(["--output", "json", "--verbose", "ping"])
        .assert()
        .success()
        .stderr(predicate::str::is_empty());
}

// ── parameter-search ──────────────────────────────────────────

#[test]
fn parameter_search_human_mode() {
    // "durability" ranks Strength (param 14) among the matches.
    cmd()
        .args(["parameter-search", "durability"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Strength"))
        .stdout(predicate::str::contains("14"));
}

#[test]
fn parameter_search_json_mode_has_success_status() {
    cmd()
        .args(["--output", "json", "parameter-search", "durability"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"success\""));
}

#[test]
fn parameter_search_json_mode_has_command_name() {
    cmd()
        .args(["--output", "json", "parameter-search", "durability"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\"command\": \"parameter-search\"",
        ));
}

#[test]
fn parameter_search_json_mode_has_matches_array() {
    cmd()
        .args(["--output", "json", "parameter-search", "durability"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"matches\""))
        .stdout(predicate::str::contains("\"number\""));
}

#[test]
fn parameter_search_no_match_human_mode() {
    cmd()
        .args(["parameter-search", "asdfqwerzzz"])
        .assert()
        .success()
        .stdout(predicate::str::contains("engineering-domain"));
}

// ── formulate-contradiction ───────────────────────────────────

#[test]
fn formulate_contradiction_human_mode() {
    cmd()
        .args([
            "formulate-contradiction",
            "--improving",
            "weight",
            "--worsening",
            "strength",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Technical"))
        .stdout(predicate::str::contains("Strength"));
}

#[test]
fn formulate_contradiction_json_mode_is_technical_with_both_params() {
    cmd()
        .args([
            "--output",
            "json",
            "formulate-contradiction",
            "--improving",
            "weight",
            "--worsening",
            "strength",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"success\""))
        .stdout(predicate::str::contains(
            "\"command\": \"formulate-contradiction\"",
        ))
        .stdout(predicate::str::contains("\"kind\": \"technical\""))
        .stdout(predicate::str::contains("\"number\": 1"))
        .stdout(predicate::str::contains("\"number\": 14"));
}

#[test]
fn formulate_contradiction_same_param_is_physical() {
    cmd()
        .args([
            "--output",
            "json",
            "formulate-contradiction",
            "--improving",
            "reliability",
            "--worsening",
            "reliability",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"kind\": \"physical\""));
}
