use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Stats (22 total, matching C++ Constants.h)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stat {
    Charisma = 0,
    Happiness = 1,
    Libido = 2,
    Constitution = 3,
    Intelligence = 4,
    Confidence = 5,
    Mana = 6,
    Agility = 7,
    Fame = 8,
    Level = 9,
    AskPrice = 10,
    HousePerc = 11,
    Exp = 12,
    Age = 13,
    Obedience = 14,
    Spirit = 15,
    Beauty = 16,
    Tiredness = 17,
    Health = 18,
    PCFear = 19,
    PCLove = 20,
    PCHate = 21,
}

impl Stat {
    pub const COUNT: usize = 22;

    pub const ALL: [Stat; Self::COUNT] = [
        Stat::Charisma,
        Stat::Happiness,
        Stat::Libido,
        Stat::Constitution,
        Stat::Intelligence,
        Stat::Confidence,
        Stat::Mana,
        Stat::Agility,
        Stat::Fame,
        Stat::Level,
        Stat::AskPrice,
        Stat::HousePerc,
        Stat::Exp,
        Stat::Age,
        Stat::Obedience,
        Stat::Spirit,
        Stat::Beauty,
        Stat::Tiredness,
        Stat::Health,
        Stat::PCFear,
        Stat::PCLove,
        Stat::PCHate,
    ];
}

// ---------------------------------------------------------------------------
// Skills (10 total, matching C++ Constants.h)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Skill {
    Anal = 0,
    Magic = 1,
    BDSM = 2,
    NormalSex = 3,
    Beastiality = 4,
    Group = 5,
    Lesbian = 6,
    Service = 7,
    Strip = 8,
    Combat = 9,
}

impl Skill {
    pub const COUNT: usize = 10;

    pub const ALL: [Skill; Self::COUNT] = [
        Skill::Anal,
        Skill::Magic,
        Skill::BDSM,
        Skill::NormalSex,
        Skill::Beastiality,
        Skill::Group,
        Skill::Lesbian,
        Skill::Service,
        Skill::Strip,
        Skill::Combat,
    ];
}

// ---------------------------------------------------------------------------
// Status conditions (11 total)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Status {
    None = 0,
    Poisoned = 1,
    BadlyPoisoned = 2,
    Pregnant = 3,
    PregnantByPlayer = 4,
    Slave = 5,
    HasDaughter = 6,
    HasSon = 7,
    Inseminated = 8,
    Controlled = 9,
    Catacombs = 10,
}

impl Status {
    pub const COUNT: usize = 11;
}

// ---------------------------------------------------------------------------
// Jobs (67 total, matching C++ Constants.h JOB_*)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
pub enum JobType {
    // General (filter 0)
    Resting = 0,
    Training = 1,
    Cleaning = 2,
    Security = 3,
    Advertising = 4,
    Matron = 5,
    Torturer = 6,
    ExploreCatacombs = 7,
    BeastCapture = 8,
    BeastCarer = 9,
    // Brothel (filter 1)
    WhoreBrothel = 10,
    WhoreStreets = 11,
    BrothelStripper = 12,
    Masseuse = 13,
    // Gambling Hall (filter 2)
    CustomerService = 14,
    WhoreGambHall = 15,
    Dealer = 16,
    Entertainment = 17,
    XXXEntertainment = 18,
    // Bar (filter 3)
    Barmaid = 19,
    Waitress = 20,
    Stripper = 21,
    WhoreBar = 22,
    Singer = 23,
    // Movie Studio (filter 4)
    FilmBeast = 24,
    FilmSex = 25,
    FilmAnal = 26,
    FilmLesbian = 27,
    FilmBondage = 28,
    Fluffer = 29,
    CameraMage = 30,
    // 31 is unused in original
    CrystalPurifier = 32,
    // Community Centre (filter 5)
    CollectDonations = 33,
    FeedPoor = 34,
    MakeItems = 35,
    SellItems = 36,
    CommunityService = 37,
    // Drug Lab (filter 6)
    VirasPlantFucker = 38,
    ShroudGrower = 39,
    FairyDuster = 40,
    DrugDealer = 41,
    // Alchemist Lab (filter 7)
    FindRegents = 42,
    BrewPotions = 43,
    PotionTester = 44,
    // Arena (filter 8)
    FightBeasts = 45,
    Wrestle = 46,
    FightToDeath = 47,
    FightVolunteers = 48,
    CollectBets = 49,
    // Training Centre (filter 9)
    TeachBDSM = 50,
    TeachSex = 51,
    TeachBeast = 52,
    TeachMagic = 53,
    TeachCombat = 54,
    Daycare = 55,
    Schooling = 56,
    TeachDancing = 57,
    TeachService = 58,
    Train = 59,
    // Clinic (filter 10)
    Doctor = 60,
    GetAbort = 61,
    PhysicalSurgery = 62,
    Healing = 63,
    RepairShop = 64,
    // Unassignable
    InDungeon = 65,
    Runaway = 66,
}

