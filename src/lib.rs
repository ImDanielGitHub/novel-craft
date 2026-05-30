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
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use unicode_segmentation::UnicodeSegmentation;

const PROJECT_DIR: &str = ".novel";
const DEFAULT_RULES: &str = include_str!("../rules/default.yml");

const SKILLS: &[(&str, &str)] = &[
    (
        "novel-craft-agentic-writer/SKILL.md",
        include_str!("../skills/novel-craft-agentic-writer/SKILL.md"),
    ),
    (
        "novel-craft-character-review/SKILL.md",
        include_str!("../skills/novel-craft-character-review/SKILL.md"),
    ),
    (
        "novel-craft-continuity-sync/SKILL.md",
        include_str!("../skills/novel-craft-continuity-sync/SKILL.md"),
    ),
    (
        "novel-craft-creativity-engine/SKILL.md",
        include_str!("../skills/novel-craft-creativity-engine/SKILL.md"),
    ),
    (
        "novel-craft-dialogue-review/SKILL.md",
        include_str!("../skills/novel-craft-dialogue-review/SKILL.md"),
    ),
    (
        "novel-craft-draft-review/SKILL.md",
        include_str!("../skills/novel-craft-draft-review/SKILL.md"),
    ),
    (
        "novel-craft-book-audit/SKILL.md",
        include_str!("../skills/novel-craft-book-audit/SKILL.md"),
    ),
    (
        "novel-craft-line-review/SKILL.md",
        include_str!("../skills/novel-craft-line-review/SKILL.md"),
    ),
    (
        "novel-craft-memory-sync/SKILL.md",
        include_str!("../skills/novel-craft-memory-sync/SKILL.md"),
    ),
    (
        "novel-craft-next-chapter/SKILL.md",
        include_str!("../skills/novel-craft-next-chapter/SKILL.md"),
    ),
    (
        "novel-craft-rulebook-review/SKILL.md",
        include_str!("../skills/novel-craft-rulebook-review/SKILL.md"),
    ),
    (
        "novel-craft-scene-planner/SKILL.md",
        include_str!("../skills/novel-craft-scene-planner/SKILL.md"),
    ),
    (
        "novel-craft-writing-support/SKILL.md",
        include_str!("../skills/novel-craft-writing-support/SKILL.md"),
    ),
    (
        "aliases/novel-character-review/SKILL.md",
        include_str!("../skills/aliases/novel-character-review/SKILL.md"),
    ),
    (
        "aliases/novel-continuity-sync/SKILL.md",
        include_str!("../skills/aliases/novel-continuity-sync/SKILL.md"),
    ),
    (
        "aliases/novel-creativity-architect/SKILL.md",
        include_str!("../skills/aliases/novel-creativity-architect/SKILL.md"),
    ),
    (
        "aliases/novel-dialogue-review/SKILL.md",
        include_str!("../skills/aliases/novel-dialogue-review/SKILL.md"),
    ),
    (
        "aliases/novel-evaluation-review/SKILL.md",
        include_str!("../skills/aliases/novel-evaluation-review/SKILL.md"),
    ),
    (
        "aliases/novel-full-book-review/SKILL.md",
        include_str!("../skills/aliases/novel-full-book-review/SKILL.md"),
    ),
    (
        "aliases/novel-line-edit/SKILL.md",
        include_str!("../skills/aliases/novel-line-edit/SKILL.md"),
    ),
    (
        "aliases/novel-memory-diff/SKILL.md",
        include_str!("../skills/aliases/novel-memory-diff/SKILL.md"),
    ),
    (
        "aliases/novel-next-chapter/SKILL.md",
        include_str!("../skills/aliases/novel-next-chapter/SKILL.md"),
    ),
    (
        "aliases/novel-rulebook-review/SKILL.md",
        include_str!("../skills/aliases/novel-rulebook-review/SKILL.md"),
    ),
    (
        "aliases/novel-scene-architect/SKILL.md",
        include_str!("../skills/aliases/novel-scene-architect/SKILL.md"),
    ),
    (
        "aliases/general-writing/SKILL.md",
        include_str!("../skills/aliases/general-writing/SKILL.md"),
    ),
];

struct WritingProfile {
    id: &'static str,
    purpose: &'static str,
    reader: &'static str,
    success_criteria: &'static [&'static str],
    failure_modes: &'static [&'static str],
    review_pass: &'static [&'static str],
    final_gate: &'static [&'static str],
}

const GENERAL_WRITING_PROFILE: WritingProfile = WritingProfile {
    id: "novel-craft-writing-support",
    purpose: "Support prose around a novel project: plain names, clear docs, natural wording, and honest claims.",
    reader: "A real person or agent trying to understand, use, or maintain the writing project.",
    success_criteria: &[
        "meaning is preserved",
        "names use plain words a busy person would search for",
        "claims are concrete or clearly labelled as judgement",
        "examples and commands come before broad product language",
        "tone fits the reader and task",
    ],
    failure_modes: &[
        "generic AI phrasing",
        "architecture-heavy names",
        "marketing-first docs",
        "unsupported confidence",
        "internal builder vocabulary in user-facing places",
    ],
    review_pass: &[
        "cut filler",
        "rename things by their job",
        "replace vague claims with proof, dates, examples, or constraints",
        "move commands and next steps before vision",
        "check facts before making prose sound confident",
    ],
    final_gate: &[
        "Would the intended reader know what this means in one pass?",
        "Would a busy person understand the name in three seconds?",
        "Can an agent use the docs without guessing the workflow?",
        "Are important claims sourced, evidenced, or clearly marked as opinion?",
    ],
};

const CREATIVE_METHODS: &[(&str, &str, &str)] = &[
    (
        "diverge_converge",
        "Generate 12-20 distinct versions before choosing.",
        "Score each option for hook, relatability, novelty, genre fit, and expansion potential.",
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

const STORY_ATLAS_GENRES: &[&str] = &[
    "fantasy",
    "science fiction",
    "horror",
    "romance",
    "mystery",
    "thriller",
    "adventure",
    "historical fiction",
    "literary fiction",
    "comedy",
    "drama",
    "western",
    "crime",
    "war",
    "survival",
    "dystopian",
    "utopian",
    "post-apocalyptic",
    "mythic fiction",
    "fairy-tale retelling",
    "superhero",
    "sports",
    "slice of life",
    "coming of age",
    "family saga",
    "political fiction",
    "court intrigue",
    "medical fiction",
    "legal fiction",
    "military fiction",
    "spy fiction",
    "heist fiction",
    "noir",
    "satire",
    "paranormal",
    "urban fiction",
    "portal fiction",
    "game fiction",
    "cultivation",
    "wuxia",
    "xianxia",
    "litRPG",
    "progression fantasy",
    "isekai",
    "monster evolution",
    "academy fiction",
    "beast taming",
    "space opera",
    "cyberpunk",
    "solarpunk",
];

const STORY_ATLAS_SUBGENRES: &[&str] = &[
    "dark fantasy",
    "epic fantasy",
    "grimdark",
    "cozy fantasy",
    "low fantasy",
    "high fantasy",
    "gaslamp fantasy",
    "sword and sorcery",
    "magical academy",
    "rational magic",
    "dungeon core",
    "tower climb",
    "system apocalypse",
    "VRMMO",
    "space western",
    "military science fiction",
    "hard science fiction",
    "soft science fiction",
    "time travel",
    "alternate history",
    "cosmic horror",
    "folk horror",
    "body horror",
    "psychological horror",
    "romantic suspense",
    "enemies to lovers romance",
    "rejected bond romance",
    "secret identity romance",
    "regency romance",
    "small town romance",
    "locked room mystery",
    "police procedural",
    "legal thriller",
    "medical thriller",
    "political thriller",
    "revenge thriller",
    "assassin thriller",
    "heist thriller",
    "survival horror",
    "found family adventure",
    "beast companion adventure",
    "merchant kingdom building",
    "crafting progression",
    "settlement building",
    "villainess reincarnation",
    "regression fantasy",
    "second chance fantasy",
    "nonhuman protagonist",
    "underwater civilization",
    "desert empire",
];

const STORY_ATLAS_TROPES: &[&str] = &[
    "chosen one",
    "reluctant hero",
    "underdog climb",
    "weak to strong",
    "found family",
    "rivals to allies",
    "enemies to lovers",
    "mentor with secrets",
    "hidden heir",
    "lost civilization",
    "forbidden magic",
    "costly power",
    "sentient weapon",
    "bonded beast",
    "talking companion",
    "trial by combat",
    "academy tournament",
    "guild contract",
    "secret identity",
    "double life",
    "prophecy misunderstood",
    "unreliable system",
    "fake weakness",
    "public humiliation",
    "exile and return",
    "revenge quest",
    "redemption arc",
    "fall from grace",
    "deal with a monster",
    "curse with benefits",
    "time loop",
    "regression",
    "body swap",
    "portal to another world",
    "monster protagonist",
    "villain protagonist",
    "antihero code",
    "heist crew",
    "siege defense",
    "political marriage",
    "arranged alliance",
    "betrayal reveal",
    "mystery box",
    "power with drawback",
    "training arc",
    "tournament arc",
    "rescue mission",
    "journey to the capital",
    "final exam",
    "kingdom building",
];

const STORY_ATLAS_SUBTROPES: &[&str] = &[
    "truth-binding flaw",
    "power reveals location",
    "skill grows through use",
    "rank hides forbidden branch",
    "quest log lies",
    "inventory stores debt",
    "companion evolves from care",
    "beast mirrors wound",
    "mentor tests loyalty",
    "rival knows the weakness",
    "villain offers correct solution",
    "safe zone has moral cost",
    "healing creates dependency",
    "crafting beats combat",
    "support class controls logistics",
    "status screen becomes public",
    "title grants legal burden",
    "curse blocks direct violence",
    "name has power",
    "map changes after boundary crossed",
    "first kill changes soul",
    "first mercy changes faction",
    "auction reveals status",
    "academy lesson becomes weapon",
    "trial copies fear",
    "dungeon ecology has politics",
    "monster food unlocks trait",
    "nonhuman body changes desire",
    "flashback changes present choice",
    "chapter ends on unpaid debt",
    "side character advances off page",
    "romance creates tactical risk",
    "family secret changes inheritance",
    "fake death creates opportunity",
    "public duel changes law",
    "settlement needs winter food",
    "guild hides fatal rule",
    "artifact chooses cost",
    "magic requires memory",
    "skill cooldown changes strategy",
    "summon has its own goal",
    "dream realm leaves real scar",
    "enemy saves hero for selfish reason",
    "betrayal protects wrong person",
    "prophecy is a contract",
    "victory creates owner",
    "level up attracts hunter",
    "lawful villain controls resources",
    "small kindness becomes oath",
    "final scene reverses first image",
];

const STORY_MIXING_PROTOCOL: &[&str] = &[
    "Start with one genre, one subgenre, two tropes, and one sub-trope.",
    "Fold ingredients together causally: each ingredient should change choices, rules, pressure, or consequences.",
    "Discard combinations that only rename the same plot with more decoration.",
    "For every familiar genre draw, add one cost, contradiction, unusual body rule, social price, or world-rule consequence.",
    "Name the first chapter image, micro-scene, chapter-one turn, and wider story engine before drafting.",
    "Generate several mixes, reject the ones with weak causality, then choose with evidence.",
];

const NOVEL_EXCELLENCE_STANDARD: &[&str] = &[
    "banger first chapter: open on a wound, injustice, problem, danger, desire, or wonder the reader understands immediately",
    "micro-scene before macro-roadmap: usually dramatise the smallest working unit of the premise before leaning on kingdoms, empires, domains, future ranks, or upgrade ladders",
    "reader draw: make the genre reward visible before spending trust on lore",
    "fresh familiar: pair recognisable pleasure with one causal twist, cost, contradiction, or unusual body/world rule",
    "costly advantage: every power, secret, bond, system, status, or skill creates a limit, debt, exposure risk, moral cost, or new enemy",
    "scene engine: every scene has a goal, conflict, turn, consequence, and next pressure",
    "chapter engine: every chapter changes power, knowledge, status, relationship, danger, open question, or territory",
    "wider story engine: track mystery, relationship, status, power, threat, territory, and theme pressure across arcs",
    "continuation reason: end chapters on a fair decision, reveal, unpaid debt, threat, relationship shift, or progress milestone",
    "human texture: concrete wants, shame, hunger, loyalty, fear, pride, tenderness, and contradiction beat abstract destiny",
    "revision loop: draft, self-critique, revise once, gate, compare alternatives, then decide with judgement",
];

const OPENING_MACRO_TERMS: &[&str] = &[
    "kingdom",
    "kingdom-building",
    "empire",
    "throne",
    "crown",
    "sovereign",
    "ruler",
    "domain",
    "territory",
    "citizen",
    "citizens",
    "vassal",
    "tax",
    "taxes",
    "law",
    "laws",
    "rank",
    "class",
    "level",
    "levels",
    "upgrade",
    "upgrades",
    "skill tree",
    "system",
    "stats",
    "status screen",
];

const OPENING_MICRO_TERMS: &[&str] = &[
    "bread", "coin", "door", "room", "roof", "floor", "hand", "blood", "snow", "cold", "hunger",
    "hungry", "bowl", "water", "fire", "smoke", "lock", "key", "shelter", "meal", "child", "girl",
    "boy", "debt", "wound", "fever", "cart", "street", "wall", "knife", "rope", "bell", "ledger",
    "name", "breath", "mouth",
];

const OPENING_ACTION_TERMS: &[&str] = &[
    "opened",
    "ran",
    "held",
    "stole",
    "knocked",
    "fell",
    "woke",
    "wakes",
    "dragged",
    "grabbed",
    "cut",
    "bled",
    "fed",
    "hid",
    "lifted",
    "carried",
    "locked",
    "unlocked",
    "asked",
    "refused",
    "chose",
    "choose",
    "paid",
    "bargained",
    "broke",
    "kept",
    "saved",
    "turned",
    "stepped",
    "crossed",
    "pushed",
    "pulled",
];

const OPENING_ANNOUNCEMENT_PHRASES: &[&str] = &[
    "kingdom-building",
    "kingdom building",
    "build a kingdom",
    "building a kingdom",
    "found a kingdom",
    "create a kingdom",
    "system is for",
    "this system is",
    "the system was designed",
    "domain seed",
    "citizens: 0",
    "combat value",
    "property: none",
    "class:",
    "rank:",
    "in this world",
    "for centuries",
    "was founded",
    "upgrade ladder",
    "power progression ladder",
];

type TropeAxes = &'static [(&'static str, &'static [&'static str])];

const SYSTEM_ISEKAI_TROPE_AXES: &[(&str, &[&str])] = &[
    (
        "entry",
        &[
            "summoned as the wrong hero",
            "reincarnated as a weak local body",
            "falls into a tutorial dungeon",
            "wakes after being sacrificed",
            "transmigrates into a doomed minor villain",
            "arrives as a nameless extra in a prophecy",
            "inherits a collapsing settlement nobody else wants",
            "respawns at the hour a border fort is abandoned",
        ],
    ),
    (
        "system",
        &[
            "status screen with hidden costs",
            "skill tree that grows from choices, not grinding",
            "class system that mislabels the hero",
            "quest log written by an unreliable patron",
            "inventory that stores debts, memories, or civic obligations",
            "leveling through teaching, repair, cooking, healing, building, or diplomacy",
            "settlement ledger that upgrades solved local bottlenecks",
            "reputation board where residents can accept or reject policies",
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
            "cannot own land directly",
            "commands fail unless locals consent",
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
            "food, water, and shelter logistics become the real power ladder",
            "fair laws become defensive infrastructure",
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
            "tax auditors can legally dissolve the settlement",
            "refugees arrive faster than shelter can be built",
        ],
    ),
    (
        "freshness_twist",
        &[
            "the system rewards solved civic problems, not kills",
            "quests are generated by people who need help, not by gods",
            "leveling makes the hero more responsible, not freer",
            "the weakest class controls supply lines",
            "stats are public, so secrecy matters more than strength",
            "the tutorial is a scam and the monsters know it",
            "land upgrades through solved disputes, not conquest",
            "buildings level only when residents actually use them",
        ],
    ),
];

