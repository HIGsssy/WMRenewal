use serde::{Deserialize, Serialize};

use crate::enums::{ActionType, JobType, Skill, Stat, Status};

/// Number of per-girl script flags (from C++ NUM_GIRLFLAGS).
pub const NUM_GIRL_FLAGS: usize = 30;

/// Maximum traits a girl can have.
pub const MAX_TRAITS: usize = 60;

/// Maximum inventory slots per girl.
pub const MAX_INVENTORY: usize = 40;

/// Complete girl data model, matching C++ `sGirl` / `Girl` struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Girl {
    pub name: String,
    pub realname: String,
    pub desc: String,

    // Core stats & skills
    pub stats: [i32; Stat::COUNT],
    pub stat_mods: [i32; Stat::COUNT],
    pub temp_stats: [i32; Stat::COUNT],
    pub skills: [i32; Skill::COUNT],
    pub skill_mods: [i32; Skill::COUNT],
    pub temp_skills: [i32; Skill::COUNT],

    // Traits
    pub traits: Vec<String>,
    pub remembered_traits: Vec<String>,

    // Status
    pub status: GirlStatus,
    pub virgin: bool,
    pub use_anti_preg: bool,
    pub withdrawals: u8,

    // Jobs
    pub job_day: Option<JobType>,
    pub job_night: Option<JobType>,
    pub prev_job_day: Option<JobType>,
    pub prev_job_night: Option<JobType>,

    // Enjoyment of action types
    pub enjoyment: [i32; ActionType::COUNT],

    // Inventory (item IDs into global item list)
    pub inventory: Vec<usize>,
    pub equipped: Vec<bool>,

    // Pregnancy / life
    pub weeks_pregnant: i32,
    pub preg_cooldown: i32,
    pub weeks_past: u32,
    pub birthday: u32,
    pub num_customers: u64,
    pub money: i32,
    pub accommodation_level: u8,
    pub pay: i32,

    // Run away / spotted
    pub run_away: u8,
    pub spotted: u8,
    pub tortured_today: bool,
    pub just_gave_birth: bool,

    // Per-girl script flags
    pub flags: [bool; NUM_GIRL_FLAGS],

    // Days unhappy counter
    pub days_unhappy: u8,

    // Fetish type bitmask
    pub fetish_types: u64,

    // Children
    pub children: Vec<Child>,
}

impl Default for Girl {
    fn default() -> Self {
        Self {
            name: String::new(),
            realname: String::new(),
            desc: String::new(),
            stats: [0; Stat::COUNT],
            stat_mods: [0; Stat::COUNT],
            temp_stats: [0; Stat::COUNT],
            skills: [0; Skill::COUNT],
            skill_mods: [0; Skill::COUNT],
            temp_skills: [0; Skill::COUNT],
            traits: Vec::new(),
            remembered_traits: Vec::new(),
            status: GirlStatus::default(),
            virgin: false,
            use_anti_preg: false,
            withdrawals: 0,
            job_day: None,
            job_night: None,
            prev_job_day: None,
            prev_job_night: None,
            enjoyment: [0; ActionType::COUNT],
            inventory: Vec::new(),
            equipped: Vec::new(),
            weeks_pregnant: 0,
            preg_cooldown: 0,
            weeks_past: 0,
            birthday: 0,
            num_customers: 0,
            money: 0,
            accommodation_level: 0,
            pay: 0,
            run_away: 0,
            spotted: 0,
            tortured_today: false,
            just_gave_birth: false,
            flags: [false; NUM_GIRL_FLAGS],
            days_unhappy: 0,
            fetish_types: 0,
            children: Vec::new(),
        }
    }
}

/// Status flags for a girl. Mirrors C++ status bitmask.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GirlStatus {
    pub statuses: Vec<Status>,
}

/// A child belonging to a girl.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Child {
    pub is_girl: bool,
    pub age_weeks: u32,
    pub grown: bool,
}

// -- XML deserialization structs for Girls.girlsx --

#[derive(Debug, Deserialize)]
pub struct GirlsXml {
    #[serde(rename = "Girl", default)]
    pub girls: Vec<GirlXml>,
}

