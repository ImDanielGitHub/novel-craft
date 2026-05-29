use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::{Args, Parser, Subcommand};
use comfy_table::Table;
use regex::Regex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_yaml::{Mapping, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use unicode_segmentation::UnicodeSegmentation;

const PROJECT_DIR: &str = ".novel";
const DEFAULT_RULES: &str = include_str!("../rules/default.yml");

const SKILLS: &[(&str, &str)] = &[
    (
        "novel-character-review/SKILL.md",
        include_str!("../skills/novel-character-review/SKILL.md"),
    ),
    (
        "novel-continuity-sync/SKILL.md",
        include_str!("../skills/novel-continuity-sync/SKILL.md"),
    ),
    (
        "novel-creativity-architect/SKILL.md",
        include_str!("../skills/novel-creativity-architect/SKILL.md"),
    ),
    (
        "novel-dialogue-review/SKILL.md",
        include_str!("../skills/novel-dialogue-review/SKILL.md"),
    ),
    (
        "novel-evaluation-review/SKILL.md",
        include_str!("../skills/novel-evaluation-review/SKILL.md"),
    ),
    (
        "novel-full-book-review/SKILL.md",
        include_str!("../skills/novel-full-book-review/SKILL.md"),
    ),
    (
        "novel-line-edit/SKILL.md",
        include_str!("../skills/novel-line-edit/SKILL.md"),
    ),
    (
        "novel-memory-diff/SKILL.md",
        include_str!("../skills/novel-memory-diff/SKILL.md"),
    ),
    (
        "novel-next-chapter/SKILL.md",
        include_str!("../skills/novel-next-chapter/SKILL.md"),
    ),
    (
        "novel-rulebook-review/SKILL.md",
        include_str!("../skills/novel-rulebook-review/SKILL.md"),
    ),
    (
        "novel-scene-architect/SKILL.md",
        include_str!("../skills/novel-scene-architect/SKILL.md"),
    ),
];

const CREATIVE_METHODS: &[(&str, &str, &str)] = &[
    (
        "diverge_converge",
        "Generate 12-20 distinct versions before choosing.",
        "Score each option for hook, relatability, novelty, genre promise, and expansion potential.",
    ),
    (
        "morphological_matrix",
        "Mix protagonist, wound, world, system rule, power cost, social arena, and pressure.",
        "Build a table of options and combine unusual but coherent cells.",
    ),
    (
        "trope_twist",
        "Use a familiar trope, then change one pressure point.",
        "Name the base trope, stale version, twist, and new consequences.",
    ),
    (
        "constraint_remix",
        "Make power interesting by restricting it.",
        "For every advantage, add a cost, cooldown, social consequence, moral price, or blind spot.",
    ),
    (
        "analogical_transfer",
        "Borrow structure from another domain without copying surface details.",
        "Map a craft, ecosystem, job, sport, myth, or historical pattern onto the story engine.",
    ),
    (
        "inversion",
        "Flip the genre default.",
        "Ask what the genre normally rewards, then make the winning move require the opposite.",
    ),
    (
        "scamper",
        "Substitute, combine, adapt, modify, put to another use, eliminate, or reverse.",
        "Keep only changes that create new conflict.",
    ),
    (
        "triz_contradiction",
        "Make two desirable goals impossible to satisfy at once.",
        "Define the contradiction, then invent a system, cost, or plot turn that resolves it.",
    ),
    (
        "self_refine",
        "Draft, critique, revise, and preserve why the revision is better.",
        "Use rule cards as review guides, not commandments.",
    ),
    (
        "tree_of_thought",
        "Explore several story branches before committing.",
        "Branch hooks, power systems, wounds, and first turns before selecting.",
    ),
    (
        "creative_debate",
        "Prevent one bland idea from winning by default.",
        "Let genre fan, craft editor, novelty critic, and readability editor each attack the premise.",
    ),
    (
        "reading_level_gate",
        "Make prose easier without making the story simpler.",
        "Prefer vivid common words and keep one idea per paragraph beat.",
    ),
];

const TROPE_AXES: &[(&str, &[&str])] = &[
    (
        "entry",
        &[
            "summoned as the wrong hero",
            "reincarnated as a weak local body",
            "falls into a tutorial dungeon",
            "wakes after being sacrificed",
            "transmigrates into a doomed minor villain",
            "arrives as a nameless extra in a prophecy",
        ],
    ),
    (
        "system",
        &[
            "status screen with hidden costs",
            "skill tree that grows from choices, not grinding",
            "class system that mislabels the hero",
            "quest log written by an unreliable patron",
            "inventory that stores debts, memories, or promises",
            "leveling through teaching, repair, cooking, healing, building, or diplomacy",
        ],
    ),
    (
        "weakness",
        &[
            "zero combat stats",
            "curse that makes direct violence fail",
            "body too young, sick, small, old, or injured",
            "language barrier",
            "social rank below notice",
            "power works only when helping someone else",
        ],
    ),
    (
        "growth",
        &[
            "weak skill gains absurd depth through clever use",
            "support class becomes strategic core",
            "crafting/logistics beat brute force",
            "monster ecology knowledge becomes power",
            "trust network becomes a kingdom-scale system",
            "defensive skill turns into territory control",
        ],
    ),
    (
        "social_arena",
        &[
            "adventurer guild",
            "frontier village",
            "beast-tamer caravan",
            "noble academy",
            "dungeon town",
            "ruined shrine city",
            "floating market",
            "border fort",
        ],
    ),
    (
        "pressure",
        &[
            "debt collector with legal authority",
            "rival reincarnator with a louder cheat",
            "monster wave timer",
            "noble family wants the hero as property",
            "church declares the system heresy",
            "guild hides fatal information from beginners",
            "winter arrives before food does",
        ],
    ),
    (
        "freshness_twist",
        &[
            "the system rewards keeping promises, not kills",
            "quests are generated by people who need help, not by gods",
            "leveling makes the hero more responsible, not freer",
            "the weakest class controls supply lines",
            "stats are public, so secrecy matters more than strength",
            "the tutorial is a scam and the monsters know it",
        ],
    ),
];

const EVAL_DIMENSIONS: &[(&str, &str)] = &[
    (
        "hook_and_promise",
        "Does the opening quickly create desire, danger, wonder, injustice, or curiosity?",
    ),
    (
        "readability_and_flow",
        "Can a tired reader follow the prose, system rules, scene turns, and dialogue?",
    ),
    (
        "causal_coherence",
        "Do events follow because/therefore logic instead of merely happening?",
    ),
    (
        "character_agency",
        "Does the protagonist make meaningful choices under pressure?",
    ),
    (
        "character_consistency",
        "Do voice, knowledge, motives, powers, relationships, and behavior stay consistent?",
    ),
    (
        "emotional_depth",
        "Does emotion progress through behavior, pressure, subtext, and choice?",
    ),
    (
        "conflict_handling",
        "Does the scene contain resistance, leverage, power shift, or consequence?",
    ),
    (
        "novelty_and_specificity",
        "Does the draft mix familiar promise with fresh, concrete details?",
    ),
    (
        "voice_and_language",
        "Do word choice, rhythm, dialogue, imagery, and distance fit the POV?",
    ),
    (
        "progression_payoff",
        "Does the chapter advance capability, knowledge, status, stakes, relationship, or promise state?",
    ),
];

const GENERIC_PHRASES: &[&str] = &[
    "little did he know",
    "for as long as he could remember",
    "unlike anything he had ever seen",
    "ancient evil",
    "chosen one",
    "destined for greatness",
    "heart pounded",
    "breath caught",
    "eyes widened",
    "something inside him",
    "the fate of the world",
    "everything changed",
    "in that moment",
];

const FILTER_WORDS: &[&str] = &[
    "saw", "seen", "heard", "felt", "noticed", "realized", "realised", "wondered", "knew",
    "thought", "watched", "looked", "seemed",
];

const EMOTION_WORDS: &[&str] = &[
    "angry",
    "sad",
    "afraid",
    "terrified",
    "scared",
    "ashamed",
    "guilty",
    "lonely",
    "rage",
    "panic",
    "fear",
    "dread",
];

#[derive(Parser)]
#[command(name = "novel-craft")]
#[command(bin_name = "novel-craft")]
#[command(version)]
#[command(about = "A model-neutral writing-quality CLI, starting with long-form fiction.")]
#[command(after_help = "Install:
  npx novel-craft start

Source checkout:
  cargo run --bin novel-craft -- start --no-input --defaults --json
  cargo build --release

Agent-friendly flags:
  --json       emit machine-readable output where supported
  --out PATH   write deterministic packets/reports to a file
  --no-input   prevent interactive prompts on guided commands
  --defaults   accept guided-command defaults

Model boundary:
  Novel Craft emits prompt packets, rubrics, rule guides, and reports only.
  It does not call models, store API keys, scrape hosted fiction, or publish anything for you.

Direction:
  The first public domain pack is long-form fiction. The underlying rule engine is designed to grow into purpose profiles for copy, essays, reports, emails, proposals, product writing, and technical docs.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(
        about = "Run the guided story setup wizard.",
        after_help = "Beginner flow:
  npx novel-craft start

Agent/non-interactive flow:
  novel-craft start --no-input --defaults --json

The wizard creates .novel/ project state and a model-ready start packet. It does not call an LLM."
    )]
    Start(StartArgs),
    Init(InitArgs),
    #[command(about = "Run read-only install, asset, wrapper, and project checks.")]
    Doctor(DoctorArgs),
    Skills {
        #[command(subcommand)]
        command: SkillsCommand,
    },
    Rules {
        #[command(subcommand)]
        command: RulesCommand,
    },
    Creative {
        #[command(subcommand)]
        command: CreativeCommand,
    },
    Eval {
        #[command(subcommand)]
        command: EvalCommand,
    },
    Scene {
        #[command(subcommand)]
        command: SceneCommand,
    },
    Character {
        #[command(subcommand)]
        command: CharacterCommand,
    },
    Plot {
        #[command(subcommand)]
        command: PlotCommand,
    },
    Matrix {
        #[command(subcommand)]
        command: MatrixCommand,
    },
    Context {
        #[command(subcommand)]
        command: ContextCommand,
    },
    Lint {
        #[command(subcommand)]
        command: LintCommand,
    },
    Audit {
        #[command(subcommand)]
        command: AuditCommand,
    },
    Memory {
        #[command(subcommand)]
        command: MemoryCommand,
    },
    Export {
        #[command(subcommand)]
        command: ExportCommand,
    },
    Draft(TargetArgs),
    Next(TargetArgs),
    Analyse(FileReportArgs),
    Review(ReviewArgs),
    Revise(ReviseArgs),
    Diff(DiffArgs),
    FullBook(FullBookArgs),
}