const BREAKOUT_SERIAL_AXES: &[(&str, &[&str])] = &[
    (
        "opening_wound",
        &[
            "poverty makes survival the first antagonist",
            "illness or disability makes freedom feel physical",
            "public rejection destroys status in one scene",
            "family betrayal turns home into enemy territory",
            "a useless label is placed on a secretly valuable person",
            "debt, contract, or law makes the hero owned by someone else",
        ],
    ),
    (
        "reader_draw",
        &[
            "system with visible choices and rewards",
            "weak-to-strong climb with measurable stages",
            "beast bond, summon bond, or companion growth",
            "nonhuman body with evolving instincts",
            "academy, clan, guild, pack, or elite recruitment ladder",
            "romance hook with betrayal, secret identity, or rejected bond",
        ],
    ),
    (
        "special_sauce",
        &[
            "power solves one problem while creating a social cost",
            "the hero is first in a path no one can teach",
            "a flaw makes every victory morally or tactically dangerous",
            "the world feels like an ecosystem instead of a stage",
            "side characters have their own leverage and growth",
            "mystery answers one question while opening a deeper one",
        ],
    ),
    (
        "retention_engine",
        &[
            "chapter ends on a decision, reveal, debt, or threat",
            "progress bar advances in power, status, knowledge, or relationship",
            "short-term goal feeds a long-term mystery",
            "reader can predict the next reward but not the cost",
            "update rhythm builds trust with the audience",
            "community questions become future payoff pressure",
        ],
    ),
    (
        "breakout_gate",
        &[
            "grammar and clarity do not block immersion",
            "characters are memorable and necessary to the plot",
            "world has geography, culture, history, system, or technology depth",
            "theme match is obvious from the first arc",
            "core hook is clear in one sentence",
            "adaptation image is easy to imagine as a cover, comic panel, or trailer",
        ],
    ),
];

const NIGHTMARE_SURVIVAL_AXES: &[(&str, &[&str])] = &[
    (
        "entry",
        &[
            "dystopian poverty lottery selects the protagonist",
            "trial realm turns one fear into a living geography",
            "public power awakening carries a private curse",
            "survival exam begins before the hero understands the rules",
            "a nightmare copies the hero's worst social wound",
            "the first victory brands the hero as useful and dangerous",
        ],
    ),
    (
        "flaw",
        &[
            "cannot lie when secrecy is survival",
            "power reveals location to enemies",
            "shadow, reflection, or memory can betray the hero",
            "every boon creates an owner, oath, or dependency",
            "the hero wins by cunning but loses trust",
            "the system rewards survival without explaining the price",
        ],
    ),
    (
        "mystery_ecology",
        &[
            "ruined civilisation visible through broken tools",
            "monsters imply an older disaster",
            "names have power because history remembers them",
            "safe zones carry moral compromise",
            "each realm answers one rule and breaks another",
            "mythology appears as evidence, not exposition",
        ],
    ),
    (
        "relationship_pressure",
        &[
            "ally can save the hero but also own them",
            "truth-telling creates intimacy and tactical danger",
            "rival understands the hero's flaw too well",
            "team survival conflicts with private freedom",
            "trust is earned through cost rather than speeches",
            "romance or friendship tests whether the hero can stay unowned",
        ],
    ),
];

const RATIONAL_MAGUS_AXES: &[(&str, &[&str])] = &[
    (
        "magic_logic",
        &[
            "spellcraft follows repeatable physical laws",
            "mana core growth requires study and bodily consequence",
            "healing, forging, and combat share one underlying theory",
            "old superstition hides a practical mechanism",
            "new power is earned through experiment, not sudden blessing",
            "politics changes which discoveries can be used openly",
        ],
    ),
    (
        "moral_pressure",
        &[
            "broken protagonist does good for selfish reasons",
            "protector institution permits harm for stability",
            "villain has a coherent dream that could improve the world",
            "family love challenges misanthropy",
            "romance is tested by maturity instead of possession",
            "power forces the hero to choose between safety and empathy",
        ],
    ),
    (
        "world_scope",
        &[
            "academy lesson becomes political weapon",
            "local spell problem reveals national history",
            "war, nobility, monsters, and research pull on the same rule",
            "side characters continue changing off-page",
            "open world saga grows from earlier technical choices",
            "social systems resist brute-force solutions",
        ],
    ),
];

const BEAST_BOND_PROGRESSION_AXES: &[(&str, &[&str])] = &[
    (
        "bond_hook",
        &[
            "first person on an uncharted beast path",
            "poor recruit gets one experimental awakening chance",
            "companion is partner, not disposable tool",
            "taming requires trust before command",
            "evolution path is discovered through care and risk",
            "beast instincts solve what human status cannot",
        ],
    ),
    (
        "progression_mystery",
        &[
            "rank system hides a legendary branch",
            "training success creates attention from elites",
            "food, habitat, and temperament shape growth",
            "each beast unlocks a new social problem",
            "first discovery rewrites the academy manual",
            "nation needs the power before it understands the cost",
        ],
    ),
    (
        "ensemble_value",
        &[
            "each companion changes tactics and emotional stakes",
            "bond scene reveals character, not only mechanics",
            "rival bond exposes the hero's blind spot",
            "mentor knows old rules that the new path breaks",
            "comic animal beat relieves pressure without breaking tension",
            "beast danger shows the world beyond school walls",
        ],
    ),
];

const VR_CULTIVATION_AXES: &[(&str, &[&str])] = &[
    (
        "dual_world",
        &[
            "game freedom contrasts a trapped real body",
            "VR realm may be more real than players believe",
            "real-world weakness becomes in-game sensitivity",
            "sibling bond anchors the power fantasy",
            "online fame threatens offline secrecy",
            "game progress changes social value outside the game",
        ],
    ),
    (
        "cultivation_payoff",
        &[
            "hidden quest reveals spiritual law",
            "naive genius misunderstands status but grasps essence",
            "NPC treats the hero as person, not player",
            "auction, sect, manual, or tournament turns progress public",
            "overpowered talent stays charming through innocence",
            "new realm opens because the hero changes how others see him",
        ],
    ),
    (
        "pacing_guard",
        &[
            "no ten chapters of past for one chapter of present",
            "avoid filler errands without power, relationship, or mystery movement",
            "each flashback must change the present choice",
            "face-slap scenes should reveal a new rule or status",
            "real and game worlds must trade consequences",
            "reader fantasy stays clean by cutting repetitive explanations",
        ],
    ),
];

const MONSTER_EVOLUTION_AXES: &[(&str, &[&str])] = &[
    (
        "body_problem",
        &[
            "nonhuman body makes basic movement dramatic",
            "eating, sensing, hiding, and escaping become the first skill tree",
            "comically weak form must still carry credible stakes",
            "new body grants freedom but creates disgust or vulnerability",
            "resource scarcity makes tiny gains satisfying",
            "first social contact tests whether the monster is person or prey",
        ],
    ),
    (
        "evolution_loop",
        &[
            "consume resource, gain trait, test trait, pay drawback",
            "small improvement changes map access",
            "race and class evolve on different axes",
            "choice locks one future and closes another",
            "adaptability beats raw stats",
            "power fantasy escalates without breaking challenge",
        ],
    ),
    (
        "data_balance",
        &[
            "numbers appear after desire, danger, or choice",
            "stat block changes must alter the next scene",
            "do not explain and show the same emotion twice",
            "romance develops through action before inner summary",
            "grind scenes need discovery, humour, threat, or relationship",
            "slow start earns patience only if the chapter hook is clear",
        ],
    ),
];

const HIGH_DRAMA_ROMANCE_AXES: &[(&str, &[&str])] = &[
    (
        "public_humiliation",
        &[
            "rejected by mate, family, pack, or spouse in front of witnesses",
            "substitute bride cannot reveal her true identity",
            "pregnancy, child, or heir secret changes the power balance",
            "ruthless leader misjudges the only person who loved them",
            "family betrayal forces exile from status and safety",
            "legal or social bond traps the heroine with an enemy",
        ],
    ),
    (
        "desire_pressure",
        &[
            "enemy-to-lover bond cannot be ignored",
            "powerful love interest must earn forgiveness, not claim it",
            "children expose the lie adults want hidden",
            "pack, boardroom, court, or family turns private pain public",
            "revenge and longing pull in opposite directions",
            "the heroine gains leverage before reconciliation",
        ],
    ),
    (
        "story_hook",
        &[
            "title states the scandal in one breath",
            "first chapter delivers the betrayal fast",
            "every secret has a reveal audience",
            "romance hook is tied to justice and status repair",
            "side antagonists keep pressure visible",
            "happy ending requires accountability, not only attraction",
        ],
    ),
];

const TECH_FANTASY_CELEBRATION_AXES: &[(&str, &[&str])] = &[
    (
        "launch_signal",
        &[
            "first public install works from a clean terminal",
            "release tag becomes a door everyone can open",
            "package registry accepts the name",
            "checksum files prove every build is real",
            "a local tool graduates into a public command",
            "the maintainer sees their own tool answer back",
        ],
    ),
    (
        "magic_system",
        &[
            "commands behave like spells with receipts",
            "version numbers are binding runes",
            "the registry is a city of names",
            "release assets are keys for different machines",
            "checksums are truth seals",
            "CI is the gatekeeper that only trusts evidence",
        ],
    ),
    (
        "human_pressure",
        &[
            "the creator wants the tool to prove it was worth shipping",
            "a celebration must stay accurate instead of flattering",
            "the draft has to carry joy without inventing facts",
            "the tool must help more than an unguided draft",
            "public release turns private taste into a standard",
            "quality gates must protect the moment from easy generic magic",
        ],
    ),
    (
        "freshness_turn",
        &[
            "the spell works only when the output tells the truth",
            "the victory is measured by repeatable commands, not applause",
            "the package name becomes a responsibility to future agents",
            "the better story comes from constraints, not decoration",
            "the launch reveals the tool still needs sharper gates",
            "the celebration becomes the first regression test",
        ],
    ),
    (
        "concrete_stage",
        &[
            "terminal window",
            "npm registry",
            "release page",
            "checksum file",
            "version prompt",
            "fresh install in /tmp",
            "green CI run",
            "packed tarball",
        ],
    ),
];

const GENERAL_FICTION_AXES: &[(&str, &[&str])] = &[
    (
        "story_draw",
        &[
            "a desire blocked by a specific person",
            "a secret that changes the next choice",
            "a place that demands a cost",
            "a relationship under pressure",
            "a rule that works until it matters",
            "a small object with social meaning",
        ],
    ),
    (
        "pressure",
        &[
            "deadline",
            "debt",
            "public shame",
            "misunderstanding",
            "physical danger",
            "lost trust",
            "competing duty",
            "scarce resource",
        ],
    ),
    (
        "turn",
        &[
            "new information reverses the plan",
            "help creates a worse obligation",
            "a private want becomes public",
            "the easy solution harms someone",
            "a lie protects the wrong person",
            "the character wins the wrong thing",
        ],
    ),
    (
        "texture",
        &[
            "specific work",
            "weather with consequence",
            "shared food",
            "damaged tool",
            "local ritual",
            "body memory",
            "unpaid debt",
            "ordinary noise at the wrong time",
        ],
    ),
];

const GENERAL_WRITING_AXES: &[(&str, &[&str])] = &[
    (
        "job",
        &[
            "explain a decision",
            "ask for a reply",
            "name a thing",
            "summarise evidence",
            "teach a workflow",
            "sell without hype",
            "warn about risk",
            "make a next step obvious",
        ],
    ),
    (
        "reader_need",
        &[
            "clarity",
            "trust",
            "speed",
            "proof",
            "confidence",
            "low friction",
            "accurate naming",
            "plain sequence",
        ],
    ),
    (
        "failure_to_avoid",
        &[
            "AI-sounding filler",
            "unsupported confidence",
            "abstract benefits",
            "buried action",
            "builder vocabulary",
            "over-formal tone",
            "generic conclusion",
            "missing evidence",
        ],
    ),
    (
        "revision_move",
        &[
            "replace vague claim with example",
            "move command before explanation",
            "cut decorative phrase",
            "name the actor",
            "add date or scope",
            "split an overstuffed sentence",
            "use the reader's words",
            "state the next action",
        ],
    ),
];

const EVAL_DIMENSIONS: &[(&str, &str)] = &[
    (
        "constraint_adherence",
        "Does the draft preserve required facts, avoid forbidden claims, and satisfy the user request before style is judged?",
    ),
    (
        "hook_and_reader_grip",
        "Does the opening quickly create desire, danger, wonder, injustice, or curiosity?",
    ),
    (
        "opening_micro_scene",
        "Does the first chapter show the smallest dramatic unit of the macro premise before naming the whole roadmap?",
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
        "Does the draft mix familiar genre appeal with fresh, concrete details?",
    ),
    (
        "voice_and_language",
        "Do word choice, rhythm, dialogue, imagery, and distance fit the POV?",
    ),
    (
        "progression_payoff",
        "Does the chapter advance capability, knowledge, status, stakes, relationship, or open-question state?",
    ),
    (
        "costly_power",
        "Does every major advantage create a cost, limit, exposure risk, moral pressure, or social consequence?",
    ),
    (
        "mystery_and_world_depth",
        "Does the world suggest geography, culture, history, system, technology, ecology, or myth beyond the immediate fight?",
    ),
    (
        "serial_retention",
        "Does the chapter leave a fair reason to continue: decision, reveal, debt, threat, relationship shift, or progress milestone?",
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
#[command(about = "A local fiction-writing CLI for AI agents.")]
#[command(after_help = "Install:
  npx novel-craft setup
  npx novel-craft start

Useful agent loop:
  novel-craft agent plan --idea \"weak-to-strong kingdom-building system\" --chapters 1 --genre system-isekai --json
  novel-craft creative atlas --json
  novel-craft creative brief --idea \"launch-night tech fantasy for a newly published CLI\" --genre tech-fantasy-celebration --must-include \"package name: novel-craft\" --must-avoid \"wrong version number\"
  novel-craft eval chapter draft.md --genre tech-fantasy-celebration --json
  novel-craft eval gate draft.md --must-include \"package name: novel-craft\" --must-avoid \"wrong version number\" --json
  novel-craft eval compare draft-a.md draft-b.md --json
  novel-craft writing guide

Agent-friendly flags:
  --json       emit machine-readable output where supported
  --out PATH   write deterministic packets/reports to a file
  --no-input   prevent interactive prompts on guided commands
  --defaults   accept guided-command defaults

Important:
  Lexical novelty signals are not story quality scores. Use gates and comparison packets to route judgement.")]
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

The wizard creates .novel/ project state and a start packet."
    )]
    Start(StartArgs),
    #[command(about = "Create .novel project state without the guided wizard.")]
    Init(InitArgs),
    #[command(
        about = "Run first-time setup and optionally install bundled agent skills.",
        after_help = "Interactive first run:
  npx novel-craft setup

Unattended install:
  novel-craft setup --yes --target ~/.codex/skills --json