#[derive(Debug, Deserialize)]
pub struct TraitXml {
    #[serde(rename = "@Name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GirlXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Desc", default)]
    pub desc: String,
    #[serde(rename = "@Gold", default)]
    pub gold: i32,
    // 22 Stats
    #[serde(rename = "@Charisma", default)]
    pub charisma: i32,
    #[serde(rename = "@Happiness", default)]
    pub happiness: i32,
    #[serde(rename = "@Libido", default)]
    pub libido: i32,
    #[serde(rename = "@Constitution", default)]
    pub constitution: i32,
    #[serde(rename = "@Intelligence", default)]
    pub intelligence: i32,
    #[serde(rename = "@Confidence", default)]
    pub confidence: i32,
    #[serde(rename = "@Mana", default)]
    pub mana: i32,
    #[serde(rename = "@Agility", default)]
    pub agility: i32,
    #[serde(rename = "@Fame", default)]
    pub fame: i32,
    #[serde(rename = "@Level", default)]
    pub level: i32,
    #[serde(rename = "@AskPrice", default)]
    pub ask_price: i32,
    #[serde(rename = "@House", default)]
    pub house: i32,
    #[serde(rename = "@Exp", default)]
    pub exp: i32,
    #[serde(rename = "@Age", default)]
    pub age: i32,
    #[serde(rename = "@Obedience", default)]
    pub obedience: i32,
    #[serde(rename = "@Spirit", default)]
    pub spirit: i32,
    #[serde(rename = "@Beauty", default)]
    pub beauty: i32,
    #[serde(rename = "@Tiredness", default)]
    pub tiredness: i32,
    #[serde(rename = "@Health", default)]
    pub health: i32,
    #[serde(rename = "@PCFear", default)]
    pub pc_fear: i32,
    #[serde(rename = "@PCLove", default)]
    pub pc_love: i32,
    #[serde(rename = "@PCHate", default)]
    pub pc_hate: i32,
    // 10 Skills
    #[serde(rename = "@Anal", default)]
    pub anal: i32,
    #[serde(rename = "@Magic", default)]
    pub magic: i32,
    #[serde(rename = "@BDSM", default)]
    pub bdsm: i32,
    #[serde(rename = "@NormalSex", default)]
    pub normal_sex: i32,
    #[serde(rename = "@Beastiality", default)]
    pub beastiality: i32,
    #[serde(rename = "@Group", default)]
    pub group: i32,
    #[serde(rename = "@Lesbian", default)]
    pub lesbian: i32,
    #[serde(rename = "@Service", default)]
    pub service: i32,
    #[serde(rename = "@Strip", default)]
    pub strip: i32,
    #[serde(rename = "@Combat", default)]
    pub combat: i32,
    // Status
    #[serde(rename = "@Status", default)]
    pub status: String,
    // Nested traits
    #[serde(rename = "Trait", default)]
    pub traits: Vec<TraitXml>,
}

impl GirlXml {
    /// Convert from XML representation to domain Girl.
    pub fn into_girl(self) -> Girl {
        let mut girl = Girl::default();
        girl.name = self.name;
        girl.realname = girl.name.clone();
        girl.desc = self.desc;
        girl.money = self.gold;

        // Map stats by enum index
        girl.stats[Stat::Charisma as usize] = self.charisma;
        girl.stats[Stat::Happiness as usize] = self.happiness;
        girl.stats[Stat::Libido as usize] = self.libido;
        girl.stats[Stat::Constitution as usize] = self.constitution;
        girl.stats[Stat::Intelligence as usize] = self.intelligence;
        girl.stats[Stat::Confidence as usize] = self.confidence;
        girl.stats[Stat::Mana as usize] = self.mana;
        girl.stats[Stat::Agility as usize] = self.agility;
        girl.stats[Stat::Fame as usize] = self.fame;
        girl.stats[Stat::Level as usize] = self.level;
        girl.stats[Stat::AskPrice as usize] = self.ask_price;
        girl.stats[Stat::HousePerc as usize] = self.house;
        girl.stats[Stat::Exp as usize] = self.exp;
        girl.stats[Stat::Age as usize] = self.age;
        girl.stats[Stat::Obedience as usize] = self.obedience;
        girl.stats[Stat::Spirit as usize] = self.spirit;
        girl.stats[Stat::Beauty as usize] = self.beauty;
        girl.stats[Stat::Tiredness as usize] = self.tiredness;
        girl.stats[Stat::Health as usize] = self.health;
        girl.stats[Stat::PCFear as usize] = self.pc_fear;
        girl.stats[Stat::PCLove as usize] = self.pc_love;
        girl.stats[Stat::PCHate as usize] = self.pc_hate;

        // Map skills by enum index
        girl.skills[Skill::Anal as usize] = self.anal;
        girl.skills[Skill::Magic as usize] = self.magic;
        girl.skills[Skill::BDSM as usize] = self.bdsm;
        girl.skills[Skill::NormalSex as usize] = self.normal_sex;
        girl.skills[Skill::Beastiality as usize] = self.beastiality;
        girl.skills[Skill::Group as usize] = self.group;
        girl.skills[Skill::Lesbian as usize] = self.lesbian;
        girl.skills[Skill::Service as usize] = self.service;
        girl.skills[Skill::Strip as usize] = self.strip;
        girl.skills[Skill::Combat as usize] = self.combat;

        // Traits
        girl.traits = self.traits.into_iter().map(|t| t.name).collect();

        // Status
        if self.status == "Normal" {
            // default empty status list
        }

        girl
    }
}