impl JobType {
    pub const NUM_JOBS: usize = 67;
}

// ---------------------------------------------------------------------------
// Job Filters
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JobFilter {
    General = 0,
    Brothel = 1,
    GamblingHall = 2,
    Bar = 3,
    MovieStudio = 4,
    CommunityCentre = 5,
    DrugLab = 6,
    AlchemistLab = 7,
    Arena = 8,
    TrainingCentre = 9,
    Clinic = 10,
}

impl JobFilter {
    pub const COUNT: usize = 11;
}

// ---------------------------------------------------------------------------
// Item Types
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Food,
    Ring,
    Necklace,
    Dress,
    Underwear,
    Shoes,
    Hat,
    Helmet,
    SmallWeapon,
    LargeWeapon,
    Armor,
    Shield,
    Consumable,
    Makeup,
    Misc,
}

// ---------------------------------------------------------------------------
// Effect Target
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectTarget {
    Stat,
    Skill,
    Trait,
}

// ---------------------------------------------------------------------------
// Rarity
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    /// Shop always
    Common,
    /// Shop 50% chance
    Shop50,
    /// Shop 25%
    Shop25,
    /// Shop 5%
    Shop05,
    /// Catacombs 15%
    Catacomb15,
    /// Only via scripts
    ScriptOnly,
    /// Objectives/scripts
    Reward,
}

// ---------------------------------------------------------------------------
// Trigger Types
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TriggerType {
    Random = 0,
    Shopping = 1,
    Skill = 2,
    Stat = 3,
    Status = 4,
    Money = 5,
    Meet = 6,
    Talk = 7,
    WeeksPast = 8,
    GlobalFlag = 9,
    ScriptRun = 10,
    Kidnapped = 11,
    PlayerMoney = 12,
}

impl TriggerType {
    pub const COUNT: usize = 13;
}

// ---------------------------------------------------------------------------
// Shift
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Shift {
    Day,
    Night,
}

// ---------------------------------------------------------------------------
// Gang Missions
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GangMission {
    Guarding = 0,
    Sabotage = 1,
    SpyGirls = 2,
    CaptureGirl = 3,
    Extortion = 4,
    PettyTheft = 5,
    GrandTheft = 6,
    Kidnap = 7,
    Catacombs = 8,
    Training = 9,
    Recruit = 10,
}

impl GangMission {
    pub const COUNT: usize = 11;
}

// ---------------------------------------------------------------------------
// Dungeon Reasons
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DungeonReason {
    Release = 0,
    CustomerNoPay = 1,
    GirlCaptured = 2,
    GirlKidnapped = 3,
    CustomerBeatGirl = 4,
    CustomerSpy = 5,
    Rival = 6,
    GirlWhim = 7,
    GirlSteal = 8,
    Dead = 9,
    GirlRunaway = 10,
    NewSlave = 11,
    NewGirl = 12,
    Kid = 13,
}

// ---------------------------------------------------------------------------
// Action Types
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    Combat = 0,
    Sex = 1,
    General = 2,
    WorkCleaning = 3,
    WorkMatron = 4,
    WorkBar = 5,
    WorkHall = 6,
    WorkShow = 7,
    WorkSecurity = 8,
    WorkAdvertising = 9,
    WorkTorturer = 10,
    WorkCaring = 11,
}

impl ActionType {
    pub const COUNT: usize = 12;
}

// ---------------------------------------------------------------------------
// Image Types
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageType {
    Anal = 0,
    BDSM = 1,
    Sex = 2,
    Beast = 3,
    Group = 4,
    Lesbian = 5,
    Pregnant = 6,
    Death = 7,
    Profile = 8,
    PregAnal = 9,
    PregBDSM = 10,
    PregSex = 11,
    PregBeast = 12,
    PregGroup = 13,
    PregLesbian = 14,
}

impl ImageType {
    pub const COUNT: usize = 15;
}

// ---------------------------------------------------------------------------
// Game Flags
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameFlag {
    CustomerNoPay = 0,
    DungeonGirlDie = 1,
    DungeonCustomerDie = 2,
    CustomerGamblingCheat = 3,
    RivalLose = 4,
}

impl GameFlag {
    pub const COUNT: usize = 5;
}

// ---------------------------------------------------------------------------
// Accommodation Levels
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccommodationLevel {
    Slave = 0,
    VeryPoor = 1,
    Poor = 2,
    Normal = 3,
    Good = 4,
    VeryGood = 5,
    Excellent = 6,
}