Opt out of skill installation:
  novel-craft setup --no-skills --json"
    )]
    Setup(SetupArgs),
    #[command(about = "Run read-only install, asset, wrapper, and project checks.")]
    Doctor(DoctorArgs),
    #[command(about = "Build agent-facing story and chapter workflow packets.")]
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
    },
    #[command(about = "List, export, install, and inspect bundled agent skills.")]
    Skills {
        #[command(subcommand)]
        command: SkillsCommand,
    },
    #[command(about = "Emit the bundled Novel Craft writing-support guide or JSON profile.")]
    Writing {
        #[command(subcommand)]
        command: WritingCommand,
    },
    #[command(about = "Inspect and refresh fiction craft rules.")]
    Rules {
        #[command(subcommand)]
        command: RulesCommand,
    },
    #[command(about = "Build creative prompt packets and novelty or trope reports.")]
    Creative {
        #[command(subcommand)]
        command: CreativeCommand,
    },
    #[command(about = "Run gates, rubrics, reader checks, and comparison reports.")]
    Eval {
        #[command(subcommand)]
        command: EvalCommand,
    },
    #[command(about = "Create and inspect scene cards.")]
    Scene {
        #[command(subcommand)]
        command: SceneCommand,
    },
    #[command(about = "Create or update character records.")]
    Character {
        #[command(subcommand)]
        command: CharacterCommand,
    },
    #[command(about = "Set core story seed and project-level canon.")]
    Story {
        #[command(subcommand)]
        command: StoryCommand,
    },
    #[command(about = "Track plot threads, promises, and payoffs.")]
    Plot {
        #[command(subcommand)]
        command: PlotCommand,
    },
    #[command(about = "Build and audit the story matrix.")]
    Matrix {
        #[command(subcommand)]
        command: MatrixCommand,
    },
    #[command(about = "Build agent-ready context packets.")]
    Context {
        #[command(subcommand)]
        command: ContextCommand,
    },
    #[command(about = "Run deterministic line, scene, or plot checks.")]
    Lint {
        #[command(subcommand)]
        command: LintCommand,
    },
    #[command(about = "Run continuity, repetition, and causality audits.")]
    Audit {
        #[command(subcommand)]
        command: AuditCommand,
    },
    #[command(about = "Extract, commit, and sync story memory.")]
    Memory {
        #[command(subcommand)]
        command: MemoryCommand,
    },
    #[command(about = "Export reports to shareable formats.")]
    Export {
        #[command(subcommand)]
        command: ExportCommand,
    },
    #[command(about = "Build a drafting prompt for a target scene.")]
    Draft(TargetArgs),
    #[command(about = "Build a next-chapter planning packet.")]
    Next(TargetArgs),
    #[command(about = "Analyse a draft with bundled fiction checks.")]
    Analyse(FileReportArgs),
    #[command(about = "Create a focused review packet for a draft.")]
    Review(ReviewArgs),
    #[command(about = "Create a targeted revision packet.")]
    Revise(ReviseArgs),
    #[command(about = "Compare two files and summarise the change.")]
    Diff(DiffArgs),
    #[command(about = "Audit a folder or manuscript for long-form risks.")]
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
        default_value = "a settlement system that rewards useful repairs, protected resources, and earned public trust"
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
struct SetupArgs {
    #[arg(long)]
    target: Option<PathBuf>,
    #[arg(long, short = 'y', conflicts_with = "no_skills")]
    yes: bool,
    #[arg(long)]
    no_skills: bool,
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    json: bool,
}

