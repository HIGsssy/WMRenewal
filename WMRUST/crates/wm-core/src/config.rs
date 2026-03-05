use serde::{Deserialize, Serialize};

/// Game configuration matching all sections of config.xml.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameConfig {
    pub initial: InitialConfig,
    pub income: IncomeConfig,
    pub expenses: ExpensesConfig,
    pub gambling: GamblingConfig,
    pub tax: TaxConfig,
    pub pregnancy: PregnancyConfig,
    pub gangs: GangsConfig,
    pub prostitution: ProstitutionConfig,
    pub items: ItemsConfig,
    pub fonts: FontsConfig,
    pub debug: DebugConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialConfig {
    pub gold: i32,
    pub girl_meet: i32,
    pub slave_house_perc: i32,
    pub auto_use_items: bool,
    pub torture_trait_week_mod: i32,
}

impl Default for InitialConfig {
    fn default() -> Self {
        Self {
            gold: 4000,
            girl_meet: 30,
            slave_house_perc: 100,
            auto_use_items: false,
            torture_trait_week_mod: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeConfig {
    pub extortion: f64,
    pub brothel_work: f64,
    pub street_work: f64,
    pub movie_income: f64,
    pub stripper_work: f64,
    pub barmaid_work: f64,
    pub slave_sales: f64,
    pub item_sales: f64,
}

impl Default for IncomeConfig {
    fn default() -> Self {
        Self {
            extortion: 1.0,
            brothel_work: 1.0,
            street_work: 1.0,
            movie_income: 1.0,
            stripper_work: 1.0,
            barmaid_work: 1.0,
            slave_sales: 1.0,
            item_sales: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpensesConfig {
    pub training: f64,
    pub movie_cost: f64,
    pub actress_wages: f64,
    pub goon_wages: f64,
    pub matron_wages: f64,
    pub girl_support: f64,
    pub consumables: f64,
    pub items: f64,
    pub slaves_bought: f64,
    pub buy_brothel: f64,
    pub brothel_support: f64,
    pub bar_support: f64,
    pub casino_support: f64,
    pub bribes: f64,
    pub fines: f64,
    pub advertising: f64,
}

impl Default for ExpensesConfig {
    fn default() -> Self {
        Self {
            training: 0.0,
            movie_cost: 0.0,
            actress_wages: 0.0,
            goon_wages: 1.0,
            matron_wages: 1.0,
            girl_support: 1.0,
            consumables: 1.0,
            items: 0.5,
            slaves_bought: 1.0,
            buy_brothel: 1.0,
            brothel_support: 1.0,
            bar_support: 1.0,
            casino_support: 1.0,
            bribes: 1.0,
            fines: 1.0,
            advertising: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamblingConfig {
    pub odds: i32,
    pub base: i32,
    pub spread: i32,
    pub house_factor: f64,
    pub customer_factor: f64,
}

impl Default for GamblingConfig {
    fn default() -> Self {
        Self {
            odds: 49,
            base: 79,
            spread: 400,
            house_factor: 1.0,
            customer_factor: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxConfig {
    pub rate: f64,
    pub minimum: f64,
    pub laundry: f64,
}

impl Default for TaxConfig {
    fn default() -> Self {
        Self {
            rate: 0.06,
            minimum: 0.01,
            laundry: 0.25,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PregnancyConfig {
    pub player_chance: i32,
    pub customer_chance: i32,
    pub monster_chance: i32,
    pub good_sex_factor: f64,
    pub chance_of_girl: i32,
    pub weeks_pregnant: i32,
    pub weeks_till_grown: i32,
    pub cool_down: i32,
}

impl Default for PregnancyConfig {
    fn default() -> Self {
        Self {
            player_chance: 8,
            customer_chance: 8,
            monster_chance: 8,
            good_sex_factor: 2.0,
            chance_of_girl: 50,
            weeks_pregnant: 38,
            weeks_till_grown: 60,
            cool_down: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GangsConfig {
    pub max_recruit_list: i32,
    pub start_random: i32,
    pub start_boosted: i32,
    pub init_member_min: i32,
    pub init_member_max: i32,
    pub chance_remove_unwanted: i32,
    pub add_new_weekly_min: i32,
    pub add_new_weekly_max: i32,
}

impl Default for GangsConfig {
    fn default() -> Self {
        Self {
            max_recruit_list: 12,
            start_random: 2,
            start_boosted: 2,
            init_member_min: 1,
            init_member_max: 10,
            chance_remove_unwanted: 33,
            add_new_weekly_min: 0,
            add_new_weekly_max: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProstitutionConfig {
    pub rape_brothel: f64,
    pub rape_street: f64,
}

impl Default for ProstitutionConfig {
    fn default() -> Self {
        Self {
            rape_brothel: 1.0,
            rape_street: 5.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsConfig {
    pub auto_combat_equip: bool,
    pub rarity_colors: [String; 7],
}

impl Default for ItemsConfig {
    fn default() -> Self {
        Self {
            auto_combat_equip: true,
            rarity_colors: [
                "#000000".to_string(),
                "#000050".to_string(),
                "#0000A0".to_string(),
                "#0000F0".to_string(),
                "#004000".to_string(),
                "#006000".to_string(),
                "#006000".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontsConfig {
    pub normal: String,
    pub fixed: String,
    pub antialias: bool,
}

impl Default for FontsConfig {
    fn default() -> Self {
        Self {
            normal: "DejaVuSans.ttf".to_string(),
            fixed: "DejaVuSansMono.ttf".to_string(),
            antialias: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DebugConfig {
    pub log_all: bool,
    pub log_items: bool,
    pub log_girls: bool,
    pub log_rgirls: bool,
    pub log_fonts: bool,
    pub log_torture: bool,
}

// -- XML deserialization --

#[derive(Debug, Deserialize)]
pub struct ConfigXml {
    #[serde(rename = "Initial")]
    pub initial: Option<InitialXml>,
    #[serde(rename = "Income")]
    pub income: Option<IncomeXml>,
    #[serde(rename = "Expenses")]
    pub expenses: Option<ExpensesXml>,
    #[serde(rename = "Gambling")]
    pub gambling: Option<GamblingXml>,
    #[serde(rename = "Tax")]
    pub tax: Option<TaxXml>,
    #[serde(rename = "Pregnancy")]
    pub pregnancy: Option<PregnancyXml>,
    #[serde(rename = "Gangs")]
    pub gangs: Option<GangsXml>,
    #[serde(rename = "Prostitution")]
    pub prostitution: Option<ProstitutionXml>,
    #[serde(rename = "Items")]
    pub items: Option<ItemsXmlSection>,
    #[serde(rename = "Fonts")]
    pub fonts: Option<FontsXml>,
    #[serde(rename = "Debug")]
    pub debug: Option<DebugXml>,
}

#[derive(Debug, Deserialize)]
pub struct InitialXml {
    #[serde(rename = "@Gold", default)]
    pub gold: Option<String>,
    #[serde(rename = "@GirlMeet", default)]
    pub girl_meet: Option<String>,
    #[serde(rename = "@SlaveHousePerc", default)]
    pub slave_house_perc: Option<String>,
    #[serde(rename = "@AutoUseItems", default)]
    pub auto_use_items: Option<String>,
    #[serde(rename = "@TortureTraitWeekMod", default)]
    pub torture_trait_week_mod: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IncomeXml {
    #[serde(rename = "@ExtortionIncome", default)]
    pub extortion: Option<String>,
    #[serde(rename = "@GirlsWorkBrothel", default)]
    pub brothel_work: Option<String>,
    #[serde(rename = "@GirlsWorkStreet", default)]
    pub street_work: Option<String>,
    #[serde(rename = "@MovieIncome", default)]
    pub movie_income: Option<String>,
    #[serde(rename = "@StripperIncome", default)]
    pub stripper_work: Option<String>,
    #[serde(rename = "@BarmaidIncome", default)]
    pub barmaid_work: Option<String>,
    #[serde(rename = "@SlaveSales", default)]
    pub slave_sales: Option<String>,
    #[serde(rename = "@ItemSales", default)]
    pub item_sales: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExpensesXml {
    #[serde(rename = "@Training", default)]
    pub training: Option<String>,
    #[serde(rename = "@MovieCost", default)]
    pub movie_cost: Option<String>,
    #[serde(rename = "@ActressWages", default)]
    pub actress_wages: Option<String>,
    #[serde(rename = "@GoonWages", default)]
    pub goon_wages: Option<String>,
    #[serde(rename = "@MatronWages", default)]
    pub matron_wages: Option<String>,
    #[serde(rename = "@GirlSupport", default)]
    pub girl_support: Option<String>,
    #[serde(rename = "@Consumables", default)]
    pub consumables: Option<String>,
    #[serde(rename = "@Items", default)]
    pub items: Option<String>,
    #[serde(rename = "@SlavesBought", default)]
    pub slaves_bought: Option<String>,
    #[serde(rename = "@BuyBrothel", default)]
    pub buy_brothel: Option<String>,
    #[serde(rename = "@BrothelSupport", default)]
    pub brothel_support: Option<String>,
    #[serde(rename = "@BarSupport", default)]
    pub bar_support: Option<String>,
    #[serde(rename = "@CasinoSupport", default)]
    pub casino_support: Option<String>,
    #[serde(rename = "@Bribes", default)]
    pub bribes: Option<String>,
    #[serde(rename = "@Fines", default)]
    pub fines: Option<String>,
    #[serde(rename = "@Advertising", default)]
    pub advertising: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GamblingXml {
    #[serde(rename = "@Odds", default)]
    pub odds: Option<String>,
    #[serde(rename = "@Base", default)]
    pub base: Option<String>,
    #[serde(rename = "@Spread", default)]
    pub spread: Option<String>,
    #[serde(rename = "@HouseFactor", default)]
    pub house_factor: Option<String>,
    #[serde(rename = "@CustomerFactor", default)]
    pub customer_factor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TaxXml {
    #[serde(rename = "@Rate", default)]
    pub rate: Option<String>,
    #[serde(rename = "@Minimum", default)]
    pub minimum: Option<String>,
    #[serde(rename = "@Laundry", default)]
    pub laundry: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PregnancyXml {
    #[serde(rename = "@PlayerChance", default)]
    pub player_chance: Option<String>,
    #[serde(rename = "@CustomerChance", default)]
    pub customer_chance: Option<String>,
    #[serde(rename = "@MonsterChance", default)]
    pub monster_chance: Option<String>,
    #[serde(rename = "@GoodSexFactor", default)]
    pub good_sex_factor: Option<String>,
    #[serde(rename = "@ChanceOfGirl", default)]
    pub chance_of_girl: Option<String>,
    #[serde(rename = "@WeeksPregnant", default)]
    pub weeks_pregnant: Option<String>,
    #[serde(rename = "@WeeksTillGrown", default)]
    pub weeks_till_grown: Option<String>,
    #[serde(rename = "@CoolDown", default)]
    pub cool_down: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GangsXml {
    #[serde(rename = "@MaxRecruitList", default)]
    pub max_recruit_list: Option<String>,
    #[serde(rename = "@StartRandom", default)]
    pub start_random: Option<String>,
    #[serde(rename = "@StartBoosted", default)]
    pub start_boosted: Option<String>,
    #[serde(rename = "@InitMemberMin", default)]
    pub init_member_min: Option<String>,
    #[serde(rename = "@InitMemberMax", default)]
    pub init_member_max: Option<String>,
    #[serde(rename = "@ChanceRemoveUnwanted", default)]
    pub chance_remove_unwanted: Option<String>,
    #[serde(rename = "@AddNewWeeklyMin", default)]
    pub add_new_weekly_min: Option<String>,
    #[serde(rename = "@AddNewWeeklyMax", default)]
    pub add_new_weekly_max: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProstitutionXml {
    #[serde(rename = "@RapeBrothel", default)]
    pub rape_brothel: Option<String>,
    #[serde(rename = "@RapeStreet", default)]
    pub rape_street: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ItemsXmlSection {
    #[serde(rename = "@AutoCombatEquip", default)]
    pub auto_combat_equip: Option<String>,
    #[serde(rename = "@RarityColor0", default)]
    pub rarity_color_0: Option<String>,
    #[serde(rename = "@RarityColor1", default)]
    pub rarity_color_1: Option<String>,
    #[serde(rename = "@RarityColor2", default)]
    pub rarity_color_2: Option<String>,
    #[serde(rename = "@RarityColor3", default)]
    pub rarity_color_3: Option<String>,
    #[serde(rename = "@RarityColor4", default)]
    pub rarity_color_4: Option<String>,
    #[serde(rename = "@RarityColor5", default)]
    pub rarity_color_5: Option<String>,
    #[serde(rename = "@RarityColor6", default)]
    pub rarity_color_6: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FontsXml {
    #[serde(rename = "@Normal", default)]
    pub normal: Option<String>,
    #[serde(rename = "@Fixed", default)]
    pub fixed: Option<String>,
    #[serde(rename = "@Antialias", default)]
    pub antialias: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DebugXml {
    #[serde(rename = "@LogAll", default)]
    pub log_all: Option<String>,
    #[serde(rename = "@LogItems", default)]
    pub log_items: Option<String>,
    #[serde(rename = "@LogGirls", default)]
    pub log_girls: Option<String>,
    #[serde(rename = "@LogRGirls", default)]
    pub log_rgirls: Option<String>,
    #[serde(rename = "@LogFonts", default)]
    pub log_fonts: Option<String>,
    #[serde(rename = "@LogTorture", default)]
    pub log_torture: Option<String>,
}

/// Helper: parse a percentage string like "6%" or "49%" into an integer.
fn parse_percent(s: &str) -> Option<i32> {
    s.trim_end_matches('%').parse().ok()
}

/// Helper: parse a percentage string into an f64 fraction (e.g., "6%" -> 0.06).
fn parse_percent_f64(s: &str) -> Option<f64> {
    parse_percent(s).map(|v| v as f64 / 100.0)
}

/// Helper: parse a string as f64, stripping any trailing '%'.
fn parse_f64(s: &str) -> Option<f64> {
    s.trim_end_matches('%').parse().ok()
}

/// Helper: parse a string as i32, stripping any trailing '%'.
fn parse_i32(s: &str) -> Option<i32> {
    s.trim_end_matches('%').parse().ok()
}

/// Helper: parse "true"/"false" string.
fn parse_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

impl ConfigXml {
    /// Convert from XML representation into domain GameConfig.
    pub fn into_config(self) -> GameConfig {
        let mut cfg = GameConfig::default();

        if let Some(init) = self.initial {
            if let Some(v) = init.gold.as_deref().and_then(parse_i32) {
                cfg.initial.gold = v;
            }
            if let Some(v) = init.girl_meet.as_deref().and_then(parse_i32) {
                cfg.initial.girl_meet = v;
            }
            if let Some(v) = init.slave_house_perc.as_deref().and_then(parse_i32) {
                cfg.initial.slave_house_perc = v;
            }
            if let Some(v) = init.auto_use_items.as_deref().and_then(parse_bool) {
                cfg.initial.auto_use_items = v;
            }
            if let Some(v) = init.torture_trait_week_mod.as_deref().and_then(parse_i32) {
                cfg.initial.torture_trait_week_mod = v;
            }
        }

        if let Some(inc) = self.income {
            if let Some(v) = inc.extortion.as_deref().and_then(parse_f64) {
                cfg.income.extortion = v;
            }
            if let Some(v) = inc.brothel_work.as_deref().and_then(parse_f64) {
                cfg.income.brothel_work = v;
            }
            if let Some(v) = inc.street_work.as_deref().and_then(parse_f64) {
                cfg.income.street_work = v;
            }
            if let Some(v) = inc.movie_income.as_deref().and_then(parse_f64) {
                cfg.income.movie_income = v;
            }
            if let Some(v) = inc.stripper_work.as_deref().and_then(parse_f64) {
                cfg.income.stripper_work = v;
            }
            if let Some(v) = inc.barmaid_work.as_deref().and_then(parse_f64) {
                cfg.income.barmaid_work = v;
            }
            if let Some(v) = inc.slave_sales.as_deref().and_then(parse_f64) {
                cfg.income.slave_sales = v;
            }
            if let Some(v) = inc.item_sales.as_deref().and_then(parse_f64) {
                cfg.income.item_sales = v;
            }
        }

        if let Some(exp) = self.expenses {
            if let Some(v) = exp.training.as_deref().and_then(parse_f64) {
                cfg.expenses.training = v;
            }
            if let Some(v) = exp.movie_cost.as_deref().and_then(parse_f64) {
                cfg.expenses.movie_cost = v;
            }
            if let Some(v) = exp.actress_wages.as_deref().and_then(parse_f64) {
                cfg.expenses.actress_wages = v;
            }
            if let Some(v) = exp.goon_wages.as_deref().and_then(parse_f64) {
                cfg.expenses.goon_wages = v;
            }
            if let Some(v) = exp.matron_wages.as_deref().and_then(parse_f64) {
                cfg.expenses.matron_wages = v;
            }
            if let Some(v) = exp.girl_support.as_deref().and_then(parse_f64) {
                cfg.expenses.girl_support = v;
            }
            if let Some(v) = exp.consumables.as_deref().and_then(parse_f64) {
                cfg.expenses.consumables = v;
            }
            if let Some(v) = exp.items.as_deref().and_then(parse_f64) {
                cfg.expenses.items = v;
            }
            if let Some(v) = exp.slaves_bought.as_deref().and_then(parse_f64) {
                cfg.expenses.slaves_bought = v;
            }
            if let Some(v) = exp.buy_brothel.as_deref().and_then(parse_f64) {
                cfg.expenses.buy_brothel = v;
            }
            if let Some(v) = exp.brothel_support.as_deref().and_then(parse_f64) {
                cfg.expenses.brothel_support = v;
            }
            if let Some(v) = exp.bar_support.as_deref().and_then(parse_f64) {
                cfg.expenses.bar_support = v;
            }
            if let Some(v) = exp.casino_support.as_deref().and_then(parse_f64) {
                cfg.expenses.casino_support = v;
            }
            if let Some(v) = exp.bribes.as_deref().and_then(parse_f64) {
                cfg.expenses.bribes = v;
            }
            if let Some(v) = exp.fines.as_deref().and_then(parse_f64) {
                cfg.expenses.fines = v;
            }
            if let Some(v) = exp.advertising.as_deref().and_then(parse_f64) {
                cfg.expenses.advertising = v;
            }
        }

        if let Some(gamb) = self.gambling {
            if let Some(v) = gamb.odds.as_deref().and_then(parse_i32) {
                cfg.gambling.odds = v;
            }
            if let Some(v) = gamb.base.as_deref().and_then(parse_i32) {
                cfg.gambling.base = v;
            }
            if let Some(v) = gamb.spread.as_deref().and_then(parse_i32) {
                cfg.gambling.spread = v;
            }
            if let Some(v) = gamb.house_factor.as_deref().and_then(parse_f64) {
                cfg.gambling.house_factor = v;
            }
            if let Some(v) = gamb.customer_factor.as_deref().and_then(parse_f64) {
                cfg.gambling.customer_factor = v;
            }
        }

        if let Some(tax) = self.tax {
            if let Some(v) = tax.rate.as_deref().and_then(parse_percent_f64) {
                cfg.tax.rate = v;
            }
            if let Some(v) = tax.minimum.as_deref().and_then(parse_percent_f64) {
                cfg.tax.minimum = v;
            }
            if let Some(v) = tax.laundry.as_deref().and_then(parse_percent_f64) {
                cfg.tax.laundry = v;
            }
        }

        if let Some(preg) = self.pregnancy {
            if let Some(v) = preg.player_chance.as_deref().and_then(parse_i32) {
                cfg.pregnancy.player_chance = v;
            }
            if let Some(v) = preg.customer_chance.as_deref().and_then(parse_i32) {
                cfg.pregnancy.customer_chance = v;
            }
            if let Some(v) = preg.monster_chance.as_deref().and_then(parse_i32) {
                cfg.pregnancy.monster_chance = v;
            }
            if let Some(v) = preg.good_sex_factor.as_deref().and_then(parse_f64) {
                cfg.pregnancy.good_sex_factor = v;
            }
            if let Some(v) = preg.chance_of_girl.as_deref().and_then(parse_i32) {
                cfg.pregnancy.chance_of_girl = v;
            }
            if let Some(v) = preg.weeks_pregnant.as_deref().and_then(parse_i32) {
                cfg.pregnancy.weeks_pregnant = v;
            }
            if let Some(v) = preg.weeks_till_grown.as_deref().and_then(parse_i32) {
                cfg.pregnancy.weeks_till_grown = v;
            }
            if let Some(v) = preg.cool_down.as_deref().and_then(parse_i32) {
                cfg.pregnancy.cool_down = v;
            }
        }

        if let Some(gangs) = self.gangs {
            if let Some(v) = gangs.max_recruit_list.as_deref().and_then(parse_i32) {
                cfg.gangs.max_recruit_list = v;
            }
            if let Some(v) = gangs.start_random.as_deref().and_then(parse_i32) {
                cfg.gangs.start_random = v;
            }
            if let Some(v) = gangs.start_boosted.as_deref().and_then(parse_i32) {
                cfg.gangs.start_boosted = v;
            }
            if let Some(v) = gangs.init_member_min.as_deref().and_then(parse_i32) {
                cfg.gangs.init_member_min = v;
            }
            if let Some(v) = gangs.init_member_max.as_deref().and_then(parse_i32) {
                cfg.gangs.init_member_max = v;
            }
            if let Some(v) = gangs.chance_remove_unwanted.as_deref().and_then(parse_i32) {
                cfg.gangs.chance_remove_unwanted = v;
            }
            if let Some(v) = gangs.add_new_weekly_min.as_deref().and_then(parse_i32) {
                cfg.gangs.add_new_weekly_min = v;
            }
            if let Some(v) = gangs.add_new_weekly_max.as_deref().and_then(parse_i32) {
                cfg.gangs.add_new_weekly_max = v;
            }
        }

        if let Some(prost) = self.prostitution {
            if let Some(v) = prost.rape_brothel.as_deref().and_then(parse_f64) {
                cfg.prostitution.rape_brothel = v;
            }
            if let Some(v) = prost.rape_street.as_deref().and_then(parse_f64) {
                cfg.prostitution.rape_street = v;
            }
        }

        if let Some(items) = self.items {
            if let Some(v) = items.auto_combat_equip.as_deref().and_then(parse_bool) {
                cfg.items.auto_combat_equip = v;
            }
            let colors = [
                &items.rarity_color_0,
                &items.rarity_color_1,
                &items.rarity_color_2,
                &items.rarity_color_3,
                &items.rarity_color_4,
                &items.rarity_color_5,
                &items.rarity_color_6,
            ];
            for (i, color) in colors.iter().enumerate() {
                if let Some(c) = color {
                    cfg.items.rarity_colors[i] = c.clone();
                }
            }
        }

        if let Some(fonts) = self.fonts {
            if let Some(v) = fonts.normal {
                cfg.fonts.normal = v;
            }
            if let Some(v) = fonts.fixed {
                cfg.fonts.fixed = v;
            }
            if let Some(v) = fonts.antialias.as_deref().and_then(parse_bool) {
                cfg.fonts.antialias = v;
            }
        }

        if let Some(debug) = self.debug {
            if let Some(v) = debug.log_all.as_deref().and_then(parse_bool) {
                cfg.debug.log_all = v;
            }
            if let Some(v) = debug.log_items.as_deref().and_then(parse_bool) {
                cfg.debug.log_items = v;
            }
            if let Some(v) = debug.log_girls.as_deref().and_then(parse_bool) {
                cfg.debug.log_girls = v;
            }
            if let Some(v) = debug.log_rgirls.as_deref().and_then(parse_bool) {
                cfg.debug.log_rgirls = v;
            }
            if let Some(v) = debug.log_fonts.as_deref().and_then(parse_bool) {
                cfg.debug.log_fonts = v;
            }
            if let Some(v) = debug.log_torture.as_deref().and_then(parse_bool) {
                cfg.debug.log_torture = v;
            }
        }

        cfg
    }
}