#[derive(Args, Clone)]
struct StartArgs {
    #[arg(long, default_value = "Untitled Novel")]
    title: String,
    #[arg(long, default_value = "weak-to-strong kingdom-building fantasy")]
    idea: String,
    #[arg(long, default_value = "system-isekai")]
    genre: String,
    #[arg(long, default_value = "beginner")]
    writer_level: String,
    #[arg(
        long,
        default_value = "webnovel readers who want fast wonder and clear stakes"
    )]
    audience: String,
    #[arg(long, default_value = "clear, vivid, emotionally direct")]
    tone: String,
    #[arg(long, default_value = "6-8")]
    reading_level: String,
    #[arg(long, default_value = "full workflow packet")]
    desired_output: String,
    #[arg(long)]
    include_trope: Vec<String>,
    #[arg(long)]
    avoid: Vec<String>,
    #[arg(
        long,
        default_value = "protect someone weaker without becoming owned by stronger people"
    )]
    protagonist_want: String,
    #[arg(long, default_value = "believes asking for help creates debt")]
    protagonist_wound: String,
    #[arg(
        long,
        default_value = "frontier settlement on the edge of a failing kingdom"
    )]
    world: String,
    #[arg(
        long,
        default_value = "a system that rewards kept promises, useful repairs, and public trust"
    )]
    power_system: String,
    #[arg(long, default_value = "balanced")]
    autonomy: String,
    #[arg(long)]
    no_input: bool,
    #[arg(long)]
    defaults: bool,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct InitArgs {
    #[arg(long, short, default_value = "Untitled Novel")]
    title: String,
    #[arg(long, short, default_value = "system-isekai")]
    genre: String,
    #[arg(long, default_value = "deep-third")]
    pov: String,
    #[arg(long)]
    force: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Args, Clone)]
struct DoctorArgs {
    #[arg(long)]
    json: bool,
}