#[derive(Args, Clone)]
struct TargetArgs {
    target: Option<String>,
    #[arg(long)]
    from: Option<PathBuf>,
    #[arg(long, default_value = "1200-2500 words")]
    word_count: String,
    #[arg(long, default_value = "deep third unless project state says otherwise")]
    pov: String,
    #[arg(long)]
    must_include: Vec<String>,
    #[arg(long)]
    avoid: Vec<String>,
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
    json: bool,
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
enum AgentCommand {
    #[command(about = "Build an agent-facing plan for one or more finished chapters.")]
    Plan(AgentPlanArgs),
}

#[derive(Args, Clone)]
struct AgentPlanArgs {
    #[arg(long)]
    idea: String,
    #[arg(long, default_value_t = 1)]
    chapters: usize,
    #[arg(long, default_value = "general-fiction")]
    genre: String,
    #[arg(long, default_value = "fast-webnovel")]
    profile: String,
    #[arg(long)]
    avoid: Vec<String>,
    #[arg(long)]
    must_include: Vec<String>,
    #[arg(long)]
    must_avoid: Vec<String>,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum SkillsCommand {
    #[command(about = "List bundled skills.")]
    List {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Export bundled skills to a directory.")]
    Export {
        #[arg(long)]
        out: PathBuf,
    },
    #[command(about = "Install bundled skills into an agent skills directory.")]
    Install {
        #[arg(long)]
        target: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    #[command(about = "Check installed bundled skills.")]
    Doctor {
        #[arg(long)]
        target: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum WritingCommand {
    #[command(about = "Show the bundled Novel Craft writing-support profile.")]
    Show {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "Emit a Novel Craft writing-support guide.")]
    Guide {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum RulesCommand {
    #[command(about = "List active rule cards.")]
    List {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Emit a rule guide.")]
    Guide {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "Audit the project rule setup.")]
    Audit {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Refresh project-local rules from bundled defaults.")]
    Refresh {
        #[arg(long)]
        backup: bool,
    },
}

#[derive(Subcommand)]
enum CreativeCommand {
    #[command(about = "List broad mix-and-match story genres, subgenres, tropes, and sub-tropes.")]
    Atlas {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "List creativity methods for premise generation.")]
    Methods {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Show trope axes for a genre or profile.")]
    Tropes {
        #[arg(long, default_value = "system-isekai")]
        genre: String,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Build an agent-ready creative brief from a user prompt.")]
    Brief(CreativeBriefArgs),
    #[command(about = "Diagnose readability and word-choice risks in a draft.")]
    Diagnose(FileReportArgs),
    #[command(about = "Report lexical novelty signals for a draft.")]
    Novelty(NoveltyArgs),
    #[command(about = "Check trope use, cost, twist, and saturation risk.")]
    TropeCheck(TropeCheckArgs),
    #[command(about = "Generate a multi-contender premise tournament packet.")]
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
        default_value = "serial fiction readers who want fast wonder, clear stakes, and a reason to continue"
    )]
    audience: String,
    #[arg(long, default_value = "6-8")]
    reading_grade: String,
    #[arg(long)]
    trope: Vec<String>,
    #[arg(long)]
    avoid: Vec<String>,
    #[arg(long)]
    must_include: Vec<String>,
    #[arg(long)]
    must_avoid: Vec<String>,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct NoveltyArgs {
    path: PathBuf,
    #[arg(long, default_value = "general-fiction")]
    genre: String,
    #[arg(long)]
    experimental_score: bool,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct TropeCheckArgs {
    path: PathBuf,
    #[arg(long, default_value = "system-isekai")]
    genre: String,
    #[arg(long)]
    json: bool,
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
    must_include: Vec<String>,
    #[arg(long)]
    must_avoid: Vec<String>,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum EvalCommand {
    #[command(about = "Emit a multi-dimensional creative-writing rubric.")]
    Rubric {
        #[arg(long, default_value = "system-isekai")]
        genre: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "Create an evidence-based score sheet for one draft.")]
    Sheet(FileReportArgs),
    #[command(about = "Review a story or chapter file after drafting.")]
    Story(StoryReportArgs),
    #[command(about = "Review a chapter file after drafting.")]
    Chapter(StoryReportArgs),
    #[command(about = "Run the deterministic draft guidance gate.")]
    Gate(GateArgs),
    #[command(about = "Compare two drafts without choosing an automatic winner.")]
    Compare {
        a: PathBuf,
        b: PathBuf,
        #[arg(long, default_value = "fast-webnovel")]
        profile: String,
        #[arg(long)]
        must_include: Vec<String>,
        #[arg(long)]
        must_avoid: Vec<String>,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "List built-in reader profiles.")]
    ReaderProfiles {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Check a draft against a reader profile.")]
    ReaderCheck {
        path: PathBuf,
        #[arg(long, default_value = "fast-webnovel")]
        profile: String,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "Compare dialogue fingerprints across drafts.")]
    VoiceDrift {
        paths: Vec<PathBuf>,
        #[arg(long, default_value = "")]
        character: String,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "Store beta-reader or human feedback as evidence.")]
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
    #[command(about = "Summarise stored feedback.")]
    FeedbackReport {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Add a liked, disliked, or mixed sample for calibration.")]
    CalibrateAdd {
        path: PathBuf,
        #[arg(long)]
        label: String,
        #[arg(long)]
        reason: Vec<String>,
        #[arg(long)]
        tag: Vec<String>,
    },
    #[command(about = "Summarise local taste calibration samples.")]
    CalibrateReport {
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Export a pairwise preference record.")]
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
    #[command(about = "Summarise exported pairwise preference records.")]
    RewardReport {
        #[arg(long)]
        path: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Args, Clone)]
struct StoryReportArgs {
    path: PathBuf,
    #[arg(long, default_value = "general-fiction")]
    genre: String,
    #[arg(long, default_value = "fast-webnovel")]
    profile: String,
    #[arg(long)]
    must_include: Vec<String>,
    #[arg(long)]
    must_avoid: Vec<String>,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct GateArgs {
    path: PathBuf,
    #[arg(long, default_value = "fast-webnovel")]
    profile: String,
    #[arg(long)]
    must_include: Vec<String>,
    #[arg(long)]
    must_avoid: Vec<String>,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum SceneCommand {
    #[command(about = "Create a structured scene card.")]
    Create(Box<SceneCreateArgs>),
    #[command(about = "Show a scene card.")]
    Show {
        id: String,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Infer a scene card from draft text.")]
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
    #[command(about = "Add a character record.")]
    Add(CharacterArgs),
    #[command(about = "Update a character record.")]
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
enum StoryCommand {
    #[command(about = "Set or update the core story seed used by context and draft packets.")]
    Set(StorySetArgs),
}

#[derive(Args, Clone)]
struct StorySetArgs {
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    genre: Option<String>,
    #[arg(long)]
    premise: Option<String>,
    #[arg(long)]
    protagonist: Option<String>,
    #[arg(long)]
    protagonist_want: Option<String>,
    #[arg(long)]
    protagonist_wound: Option<String>,
    #[arg(long)]
    world: Option<String>,
    #[arg(long)]
    power_system: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum PlotCommand {
    #[command(about = "Create or update a plot thread.")]
    Thread(PlotThreadArgs),
    #[command(about = "Record an open promise.")]
    AddPromise(PlotPromiseArgs),
    #[command(about = "Record a promise payoff.")]
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
    #[command(about = "Build the story matrix from project state.")]
    Build {
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Audit matrix repetition, causality, and promise load.")]
    Audit {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "Report hot, stale, and payoff-ready story threads.")]
    Heatmap {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ContextCommand {
    #[command(about = "Build a layered context packet for a scene or chapter.")]
    Build(TargetArgs),
}

#[derive(Subcommand)]
enum LintCommand {
    #[command(about = "Run deterministic line-level checks.")]
    Line(FileReportArgs),
    #[command(about = "Run scene-level checks.")]
    Scene(FileReportArgs),
    #[command(about = "Run plot-level checks.")]
    Plot(FileReportArgs),
}

#[derive(Subcommand)]
enum AuditCommand {
    #[command(about = "Check draft continuity against project state.")]
    Continuity(FileReportArgs),
    #[command(about = "Check repeated words, beats, and gestures.")]
    Repetition(FileReportArgs),
    #[command(about = "Run the therefore/because causality audit.")]
    Causality {
        path: Option<PathBuf>,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum MemoryCommand {
    #[command(about = "Extract a reviewable memory diff from text.")]
    Extract {
        path: PathBuf,
        #[arg(long)]
        scene_id: Option<String>,
        #[arg(long)]
        review: bool,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    #[command(about = "Commit an approved memory diff.")]
    Commit { diff: PathBuf },
    #[command(about = "Sync memory from a project file.")]
    Sync { path: PathBuf },
}

#[derive(Subcommand)]
enum ExportCommand {
    #[command(about = "Export Markdown content as HTML.")]
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
        Command::Setup(args) => command_setup(args),
        Command::Doctor(args) => command_doctor(args),
        Command::Agent { command } => command_agent(command),
        Command::Skills { command } => command_skills(command),
        Command::Writing { command } => command_writing(command),
        Command::Rules { command } => command_rules(command),
        Command::Creative { command } => command_creative(command),
        Command::Eval { command } => command_eval(command),
        Command::Scene { command } => command_scene(command),
        Command::Character { command } => command_character(command),
        Command::Story { command } => command_story(command),
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
    write_story_seed_from_start(&root, &args)?;
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
            "title": args.title,
            "idea": args.idea,
            "genre": args.genre
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

fn command_setup(args: SetupArgs) -> Result<()> {
    let target = args.target.unwrap_or_else(default_skill_target);
    let primary_skills = primary_skill_names();
    let install_later_command = format!("novel-craft skills install --target {}", target.display());
    let can_prompt = !args.json && io::stdin().is_terminal() && io::stdout().is_terminal();
    let should_install = if args.no_skills {
        false
    } else if args.yes {
        true
    } else if can_prompt {
        print_setup_wizard(&target, &primary_skills);
        confirm("Install these skills now?", true)?
    } else {
        false
    };

    let installed_paths = if should_install {
        install_skills_to(&target, args.dry_run)?
    } else {
        Vec::new()
    };
    let note = if should_install && args.dry_run {
        "Dry run complete. No skill files were written."
    } else if should_install {
        "Bundled Novel Craft skills installed."
    } else {
        "Bundled Novel Craft skills were not installed. Install them later with the command shown."
    };

    let data = json!({
        "status": "ok",
        "command": "setup",
        "target": target.display().to_string(),
        "dry_run": args.dry_run,
        "install_requested": should_install,
        "skills_installed": should_install && !args.dry_run,
        "opted_out": !should_install,
        "primary_skills": primary_skills,
        "embedded_skill_count": SKILLS.len(),
        "deprecated_alias_count": SKILLS.iter().filter(|(name, _)| name.starts_with("aliases/")).count(),
        "installed_paths": installed_paths,
        "install_later_command": install_later_command,
        "why_skills_matter": "The bundled skills are crucial for Novel Craft to work correctly in an agent workflow: they teach planning, drafting, review, continuation, and memory use.",
        "note": note
    });

    if args.json {
        print_json(data)
    } else {
        if !can_prompt {
            println!("Novel Craft setup");
            println!("Bundled skills:");
            for skill in primary_skill_names() {
                println!("- {skill}");
            }
            println!();
        }
        println!("{note}");
        if should_install {
            println!("Skill target: {}", target.display());
            println!(
                "Skill files: {}",
                data["installed_paths"].as_array().map_or(0, Vec::len)
            );
        } else {
            println!("Install later: {install_later_command}");
        }
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
        "scope": "local project, package, and embedded asset checks"
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

fn write_story_seed_from_start(root: &Path, args: &StartArgs) -> Result<()> {
    let path = root.join(PROJECT_DIR).join("state").join("story-seed.yml");
    let data = json!({
        "title": args.title,
        "genre": args.genre,
        "premise": args.idea,
        "protagonist_want": args.protagonist_want,
        "protagonist_wound": args.protagonist_wound,
        "world": args.world,
        "power_system": args.power_system,
        "audience": args.audience,
        "tone": args.tone,
        "reading_level": args.reading_level,
        "updated_at": now()
    });
    write_yaml_json(&path, &data)
}

fn update_project_config_from_story_set(root: &Path, args: &StorySetArgs) -> Result<()> {
    if args.title.is_none() && args.genre.is_none() {
        return Ok(());
    }
    let path = root.join(PROJECT_DIR).join("project.yml");
    let mut config: ProjectConfig = read_yaml_value(&path)
        .and_then(|value| serde_yaml::from_value(value).ok())
        .unwrap_or_default();
    if let Some(title) = args.title.as_ref().filter(|value| !value.trim().is_empty()) {
        config.title = title.clone();
    }
    if let Some(genre) = args.genre.as_ref().filter(|value| !value.trim().is_empty()) {
        config.genre_profile = genre.clone();
    }
    if config.created_at.is_empty() {
        config.created_at = now();
    }
    write_yaml(&path, &config)
}

fn write_story_seed_character(root: &Path, name: &str, args: &StorySetArgs) -> Result<()> {
    let path = root
        .join(PROJECT_DIR)
        .join("characters")
        .join(format!("{}.yml", slug(name)));
    if path.exists() {
        return Ok(());
    }
    let data = json!({
        "name": name,
        "age": null,
        "appearance": "",
        "voice": "",
        "traits": [],
        "motives": args.protagonist_want.iter().cloned().collect::<Vec<_>>(),
        "wounds": args.protagonist_wound.iter().cloned().collect::<Vec<_>>(),
        "secrets": [],
        "knowledge": [],
        "updated_at": now()
    });
    write_yaml_json(&path, &data)
}

fn command_agent(command: AgentCommand) -> Result<()> {
    match command {
        AgentCommand::Plan(args) => {
            let data = agent_plan_json(&args);
            if args.json {
                write_json_or_print(args.out, &data)
            } else {
                let packet = agent_plan_packet(&args);
                write_or_print(args.out, &packet)
            }
        }
    }
}

fn command_skills(command: SkillsCommand) -> Result<()> {
    match command {
        SkillsCommand::List { json } => {
            let names: Vec<&str> = SKILLS.iter().map(|(name, _)| *name).collect();
            if json {
                let skill_records: Vec<_> = SKILLS
                    .iter()
                    .map(|(name, _)| {
                        json!({
                            "path": name,
                            "deprecated_alias": name.starts_with("aliases/")
                        })
                    })
                    .collect();
                print_json(json!({ "skills": names, "skill_records": skill_records }))
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
            let paths = install_skills_to(&target, dry_run)?;
            for path in paths {
                if dry_run {
                    println!("would write {path}");
                } else {
                    println!("wrote {path}");
                }
            }
            Ok(())
        }
        SkillsCommand::Doctor { target, json } => {
            let data = json!({
                "embedded_skill_count": SKILLS.len(),
                "target": target.as_ref().map(|p| p.display().to_string()),
                "target_exists": target.as_ref().map(|p| p.exists()),
                "scope": "embedded Novel Craft skills"
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

fn primary_skill_names() -> Vec<&'static str> {
    SKILLS
        .iter()
        .filter_map(|(name, _)| {
            if name.starts_with("aliases/") {
                None
            } else {
                name.strip_suffix("/SKILL.md")
            }
        })
        .collect()
}

fn default_skill_target() -> PathBuf {
    if let Ok(value) = std::env::var("NOVEL_CRAFT_SKILLS_DIR") {
        if !value.trim().is_empty() {
            return PathBuf::from(value);
        }
    }
    if let Ok(value) = std::env::var("CODEX_HOME") {
        if !value.trim().is_empty() {
            return PathBuf::from(value).join("skills");
        }
    }
    if let Ok(value) = std::env::var("HOME") {
        if !value.trim().is_empty() {
            return PathBuf::from(value).join(".codex").join("skills");
        }
    }
    PathBuf::from(".").join("skills")
}

fn install_skills_to(target: &Path, dry_run: bool) -> Result<Vec<String>> {
    let mut paths = Vec::new();
    for (name, body) in SKILLS {
        let path = target.join(name);
        if !dry_run {
            write_text(&path, body)?;
        }
        paths.push(path.display().to_string());
    }
    Ok(paths)
}

fn print_setup_wizard(target: &Path, primary_skills: &[&str]) {
    println!("Novel Craft setup");
    println!();
    println!("This startup wizard can install the bundled Novel Craft skills.");
    println!(
        "These skills are crucial for Novel Craft to work correctly in an agent workflow: they teach the agent when to plan, draft, review, continue, and sync story memory with the CLI."
    );
    println!();
    println!("Following skills:");
    for skill in primary_skills {
        println!("- {skill}");
    }
    println!();
    println!(
        "Deprecated alias stubs are also included for one release so older skill names keep working."
    );
    println!("Install target: {}", target.display());
    println!("You can opt out now and install later with:");
    println!("  novel-craft skills install --target {}", target.display());
    println!();
}

fn command_writing(command: WritingCommand) -> Result<()> {
    match command {
        WritingCommand::Show { json, out } => {
            let data = profile_json(&GENERAL_WRITING_PROFILE);
            if json {
                print_json(data)
            } else {
                write_or_print(out, &serde_yaml::to_string(&data)?)
            }
        }
        WritingCommand::Guide { json, out } => {
            let packet = profile_packet(&GENERAL_WRITING_PROFILE);
            if json {
                print_json(json!({
                    "guide": GENERAL_WRITING_PROFILE.id,
                    "packet": packet
                }))
            } else {
                write_or_print(out, &packet)
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
        CreativeCommand::Atlas { json } => {
            if json {
                print_json(story_atlas_json())
            } else {
                write_or_print(None, &story_atlas_text())
            }
        }
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
        CreativeCommand::Tropes { genre, json } => {
            let data = tropes_json(&genre);
            if json {
                print_json(data)
            } else {
                println!("{}", serde_yaml::to_string(&data)?);
                Ok(())
            }
        }
        CreativeCommand::Brief(args) => {
            let text = creative_brief(&args);
            write_or_print(args.out, &text)
        }
        CreativeCommand::Diagnose(args) => report_file(
            args,
            |path, text| json!({"path": path, "metrics": metrics(text), "word_choice": word_choice_diagnostics(text)}),
        ),
        CreativeCommand::Novelty(args) => {
            let text = read_text(&args.path)?;
            let data = json!({
                "path": args.path,
                "genre": args.genre,
                "novelty": novelty_analysis(&text, args.experimental_score)
            });
            if args.json {
                print_json(data)
            } else {
                write_or_print(
                    args.out,
                    &format!(
                        "# Lexical Novelty Signals\n\n{}",
                        serde_yaml::to_string(&data)?
                    ),
                )
            }
        }
        CreativeCommand::TropeCheck(args) => {
            let text = read_text(&args.path)?;
            let data = json!({
                "path": args.path,
                "genre": args.genre,
                "trope_saturation": trope_saturation(&text, &args.genre)
            });
            if args.json {
                print_json(data)
            } else {
                write_or_print(
                    args.out,
                    &format!("# Trope Check\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
        CreativeCommand::Tournament(args) => {
            let text = tournament_text(
                &args.idea,
                &args.genre,
                args.count,
                &args.avoid,
                &args.must_include,
                &args.must_avoid,
            );
            if args.json {
                write_json_or_print(
                    args.out,
                    &json!({"idea": args.idea, "genre": args.genre, "count": args.count, "packet": text}),
                )
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
        EvalCommand::Story(args) => {
            let text = read_text(&args.path)?;
            let data = story_report(
                &args.path,
                &text,
                &args.genre,
                &args.profile,
                &args.must_include,
                &args.must_avoid,
                "story",
            );
            if args.json {
                write_json_or_print(args.out, &data)
            } else {
                write_or_print(
                    args.out,
                    &format!("# Story Review\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
        EvalCommand::Chapter(args) => {
            let text = read_text(&args.path)?;
            let data = story_report(
                &args.path,
                &text,
                &args.genre,
                &args.profile,
                &args.must_include,
                &args.must_avoid,
                "chapter",
            );
            if args.json {
                write_json_or_print(args.out, &data)
            } else {
                write_or_print(
                    args.out,
                    &format!("# Chapter Review\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
        EvalCommand::Gate(args) => {
            let text = read_text(&args.path)?;
            let data = gate_report(
                &args.path,
                &text,
                &args.profile,
                &args.must_include,
                &args.must_avoid,
            );
            if args.json {
                write_json_or_print(args.out, &data)
            } else {
                write_or_print(
                    args.out,
                    &format!("# Quality Gate\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
        EvalCommand::Compare {
            a,
            b,
            profile,
            must_include,
            must_avoid,
            json,
            out,
        } => {
            let a_text = read_text(&a)?;
            let b_text = read_text(&b)?;
            let packet = compare_text(
                &a,
                &b,
                &a_text,
                &b_text,
                &profile,
                &must_include,
                &must_avoid,
            );
            if json {
                write_json_or_print(
                    out,
                    &compare_json(
                        &a,
                        &b,
                        &a_text,
                        &b_text,
                        &profile,
                        &must_include,
                        &must_avoid,
                    ),
                )
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

fn command_story(command: StoryCommand) -> Result<()> {
    match command {
        StoryCommand::Set(args) => {
            let root = require_project()?;
            let state_path = root.join(PROJECT_DIR).join("state").join("story-seed.yml");
            let mut seed =
                read_yaml_value(&state_path).unwrap_or_else(|| Value::Mapping(Mapping::new()));
            upsert_yaml_string(&mut seed, "title", args.title.as_deref())?;
            upsert_yaml_string(&mut seed, "genre", args.genre.as_deref())?;
            upsert_yaml_string(&mut seed, "premise", args.premise.as_deref())?;
            upsert_yaml_string(&mut seed, "protagonist", args.protagonist.as_deref())?;
            upsert_yaml_string(
                &mut seed,
                "protagonist_want",
                args.protagonist_want.as_deref(),
            )?;
            upsert_yaml_string(
                &mut seed,
                "protagonist_wound",
                args.protagonist_wound.as_deref(),
            )?;
            upsert_yaml_string(&mut seed, "world", args.world.as_deref())?;
            upsert_yaml_string(&mut seed, "power_system", args.power_system.as_deref())?;
            upsert_yaml_string(&mut seed, "updated_at", Some(&now()))?;
            write_yaml_value(&state_path, &seed)?;

            update_project_config_from_story_set(&root, &args)?;
            if let Some(name) = args
                .protagonist
                .as_ref()
                .filter(|name| !name.trim().is_empty())
            {
                write_story_seed_character(&root, name, &args)?;
            }

            let data = json!({
                "status": "ok",
                "path": state_path,
                "story_seed": yaml_to_json(&seed)
            });
            if args.json {
                print_json(data)
            } else {
                println!("Story seed updated: {}", state_path.display());
                Ok(())
            }
        }
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
        AuditCommand::Causality { path, json, out } => {
            let data = if let Some(path) = path {
                let text = read_text(&path)?;
                causality_report(&path, &text)
            } else {
                let root = require_project()?;
                matrix_audit_data(&build_matrix(&root)?)
            };
            if json {
                write_json_or_print(out, &data)
            } else {
                write_or_print(
                    out,
                    &format!("# Causality Audit\n\n{}", serde_yaml::to_string(&data)?),
                )
            }
        }
    }
}

fn command_memory(command: MemoryCommand) -> Result<()> {
    match command {
        MemoryCommand::Extract {
            path,
            scene_id,
            review,
            json,
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
                "open_loops": open_loops_report(&text),
                "scene_change": scene_change_report(&text),
                "progression_and_power": progression_and_power_report(&text),
                "review_required": true,
                "review_notes": if review {
                    vec![
                        "Approve only facts that should become canon.",
                        "Reject accidental prose, speculation, and temporary uncertainty.",
                        "Commit approved memory with `novel-craft memory commit <diff.yml>`."
                    ]
                } else {
                    Vec::new()
                },
                "warnings": ["Review this diff before commit."],
                "created_at": now()
            });
            let root = require_project()?;
            if json {
                write_json_or_print(out, &data)
            } else {
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

fn next_chapter_from_draft(path: &Path, text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    json!({
        "source": path,
        "summary": {
            "next_best_action": "Plan the next chapter from the previous ending, open loops, cost, and power/status delta.",
            "risk_notes": ["Do not simply repeat the context packet; convert the last chapter's consequences into the next scene pressure."]
        },
        "what_changed": scene_change_report(text),
        "new_open_loops": open_loops_report(text),
        "reader_retention": reader_retention_report(text),
        "power_status_delta": progression_and_power_report(text),
        "natural_next_chapter_setup": {
            "immediate_goal": infer_next_goal(&lower),
            "immediate_conflict": infer_next_conflict(&lower),
            "relationship_state": infer_relationship_state(&lower),
            "new_cost": infer_new_cost(&lower),
            "next_hook": infer_next_hook(&lower)
        },
        "continuity_reminders": [
            "Carry forward any injury, debt, object, relationship shift, revealed rule, and unanswered question from the source chapter.",
            "Do not repeat the same opening beat; escalate along a different axis such as social pressure, scarcity, moral cost, location, or relationship leverage.",
            "Pay off at least one small expectation while opening one larger question."
        ],
        "next_chapter_card": {
            "goal": "Give the POV a visible next objective caused by the previous chapter's turn.",
            "conflict": "Make the new obstacle specific and active.",
            "turn": "Change the story state by the end of the chapter.",
            "cost": "Make the attempted win cost safety, trust, time, status, resources, or certainty.",
            "exit_hook": "End with a fair reason to read the following chapter."
        }
    })
}

fn infer_next_goal(lower: &str) -> &'static str {
    if contains_any(lower, &["stair", "stairs", "service stair"]) {
        "survive the stair and learn what it demands next"
    } else if contains_any(lower, &["door", "gate", "opened"]) {
        "cross the newly opened threshold without losing control of the cost"
    } else if contains_any(lower, &["marked", "brand", "claim"]) {
        "understand the mark before someone uses it against the protagonist"
    } else {
        "pursue the concrete objective caused by the previous chapter's final turn"
    }
}

fn infer_next_conflict(lower: &str) -> &'static str {
    if contains_any(lower, &["debt", "unpaid", "owed", "toll"]) {
        "the debt comes due before the protagonist understands the rules"
    } else if contains_any(lower, &["collector", "authority", "guard"]) {
        "an official or enforcer controls the route forward"
    } else if contains_any(lower, &["woke", "wake", "bell"]) {
        "the deadline ends and the dormant danger becomes active"
    } else {
        "the previous win creates a sharper obstacle instead of a clean escape"
    }
}

fn infer_relationship_state(lower: &str) -> &'static str {
    if contains_any(lower, &["lio", "ward", "child", "brother", "sister"]) {
        "the protected person is now a continuing pressure, not a prop"
    } else if contains_any(lower, &["trusted", "betrayed", "owed", "promised"]) {
        "trust, debt, or obligation has shifted and should affect the next choice"
    } else {
        "carry forward any new trust, leverage, suspicion, or dependency from the last scene"
    }
}

fn infer_new_cost(lower: &str) -> &'static str {
    if contains_any(lower, &["marked", "brand", "claim"]) {
        "the protagonist is marked or claimable"
    } else if contains_any(lower, &["debt", "owed", "toll"]) {
        "the protagonist owes a concrete debt"
    } else if contains_any(lower, &["blood", "wound", "pain"]) {
        "the victory has a bodily cost"
    } else {
        "name the cost of the previous chapter's apparent progress"
    }
}

fn infer_next_hook(lower: &str) -> &'static str {
    if contains_any(lower, &["wake", "woke", "bell"]) {
        "what wakes when the bell ends?"
    } else if contains_any(lower, &["stair", "stairs"]) {
        "what does the stair demand before it lets them climb?"
    } else if contains_any(lower, &["system", "ledger", "rank", "skill"]) {
        "what rule does the system reveal only after it hurts?"
    } else {
        "what consequence of the last chapter arrives first?"
    }
}

fn command_draft(args: TargetArgs, title: &str) -> Result<()> {
    let root = require_project()?;
    let target = args.target.unwrap_or_else(|| "next-scene".to_string());
    let packet = context_packet(&root, &target)?;
    let source_signals = if let Some(path) = &args.from {
        let text = read_text(path)?;
        Some(next_chapter_from_draft(path, &text))
    } else {
        None
    };
    let source_review = if let Some(signals) = &source_signals {
        format!(
            "\n## Source Draft Signals\n```yaml\n{}\n```\n",
            serde_yaml::to_string(signals)?
        )
    } else {
        String::new()
    };
    let text = format!(
        "# {title}: {target}\n\n## Prose Brief\n- Target length: {}\n- POV: {}\n- Must include: {}\n- Avoid: {}\n- Opening: start with a present-tense scene pressure before explaining the full roadmap.\n- Scene turn: the chapter must change plot, character, relationship, knowledge, stakes, or reader expectation.\n- Exposition: keep status screens, system rules, lore, and world labels subordinate to action, dialogue, discovery, cost, or consequence.\n- Ending: close with a fair continuation reason, not a fake cliffhanger.\n- Canon: preserve project state unless the scene card explicitly changes it.\n{}\n{packet}",
        args.word_count,
        args.pov,
        join_strings(&args.must_include),
        join_strings(&args.avoid),
        source_review
    );
    if args.json {
        print_json(json!({
            "target": target,
            "prompt": text,
            "source_draft_signals": source_signals
        }))
    } else {
        write_or_print(args.out, &text)
    }
}

fn quick_action_summary(text: &str, issues: &[Issue]) -> serde_json::Value {
    let opening = opening_promise_report(text);
    let open_loops = open_loops_report(text);
    let next_best_action = if opening["status"].as_str() == Some("warn") {
        "Revise the opening so the premise appears through a present choice, cost, or consequence."
    } else if !issues.is_empty() {
        "Review line-level signals and keep only the ones that are intentional and effective."
    } else if open_loops["question_mark_count"].as_u64().unwrap_or(0) == 0 {
        "Check that the ending leaves a clear continuation question or pressure."
    } else {
        "Run a focused review pass for the next craft risk: prose, structure, dialogue, or continuity."
    };
    json!({
        "top_findings": revision_priorities(
            issues,
            &constraint_adherence(text, &[], &[]),
            &opening,
            &trope_saturation(text, "general-fiction")
        )["items"],
        "next_best_action": next_best_action,
        "risk_notes": [
            "This is a routing summary; inspect the underlying sections before revising.",
            "A clean deterministic pass does not prove the chapter is finished."
        ]
    })
}

fn focused_review_report(path: &Path, text: &str, rubric: &str) -> serde_json::Value {
    let issues = lint_text(text);
    let rubric_key = normalize_key(rubric);
    let mut sections = serde_json::Map::new();
    match rubric_key.as_str() {
        "prose" | "line" | "line-review" => {
            sections.insert("prose_review".to_string(), prose_review(text));
            sections.insert("voice_review".to_string(), voice_review(text));
            sections.insert("line_lint".to_string(), json!(issues));
        }
        "dialogue" | "relationship" => {
            sections.insert(
                "relationship_and_dialogue".to_string(),
                relationship_and_dialogue_report(text),
            );
            sections.insert("voice_review".to_string(), voice_review(text));
        }
        "structure" | "chapter" | "scene" => {
            sections.insert("chapter_spine".to_string(), chapter_spine_report(text));
            sections.insert("scene_change".to_string(), scene_change_report(text));
            sections.insert(
                "reader_retention".to_string(),
                reader_retention_report(text),
            );
        }
        "continuity" | "memory" => {
            sections.insert("open_loops".to_string(), open_loops_report(text));
            sections.insert(
                "progression_and_power".to_string(),
                progression_and_power_report(text),
            );
            sections.insert(
                "canon_candidates".to_string(),
                json!(extract_fact_candidates(text)),
            );
        }
        _ => {
            sections.insert("chapter_spine".to_string(), chapter_spine_report(text));
            sections.insert("scene_change".to_string(), scene_change_report(text));
            sections.insert(
                "reader_retention".to_string(),
                reader_retention_report(text),
            );
            sections.insert("prose_review".to_string(), prose_review(text));
            sections.insert("voice_review".to_string(), voice_review(text));
            sections.insert("open_loops".to_string(), open_loops_report(text));
        }
    }
    json!({
        "path": path,
        "rubric": rubric,
        "summary": quick_action_summary(text, &issues),
        "metrics": metrics(text),
        "lint": lint_summary(&issues),
        "sections": sections,
        "review_questions": focused_review_questions(&rubric_key)
    })
}

fn focused_review_questions(rubric: &str) -> Vec<&'static str> {
    match rubric {
        "prose" | "line" | "line-review" => vec![
            "Where does the prose explain an emotion that could be embodied through action, gesture, or sensory detail?",
            "Which sentences have the same shape and could use rhythm contrast?",
            "Where does diction become generic instead of belonging to this POV?",
            "Which paragraph would be clearest if split, trimmed, or re-ordered?",
        ],
        "dialogue" | "relationship" => vec![
            "What does each speaker want underneath the surface topic?",
            "Who has leverage at the start, and who has it at the end?",
            "Which line states the feeling too directly?",
            "What relationship state changes by the end of the exchange?",
        ],
        "structure" | "chapter" | "scene" => vec![
            "What does the POV character want in this chapter?",
            "What resists them?",
            "What turns by the end?",
            "What does the turn cost, and why does the next chapter feel necessary?",
        ],
        "continuity" | "memory" => vec![
            "Which facts should become canon?",
            "Which questions need payoff windows?",
            "Which injuries, debts, objects, powers, or relationship shifts must carry forward?",
            "Which candidate facts are only temporary uncertainty and should not be committed?",
        ],
        _ => vec![
            "What is the highest-impact revision pass?",
            "Which warning is actually an intentional effect?",
            "Which scene-level change is weakest?",
            "What single edit most improves reader grip?",
        ],
    }
}

fn revision_packet(path: &Path, text: &str, pass: &str) -> serde_json::Value {
    let issues = lint_text(text);
    let review = focused_review_report(path, text, pass);
    let optional_priorities = if issues.is_empty() {
        vec![
            "Check whether the main relationship turn lands emotionally.",
            "Check whether named setting/system terms appear because the scene needs them.",
            "Check whether the ending threat or question is concrete enough to pull the next chapter.",
        ]
    } else {
        Vec::new()
    };
    json!({
        "path": path,
        "pass": pass,
        "summary": review["summary"],
        "deterministic_findings": issues,
        "focused_review": review["sections"],
        "optional_priorities": optional_priorities,
        "next_best_action": if optional_priorities.is_empty() {
            "Revise the ranked findings first, then run eval chapter again."
        } else {
            "No deterministic line issues surfaced for this pass. Use the optional priorities for human-quality review."
        }
    })
}

fn command_analyse(args: FileReportArgs) -> Result<()> {
    let text = read_text(&args.path)?;
    let issues = lint_text(&text);
    let data = json!({
        "path": args.path,
        "summary": quick_action_summary(&text, &issues),
        "metrics": metrics(&text),
        "issues": issues,
        "novelty": novelty_analysis(&text, false),
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
    let data = focused_review_report(&args.path, &text, &args.rubric);
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
    let data = revision_packet(&args.path, &text, &args.pass);
    if args.json {
        write_json_or_print(args.out, &data)
    } else {
        let report = format!(
            "# Revision Plan: {}\n\nPass: `{}`\n\n{}\n",
            args.path.display(),
            args.pass,
            serde_yaml::to_string(&data)?
        );
        let out = args
            .out
            .unwrap_or_else(|| args.path.with_extension(format!("{}.review.md", args.pass)));
        write_text(&out, &report)?;
        println!("Revision packet written: {}", out.display());
        print_ignored_project_state_note(&out);
        Ok(())
    }
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

fn confirm(label: &str, default: bool) -> Result<bool> {
    let suffix = if default { "Y/n" } else { "y/N" };
    loop {
        print!("{label} [{suffix}]: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim().to_ascii_lowercase();
        match trimmed.as_str() {
            "" => return Ok(default),
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Please answer yes or no."),
        }
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
        print_ignored_project_state_note(&path);
    } else {
        print!("{text}");
    }
    Ok(())
}

fn print_ignored_project_state_note(path: &Path) {
    if path
        .components()
        .any(|component| component.as_os_str() == PROJECT_DIR)
    {
        println!(
            "Note: {} is under ignored project state. Export or copy approved work when you need a shareable/tracked artifact.",
            path.display()
        );
    }
}

fn print_json<T: Serialize>(value: T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn write_json_or_print(out: Option<PathBuf>, value: &serde_json::Value) -> Result<()> {
    if let Some(path) = out {
        write_text(&path, &serde_json::to_string_pretty(value)?)?;
        println!("Written: {}", path.display());
        Ok(())
    } else {
        print_json(value)
    }
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

fn opening_window(text: &str, word_limit: usize) -> String {
    text.unicode_words()
        .take(word_limit)
        .collect::<Vec<_>>()
        .join(" ")
}

fn opening_promise_report(text: &str) -> serde_json::Value {
    let opening = opening_window(text, 260).to_lowercase();
    let macro_hits = count_hits(&opening, OPENING_MACRO_TERMS);
    let micro_hits = count_hits(&opening, OPENING_MICRO_TERMS);
    let action_hits = count_hits(&opening, OPENING_ACTION_TERMS);
    let announcement_hits = count_hits(&opening, OPENING_ANNOUNCEMENT_PHRASES);
    let macro_count = macro_hits.len();
    let micro_count = micro_hits.len();
    let action_count = action_hits.len();
    let announcement_count = announcement_hits.len();
    let risk = if announcement_count > 0 || (macro_count >= 5 && micro_count < 4) {
        "high"
    } else if macro_count >= 4 && action_count < 3 {
        "medium"
    } else {
        "low"
    };
    let status = if risk == "high" || risk == "medium" {
        "warn"
    } else {
        "pass"
    };
    json!({
        "status": status,
        "risk": risk,
        "method": "Heuristic opening-window check. It cannot judge taste, but it can flag openings that may be leaning on macro labels before dramatising a small scene.",
        "opening_window_words": opening.split_whitespace().count(),
        "macro_scale_hits": macro_hits,
        "micro_scene_hits": micro_hits,
        "action_hits": action_hits,
        "announcement_hits": announcement_hits,
        "review_questions": [
            "What is the smallest working unit of the story appeal in this chapter?",
            "Can the opening show one room, one person, one need, one rule, one cost, or one choice before naming the larger arc?",
            "Which macro labels can be delayed until after the reader has watched the mechanic affect a decision?",
            "Does every early system/world term change the present choice, obstacle, cost, or consequence?"
        ],
        "revision_hint": "Usually seed the macro appeal through micro-action. For kingdom-building, try one protected person, room, rule, meal, door, ledger, oath, or boundary before leaning on kingdoms, domains, empires, or future upgrade ladders."
    })
}

fn profile_json(profile: &WritingProfile) -> serde_json::Value {
    json!({
        "id": profile.id,
        "purpose": profile.purpose,
        "reader": profile.reader,
        "success_criteria": profile.success_criteria,
        "failure_modes": profile.failure_modes,
        "review_pass": profile.review_pass,
        "final_gate": profile.final_gate
    })
}

fn profile_packet(profile: &WritingProfile) -> String {
    format!(
        "# Novel Craft Writing Support Guide\n\n## Purpose\n{}\n\n## Reader\n{}\n\n## Success Criteria\n{}\n\n## Failure Modes\n{}\n\n## Review Pass\n{}\n\n## Final Gate\n{}\n",
        profile.purpose,
        profile.reader,
        bullet_lines(profile.success_criteria),
        bullet_lines(profile.failure_modes),
        bullet_lines(profile.review_pass),
        bullet_lines(profile.final_gate),
    )
}

fn bullet_lines(items: &[&str]) -> String {
    items
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n")
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

fn novelty_analysis(text: &str, experimental_score: bool) -> serde_json::Value {
    let lower = text.to_lowercase();
    let generic = count_hits(&lower, GENERIC_PHRASES);
    let freshness = count_hits(
        &lower,
        &[
            "cost",
            "cooldown",
            "limitation",
            "debt",
            "constraint",
            "trade-off",
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
    let freshness_categories = freshness.len();
    let concrete_categories = concrete.len();
    let generic_categories = generic.len();
    let score = 50 + (freshness_categories * 6 + concrete_categories * 2) as isize
        - (generic_categories * 8) as isize;
    let mut data = serde_json::Map::new();
    data.insert(
        "lexical_novelty_signals".to_string(),
        json!({
            "method": "Local lexical signal report; not a story quality score.",
            "signal_counts": {
                "freshness_categories": freshness_categories,
                "concrete_object_categories": concrete_categories,
                "generic_phrase_categories": generic_categories
            },
            "freshness_signal_hits": freshness,
            "concrete_object_hits": concrete,
            "generic_phrase_hits": generic,
            "limitations": [
                "Counts surface words, not originality.",
                "Does not judge factual accuracy, voice, causality, emotional effect, or task fit.",
                "A worse draft can have more lexical signals."
            ]
        }),
    );
    if experimental_score {
        data.insert(
            "experimental_score".to_string(),
            json!({
                "value": score.clamp(0, 100),
                "label": "not a quality score",
                "use": "debugging only; do not compare drafts by this number"
            }),
        );
    }
    data.insert("review_questions".to_string(), json!([
        "Name the familiar genre appeal.",
        "Name the freshness twist.",
        "List three details that could not be swapped into another generic story.",
        "Name any required fact the draft must preserve before style is judged.",
        "If the twist is missing, propose alternatives using cost, contradiction, role inversion, or system limitation."
    ]));
    serde_json::Value::Object(data)
}

fn lint_summary(issues: &[Issue]) -> serde_json::Value {
    let mut by_rule: BTreeMap<String, usize> = BTreeMap::new();
    for issue in issues {
        *by_rule.entry(issue.rule_id.clone()).or_default() += 1;
    }
    json!({
        "issue_count": issues.len(),
        "by_rule": by_rule
    })
}

fn revision_priorities(
    issues: &[Issue],
    constraints: &serde_json::Value,
    opening: &serde_json::Value,
    trope: &serde_json::Value,
) -> serde_json::Value {
    let mut items = Vec::new();
    if constraints["status"].as_str() == Some("fail") {
        items.push(json!({
            "rank": items.len() + 1,
            "level": "blocker",
            "focus": "constraint adherence",
            "action": "Restore required facts and remove forbidden claims before judging style."
        }));
    }
    if opening["status"].as_str() == Some("warn") {
        items.push(json!({
            "rank": items.len() + 1,
            "level": "high-impact craft issue",
            "focus": "opening motion",
            "action": "Revise the opening so the macro premise is dramatised through one present choice, cost, obstacle, or consequence."
        }));
    }
    if !issues.is_empty() {
        items.push(json!({
            "rank": items.len() + 1,
            "level": if issues.len() > 12 { "high-impact craft issue" } else { "line polish" },
            "focus": "line lint",
            "action": format!("Review {} line-level signals; keep any that are intentional and effective.", issues.len())
        }));
    }
    if trope["saturation_risk"].as_str() == Some("high") {
        items.push(json!({
            "rank": items.len() + 1,
            "level": "needs judgement",
            "focus": "trope stack",
            "action": "Decide whether the familiar signals are healthy genre appeal or a generic pile-up; keep only the ones that interact causally."
        }));
    }
    if items.is_empty() {
        items.push(json!({
            "rank": 1,
            "level": "next useful pass",
            "focus": "agent judgement",
            "action": "No deterministic blocker surfaced. Review goal, conflict, turn, cost, voice, and ending momentum before finalising."
        }));
    }
    json!({
        "method": "Action-ranked review leads for an agent revision pass; not an automatic quality verdict.",
        "items": items
    })
}

fn constraint_adherence(
    text: &str,
    must_include: &[String],
    must_avoid: &[String],
) -> serde_json::Value {
    let required: Vec<_> = must_include
        .iter()
        .filter(|item| !item.trim().is_empty())
        .map(|item| {
            let present = contains_case_insensitive(text, item);
            json!({"text": item, "present": present})
        })
        .collect();
    let forbidden: Vec<_> = must_avoid
        .iter()
        .filter(|item| !item.trim().is_empty())
        .map(|item| {
            let present = contains_case_insensitive(text, item);
            json!({"text": item, "present": present})
        })
        .collect();
    let missing_required: Vec<_> = required
        .iter()
        .filter(|row| !row["present"].as_bool().unwrap_or(false))
        .map(|row| row["text"].as_str().unwrap_or_default().to_string())
        .collect();
    let forbidden_present: Vec<_> = forbidden
        .iter()
        .filter(|row| row["present"].as_bool().unwrap_or(false))
        .map(|row| row["text"].as_str().unwrap_or_default().to_string())
        .collect();
    let has_constraints = !required.is_empty() || !forbidden.is_empty();
    let status = if !has_constraints {
        "unchecked"
    } else if missing_required.is_empty() && forbidden_present.is_empty() {
        "pass"
    } else {
        "fail"
    };
    json!({
        "status": status,
        "required": required,
        "forbidden": forbidden,
        "missing_required": missing_required,
        "forbidden_present": forbidden_present
    })
}

fn contains_case_insensitive(text: &str, needle: &str) -> bool {
    text.to_lowercase().contains(&needle.to_lowercase())
}

fn gate_report(
    path: &Path,
    text: &str,
    profile: &str,
    must_include: &[String],
    must_avoid: &[String],
) -> serde_json::Value {
    let issues = lint_text(text);
    let reader = reader_check(text, profile);
    let constraints = constraint_adherence(text, must_include, must_avoid);
    let opening = opening_promise_report(text);
    let trope = trope_saturation(text, "system-isekai");
    let priorities = revision_priorities(&issues, &constraints, &opening, &trope);
    let reader_warning_count = reader["warnings"].as_array().map_or(0, Vec::len);
    let constraint_status = constraints["status"].as_str().unwrap_or("unchecked");
    let opening_status = opening["status"].as_str().unwrap_or("pass");
    let status = if constraint_status == "fail" {
        "fail"
    } else if !issues.is_empty() || reader_warning_count > 0 || opening_status == "warn" {
        "warn"
    } else {
        "pass"
    };
    json!({
        "path": path,
        "status": status,
        "profile": profile,
        "metrics": metrics(text),
        "lint": lint_summary(&issues),
        "issues": issues,
        "reader_profile": reader,
        "constraint_adherence": constraints,
        "opening_promise": opening,
        "revision_priorities": priorities,
        "novelty": novelty_analysis(text, false),
        "requires_reviewer_judgement": true,
        "gate_notes": [
            "pass means deterministic checks did not find configured blockers",
            "warn means review likely issues before calling the draft final",
            "fail means a required fact is missing or a forbidden claim appears",
            "opening_promise is guidance; it warns when the opening may be announcing the macro premise before dramatising the smallest useful scene"
        ]
    })
}

fn chapter_spine_report(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    let opening = opening_promise_report(text);
    let opening_status = opening["status"].as_str().unwrap_or("pass");
    let exposition_warning = if opening_status == "warn" {
        vec!["Consider exposition pressure and delayed story motion: the opening may be explaining the macro premise before the reader sees a present choice, cost, or consequence."]
    } else {
        Vec::new()
    };
    json!({
        "method": "Heuristic chapter-spine review. Treat this as a checklist for agent/human judgement, not a verdict.",
        "hook": craft_signal(
            &lower,
            &["blood", "deadline", "danger", "debt", "stole", "missing", "woke", "kill", "hanged", "choice", "?"],
            "Check whether the first page creates unresolved tension, desire, danger, mystery, voice, or contradiction.",
        ),
        "orientation": craft_signal(
            &lower,
            &["room", "door", "city", "village", "kingdom", "street", "forest", "ship", "academy", "table", "gate"],
            "Check whether the reader can tell where they are, who is present, and what matters.",
        ),
        "goal": craft_signal(
            &lower,
            &["must", "wanted", "needed", "tried", "decided", "choice", "save", "escape", "steal", "win", "survive"],
            "Check whether the POV character wants something visible in this chapter.",
        ),
        "obstacle": craft_signal(
            &lower,
            &["but", "refused", "blocked", "guard", "rival", "enemy", "debt", "law", "curse", "danger", "couldn't", "could not"],
            "Check what resists the goal: person, rule, scarcity, danger, wound, lie, or missing information.",
        ),
        "escalation": craft_signal(
            &lower,
            &["worse", "before", "suddenly", "backfired", "recognised", "recognized", "revealed", "another", "too late", "instead"],
            "Check whether the first attempt complicates the problem instead of solving it cleanly.",
        ),
        "turn": craft_signal(
            &lower,
            &["decided", "chose", "learned", "discovered", "revealed", "betrayed", "opened", "changed", "lost", "found"],
            "Check what changes by the end: reveal, decision, betrayal, danger, price, victory, loss, or opportunity.",
        ),
        "cost": craft_signal(
            &lower,
            &["cost", "price", "debt", "blood", "wound", "injury", "lost", "sacrifice", "trust", "reputation", "guilt"],
            "Check what the chapter costs in safety, trust, time, status, morality, money, body, or future options.",
        ),
        "exit_hook": exit_hook_signal(text),
        "warnings": exposition_warning
    })
}

fn scene_change_report(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    json!({
        "method": "A chapter should change plot, character, relationship, knowledge, stakes, or reader expectation.",
        "plot_change": craft_signal(&lower, &["because", "therefore", "so", "after", "result", "opened", "closed"], "Check what plot state is different after the chapter."),
        "character_change": craft_signal(&lower, &["decided", "chose", "learned", "admitted", "refused", "promised"], "Check whether the character changes tactic, belief, self-knowledge, or commitment."),
        "relationship_change": craft_signal(&lower, &["trusted", "betrayed", "lied", "kissed", "owed", "friend", "enemy", "ally", "brother", "mother", "father"], "Check whether a relationship shifts in trust, leverage, intimacy, rivalry, debt, or threat."),
        "knowledge_change": craft_signal(&lower, &["learned", "discovered", "revealed", "clue", "secret", "realized", "realised", "truth"], "Check what the reader or POV knows now that they did not know before."),
        "stakes_change": craft_signal(&lower, &["deadline", "risk", "danger", "death", "cost", "lose", "lost", "execute", "hanged", "war"], "Check whether failure is clearer, closer, or more expensive."),
        "reader_expectation_change": craft_signal(&lower, &["question", "mystery", "system", "power", "curse", "rank", "kingdom", "reveal", "choice"], "Check what reader expectation was paid, opened, escalated, or transformed.")
    })
}

fn reader_retention_report(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    json!({
        "method": "Retention signals are prompts for revision. They do not predict real reader behaviour.",
        "open_question": craft_signal(&lower, &["?", "why", "who", "what", "where", "when", "secret", "missing"], "Check what question makes the reader continue."),
        "emotional_attachment": craft_signal(&lower, &["mother", "father", "sister", "brother", "child", "friend", "home", "hunger", "shame", "protect"], "Check what human attachment makes the pressure matter."),
        "payoff_expectation": craft_signal(&lower, &["training", "rank", "clue", "debt", "map", "key", "system", "reveal", "choice"], "Check what future payoff the chapter makes readers anticipate."),
        "ending_momentum": exit_hook_signal(text),
        "recurring_pleasure": craft_signal(&lower, &["banter", "training", "tactic", "mystery", "romance", "beast", "system", "craft", "duel", "investigation"], "Check what repeatable pleasure the reader can expect in later chapters.")
    })
}

fn prose_review(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    let metrics = metrics(text);
    json!({
        "method": "Local prose signals. Revise only when the pattern weakens clarity, rhythm, voice, or reader effect.",
        "clarity": {
            "average_sentence_words": metrics.average_sentence_words,
            "check": if metrics.average_sentence_words > 22.0 { "Consider splitting tangled sentences if pace or clarity suffers." } else { "Sentence length is within a broadly readable range." }
        },
        "rhythm": {
            "sentence_count": metrics.sentence_count,
            "paragraph_count": metrics.paragraph_count,
            "check": "Check whether sentence and paragraph length vary with pressure, reflection, action, and emotional landing."
        },
        "filter_word_hits": count_hits(&lower, FILTER_WORDS),
        "abstract_emotion_hits": count_hits(&lower, EMOTION_WORDS),
        "filler_phrase_hits": count_hits(&lower, &["began to", "started to", "seemed to", "in order to", "very", "really", "suddenly"]),
        "dialogue_directness": {
            "dialogue_line_count": metrics.dialogue_line_count,
            "check": "Check whether dialogue carries want, leverage, subtext, or power movement instead of only explaining feelings."
        },
        "sensory_grounding": craft_signal(&lower, &["cold", "hot", "salt", "smoke", "blood", "iron", "wet", "rough", "silence", "stink", "taste"], "Check whether sensory detail is specific and story-functional.")
    })
}

fn voice_review(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    json!({
        "method": "Voice is worldview under pressure. These prompts ask what the POV notices, values, fears, and misreads.",
        "notice_patterns": {
            "survival_terms": count_hits(&lower, &["exit", "knife", "guard", "hunger", "blood", "shelter"]),
            "status_terms": count_hits(&lower, &["rank", "title", "noble", "servant", "court", "guild"]),
            "care_terms": count_hits(&lower, &["wound", "hurt", "heal", "child", "mother", "home"])
        },
        "review_questions": [
            "What does this POV notice first that another character would not?",
            "What do they refuse to notice?",
            "What metaphor domain belongs naturally to their life?",
            "What judgement leaks through the narration?",
            "Where does the voice become generic explanation instead of character-filtered perception?"
        ]
    })
}

fn open_loops_report(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    json!({
        "method": "Heuristic open-loop prompts. Track promises manually for serious long-form projects.",
        "question_mark_count": text.matches('?').count(),
        "likely_introduced_signals": count_hits(&lower, &["missing", "secret", "why", "who", "map", "key", "curse", "oath", "prophecy", "clue", "door"]),
        "likely_payoff_signals": count_hits(&lower, &["because", "revealed", "answered", "finally", "truth", "found", "opened", "paid"]),
        "unresolved_loop_questions": [
            "What question did this chapter open?",
            "What question did it answer?",
            "Which open question now needs a payoff window?",
            "Is the reader curious, or merely confused?"
        ],
        "needs_agent_or_human_judgement": true
    })
}

fn progression_and_power_report(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    json!({
        "method": "Power/progression checks are about earned change, limits, costs, trade-offs, and consequences.",
        "progression_signals": count_hits(&lower, &["level", "rank", "class", "skill", "system", "power", "training", "upgrade", "cultivation", "domain"]),
        "cost_signals": count_hits(&lower, &["cost", "cooldown", "limit", "debt", "pain", "hunger", "memory", "trust", "exposure", "risk"]),
        "review_questions": [
            "What did the protagonist gain?",
            "Why was the gain earned rather than handed over?",
            "What problem did the gain solve?",
            "What harder problem did it create?",
            "What social, moral, bodily, or strategic consequence follows?"
        ]
    })
}

fn relationship_and_dialogue_report(text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    let metrics = metrics(text);
    json!({
        "method": "Dialogue is compressed conflict. Relationship scenes still need want, resistance, power movement, and change.",
        "dialogue_line_count": metrics.dialogue_line_count,
        "relationship_signals": count_hits(&lower, &["trust", "betray", "owed", "friend", "enemy", "ally", "love", "hate", "mother", "father", "sister", "brother"]),
        "subtext_prompts": [
            "What is the surface topic?",
            "What does each speaker actually want?",
            "Who has leverage at the start and who has it at the end?",
            "What is dodged, withheld, tested, misunderstood, or weaponised?",
            "What relationship state changes by the end?"
        ]
    })
}

fn craft_signal(text: &str, terms: &[&str], check: &str) -> serde_json::Value {
    let hits = count_hits(text, terms);
    json!({
        "status": if hits.is_empty() { "needs_agent_review" } else { "likely_signal" },
        "hits": hits,
        "check": check
    })
}

fn exit_hook_signal(text: &str) -> serde_json::Value {
    let tail = text
        .unicode_words()
        .rev()
        .take(80)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    craft_signal(
        &tail,
        &["?", "but", "until", "then", "door", "arrived", "revealed", "decided", "opened", "blood", "debt", "tomorrow"],
        "Check whether the ending creates forward pull through question, threat, decision, reveal, consequence, relationship crack, or new objective.",
    )
}

fn story_report(
    path: &Path,
    text: &str,
    genre: &str,
    profile: &str,
    must_include: &[String],
    must_avoid: &[String],
    document_kind: &str,
) -> serde_json::Value {
    let issues = lint_text(text);
    let constraints = constraint_adherence(text, must_include, must_avoid);
    let opening = opening_promise_report(text);
    let trope = trope_saturation(text, genre);
    let priorities = revision_priorities(&issues, &constraints, &opening, &trope);
    let mode = if document_kind == "chapter" {
        "post_write_chapter_review"
    } else {
        "post_write_story_review"
    };
    json!({
        "path": path,
        "mode": mode,
        "document_kind": document_kind,
        "not_a_gate": true,
        "guidance_policy": [
            "Treat findings as review leads, not automatic rewrite orders.",
            "Examples are indicators and helpers, not limits on what the story may do.",
            "Decide whether a pattern is functional for the intended reader effect before revising.",
            "Prioritise structure, prose clarity, causality, scene movement, reader grip, and revision questions over mechanical compliance."
        ],
        "genre": genre,
        "profile": profile,
        "metrics": metrics(text),
        "lint": lint_summary(&issues),
        "issues": issues,
        "reader_profile": reader_check(text, profile),
        "constraint_adherence": constraints,
        "opening_promise": opening,
        "revision_priorities": priorities,
        "chapter_spine": chapter_spine_report(text),
        "scene_change": scene_change_report(text),
        "reader_retention": reader_retention_report(text),
        "prose_review": prose_review(text),
        "voice_review": voice_review(text),
        "open_loops": open_loops_report(text),
        "progression_and_power": progression_and_power_report(text),
        "relationship_and_dialogue": relationship_and_dialogue_report(text),
        "novelty": novelty_analysis(text, false),
        "trope_saturation": trope,
        "dimensions": dimensions_json(),
        "review_questions": [
            "What is the smallest scene-level draw the opening makes?",
            "Does each major scene contain goal, conflict, turn, consequence, and next pressure?",
            "Where does prose become explanation instead of action, dialogue, discovery, cost, or consequence?",
            "Which warnings are useful signals, and which are intentional craft choices?",
            "What single revision pass would most improve reader grip without flattening voice?"
        ]
    })
}

fn compare_json(
    a: &Path,
    b: &Path,
    a_text: &str,
    b_text: &str,
    profile: &str,
    must_include: &[String],
    must_avoid: &[String],
) -> serde_json::Value {
    let a_gate = gate_report(a, a_text, profile, must_include, must_avoid);
    let b_gate = gate_report(b, b_text, profile, must_include, must_avoid);
    json!({
        "a": a_gate,
        "b": b_gate,
        "dimensions": dimensions_json(),
        "winner": null,
        "winner_policy": "No automatic winner. Use the evidence and record the chosen version with eval reward-export.",
        "comparison_questions": [
            "Which draft better preserves the required facts?",
            "Which draft creates the stronger reader effect for the target profile?",
            "Which draft has more specific, non-swappable details?",
            "Which issues are intentional craft choices rather than mistakes?",
            "Would reversing A/B order change the judgement?"
        ]
    })
}

fn normalize_key(value: &str) -> String {
    value.to_ascii_lowercase().replace(['_', ' '], "-")
}

fn trope_axes_for(genre: &str) -> (&'static str, TropeAxes) {
    match normalize_key(genre).as_str() {
        "breakout-serial" | "serial-breakout" => ("breakout-serial", BREAKOUT_SERIAL_AXES),
        "nightmare-survival" | "shadow-survival" | "dark-progression" => {
            ("nightmare-survival", NIGHTMARE_SURVIVAL_AXES)
        }
        "rational-magus" | "magus-progression" | "western-magus" => {
            ("rational-magus", RATIONAL_MAGUS_AXES)
        }
        "beast-bond-progression" | "beast-master" | "beast-taming" => {
            ("beast-bond-progression", BEAST_BOND_PROGRESSION_AXES)
        }
        "vr-cultivation" | "cultivation-online" | "cultivation-vrmmorpg" => {
            ("vr-cultivation", VR_CULTIVATION_AXES)
        }
        "monster-evolution" | "slime-evolution" | "nonhuman-evolution" => {
            ("monster-evolution", MONSTER_EVOLUTION_AXES)
        }
        "high-drama-romance" | "rejected-bond-romance" | "secret-baby-romance" => {
            ("high-drama-romance", HIGH_DRAMA_ROMANCE_AXES)
        }
        "tech-fantasy-celebration" | "tech-fantasy" | "launch-story" => {
            ("tech-fantasy-celebration", TECH_FANTASY_CELEBRATION_AXES)
        }
        "general-fiction" | "fiction" => ("general-fiction", GENERAL_FICTION_AXES),
        "general-writing" | "writing" | "nonfiction" => ("general-writing", GENERAL_WRITING_AXES),
        _ => ("system-isekai", SYSTEM_ISEKAI_TROPE_AXES),
    }
}

fn trope_axis_map(axes: TropeAxes) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    for (axis, values) in axes {
        map.insert((*axis).to_string(), json!(values));
    }
    map
}

fn lexical_signal_count(novelty: &serde_json::Value, key: &str) -> usize {
    novelty["lexical_novelty_signals"]["signal_counts"][key]
        .as_u64()
        .unwrap_or(0) as usize
}

fn trope_saturation(text: &str, genre: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    let mut hits = BTreeMap::new();
    let (resolved_genre, axes) = trope_axes_for(genre);
    for (axis, values) in axes {
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
    let saturation_risk = if familiar_count >= 5 {
        "high"
    } else if familiar_count >= 3 {
        "medium"
    } else {
        "low"
    };
    json!({
        "genre": resolved_genre,
        "axis_hits": hits,
        "familiar_trope_hit_count": familiar_count,
        "saturation_risk": saturation_risk,
        "interpretation": {
            "healthy_genre_signal": "Familiar genre signals are useful when they set reader expectations and then interact with cost, choice, consequence, or character pressure.",
            "overloaded_trope_stack": "Risk rises when many labels appear before the scene proves why they belong together.",
            "generic_execution_risk": "The problem is not using common tropes; the problem is using them without a specific causal relationship, fresh cost, or character-level pressure."
        },
        "review_questions": [
            "Which trope is intentional reader appeal?",
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
        "breakout-serial": {
            "description": "Long-form serial fiction aiming for strong reader grip, theme fit, and clear adaptation value.",
            "grade_max": 9.0,
            "avg_sentence_max": 18.5,
            "review_focus": ["hard hook in chapter one", "costly progression", "memorable cast roles", "world depth beyond fights", "chapter-end continuation reason"]
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
        "breakout-serial" => 18.5,
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

fn agent_plan_json(args: &AgentPlanArgs) -> serde_json::Value {
    let chapter_count = args.chapters.max(1);
    let (resolved_genre, axes) = trope_axes_for(&args.genre);
    json!({
        "mode": "agent_chapter_plan",
        "task_facts": {
            "seed_idea": &args.idea,
            "genre": resolved_genre,
            "profile": &args.profile,
            "chapters_requested": chapter_count,
            "avoid": &args.avoid,
            "must_include": &args.must_include,
            "must_avoid": &args.must_avoid
        },
        "missing_story_questions": missing_story_questions(&args.idea, chapter_count),
        "reader_effect_guidance": [
            "Treat this as reader-effect guidance, not literal oath magic: what question, desire, pressure, or pleasure makes the next page feel necessary.",
            "Usually dramatise the big premise through one small present-tense choice, cost, danger, hunger, shame, wonder, injustice, or relationship pressure before explaining the full system.",
            "Do not choose literal oath, vow, or contract-magic systems unless the user asks for them or they beat other options on scene pressure.",
            "Examples are indicators and helpers, not hard limits. Keep them only when they improve the scene."
        ],
        "contender_generation_rules": [
            "Generate 8-12 contenders before drafting unless the user already supplied a locked direction.",
            "Each contender must name the familiar genre appeal, freshness twist, opening wound, first hard choice, cost of advantage, scene image, wider story engine, and page-turn reason.",
            "Reject or rework contenders that only explain the premise, give power without cost, miss required facts, use forbidden claims, or lack a scene-level turn.",
            "Use the genre axes and atlas as ingredient maps; fold ingredients together causally instead of stacking labels."
        ],
        "comparison_protocol": [
            "Compare contenders pairwise for hook, desire, pressure, cost, change, genre fit, voice opportunity, world pressure, and chapter-end momentum.",
            "Choose the best direction with evidence. Do not choose because it has more trope labels or lexical novelty signals.",
            "If two directions tie, prefer the one with clearer scene action and a sharper cost."
        ],
        "chapter_cards": agent_chapter_cards(chapter_count),
        "drafting_instructions": [
            "Write finished prose in Markdown after planning; Novel Craft does not write it for you.",
            "For every chapter, make the POV character want something visible, meet resistance, make or face a turn, pay a cost, and leave a fair reason to continue.",
            "Keep explanations subordinate to action, dialogue, discovery, cost, or consequence.",
            "For a first chapter, make the reader trust the story before asking them to care about the whole roadmap."
        ],
        "revision_loop": [
            "Save the draft as a Markdown file.",
            "Run `novel-craft eval chapter <draft.md> --genre <genre-or-profile> --profile <profile> --json`.",
            "Revise for the most important structure/prose/retention leads; ignore warnings that are clearly intentional and effective.",
            "If there are hard facts, run `novel-craft eval gate <draft.md> --must-include <fact> --must-avoid <bad claim> --json`.",
            "Compare revisions with `novel-craft eval compare old.md new.md --json`; final judgement remains with the agent/human."
        ],
        "post_write_commands": [
            format!("novel-craft eval chapter <draft.md> --genre {resolved_genre} --profile {} --json", args.profile),
            format!("novel-craft eval story <draft.md> --genre {resolved_genre} --profile {} --json", args.profile),
            "novel-craft eval compare old.md new.md --json".to_string(),
            "novel-craft creative novelty <draft.md> --json".to_string()
        ],
        "profile_axes": trope_axis_map(axes),
        "craft_only_scope": [
            "This public packet covers story craft and chapter review.",
            "Platform rules, award eligibility, contracts, rights, monetisation, and AI disclosure requirements are out of scope for bundled defaults and should be checked live when needed."
        ]
    })
}

fn agent_plan_packet(args: &AgentPlanArgs) -> String {
    let data = agent_plan_json(args);
    let facts = &data["task_facts"];
    let chapters = data["chapter_cards"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let chapter_text = chapters
        .iter()
        .map(|chapter| {
            format!(
                "### Chapter {}\n- Job: {}\n- Goal: {}\n- Conflict: {}\n- Turn: {}\n- Cost: {}\n- Exit hook: {}\n- Open loop: {}\n",
                chapter["chapter"].as_u64().unwrap_or(1),
                chapter["job"].as_str().unwrap_or("draft a focused chapter"),
                chapter["goal"].as_str().unwrap_or("give the POV a visible want"),
                chapter["conflict"].as_str().unwrap_or("make resistance specific"),
                chapter["turn"].as_str().unwrap_or("change the situation"),
                chapter["cost"].as_str().unwrap_or("make the outcome cost something"),
                chapter["exit_hook"].as_str().unwrap_or("leave a fair continuation reason"),
                chapter["open_loop_guidance"].as_str().unwrap_or("track one question"),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# Agent Chapter Plan\n\n## Task Facts\n- Seed idea: {}\n- Genre/profile: {}\n- Reader profile: {}\n- Chapters: {}\n- Avoid: {}\n- Must include: {}\n- Must avoid: {}\n\n## Missing Story Questions\n{}\n\n## Reader Effect Guidance\n{}\n\n## Contender Rules\n{}\n\n## Comparison Protocol\n{}\n\n## Chapter Cards\n{}\n## Drafting Instructions\n{}\n\n## Revision Loop\n{}\n\n## Post-Write Commands\n{}\n\n## Scope\nThis packet is craft-only. Check platform, award, publishing, rights, monetisation, and AI-disclosure requirements live if the user asks for those decisions.\n",
        facts["seed_idea"].as_str().unwrap_or(""),
        facts["genre"].as_str().unwrap_or("general-fiction"),
        facts["profile"].as_str().unwrap_or("fast-webnovel"),
        facts["chapters_requested"].as_u64().unwrap_or(1),
        join_json_strings(&facts["avoid"]),
        join_json_strings(&facts["must_include"]),
        join_json_strings(&facts["must_avoid"]),
        bullet_json_strings(&data["missing_story_questions"]),
        bullet_json_strings(&data["reader_effect_guidance"]),
        bullet_json_strings(&data["contender_generation_rules"]),
        bullet_json_strings(&data["comparison_protocol"]),
        chapter_text,
        bullet_json_strings(&data["drafting_instructions"]),
        bullet_json_strings(&data["revision_loop"]),
        bullet_json_strings(&data["post_write_commands"]),
    )
}

fn missing_story_questions(idea: &str, chapter_count: usize) -> Vec<String> {
    let lower = idea.to_lowercase();
    let mut questions = Vec::new();
    if !contains_any(
        &lower,
        &["want", "must", "needs", "trying", "goal", "save", "escape"],
    ) {
        questions.push("What does the protagonist visibly want in the first chapter?".to_string());
    }
    if !contains_any(
        &lower,
        &[
            "but", "enemy", "rival", "danger", "debt", "curse", "war", "blocked",
        ],
    ) {
        questions.push("What resists them immediately?".to_string());
    }
    if !contains_any(
        &lower,
        &["cost", "price", "debt", "sacrifice", "wound", "risk"],
    ) {
        questions.push("What does the first useful gain cost?".to_string());
    }
    if !contains_any(
        &lower,
        &[
            "world", "kingdom", "city", "village", "academy", "ship", "frontier",
        ],
    ) {
        questions.push(
            "Where does the opening scene happen, and what local pressure does that place create?"
                .to_string(),
        );
    }
    if chapter_count > 1 && !contains_any(&lower, &["arc", "volume", "series", "mystery", "long"]) {
        questions.push("What larger open question should connect these chapters?".to_string());
    }
    if questions.is_empty() {
        questions.push("What single reader effect matters most: tension, wonder, dread, romance, humour, mystery, or catharsis?".to_string());
    }
    questions
}

fn agent_chapter_cards(chapter_count: usize) -> Vec<serde_json::Value> {
    (1..=chapter_count)
        .map(|index| {
            let job = if index == 1 {
                "Prove the core appeal through the smallest dramatic scene before explaining the full roadmap."
            } else {
                "Pay off one prior pressure, escalate another, and make the wider arc feel more necessary."
            };
            json!({
                "chapter": index,
                "job": job,
                "goal": if index == 1 { "Give the POV character one visible, urgent want." } else { "Give the POV a next-step objective caused by the previous chapter." },
                "conflict": "Make resistance specific: person, rule, scarcity, danger, wound, lie, social cost, or missing information.",
                "turn": "Something must change by the end: reveal, choice, betrayal, loss, gain, relationship shift, or new danger.",
                "cost": "Victory should cost something: debt, injury, exposure, guilt, obligation, time pressure, or a harder future choice.",
                "exit_hook": if index == chapter_count { "End with a fair continuation reason, not a fake cliffhanger." } else { "End by answering one question and opening the next chapter's continuation pressure." },
                "open_loop_guidance": "Track what question is opened, escalated, paid off, or transformed."
            })
        })
        .collect()
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn join_json_strings(value: &serde_json::Value) -> String {
    value
        .as_array()
        .map(|items| {
            let strings: Vec<_> = items.iter().filter_map(|item| item.as_str()).collect();
            if strings.is_empty() {
                "none".to_string()
            } else {
                strings.join(", ")
            }
        })
        .unwrap_or_else(|| "none".to_string())
}

fn join_strings(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

fn bullet_json_strings(value: &serde_json::Value) -> String {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str())
                .map(|item| format!("- {item}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .filter(|text| !text.is_empty())
        .unwrap_or_else(|| "- none".to_string())
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
        "method": "Lexical dialogue fingerprint; final voice judgement belongs to the reviewer."
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

fn story_atlas_json() -> serde_json::Value {
    json!({
        "purpose": "Broad mix-and-match story ingredients for agents. Use this before drafting when the user prompt is loose, narrow, or too familiar.",
        "counts": {
            "genres": STORY_ATLAS_GENRES.len(),
            "subgenres": STORY_ATLAS_SUBGENRES.len(),
            "tropes": STORY_ATLAS_TROPES.len(),
            "subtropes": STORY_ATLAS_SUBTROPES.len()
        },
        "genres": STORY_ATLAS_GENRES,
        "subgenres": STORY_ATLAS_SUBGENRES,
        "tropes": STORY_ATLAS_TROPES,
        "subtropes": STORY_ATLAS_SUBTROPES,
        "mixing_protocol": STORY_MIXING_PROTOCOL,
        "quality_standard": NOVEL_EXCELLENCE_STANDARD
    })
}

fn story_atlas_text() -> String {
    format!(
        "# Story Atlas\n\nUse this as a broad ingredient library before drafting. Pick from different sections, then fold the ingredients together causally instead of stacking labels.\n\n## Genres (50)\n{}\n\n## Subgenres (50)\n{}\n\n## Tropes (50)\n{}\n\n## Sub-Tropes (50)\n{}\n\n## Mixing Protocol\n{}\n\n## Always-On Novel Excellence Standard\n{}\n",
        numbered_lines(STORY_ATLAS_GENRES),
        numbered_lines(STORY_ATLAS_SUBGENRES),
        numbered_lines(STORY_ATLAS_TROPES),
        numbered_lines(STORY_ATLAS_SUBTROPES),
        bullet_lines(STORY_MIXING_PROTOCOL),
        bullet_lines(NOVEL_EXCELLENCE_STANDARD),
    )
}

fn numbered_lines(items: &[&str]) -> String {
    items
        .iter()
        .enumerate()
        .map(|(index, item)| format!("{}. {item}", index + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

fn creative_brief(args: &CreativeBriefArgs) -> String {
    let (resolved_genre, axes) = trope_axes_for(&args.genre);
    format!(
        "# Creative Brief\n\n## Task Facts\n- Seed idea: {idea}\n- Genre/profile: {resolved_genre}\n- Audience: {audience}\n- Reading level target: grade {grade}\n- Required tropes or ingredients: {required}\n- Avoid: {avoid}\n- Must include: {must_include}\n- Must avoid: {must_avoid}\n\n## Always-On Novel Excellence Standard\n{novel_standard}\n\n## Opening Guidance\nShow the macro experience through micro-action when it helps the reader. For chapter one, prefer the smallest working unit of the premise before the whole roadmap. A kingdom-building story might begin with one room, one meal, one door, one ledger, one dispute, one protected child, or one boundary. Treat these as indicators, not hard limits.\n\n## Literal Oath/Vow Guardrail\nWhen this packet says reader draw or story appeal, do not turn that into literal oath, vow, or contract magic unless the user asks for it or it wins on scene pressure after comparison.\n\n## Required Flow\n1. Restate the task facts in plain words.\n2. Use `novel-craft creative atlas --json` if the premise needs more breadth, then generate 8-12 distinct contenders before drafting.\n3. For each contender, name the familiar genre appeal, opening wound, freshness twist, first hard choice, power cost, first-chapter image, micro-scene, wider story engine, and why a reader turns the page.\n4. Flag and rework contenders that miss required facts, use forbidden claims, rely only on decorative objects, over-explain the macro premise before micro-action, default to literal oath/vow systems without user request, or give the hero power without consequence.\n5. Choose one contender with evidence across hook, theme fit, reader grip, world depth, character pressure, serial retention, and chapter structure.\n6. Draft the piece.\n7. Self-critique for constraint adherence, reader effect, costly power, specificity, voice, factual safety, opening micro-scene, chapter-end continuation reason, and long-arc engine.\n8. Revise once.\n9. Run `novel-craft eval story <draft.md> --genre {resolved_genre} --json` for post-writing guidance, then use `eval gate` only when hard facts or forbidden claims need a pass/warn/fail check.\n\n## Profile Axes\n{profile_axes}\n\n## Methods\n{methods}\n",
        idea = args.idea,
        audience = args.audience,
        grade = args.reading_grade,
        required = join_or_none(&args.trope),
        avoid = join_or_none(&args.avoid),
        must_include = join_or_none(&args.must_include),
        must_avoid = join_or_none(&args.must_avoid),
        novel_standard = bullet_lines(NOVEL_EXCELLENCE_STANDARD),
        profile_axes = axes.iter()
            .map(|(axis, values)| format!("- `{axis}`: {}", values.join("; ")))
            .collect::<Vec<_>>()
            .join("\n"),
        methods = CREATIVE_METHODS.iter().map(|(id, _, instruction)| format!("- `{id}`: {instruction}")).collect::<Vec<_>>().join("\n")
    )
}

fn tournament_text(
    idea: &str,
    genre: &str,
    count: usize,
    avoid: &[String],
    must_include: &[String],
    must_avoid: &[String],
) -> String {
    let (resolved_genre, axes) = trope_axes_for(genre);
    let mut lines = vec![
        "# Prompt Tournament Pack".to_string(),
        String::new(),
        format!("- Seed idea: {idea}"),
        format!("- Genre/profile: {resolved_genre}"),
        format!("- Avoid: {}", join_or_none(avoid)),
        format!("- Must include: {}", join_or_none(must_include)),
        format!("- Must avoid: {}", join_or_none(must_avoid)),
        String::new(),
        "## Always-On Novel Excellence Standard".to_string(),
        bullet_lines(NOVEL_EXCELLENCE_STANDARD),
        String::new(),
        "Opening guidance: show the macro experience through micro-action when it helps. Avoid leaning on the whole roadmap before the reader sees one concrete need, choice, cost, or consequence.".to_string(),
        "Literal oath/vow guardrail: do not default to oath, vow, or contract-magic systems unless the user asks for them or they clearly beat other options on scene pressure.".to_string(),
        "Required flow: restate facts, use `novel-craft creative atlas --json` if breadth is thin, draft each contender as a 250-400 word opening concept, flag unsafe or consequence-free contenders, compare pairwise across hook, micro-scene, theme fit, costly power, world depth, serial retention, chapter structure, and wider story engine, choose with evidence, then run `novel-craft eval story` on the final draft.".to_string(),
        String::new(),
    ];
    for index in 0..count.max(1) {
        lines.push(format!("## Contender {}", index + 1));
        for (axis_index, (axis, values)) in axes.iter().enumerate() {
            lines.push(format!(
                "- {axis}: {}",
                values[(index + axis_index) % values.len()]
            ));
        }
        lines.push("- Name familiar genre appeal, opening wound, freshness twist, hard choice, power cost, world-depth signal, wider story engine, first-chapter image, micro-scene, and page-turn reason.".to_string());
        lines.push(String::new());
    }
    lines.join("\n")
}

fn start_packet(args: &StartArgs) -> String {
    format!(
        "# Novel Craft Start Packet\n\n## Project\n- Title: {}\n- Writer level: {}\n- Audience: {}\n- Genre: {}\n- Reading level: {}\n- Tone: {}\n- Desired output: {}\n- Autonomy: {}\n\n## Story Seed\n- Idea: {}\n- Include tropes: {}\n- Avoid: {}\n- Protagonist want: {}\n- Protagonist wound: {}\n- World: {}\n- Power system: {}\n\n## Always-On Novel Excellence Standard\n{}\n\n## Opening Guidance\nShow the macro experience through micro-action when it helps. Examples are indicators, not hard limits.\n\n## Literal Oath/Vow Guardrail\nDo not default to oath, vow, or contract-magic systems unless the user explicitly asks for them or they win after comparison.\n\n## First Workflow\n1. Run `novel-craft creative atlas --json` when the idea needs more breadth.\n2. Generate 8-12 premise contenders before drafting.\n3. Flag and rework contenders that miss required facts, ignore avoid-list items, lack a banger first chapter, over-explain the macro premise before micro-action, or rely on decorative novelty.\n4. Choose one contender with evidence across hook, micro-scene, constraint adherence, reader effect, specificity, chapter structure, and wider story engine.\n5. Create a scene card with goal, conflict, turn, stakes, open questions, and do-not-repeat notes.\n6. Build the context packet.\n7. Draft with the context packet.\n8. Run `novel-craft eval story <draft.md> --json` as the normal post-writing review.\n9. Use `novel-craft eval gate <draft.md> --json` when hard constraints need pass/warn/fail checking.\n10. Compare revisions with `novel-craft eval compare old.md new.md --json`; do not use lexical novelty as a winner.\n11. Extract a memory diff only after the canon change is approved.\n",
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
        bullet_lines(NOVEL_EXCELLENCE_STANDARD),
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

fn compare_text(
    a: &Path,
    b: &Path,
    a_text: &str,
    b_text: &str,
    profile: &str,
    must_include: &[String],
    must_avoid: &[String],
) -> String {
    let a_gate = gate_report(a, a_text, profile, must_include, must_avoid);
    let b_gate = gate_report(b, b_text, profile, must_include, must_avoid);
    let a_novelty = &a_gate["novelty"];
    let b_novelty = &b_gate["novelty"];
    format!(
        "# Pairwise Creative Writing Comparison\n\n- Version A: {}\n- Version B: {}\n- Profile: {}\n- A status: {}\n- B status: {}\n- A issues: {}\n- B issues: {}\n- A lexical concrete categories: {}\n- B lexical concrete categories: {}\n\nNo automatic winner is assigned. Lexical novelty signals route attention; they do not decide quality.\n\n{}\n",
        a.display(),
        b.display(),
        profile,
        a_gate["status"].as_str().unwrap_or("unknown"),
        b_gate["status"].as_str().unwrap_or("unknown"),
        a_gate["lint"]["issue_count"].as_u64().unwrap_or(0),
        b_gate["lint"]["issue_count"].as_u64().unwrap_or(0),
        lexical_signal_count(a_novelty, "concrete_object_categories"),
        lexical_signal_count(b_novelty, "concrete_object_categories"),
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

fn tropes_json(genre: &str) -> serde_json::Value {
    let (resolved_genre, axes) = trope_axes_for(genre);
    json!({"genre": resolved_genre, "tropes": trope_axis_map(axes)})
}

fn rule_guide(json_output: bool) -> Result<String> {
    let rules: Value = serde_yaml::from_str(DEFAULT_RULES)?;
    if json_output {
        return Ok(serde_json::to_string_pretty(&rules_to_json(&rules))?);
    }
    let mut out = String::from("# Novel Craft Rule Guide\n\nRules are effects, not commandments. Deterministic checks are leads, not verdicts.\n\n");
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

fn upsert_yaml_string(value: &mut Value, key: &str, new_value: Option<&str>) -> Result<()> {
    let Some(new_value) = new_value.map(str::trim).filter(|item| !item.is_empty()) else {
        return Ok(());
    };
    let map = value
        .as_mapping_mut()
        .context("state file is not a mapping")?;
    map.insert(
        Value::String(key.to_string()),
        Value::String(new_value.to_string()),
    );
    Ok(())
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

fn read_yaml_records(root: &Path, folder: &str) -> Result<Vec<serde_json::Value>> {
    let dir = root.join(PROJECT_DIR).join(folder);
    let mut paths = Vec::new();
    if !dir.exists() {
        return Ok(Vec::new());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("yml" | "yaml")
        ) {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths
        .iter()
        .filter_map(|path| read_yaml_value(path).map(|value| yaml_to_json(&value)))
        .collect())
}

fn yaml_sequence_field(value: &Value, key: &str) -> Vec<serde_json::Value> {
    value
        .get(key)
        .and_then(Value::as_sequence)
        .map(|items| items.iter().map(yaml_to_json).collect())
        .unwrap_or_default()
}

fn merge_json_records(
    mut base: Vec<serde_json::Value>,
    extra: Vec<serde_json::Value>,
    key: &str,
) -> Vec<serde_json::Value> {
    let mut seen = BTreeMap::new();
    for (idx, item) in base.iter().enumerate() {
        if let Some(id) = item.get(key).and_then(|value| value.as_str()) {
            seen.insert(id.to_string(), idx);
        }
    }
    for item in extra {
        if let Some(id) = item.get(key).and_then(|value| value.as_str()) {
            if let Some(idx) = seen.get(id).copied() {
                base[idx] = item;
            } else {
                seen.insert(id.to_string(), base.len());
                base.push(item);
            }
        } else {
            base.push(item);
        }
    }
    base
}

fn find_json_record(
    value: &serde_json::Value,
    key: &str,
    wanted: &str,
) -> Option<serde_json::Value> {
    let wanted_slug = slug(wanted);
    value.as_array()?.iter().find_map(|item| {
        let id = item.get(key).and_then(|value| value.as_str())?;
        if id == wanted || slug(id) == wanted_slug {
            Some(item.clone())
        } else {
            None
        }
    })
}

fn build_matrix(root: &Path) -> Result<serde_json::Value> {
    let plot_matrix = read_yaml_value(&root.join(PROJECT_DIR).join("plot-matrix.yml"))
        .unwrap_or_else(|| Value::Mapping(Mapping::new()));
    let scenes = merge_json_records(
        yaml_sequence_field(&plot_matrix, "scenes"),
        read_yaml_records(root, "scene-cards")?,
        "id",
    );
    let plot_threads = merge_json_records(
        yaml_sequence_field(&plot_matrix, "plot_threads"),
        read_yaml_records(root, "plot-threads")?,
        "id",
    );
    let promises = yaml_sequence_field(&plot_matrix, "promises");
    let payoffs = yaml_sequence_field(&plot_matrix, "payoffs");
    let progression = yaml_sequence_field(&plot_matrix, "progression");
    let characters = read_yaml_records(root, "characters")?;
    let story_seed = read_yaml_value(&root.join(PROJECT_DIR).join("state").join("story-seed.yml"))
        .map(|value| yaml_to_json(&value))
        .unwrap_or(serde_json::Value::Null);
    Ok(json!({
        "story_seed": story_seed,
        "characters": characters,
        "plot_threads": plot_threads,
        "promises": promises,
        "payoffs": payoffs,
        "scenes": scenes,
        "progression": progression
    }))
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
        "doctrine": "Use this audit as revision evidence; do not treat it as final taste judgement."
    })
}

fn causality_report(path: &Path, text: &str) -> serde_json::Value {
    let lower = text.to_lowercase();
    let causal_hits = count_hits(
        &lower,
        &[
            "because",
            "therefore",
            "so",
            "after",
            "result",
            "forced",
            "cost",
            "consequence",
            "instead",
        ],
    );
    let weak_sequence_hits = count_hits(
        &lower,
        &["then", "suddenly", "somehow", "and then", "just as"],
    );
    let scene_change = scene_change_report(text);
    let status = if causal_hits.is_empty() && weak_sequence_hits.values().sum::<usize>() >= 2 {
        "warn"
    } else if causal_hits.is_empty() {
        "needs_review"
    } else {
        "likely_signal"
    };
    json!({
        "path": path,
        "status": status,
        "summary": {
            "next_best_action": if status == "warn" {
                "Replace coincidence or sequence-only movement with because/therefore pressure."
            } else {
                "Check whether each major beat causes the next beat rather than merely following it."
            },
            "risk_notes": [
                "Causality is partly semantic; treat this as a routing report.",
                "A chapter can be causal without using the exact word because."
            ]
        },
        "causal_connector_hits": causal_hits,
        "weak_sequence_hits": weak_sequence_hits,
        "scene_change": scene_change,
        "review_questions": [
            "Which choice causes the next obstacle?",
            "Which obstacle forces the next tactic?",
            "Which success creates a new cost?",
            "Where does the chapter say then when it should imply because?",
            "Which event could be cut without changing later events?"
        ]
    })
}

fn promise_heatmap(matrix: &serde_json::Value) -> serde_json::Value {
    let scenes = matrix["scenes"].as_array().cloned().unwrap_or_default();
    let mut promises = matrix["promises"].as_array().cloned().unwrap_or_default();
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
    let target_scene =
        find_json_record(&matrix["scenes"], "id", target).unwrap_or(serde_json::Value::Null);
    let story_seed = &matrix["story_seed"];
    let characters = &matrix["characters"];
    let plot_threads = &matrix["plot_threads"];
    let promises = &matrix["promises"];
    let progression = &matrix["progression"];
    Ok(format!(
        "# Context Packet: {target}\n\n## Project\n```yaml\n{project}\n```\n\n## Story Seed\n```yaml\n{}\n```\n\n## Target Scene Card\n```yaml\n{}\n```\n\n## Characters\n```yaml\n{}\n```\n\n## Open Plot Threads\n```yaml\n{}\n```\n\n## Open Promises / Open Loops\n```yaml\n{}\n```\n\n## Progression And Power Notes\n```yaml\n{}\n```\n\n## Story Matrix Summary\n```yaml\n{}\n```\n\n## Instructions\n- Preserve canon unless the scene card explicitly changes it.\n- Bring character motives, open threads, open loops, power-system costs, do-not-repeat notes, and previous scene state into the draft.\n- Run `novel-craft eval chapter <draft.md>`, then extract a reviewable memory diff after drafting.\n",
        serde_yaml::to_string(story_seed)?,
        serde_yaml::to_string(&target_scene)?,
        serde_yaml::to_string(characters)?,
        serde_yaml::to_string(plot_threads)?,
        serde_yaml::to_string(promises)?,
        serde_yaml::to_string(progression)?,
        serde_yaml::to_string(&matrix)?
    ))
}

fn extract_fact_candidates(text: &str) -> Vec<serde_json::Value> {
    let mut facts = Vec::new();
    let categories: &[(&str, &[&str])] = &[
        (
            "power_or_skill",
            &[
                "unlocked",
                "gained",
                "learned",
                "skill",
                "rank",
                "class",
                "interpose",
            ],
        ),
        (
            "status_or_mark",
            &[
                "became",
                "marked",
                "brand",
                "claim",
                "claimable",
                "debt-bearing",
                "first claim",
            ],
        ),
        (
            "character_role",
            &[
                "ward",
                "named",
                "confirmed",
                "bound",
                "protected",
                "guardian",
            ],
        ),
        (
            "world_rule",
            &[
                "recognises",
                "recognizes",
                "witnessed",
                "rule",
                "ledger",
                "authority",
                "toll",
            ],
        ),
        (
            "location_or_route",
            &[
                "exists",
                "service stair",
                "gate",
                "door",
                "floor one",
                "oathspire",
            ],
        ),
        (
            "open_loop_or_timer",
            &[
                "wake", "wakes", "bell", "deadline", "unpaid", "missing", "secret",
            ],
        ),
    ];

    for raw in text.split(['.', '!', '?', '\n']) {
        let sentence = raw.trim().trim_matches('"').trim();
        if sentence.len() < 8 {
            continue;
        }
        let lower = sentence.to_lowercase();
        for (category, needles) in categories {
            if contains_any(&lower, needles) {
                facts.push(json!({
                    "fact": sentence,
                    "category": category,
                    "confidence": "candidate",
                    "reason": "Matched a canon-change signal; review before committing."
                }));
                break;
            }
        }
        if facts.len() >= 20 {
            break;
        }
    }

    dedupe_json_facts(facts)
}

fn dedupe_json_facts(facts: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    let mut seen = BTreeMap::new();
    let mut out = Vec::new();
    for fact in facts {
        let key = fact["fact"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if seen.insert(key, true).is_none() {
            out.push(fact);
        }
    }
    out
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