#[derive(Args, Clone)]
struct TargetArgs {
    target: Option<String>,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct FileReportArgs {
    path: PathBuf,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct ReviewArgs {
    path: PathBuf,
    #[arg(long, default_value = "all")]
    rubric: String,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct ReviseArgs {
    path: PathBuf,
    #[arg(long, default_value = "prose")]
    pass: String,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct DiffArgs {
    before: PathBuf,
    after: PathBuf,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct FullBookArgs {
    path: PathBuf,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum SkillsCommand {
    List {
        #[arg(long)]
        json: bool,
    },
    Export {
        #[arg(long)]
        out: PathBuf,
    },
    Install {
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    Doctor {
        #[arg(long)]
        target: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum RulesCommand {
    List {
        #[arg(long)]
        json: bool,
    },
    Guide {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Audit {
        #[arg(long)]
        json: bool,
    },
    Refresh {
        #[arg(long)]
        backup: bool,
    },
}

#[derive(Subcommand)]
enum CreativeCommand {
    Methods {
        #[arg(long)]
        json: bool,
    },
    Tropes {
        #[arg(long)]
        json: bool,
    },
    Brief(CreativeBriefArgs),
    Diagnose(FileReportArgs),
    Novelty(FileReportArgs),
    TropeCheck(FileReportArgs),
    Tournament(TournamentArgs),
}

#[derive(Args, Clone)]
struct CreativeBriefArgs {
    #[arg(long)]
    idea: String,
    #[arg(long, default_value = "system-isekai")]
    genre: String,
    #[arg(
        long,
        default_value = "webnovel readers who want fast wonder and clear stakes"
    )]
    audience: String,
    #[arg(long, default_value = "6-8")]
    reading_grade: String,
    #[arg(long)]
    trope: Vec<String>,
    #[arg(long)]
    avoid: Vec<String>,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct TournamentArgs {
    #[arg(long)]
    idea: String,
    #[arg(long, default_value = "system-isekai")]
    genre: String,
    #[arg(long, default_value_t = 8)]
    count: usize,
    #[arg(long)]
    avoid: Vec<String>,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum EvalCommand {
    Rubric {
        #[arg(long, default_value = "system-isekai")]
        genre: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Sheet(FileReportArgs),
    Compare {
        a: PathBuf,
        b: PathBuf,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    ReaderProfiles {
        #[arg(long)]
        json: bool,
    },
    ReaderCheck {
        path: PathBuf,
        #[arg(long, default_value = "fast-webnovel")]
        profile: String,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    VoiceDrift {
        paths: Vec<PathBuf>,
        #[arg(long, default_value = "")]
        character: String,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    FeedbackAdd {
        target: String,
        #[arg(long)]
        rating: u8,
        #[arg(long)]
        comment: String,
        #[arg(long, default_value = "overall")]
        dimension: String,
        #[arg(long, default_value = "beta-reader")]
        reader: String,
    },
    FeedbackReport {
        #[arg(long)]
        json: bool,
    },
    CalibrateAdd {
        path: PathBuf,
        #[arg(long)]
        label: String,
        #[arg(long)]
        reason: Vec<String>,
        #[arg(long)]
        tag: Vec<String>,
    },
    CalibrateReport {
        #[arg(long)]
        json: bool,
    },
    RewardExport {
        a: PathBuf,
        b: PathBuf,
        #[arg(long)]
        winner: String,
        #[arg(long, default_value = "overall")]
        dimension: String,
        #[arg(long, default_value = "")]
        note: String,
        #[arg(long)]
        include_text: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    RewardReport {
        #[arg(long)]
        path: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum SceneCommand {
    Create(Box<SceneCreateArgs>),
    Show {
        id: String,
        #[arg(long)]
        json: bool,
    },
    FromText {
        path: PathBuf,
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Args, Clone)]
struct SceneCreateArgs {
    id: String,
    #[arg(long)]
    chapter: Option<String>,
    #[arg(long)]
    scene: Option<String>,
    #[arg(long, default_value = "")]
    pov: String,
    #[arg(long, default_value = "")]
    location: String,
    #[arg(long, default_value = "")]
    goal: String,
    #[arg(long, default_value = "")]
    conflict: String,
    #[arg(long, default_value = "")]
    turn: String,
    #[arg(long, default_value = "")]
    stakes: String,
    #[arg(long)]
    thread: Vec<String>,
    #[arg(long)]
    promise: Vec<String>,
    #[arg(long)]
    do_not_repeat: Vec<String>,
    #[arg(long)]
    force: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum CharacterCommand {
    Add(CharacterArgs),
    Update(CharacterArgs),
}

#[derive(Args, Clone)]
struct CharacterArgs {
    name: String,
    #[arg(long)]
    age: Option<u16>,
    #[arg(long, default_value = "")]
    appearance: String,
    #[arg(long, default_value = "")]
    voice: String,
    #[arg(long = "trait")]
    trait_: Vec<String>,
    #[arg(long)]
    motive: Vec<String>,
    #[arg(long)]
    wound: Vec<String>,
    #[arg(long)]
    secret: Vec<String>,
    #[arg(long)]
    knowledge: Vec<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum PlotCommand {
    Thread(PlotThreadArgs),
    AddPromise(PlotPromiseArgs),
    Payoff(PlotPayoffArgs),
}

#[derive(Args, Clone)]
struct PlotThreadArgs {
    id: String,
    #[arg(long, default_value = "")]
    title: String,
    #[arg(long = "type", default_value = "plot_thread")]
    type_: String,
    #[arg(long, default_value = "")]
    owner: String,
    #[arg(long, default_value = "introduced")]
    stage: String,
    #[arg(long)]
    appearance: Vec<String>,
    #[arg(long)]
    risk: Vec<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Args, Clone)]
struct PlotPromiseArgs {
    promise: String,
    #[arg(long, default_value = "")]
    source: String,
    #[arg(long)]
    thread: Option<String>,
    #[arg(long, default_value = "")]
    expected_payoff_window: String,
    #[arg(long)]
    json: bool,
}

#[derive(Args, Clone)]
struct PlotPayoffArgs {
    promise_id: String,
    #[arg(long, default_value = "")]
    note: String,
    #[arg(long, default_value = "")]
    source: String,
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum MatrixCommand {
    Build {
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    Audit {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Heatmap {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ContextCommand {
    Build(TargetArgs),
}

#[derive(Subcommand)]
enum LintCommand {
    Line(FileReportArgs),
    Scene(FileReportArgs),
    Plot(FileReportArgs),
}

#[derive(Subcommand)]
enum AuditCommand {
    Continuity(FileReportArgs),
    Repetition(FileReportArgs),
    Causality {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum MemoryCommand {
    Extract {
        path: PathBuf,
        #[arg(long)]
        scene_id: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Commit {
        diff: PathBuf,
    },
    Sync {
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum ExportCommand {
    Html {
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ProjectConfig {
    title: String,
    genre_profile: String,
    pov_mode: String,
    source_policy: String,
    style_anchors: Vec<String>,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Metrics {
    word_count: usize,
    sentence_count: usize,
    paragraph_count: usize,
    dialogue_line_count: usize,
    average_sentence_words: f64,
    flesch_kincaid_grade_estimate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    rule_id: String,
    level: String,
    line: usize,
    message: String,
    excerpt: String,
    suggestion: String,
    classification: String,
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Start(args) => command_start(args),
        Command::Init(args) => command_init(args),
        Command::Doctor(args) => command_doctor(args),
        Command::Skills { command } => command_skills(command),
        Command::Rules { command } => command_rules(command),
        Command::Creative { command } => command_creative(command),
        Command::Eval { command } => command_eval(command),
        Command::Scene { command } => command_scene(command),
        Command::Character { command } => command_character(command),
        Command::Plot { command } => command_plot(command),
        Command::Matrix { command } => command_matrix(command),
        Command::Context { command } => match command {
            ContextCommand::Build(args) => {
                let root = require_project()?;
                let target = args.target.unwrap_or_else(|| "next-scene".to_string());
                let packet = context_packet(&root, &target)?;
                write_or_print(args.out, &packet)
            }
        },
        Command::Lint { command } => command_lint(command),
        Command::Audit { command } => command_audit(command),
        Command::Memory { command } => command_memory(command),
        Command::Export { command } => command_export(command),
        Command::Draft(args) => command_draft(args, "Draft Prompt"),
        Command::Next(args) => command_draft(args, "Next Chapter Prompt"),
        Command::Analyse(args) => command_analyse(args),
        Command::Review(args) => command_review(args),
        Command::Revise(args) => command_revise(args),
        Command::Diff(args) => command_diff(args),
        Command::FullBook(args) => command_full_book(args),
    }
}

fn command_start(mut args: StartArgs) -> Result<()> {
    if !args.no_input && !args.defaults {
        args.title = prompt("Project title", &args.title)?;
        args.writer_level = prompt("Writer level", &args.writer_level)?;
        args.audience = prompt("Target audience", &args.audience)?;
        args.genre = prompt("Genre/profile", &args.genre)?;
        args.idea = prompt("Premise seed", &args.idea)?;
        args.tone = prompt("Tone", &args.tone)?;
        args.reading_level = prompt("Reading level", &args.reading_level)?;
        args.desired_output = prompt("Desired output", &args.desired_output)?;
        args.protagonist_want = prompt("Protagonist want", &args.protagonist_want)?;
        args.protagonist_wound = prompt("Protagonist wound", &args.protagonist_wound)?;
        args.world = prompt("World preference", &args.world)?;
        args.power_system = prompt("Power/system preference", &args.power_system)?;
        args.autonomy = prompt("How much should Novel Craft decide?", &args.autonomy)?;
    }

    init_project(&InitArgs {
        title: args.title.clone(),
        genre: args.genre.clone(),
        pov: "deep-third".to_string(),
        force: false,
        json: false,
    })?;

    let root = project_root()?;
    let brief = start_packet(&args);
    let out_path = args.out.unwrap_or_else(|| {
        root.join(PROJECT_DIR)
            .join("context")
            .join("start-packet.md")
    });
    write_text(&out_path, &brief)?;

    if args.json {
        print_json(json!({
            "status": "ok",
            "project": root.join(PROJECT_DIR),
            "packet": out_path,
            "model_boundary": "prompt-packets-only"
        }))
    } else {
        println!(
            "Novel Craft project ready: {}",
            root.join(PROJECT_DIR).display()
        );
        println!("Start packet written: {}", out_path.display());
        Ok(())
    }
}

fn command_init(args: InitArgs) -> Result<()> {
    init_project(&args)?;
    if args.json {
        print_json(json!({"status": "ok", "project": project_root()?.join(PROJECT_DIR)}))
    } else {
        println!(
            "Novel Craft project ready: {}",
            project_root()?.join(PROJECT_DIR).display()
        );
        Ok(())
    }
}

fn command_doctor(args: DoctorArgs) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let root = project_root()?;
    let has_project = root.join(PROJECT_DIR).exists();
    let source_wrapper = cwd.join("npm").join("bin").join("novel-craft.js");
    let wrapper_env = std::env::var("NOVEL_CRAFT_NPM_WRAPPER").ok();
    let wrapper_path = std::env::var("NOVEL_CRAFT_NPM_WRAPPER_PATH").ok();
    let npm_wrapper_available = wrapper_env.as_deref() == Some("1") || source_wrapper.exists();
    let binary = std::env::current_exe().ok();
    let rules: Value = serde_yaml::from_str(DEFAULT_RULES)?;
    let rule_count = rules.as_sequence().map_or(0, Vec::len);

    let data = json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "binary": binary.as_ref().map(|p| p.display().to_string()),
        "platform": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "target_triple": platform_target_triple()
        },
        "embedded_assets": {
            "rules_available": true,
            "rule_count": rule_count,
            "rule_bytes": DEFAULT_RULES.len(),
            "skill_count": SKILLS.len()
        },
        "project": {
            "cwd": cwd.display().to_string(),
            "project_root": root.display().to_string(),
            "has_project": has_project,
            "project_dir": root.join(PROJECT_DIR).display().to_string()
        },
        "npm": {
            "wrapper_available": npm_wrapper_available,
            "source_wrapper_exists": source_wrapper.exists(),
            "wrapper_env": wrapper_env,
            "wrapper_path": wrapper_path
        },
        "model_boundary": "prompt-packets-only",
        "llm_calls": false,
        "network_calls": false
    });

    if args.json {
        print_json(data)
    } else {
        println!("Novel Craft {}", env!("CARGO_PKG_VERSION"));
        if let Some(path) = binary {
            println!("Binary: {}", path.display());
        }
        println!("Platform: {}", platform_target_triple());
        println!("Embedded rules: {rule_count}");
        println!("Embedded skills: {}", SKILLS.len());
        println!(
            "Project: {} ({})",
            if has_project { "found" } else { "not found" },
            root.join(PROJECT_DIR).display()
        );
        println!(
            "npm wrapper: {}",
            if npm_wrapper_available {
                "available"
            } else {
                "not detected"
            }
        );
        println!("Model boundary: prompt packets only; no API keys or model calls.");
        Ok(())
    }
}

fn init_project(args: &InitArgs) -> Result<()> {
    let root = std::env::current_dir()?;
    let novel = root.join(PROJECT_DIR);
    fs::create_dir_all(&novel)?;
    for folder in [
        "characters",
        "rules",
        "reports",
        "drafts",
        "chapters",
        "scenes",
        "scene-cards",
        "plot-threads",
        "context",
        "pending-memory",
        "state",
        "evals",
    ] {
        fs::create_dir_all(novel.join(folder))?;
    }

    let config_path = novel.join("project.yml");
    if args.force || !config_path.exists() {
        let config = ProjectConfig {
            title: args.title.clone(),
            genre_profile: args.genre.clone(),
            pov_mode: args.pov.clone(),
            source_policy: "Use user-owned drafts, licensed/public-domain references, and browser-observed pattern notes. Do not scrape, train on, or imitate hosted copyrighted novels without permission.".to_string(),
            style_anchors: vec![
                "clear serial momentum".to_string(),
                "concrete sensory detail".to_string(),
                "earned progression".to_string(),
                "low-ornament dialogue tags".to_string(),
            ],
            created_at: now(),
        };
        write_yaml(&config_path, &config)?;
    }
    write_if_missing(
        &novel.join("rules").join("default.yml"),
        DEFAULT_RULES,
        args.force,
    )?;
    write_if_missing(
        &novel.join("plot-matrix.yml"),
        "arcs: []\nchapters: []\nscenes: []\npromises: []\nprogression: []\n",
        args.force,
    )?;
    write_if_missing(
        &novel.join("state").join("knowledge-state.yml"),
        "characters: {}\nreader:\n  knows: []\n  does_not_know: []\n",
        args.force,
    )?;
    write_if_missing(
        &novel.join("state").join("dynamic-state.yml"),
        "current_scene: \"\"\ncharacters: {}\nobjects: {}\nlocations: {}\n",
        args.force,
    )?;
    write_if_missing(
        &novel.join("state").join("style-profile.yml"),
        "distance: deep-third\nvoice: []\nsentence_rhythm: varied\nmotifs: []\ndo_not_overuse:\n  - sighing\n  - jaw clenching\n  - smirking\n  - eye rolling\n",
        args.force,
    )?;
    init_memory(&novel.join("memory.sqlite"))?;
    Ok(())
}

fn command_skills(command: SkillsCommand) -> Result<()> {
    match command {
        SkillsCommand::List { json } => {
            let names: Vec<&str> = SKILLS.iter().map(|(name, _)| *name).collect();
            if json {
                print_json(json!({ "skills": names }))
            } else {
                for name in names {
                    println!("{name}");
                }
                Ok(())
            }
        }
        SkillsCommand::Export { out } => {
            for (name, body) in SKILLS {
                write_text(&out.join(name), body)?;
            }
            println!("Skills exported: {}", out.display());
            Ok(())
        }
        SkillsCommand::Install { target, dry_run } => {
            for (name, body) in SKILLS {
                let path = target.join(name);
                if dry_run {
                    println!("would write {}", path.display());
                } else {
                    write_text(&path, body)?;
                    println!("wrote {}", path.display());
                }
            }
            Ok(())
        }
        SkillsCommand::Doctor { target, json } => {
            let data = json!({
                "embedded_skill_count": SKILLS.len(),
                "target": target.as_ref().map(|p| p.display().to_string()),
                "target_exists": target.as_ref().map(|p| p.exists()),
                "model_boundary": "skills call CLI and produce packets for any model"
            });
            if json {
                print_json(data)
            } else {
                println!("Embedded skills: {}", SKILLS.len());
                if let Some(path) = target {
                    println!("Target exists: {} ({})", path.exists(), path.display());
                }
                Ok(())
            }
        }
    }
}

fn command_rules(command: RulesCommand) -> Result<()> {
    match command {
        RulesCommand::List { json } => {
            let rules: Value = serde_yaml::from_str(DEFAULT_RULES)?;
            if json {
                print_json(rules_to_json(&rules))
            } else {
                let mut table = Table::new();
                table.set_header(vec!["ID", "Level", "Severity", "Effect"]);
                if let Some(items) = rules.as_sequence() {
                    for rule in items {
                        table.add_row(vec![
                            cell(rule, "id"),
                            cell(rule, "level"),
                            cell(rule, "severity"),
                            cell(rule, "effect"),
                        ]);
                    }
                }
                println!("{table}");
                Ok(())
            }
        }
        RulesCommand::Guide { json, out } => {
            let guide = rule_guide(json)?;
            write_or_print(out, &guide)
        }
        RulesCommand::Audit { json } => {
            let rules: Value = serde_yaml::from_str(DEFAULT_RULES)?;
            let mut rows = Vec::new();
            if let Some(items) = rules.as_sequence() {
                for rule in items {
                    rows.push(json!({
                        "id": cell(rule, "id"),
                        "has_examples": rule.get("examples").is_some(),
                        "has_counterexamples": rule.get("counterexamples").is_some() || rule.get("good_when").is_some(),
                    }));
                }
            }
            if json {
                print_json(json!({ "rules": rows }))
            } else {
                for row in rows {
                    println!("{row}");
                }
                Ok(())
            }
        }
        RulesCommand::Refresh { backup } => {
            let root = require_project()?;
            let path = root.join(PROJECT_DIR).join("rules").join("default.yml");
            if backup && path.exists() {
                let backup_path =
                    path.with_extension(format!("{}.bak.yml", now().replace(':', "-")));
                fs::copy(&path, &backup_path)?;
                println!("Backed up rules: {}", backup_path.display());
            }
            write_text(&path, DEFAULT_RULES)?;
            println!("Rules refreshed: {}", path.display());
            Ok(())
        }
    }
}

fn command_creative(command: CreativeCommand) -> Result<()> {
    match command {
        CreativeCommand::Methods { json } => {
            let methods: Vec<_> = CREATIVE_METHODS
                .iter()
                .map(|(id, use_case, instruction)| json!({"id": id, "use": use_case, "instruction": instruction}))
                .collect();
            if json {
                print_json(json!({ "methods": methods }))
            } else {
                for method in methods {
                    println!("{method}");
                }
                Ok(())
            }
        }
        CreativeCommand::Tropes { json } => {
            let data = tropes_json();
            if json {
                print_json(data)
            } else {
                println!("{}", serde_yaml::to_string(&data)?);
                Ok(())
            }
        }
        CreativeCommand::Brief(args) => {
            let text = creative_brief(
                &args.idea,
                &args.audience,
                &args.reading_grade,
                &args.trope,
                &args.avoid,
            );
            write_or_print(args.out, &text)
        }
        CreativeCommand::Diagnose(args) => report_file(
            args,
            |path, text| json!({"path": path, "metrics": metrics(text), "word_choice": word_choice_diagnostics(text)}),
        ),
        CreativeCommand::Novelty(args) => report_file(
            args,
            |path, text| json!({"path": path, "novelty": novelty_analysis(text)}),
        ),
        CreativeCommand::TropeCheck(args) => report_file(
            args,
            |path, text| json!({"path": path, "trope_saturation": trope_saturation(text)}),
        ),
        CreativeCommand::Tournament(args) => {
            let text = tournament_text(&args.idea, args.count, &args.avoid);
            if args.json {
                print_json(json!({"idea": args.idea, "count": args.count, "packet": text}))
            } else {
                write_or_print(args.out, &text)
            }
        }
    }
}

fn command_eval(command: EvalCommand) -> Result<()> {
    match command {
        EvalCommand::Rubric { genre, out } => write_or_print(out, &rubric_text(&genre)),
        EvalCommand::Sheet(args) => {
            let text = read_text(&args.path)?;
            let packet = eval_sheet(&args.path, &text);
            if args.json {
                print_json(
                    json!({"path": args.path, "metrics": metrics(&text), "dimensions": dimensions_json()}),
                )
            } else {
                write_or_print(args.out, &packet)
            }
        }
        EvalCommand::Compare { a, b, json, out } => {
            let a_text = read_text(&a)?;
            let b_text = read_text(&b)?;
            let packet = compare_text(&a, &b, &a_text, &b_text);
            if json {
                print_json(json!({
                    "a": a,
                    "b": b,
                    "a_metrics": metrics(&a_text),
                    "b_metrics": metrics(&b_text),
                    "dimensions": dimensions_json()
                }))
            } else {
                write_or_print(out, &packet)
            }
        }
        EvalCommand::ReaderProfiles { json } => {
            let data = reader_profiles();
            if json {
                print_json(data)
            } else {
                println!("{}", serde_yaml::to_string(&data)?);
                Ok(())
            }
        }
        EvalCommand::ReaderCheck {
            path,
            profile,
            json,
            out,
        } => {
            let text = read_text(&path)?;
            let data = reader_check(&text, &profile);
            if json {
                print_json(data)
            } else {
                write_or_print(
                    out,
                    &format!(
                        "# Reader Profile Check\n\n{}",
                        serde_yaml::to_string(&data)?
                    ),
                )
            }
        }
        EvalCommand::VoiceDrift {
            paths,
            character,
            json,
            out,
        } => {
            let data = voice_drift(&paths, &character)?;
            if json {
                print_json(data)
            } else {
                write_or_print(
                    out,
                    &format!("# Voice Drift Check\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
        EvalCommand::FeedbackAdd {
            target,
            rating,
            comment,
            dimension,
            reader,
        } => {
            if !(1..=5).contains(&rating) {
                bail!("rating must be between 1 and 5");
            }
            let root = require_project()?;
            let path = root
                .join(PROJECT_DIR)
                .join("state")
                .join("beta-feedback.yml");
            let mut rows = read_yaml_value(&path).unwrap_or_else(|| json_yaml_map("feedback"));
            push_yaml_entry(
                &mut rows,
                "feedback",
                json!({
                    "target": target,
                    "rating": rating,
                    "comment": comment,
                    "dimension": dimension,
                    "reader": reader,
                    "created_at": now()
                }),
            )?;
            write_yaml_value(&path, &rows)?;
            println!("Feedback stored.");
            Ok(())
        }
        EvalCommand::FeedbackReport { json } => state_report("beta-feedback.yml", "feedback", json),
        EvalCommand::CalibrateAdd {
            path,
            label,
            reason,
            tag,
        } => {
            let root = require_project()?;
            let out = root
                .join(PROJECT_DIR)
                .join("state")
                .join("taste-profile.yml");
            let text = read_text(&path)?;
            let mut rows = read_yaml_value(&out).unwrap_or_else(|| json_yaml_map("samples"));
            push_yaml_entry(
                &mut rows,
                "samples",
                json!({
                    "path": path,
                    "label": label,
                    "reason": reason,
                    "tag": tag,
                    "metrics": metrics(&text),
                    "created_at": now()
                }),
            )?;
            write_yaml_value(&out, &rows)?;
            println!("Taste sample stored.");
            Ok(())
        }
        EvalCommand::CalibrateReport { json } => state_report("taste-profile.yml", "samples", json),
        EvalCommand::RewardExport {
            a,
            b,
            winner,
            dimension,
            note,
            include_text,
            out,
        } => {
            let root = project_root()?;
            let out_path = out.unwrap_or_else(|| {
                root.join(PROJECT_DIR)
                    .join("evals")
                    .join("reward-pairs.jsonl")
            });
            let a_text = read_text(&a)?;
            let b_text = read_text(&b)?;
            let record = json!({
                "version_a": a,
                "version_b": b,
                "winner": winner,
                "dimension": dimension,
                "note": note,
                "metrics": {"a": metrics(&a_text), "b": metrics(&b_text)},
                "text_a": if include_text { Some(a_text.as_str()) } else { None },
                "text_b": if include_text { Some(b_text.as_str()) } else { None },
                "created_at": now()
            });
            append_line(&out_path, &serde_json::to_string(&record)?)?;
            println!("Reward pair appended: {}", out_path.display());
            Ok(())
        }
        EvalCommand::RewardReport { path, json } => {
            let root = project_root()?;
            let p = path.unwrap_or_else(|| {
                root.join(PROJECT_DIR)
                    .join("evals")
                    .join("reward-pairs.jsonl")
            });
            let count = read_text(&p)
                .map(|s| s.lines().filter(|l| !l.trim().is_empty()).count())
                .unwrap_or(0);
            let data = json!({"path": p, "pair_count": count});
            if json {
                print_json(data)
            } else {
                println!("{}", serde_yaml::to_string(&data)?);
                Ok(())
            }
        }
    }
}

fn command_scene(command: SceneCommand) -> Result<()> {
    match command {
        SceneCommand::Create(args) => {
            let args = *args;
            let root = require_project()?;
            let path = root
                .join(PROJECT_DIR)
                .join("scene-cards")
                .join(format!("{}.yml", slug(&args.id)));
            if path.exists() && !args.force {
                bail!(
                    "{} already exists; pass --force to overwrite",
                    path.display()
                );
            }
            let data = json!({
                "id": args.id,
                "chapter": args.chapter,
                "scene": args.scene,
                "status": "planned",
                "pov": args.pov,
                "location": args.location,
                "purpose": {"plot": args.goal},
                "conflict": {"external": args.conflict},
                "turn": {"description": args.turn},
                "stakes": args.stakes,
                "open_threads": args.thread,
                "promises_opened": args.promise,
                "promises_paid": [],
                "do_not_repeat": args.do_not_repeat,
                "updated_at": now()
            });
            write_yaml_json(&path, &data)?;
            if args.json {
                print_json(data)
            } else {
                println!("Scene card written: {}", path.display());
                Ok(())
            }
        }
        SceneCommand::Show { id, json } => {
            let root = require_project()?;
            let path = root
                .join(PROJECT_DIR)
                .join("scene-cards")
                .join(format!("{}.yml", slug(&id)));
            let value = read_yaml_value(&path).context("scene card not found")?;
            if json {
                print_json(yaml_to_json(&value))
            } else {
                println!("{}", serde_yaml::to_string(&value)?);
                Ok(())
            }
        }
        SceneCommand::FromText { path, id, json } => {
            let text = read_text(&path)?;
            let scene_id = id.unwrap_or_else(|| {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });
            let data = json!({
                "id": scene_id,
                "status": "drafted",
                "purpose": {"plot": scene_signal(&text, "goal")},
                "conflict": {"external": scene_signal(&text, "conflict")},
                "turn": {"description": scene_signal(&text, "turn")},
                "notes": "Extracted by heuristic scene signal pass.",
                "updated_at": now()
            });
            let root = require_project()?;
            let out = root.join(PROJECT_DIR).join("scene-cards").join(format!(
                "{}.yml",
                slug(data["id"].as_str().unwrap_or("scene"))
            ));
            write_yaml_json(&out, &data)?;
            if json {
                print_json(data)
            } else {
                println!("Scene card written: {}", out.display());
                Ok(())
            }
        }
    }
}

fn command_character(command: CharacterCommand) -> Result<()> {
    let args = match command {
        CharacterCommand::Add(args) | CharacterCommand::Update(args) => args,
    };
    let root = require_project()?;
    let path = root
        .join(PROJECT_DIR)
        .join("characters")
        .join(format!("{}.yml", slug(&args.name)));
    let data = json!({
        "name": args.name,
        "age": args.age,
        "appearance": args.appearance,
        "voice": args.voice,
        "traits": args.trait_,
        "motives": args.motive,
        "wounds": args.wound,
        "secrets": args.secret,
        "knowledge": args.knowledge,
        "updated_at": now()
    });
    write_yaml_json(&path, &data)?;
    if args.json {
        print_json(data)
    } else {
        println!("Character saved: {}", path.display());
        Ok(())
    }
}

fn command_plot(command: PlotCommand) -> Result<()> {
    match command {
        PlotCommand::Thread(args) => {
            let root = require_project()?;
            let path = root
                .join(PROJECT_DIR)
                .join("plot-threads")
                .join(format!("{}.yml", slug(&args.id)));
            let title = if args.title.is_empty() {
                args.id.clone()
            } else {
                args.title.clone()
            };
            let data = json!({
                "id": args.id,
                "title": title,
                "type": args.type_,
                "owner": args.owner,
                "status": "active",
                "current_stage": args.stage,
                "appearances": args.appearance,
                "risk": args.risk,
                "updated_at": now()
            });
            write_yaml_json(&path, &data)?;
            if args.json {
                print_json(data)
            } else {
                println!("Plot thread written: {}", path.display());
                Ok(())
            }
        }
        PlotCommand::AddPromise(args) => {
            let root = require_project()?;
            let path = root.join(PROJECT_DIR).join("plot-matrix.yml");
            let mut data = read_yaml_value(&path).unwrap_or_else(|| json_yaml_map("promises"));
            let id = format!("promise-{}", Utc::now().timestamp_millis());
            let entry = json!({
                "id": id,
                "promise": args.promise,
                "source": args.source,
                "thread": args.thread,
                "expected_payoff_window": args.expected_payoff_window,
                "status": "open",
                "created_at": now()
            });
            push_yaml_entry(&mut data, "promises", entry.clone())?;
            write_yaml_value(&path, &data)?;
            if args.json {
                print_json(entry)
            } else {
                println!("Promise added: {id}");
                Ok(())
            }
        }
        PlotCommand::Payoff(args) => {
            let root = require_project()?;
            let path = root.join(PROJECT_DIR).join("plot-matrix.yml");
            let mut data = read_yaml_value(&path).unwrap_or_else(|| json_yaml_map("payoffs"));
            let entry = json!({
                "promise_id": args.promise_id,
                "note": args.note,
                "source": args.source,
                "created_at": now()
            });
            push_yaml_entry(&mut data, "payoffs", entry.clone())?;
            write_yaml_value(&path, &data)?;
            if args.json {
                print_json(entry)
            } else {
                println!("Payoff recorded.");
                Ok(())
            }
        }
    }
}

fn command_matrix(command: MatrixCommand) -> Result<()> {
    match command {
        MatrixCommand::Build { out, json } => {
            let root = require_project()?;
            let data = build_matrix(&root)?;
            let out_path = out.unwrap_or_else(|| root.join(PROJECT_DIR).join("story-matrix.yml"));
            write_yaml_json(&out_path, &data)?;
            if json {
                print_json(data)
            } else {
                println!("Story matrix written: {}", out_path.display());
                Ok(())
            }
        }
        MatrixCommand::Audit { json, out } => {
            let root = require_project()?;
            let data = matrix_audit_data(&build_matrix(&root)?);
            if json {
                print_json(data)
            } else {
                write_or_print(
                    out,
                    &format!("# Story Matrix Audit\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
        MatrixCommand::Heatmap { json, out } => {
            let root = require_project()?;
            let data = promise_heatmap(&build_matrix(&root)?);
            if json {
                print_json(data)
            } else {
                write_or_print(
                    out,
                    &format!("# Promise Heat Map\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
    }
}

fn command_lint(command: LintCommand) -> Result<()> {
    let args = match command {
        LintCommand::Line(args) | LintCommand::Scene(args) | LintCommand::Plot(args) => args,
    };
    let text = read_text(&args.path)?;
    let issues = lint_text(&text);
    if args.json {
        print_json(json!({"metrics": metrics(&text), "issues": issues}))
    } else {
        let body = format!("# Lint Report\n\n{}\n", serde_yaml::to_string(&issues)?);
        write_or_print(args.out, &body)
    }
}

fn command_audit(command: AuditCommand) -> Result<()> {
    match command {
        AuditCommand::Continuity(args) | AuditCommand::Repetition(args) => {
            command_lint(LintCommand::Line(args))
        }
        AuditCommand::Causality { json } => {
            let root = require_project()?;
            let data = matrix_audit_data(&build_matrix(&root)?);
            if json {
                print_json(data)
            } else {
                println!("{}", serde_yaml::to_string(&data)?);
                Ok(())
            }
        }
    }
}

fn command_memory(command: MemoryCommand) -> Result<()> {
    match command {
        MemoryCommand::Extract {
            path,
            scene_id,
            out,
        } => {
            let text = read_text(&path)?;
            let scene = scene_id.unwrap_or_else(|| {
                path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });
            let data = json!({
                "scene": scene,
                "source": path,
                "new_facts": extract_fact_candidates(&text),
                "open_promises": [],
                "warnings": ["Review this diff before commit."],
                "created_at": now()
            });
            let root = require_project()?;
            let out_path = out.unwrap_or_else(|| {
                root.join(PROJECT_DIR).join("pending-memory").join(format!(
                    "{}.diff.yml",
                    slug(data["scene"].as_str().unwrap_or("scene"))
                ))
            });
            write_yaml_json(&out_path, &data)?;
            println!("Memory diff written: {}", out_path.display());
            Ok(())
        }
        MemoryCommand::Commit { diff } => {
            let root = require_project()?;
            let text = read_text(&diff)?;
            append_line(&root.join(PROJECT_DIR).join("memory.log"), &text)?;
            println!("Memory diff committed to memory.log");
            Ok(())
        }
        MemoryCommand::Sync { path } => {
            let root = require_project()?;
            let text = read_text(&path)?;
            append_line(&root.join(PROJECT_DIR).join("memory.log"), &text)?;
            println!("Memory synced from {}", path.display());
            Ok(())
        }
    }
}

fn command_export(command: ExportCommand) -> Result<()> {
    match command {
        ExportCommand::Html { input, out } => {
            let text = read_text(&input)?;
            let title = input.file_stem().unwrap_or_default().to_string_lossy();
            let body = markdown_to_html(&text);
            let page = format!("<!doctype html><html><head><meta charset=\"utf-8\"><title>{title}</title><style>body{{font-family:Georgia,serif;max-width:760px;margin:3rem auto;line-height:1.7;padding:0 1rem}}pre{{background:#f3eee2;padding:1rem;overflow:auto}}</style></head><body>{body}</body></html>");
            let out_path = out.unwrap_or_else(|| input.with_extension("html"));
            write_text(&out_path, &page)?;
            println!("HTML written: {}", out_path.display());
            Ok(())
        }
    }
}

fn command_draft(args: TargetArgs, title: &str) -> Result<()> {
    let root = require_project()?;
    let target = args.target.unwrap_or_else(|| "next-scene".to_string());
    let packet = context_packet(&root, &target)?;
    let text = format!("# {title}: {target}\n\nUse this context packet to draft model-neutral prose. Do not call any model automatically.\n\n{packet}");
    if args.json {
        print_json(json!({"target": target, "prompt": text}))
    } else {
        write_or_print(args.out, &text)
    }
}

fn command_analyse(args: FileReportArgs) -> Result<()> {
    let text = read_text(&args.path)?;
    let data = json!({
        "path": args.path,
        "metrics": metrics(&text),
        "issues": lint_text(&text),
        "novelty": novelty_analysis(&text),
        "reader_profile": reader_check(&text, "fast-webnovel")
    });
    if args.json {
        print_json(data)
    } else {
        write_or_print(
            args.out,
            &format!("# Analysis Report\n\n{}", serde_yaml::to_string(&data)?),
        )
    }
}

fn command_review(args: ReviewArgs) -> Result<()> {
    let text = read_text(&args.path)?;
    let data = json!({
        "rubric": args.rubric,
        "metrics": metrics(&text),
        "issues": lint_text(&text),
        "dimensions": dimensions_json(),
        "doctrine": "Metrics route attention; author judgement decides whether the effect is useful."
    });
    if args.json {
        print_json(data)
    } else {
        write_or_print(
            args.out,
            &format!("# Novel Craft Review\n\n{}", serde_yaml::to_string(&data)?),
        )
    }
}

fn command_revise(args: ReviseArgs) -> Result<()> {
    let text = read_text(&args.path)?;
    let report = format!(
        "# Revision Plan: {}\n\nPass: `{}`\n\nDo not rewrite automatically in v1. Use these findings as a model prompt packet.\n\n{}",
        args.path.display(),
        args.pass,
        serde_yaml::to_string(&lint_text(&text))?
    );
    let out = args
        .out
        .unwrap_or_else(|| args.path.with_extension(format!("{}.review.md", args.pass)));
    write_text(&out, &report)?;
    println!("Revision packet written: {}", out.display());
    Ok(())
}

fn command_diff(args: DiffArgs) -> Result<()> {
    let before = read_text(&args.before)?;
    let after = read_text(&args.after)?;
    let report = format!(
        "# Diff Summary\n\n- Before words: {}\n- After words: {}\n- Delta: {}\n",
        count_words(&before),
        count_words(&after),
        count_words(&after) as isize - count_words(&before) as isize
    );
    write_or_print(args.out, &report)
}

fn command_full_book(args: FullBookArgs) -> Result<()> {
    let mut files = Vec::new();
    collect_text_files(&args.path, &mut files)?;
    let mut total_words = 0;
    let mut issue_count = 0;
    for file in &files {
        let text = read_text(file)?;
        total_words += count_words(&text);
        issue_count += lint_text(&text).len();
    }
    let data = json!({"files": files, "total_words": total_words, "issue_count": issue_count});
    if args.json {
        print_json(data)
    } else {
        write_or_print(
            args.out,
            &format!("# Full Book Audit\n\n{}", serde_yaml::to_string(&data)?),
        )
    }
}

fn prompt(label: &str, default: &str) -> Result<String> {
    print!("{label} [{default}]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn platform_target_triple() -> String {
    let arch = std::env::consts::ARCH;
    let platform = match std::env::consts::OS {
        "macos" => "apple-darwin",
        "linux" => "unknown-linux-gnu",
        "windows" => "pc-windows-msvc",
        other => other,
    };
    format!("{arch}-{platform}")
}

fn project_root() -> Result<PathBuf> {
    let current = std::env::current_dir()?;
    for candidate in current.ancestors() {
        if candidate.join(PROJECT_DIR).exists() {
            return Ok(candidate.to_path_buf());
        }
    }
    Ok(current)
}

fn require_project() -> Result<PathBuf> {
    let root = project_root()?;
    if root.join(PROJECT_DIR).exists() {
        Ok(root)
    } else {
        bail!("No .novel project found. Run `novel-craft init` or `novel-craft start` first.")
    }
}

fn now() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn write_if_missing(path: &Path, text: &str, force: bool) -> Result<()> {
    if force || !path.exists() {
        write_text(path, text)?;
    }
    Ok(())
}

fn write_text(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text).with_context(|| format!("write {}", path.display()))
}

fn read_text(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("read {}", path.display()))
}

fn append_line(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{text}")?;
    Ok(())
}

fn write_yaml<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    write_text(path, &serde_yaml::to_string(value)?)
}

fn write_yaml_json(path: &Path, value: &serde_json::Value) -> Result<()> {
    write_text(path, &serde_yaml::to_string(value)?)
}

fn write_yaml_value(path: &Path, value: &Value) -> Result<()> {
    write_text(path, &serde_yaml::to_string(value)?)
}

fn read_yaml_value(path: &Path) -> Option<Value> {
    let text = fs::read_to_string(path).ok()?;
    serde_yaml::from_str(&text).ok()
}

fn write_or_print(out: Option<PathBuf>, text: &str) -> Result<()> {
    if let Some(path) = out {
        write_text(&path, text)?;
        println!("Written: {}", path.display());
    } else {
        print!("{text}");
    }
    Ok(())
}

fn print_json<T: Serialize>(value: T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn init_memory(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (id INTEGER PRIMARY KEY, event_type TEXT, name TEXT, value TEXT, source TEXT, created_at TEXT)",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS facts (key TEXT PRIMARY KEY, value TEXT, source TEXT, updated_at TEXT)",
        [],
    )?;
    Ok(())
}

fn slug(value: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash {
            out.push('-');
            dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

fn count_words(text: &str) -> usize {
    text.unicode_words().count()
}

fn sentence_count(text: &str) -> usize {
    text.split(['.', '?', '!'])
        .filter(|part| !part.trim().is_empty())
        .count()
        .max(1)
}

fn metrics(text: &str) -> Metrics {
    let word_count = count_words(text);
    let sentence_count = sentence_count(text);
    let paragraphs: Vec<_> = text
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .collect();
    let dialogue_line_count = text.matches('"').count() / 2;
    let average_sentence_words = round2(word_count as f64 / sentence_count as f64);
    Metrics {
        word_count,
        sentence_count,
        paragraph_count: paragraphs.len(),
        dialogue_line_count,
        average_sentence_words,
        flesch_kincaid_grade_estimate: round2((average_sentence_words * 0.39) + 1.8),
    }
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn lint_text(text: &str) -> Vec<Issue> {
    let passive = Regex::new(r"\b(?:was|were|is|are|been|being|got|get)\s+\w+ed\b").unwrap();
    let mut issues = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let lower = line.to_lowercase();
        if passive.is_match(&lower) {
            issues.push(issue("VOICE-PASSIVE", "line", idx + 1, "Potential passive voice.", line, "Check whether actor concealment, helplessness, victim focus, or distance is intended."));
        }
        for word in FILTER_WORDS {
            if has_word(&lower, word) {
                issues.push(issue(
                    "POV-FILTER",
                    "line",
                    idx + 1,
                    "Filter word may widen POV distance.",
                    line,
                    "Keep only if the act of perception matters.",
                ));
                break;
            }
        }
        for word in EMOTION_WORDS {
            if has_word(&lower, word) {
                issues.push(issue("EMO-EMBODY", "line", idx + 1, "Abstract emotion label.", line, "In hot scenes, show emotion through action, body response, sensory focus, subtext, or choice."));
                break;
            }
        }
        if lower.contains("as you know") {
            issues.push(issue(
                "DIA-ASYOUKNOW",
                "scene",
                idx + 1,
                "Expository dialogue marker.",
                line,
                "Turn explanation into conflict, leverage, contradiction, or discovery.",
            ));
        }
    }
    issues
}

fn issue(
    id: &str,
    level: &str,
    line: usize,
    message: &str,
    excerpt: &str,
    suggestion: &str,
) -> Issue {
    Issue {
        rule_id: id.to_string(),
        level: level.to_string(),
        line,
        message: message.to_string(),
        excerpt: excerpt.chars().take(220).collect(),
        suggestion: suggestion.to_string(),
        classification: "possibly_intentional".to_string(),
    }
}

fn has_word(text: &str, word: &str) -> bool {
    let pattern = format!(r"\b{}\b", regex::escape(word));
    Regex::new(&pattern).unwrap().is_match(text)
}

fn word_choice_diagnostics(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    json!({
        "office_trap_hits": count_hits(&lower, &["accounting", "spreadsheet", "payroll", "manager", "corporate", "copier", "cubicle", "meeting"]),
        "systems_jargon_hits": count_hits(&lower, &["interface", "optimization", "resource", "efficiency", "metric", "algorithm", "calculation"]),
        "generic_phrase_hits": count_hits(&lower, GENERIC_PHRASES),
    })
}

fn count_hits(text: &str, terms: &[&str]) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    for term in terms {
        let count = text.matches(term).count();
        if count > 0 {
            map.insert((*term).to_string(), count);
        }
    }
    map
}

fn novelty_analysis(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    let generic = count_hits(&lower, GENERIC_PHRASES);
    let freshness = count_hits(
        &lower,
        &[
            "cost",
            "cooldown",
            "limitation",
            "debt",
            "promise",
            "oath",
            "consequence",
            "contract",
            "trust",
            "reputation",
            "mislabel",
        ],
    );
    let concrete = count_hits(
        &lower,
        &[
            "knife", "key", "coin", "bread", "salt", "rope", "map", "ring", "gate", "well",
            "bridge", "scar", "wound", "torch",
        ],
    );
    let score =
        50 + (freshness.len() * 6 + concrete.len() * 2) as isize - (generic.len() * 8) as isize;
    json!({
        "novelty_score_estimate": score.clamp(0, 100),
        "method": "Local lexical proxy; use LLM/human judgement for final creative evaluation.",
        "freshness_signal_hits": freshness,
        "concrete_object_hits": concrete,
        "generic_phrase_hits": generic,
        "llm_questions": [
            "Name the familiar promise.",
            "Name the freshness twist.",
            "List three details that could not be swapped into another generic story.",
            "If the twist is missing, propose alternatives using cost, contradiction, role inversion, or system limitation."
        ]
    })
}

fn trope_saturation(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    let mut hits = BTreeMap::new();
    for (axis, values) in TROPE_AXES {
        let found: Vec<_> = values
            .iter()
            .filter(|value| {
                value
                    .split_whitespace()
                    .filter(|w| lower.contains(&w.to_lowercase()))
                    .count()
                    >= 2
            })
            .map(|value| value.to_string())
            .collect();
        if !found.is_empty() {
            hits.insert((*axis).to_string(), found);
        }
    }
    let familiar_count: usize = hits.values().map(|v| v.len()).sum();
    json!({
        "axis_hits": hits,
        "familiar_trope_hit_count": familiar_count,
        "saturation_risk": if familiar_count >= 5 { "high" } else if familiar_count >= 3 { "medium" } else { "low" },
        "llm_questions": [
            "Which trope is intentional reader promise?",
            "What cost, limitation, contradiction, or consequence freshens it?",
            "Does the scene dramatise the trope through a choice?"
        ]
    })
}

fn reader_profiles() -> serde_json::Value {
    json!({
        "fast-webnovel": {
            "description": "Clear, quick-reading serial prose for tired mobile readers.",
            "grade_max": 8.5,
            "avg_sentence_max": 18.0,
            "review_focus": ["open with desire/danger/wonder", "reveal system rules through action", "avoid dense abstractions early"]
        },
        "adult-progression": {
            "description": "Readable progression fantasy with tactical complexity.",
            "grade_max": 10.5,
            "avg_sentence_max": 22.0,
            "review_focus": ["earned growth", "visible cost", "clear causal chain"]
        },
        "literary-dense": {
            "description": "Slower, voice-forward prose where density is allowed if controlled.",
            "grade_max": 13.5,
            "avg_sentence_max": 30.0,
            "review_focus": ["density must serve voice", "protect rhythm variance", "avoid abstraction hiding scene turn"]
        }
    })
}

fn reader_check(text: &str, profile: &str) -> serde_json::Value {
    let metrics = metrics(text);
    let mut warnings = Vec::new();
    let avg_max = match profile {
        "adult-progression" => 22.0,
        "literary-dense" => 30.0,
        _ => 18.0,
    };
    if metrics.average_sentence_words > avg_max {
        warnings.push("Average sentence length is above target; split tangled sentences.");
    }
    if metrics.paragraph_count > 0 && metrics.word_count / metrics.paragraph_count > 90 {
        warnings.push("Average paragraph is heavy for mobile reading.");
    }
    json!({"profile": profile, "metrics": metrics, "warnings": warnings})
}

fn voice_drift(paths: &[PathBuf], character: &str) -> Result<serde_json::Value> {
    let mut records = Vec::new();
    let quote_re = Regex::new(r#""([^"]+)""#).unwrap();
    for path in paths {
        let text = read_text(path)?;
        let quotes: Vec<_> = quote_re
            .captures_iter(&text)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect();
        let joined = quotes.join(" ");
        records.push(json!({
            "path": path,
            "character": if character.is_empty() { "all-dialogue" } else { character },
            "line_count": quotes.len(),
            "word_count": count_words(&joined),
            "top_terms": top_terms(&joined, 10)
        }));
    }
    Ok(json!({
        "records": records,
        "method": "Lexical dialogue fingerprint; final voice judgement belongs to the LLM/human reviewer."
    }))
}

fn top_terms(text: &str, limit: usize) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text.unicode_words() {
        let word = word.to_lowercase();
        if word.len() > 3 {
            *counts.entry(word).or_default() += 1;
        }
    }
    let mut rows: Vec<_> = counts.into_iter().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    rows.truncate(limit);
    rows
}

fn creative_brief(
    idea: &str,
    audience: &str,
    grade: &str,
    tropes: &[String],
    avoid: &[String],
) -> String {
    format!(
        "# Creative Premise Brief\n\n- Seed idea: {idea}\n- Audience: {audience}\n- Reading level target: grade {grade}\n- Required tropes: {}\n- Avoid: {}\n\n## LLM Job\nGenerate 12 distinct premise options before drafting. For each, name familiar promise, freshness twist, protagonist weakness, first hard choice, and chapter-one turn.\n\n## Methods\n{}\n",
        join_or_none(tropes),
        join_or_none(avoid),
        CREATIVE_METHODS.iter().map(|(id, _, instruction)| format!("- `{id}`: {instruction}")).collect::<Vec<_>>().join("\n")
    )
}

fn tournament_text(idea: &str, count: usize, avoid: &[String]) -> String {
    let mut lines = vec![
        "# Prompt Tournament Pack".to_string(),
        String::new(),
        format!("- Seed idea: {idea}"),
        format!("- Avoid: {}", join_or_none(avoid)),
        String::new(),
        "Ask the model to draft every contender as a 250-400 word opening concept, then compare pairwise.".to_string(),
        String::new(),
    ];
    for index in 0..count.max(1) {
        lines.push(format!("## Contender {}", index + 1));
        for (axis_index, (axis, values)) in TROPE_AXES.iter().enumerate() {
            lines.push(format!(
                "- {axis}: {}",
                values[(index + axis_index) % values.len()]
            ));
        }
        lines.push("- LLM must name familiar promise, freshness twist, hard choice, cost, and page-turn reason.".to_string());
        lines.push(String::new());
    }
    lines.join("\n")
}

fn start_packet(args: &StartArgs) -> String {
    format!(
        "# Novel Craft Start Packet\n\n## Project\n- Title: {}\n- Writer level: {}\n- Audience: {}\n- Genre: {}\n- Reading level: {}\n- Tone: {}\n- Desired output: {}\n- Autonomy: {}\n\n## Story Seed\n- Idea: {}\n- Include tropes: {}\n- Avoid: {}\n- Protagonist want: {}\n- Protagonist wound: {}\n- World: {}\n- Power system: {}\n\n## Model Boundary\nNovel Craft does not call an LLM. Give this packet to the model of your choice.\n\n## First Workflow\n1. Generate premise contenders.\n2. Score with the eval rubric.\n3. Create scene card.\n4. Build context packet.\n5. Draft.\n6. Lint/review/revise.\n7. Extract memory diff.\n",
        args.title,
        args.writer_level,
        args.audience,
        args.genre,
        args.reading_level,
        args.tone,
        args.desired_output,
        args.autonomy,
        args.idea,
        join_or_none(&args.include_trope),
        join_or_none(&args.avoid),
        args.protagonist_want,
        args.protagonist_wound,
        args.world,
        args.power_system,
    )
}

fn rubric_text(genre: &str) -> String {
    let mut lines = vec![
        "# Creative Writing Evaluation Rubric".to_string(),
        String::new(),
        format!("- Genre profile: {genre}"),
        "- Score each dimension 1-5 with evidence.".to_string(),
        "- Do not collapse creative quality into one number.".to_string(),
        String::new(),
    ];
    for (id, question) in EVAL_DIMENSIONS {
        lines.push(format!("## {id}"));
        lines.push(format!("- Question: {question}"));
        lines.push("- Score 1-5:".to_string());
        lines.push("- Evidence:".to_string());
        lines.push("- Revision note:".to_string());
        lines.push(String::new());
    }
    lines.join("\n")
}

fn eval_sheet(path: &Path, text: &str) -> String {
    format!(
        "# Evaluation Sheet: {}\n\n## Metrics\n{}\n\n## Score Sheet\n{}\n",
        path.display(),
        serde_yaml::to_string(&metrics(text)).unwrap_or_default(),
        EVAL_DIMENSIONS
            .iter()
            .map(|(id, question)| format!(
                "### {id}\n- Question: {question}\n- Score 1-5:\n- Evidence:\n- Revision note:\n"
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn compare_text(a: &Path, b: &Path, a_text: &str, b_text: &str) -> String {
    format!(
        "# Pairwise Creative Writing Comparison\n\n- Version A: {}\n- Version B: {}\n- A words: {}\n- B words: {}\n\n{}\n",
        a.display(),
        b.display(),
        count_words(a_text),
        count_words(b_text),
        EVAL_DIMENSIONS
            .iter()
            .map(|(id, question)| format!("## {id}\n- Question: {question}\n- Winner: A / B / tie\n- Evidence from A:\n- Evidence from B:\n"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn dimensions_json() -> serde_json::Value {
    json!(EVAL_DIMENSIONS
        .iter()
        .map(|(id, question)| json!({"id": id, "question": question}))
        .collect::<Vec<_>>())
}

fn tropes_json() -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (axis, values) in TROPE_AXES {
        map.insert((*axis).to_string(), json!(values));
    }
    json!({"genre": "system-isekai", "tropes": map})
}

fn rule_guide(json_output: bool) -> Result<String> {
    let rules: Value = serde_yaml::from_str(DEFAULT_RULES)?;
    if json_output {
        return Ok(serde_json::to_string_pretty(&rules_to_json(&rules))?);
    }
    let mut out = String::from("# Novel Craft LLM Rule Guide\n\nRules are effects, not commandments. Deterministic checks are leads, not verdicts.\n\n");
    if let Some(items) = rules.as_sequence() {
        for rule in items {
            out.push_str(&format!(
                "## {}: {}\n\n- Level: {}\n- Effect: {}\n- Severity: {}\n\n",
                cell(rule, "id"),
                cell(rule, "name"),
                cell(rule, "level"),
                cell(rule, "effect"),
                cell(rule, "severity")
            ));
        }
    }
    Ok(out)
}

fn rules_to_json(rules: &Value) -> serde_json::Value {
    yaml_to_json(rules)
}

fn cell(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .chars()
        .take(90)
        .collect::<String>()
}

fn yaml_to_json(value: &Value) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

fn json_yaml_map(key: &str) -> Value {
    let mut map = Mapping::new();
    map.insert(Value::String(key.to_string()), Value::Sequence(vec![]));
    Value::Mapping(map)
}

fn push_yaml_entry(value: &mut Value, key: &str, entry: serde_json::Value) -> Result<()> {
    let entry = serde_yaml::to_value(entry)?;
    let map = value
        .as_mapping_mut()
        .context("state file is not a mapping")?;
    let seq = map
        .entry(Value::String(key.to_string()))
        .or_insert_with(|| Value::Sequence(vec![]))
        .as_sequence_mut()
        .context("state key is not a sequence")?;
    seq.push(entry);
    Ok(())
}

fn state_report(file: &str, key: &str, json_output: bool) -> Result<()> {
    let root = require_project()?;
    let path = root.join(PROJECT_DIR).join("state").join(file);
    let value = read_yaml_value(&path).unwrap_or_else(|| json_yaml_map(key));
    let count = value
        .get(key)
        .and_then(Value::as_sequence)
        .map(Vec::len)
        .unwrap_or(0);
    let data = json!({"path": path, "count": count, "data": yaml_to_json(&value)});
    if json_output {
        print_json(data)
    } else {
        println!("{}", serde_yaml::to_string(&data)?);
        Ok(())
    }
}

fn report_file<F>(args: FileReportArgs, build: F) -> Result<()>
where
    F: FnOnce(&Path, &str) -> serde_json::Value,
{
    let text = read_text(&args.path)?;
    let data = build(&args.path, &text);
    if args.json {
        print_json(data)
    } else {
        write_or_print(args.out, &serde_yaml::to_string(&data)?)
    }
}

fn scene_signal(text: &str, kind: &str) -> String {
    let lower = text.to_lowercase();
    match kind {
        "goal"
            if lower.contains("must") || lower.contains("wanted") || lower.contains("needed") =>
        {
            "Character has an implied goal.".to_string()
        }
        "conflict"
            if lower.contains("but") || lower.contains("refused") || lower.contains("danger") =>
        {
            "Resistance/conflict signal present.".to_string()
        }
        "turn"
            if lower.contains("decided")
                || lower.contains("revealed")
                || lower.contains("learned") =>
        {
            "Change-state signal present.".to_string()
        }
        _ => "not confidently detected".to_string(),
    }
}

fn build_matrix(root: &Path) -> Result<serde_json::Value> {
    let mut scenes = Vec::new();
    let dir = root.join(PROJECT_DIR).join("scene-cards");
    if dir.exists() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("yml") {
                if let Some(value) = read_yaml_value(&path) {
                    scenes.push(yaml_to_json(&value));
                }
            }
        }
    }
    Ok(json!({"scenes": scenes, "promises": [], "plot_threads": []}))
}

fn matrix_audit_data(matrix: &serde_json::Value) -> serde_json::Value {
    let scenes = matrix["scenes"].as_array().cloned().unwrap_or_default();
    let mut warnings = Vec::new();
    if scenes.len() >= 3 {
        warnings
            .push("Check recent scenes for repeated scene function, location, and emotional beat.");
    }
    json!({
        "scene_count": scenes.len(),
        "warnings": warnings,
        "doctrine": "Use this audit to guide an LLM review; do not treat it as final taste judgement."
    })
}

fn promise_heatmap(matrix: &serde_json::Value) -> serde_json::Value {
    let scenes = matrix["scenes"].as_array().cloned().unwrap_or_default();
    let mut promises = Vec::new();
    for scene in &scenes {
        if let Some(opened) = scene.get("promises_opened").and_then(|v| v.as_array()) {
            for promise in opened {
                promises.push(json!({
                    "promise": promise,
                    "status": "open",
                    "opened_in": scene.get("id"),
                    "heat": "low"
                }));
            }
        }
    }
    json!({"scene_count": scenes.len(), "promise_count": promises.len(), "heat": promises})
}

fn context_packet(root: &Path, target: &str) -> Result<String> {
    let project = read_text(&root.join(PROJECT_DIR).join("project.yml")).unwrap_or_default();
    let matrix = build_matrix(root)?;
    Ok(format!(
        "# Context Packet: {target}\n\n## Project\n```yaml\n{project}\n```\n\n## Story Matrix Summary\n{}\n\n## Instructions\n- Use this packet with the model of your choice.\n- Preserve canon unless the scene card explicitly changes it.\n- Run lint, eval, and memory extraction after drafting.\n",
        serde_yaml::to_string(&matrix)?
    ))
}

fn extract_fact_candidates(text: &str) -> Vec<serde_json::Value> {
    text.lines()
        .filter(|line| {
            line.contains(" learned ")
                || line.contains(" discovered ")
                || line.contains(" promised ")
        })
        .take(10)
        .map(|line| json!({"fact": line.trim(), "confidence": "candidate"}))
        .collect()
}

fn collect_text_files(path: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if path.is_file() {
        out.push(path.to_path_buf());
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_text_files(&path, out)?;
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("md" | "txt")
        ) {
            out.push(path);
        }
    }
    Ok(())
}

fn markdown_to_html(markdown: &str) -> String {
    markdown
        .lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix("# ") {
                format!("<h1>{}</h1>", html_escape(rest))
            } else if let Some(rest) = line.strip_prefix("## ") {
                format!("<h2>{}</h2>", html_escape(rest))
            } else if line.trim().is_empty() {
                String::new()
            } else {
                format!("<p>{}</p>", html_escape(line))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}
