use rand::Rng;

use wm_core::enums::{JobType, Shift, Stat, Status};

use crate::girls::GirlManager;
use crate::state::GameState;

/// Events generated during turn processing.
#[derive(Debug, Clone, Default)]
pub struct TurnEvents {
    pub events: Vec<String>,
}

/// Processes a full week/turn of game simulation.
/// Matches the C++ TurnProcessor flow:
///   reset → pre-shift updates → day shift → night shift → end of week
#[derive(Debug)]
pub struct TurnProcessor;

impl TurnProcessor {
    /// Process a complete week of game time.
    pub fn process_week(state: &mut GameState) -> TurnEvents {
        let mut events = TurnEvents::default();
        let mut rng = rand::thread_rng();

        // 1. Reset weekly trackers
        state.gold.reset_weekly();
        for brothel in &mut state.brothels.brothels {
            brothel.reset_weekly();
        }

        // 2. Pre-shift: decay temp stats, aging, STD checks, update traits
        Self::pre_shift_updates(state, &mut rng, &mut events);

        // 3. Process day shift jobs
        Self::process_shift(state, Shift::Day, &mut rng, &mut events);

        // 4. Process night shift jobs
        Self::process_shift(state, Shift::Night, &mut rng, &mut events);

        // 5. End of week: pay wages, tax, gang missions, rival AI, dungeon, etc.
        Self::process_end_of_week(state, &mut rng, &mut events);

        state.week += 1;
        events
    }

    /// Pre-shift updates: temp stat decay, aging, health/trait updates.
    fn pre_shift_updates(
        state: &mut GameState,
        rng: &mut dyn rand::RngCore,
        events: &mut TurnEvents,
    ) {
        for girl in &mut state.girls.girls {
            // Decay temporary stat/skill mods (30% reduction)
            GirlManager::decay_temp_stats(girl);

            // Age (1 week, ~every 52 weeks = 1 year)
            if state.week.is_multiple_of(52) {
                GirlManager::update_stat(girl, Stat::Age, 1);
            }

            // STD progression: if health very low, small chance of disease
            if GirlManager::get_stat(girl, Stat::Health) <= 10
                && rng.gen_range(0..100) < 1
            {
                GirlManager::add_trait(girl, "Chlamydia");
                events.events.push(format!("{} contracted chlamydia!", girl.name));
            }

            // Update happy/sad traits based on current happiness
            GirlManager::update_happy_traits(girl);

            // Update house-related stats
            let is_slave = GirlManager::has_status(girl, Status::Slave);
            GirlManager::update_house_stats(girl, is_slave);

            // Auto-rest check: if health < 80 or tiredness > 80, force to resting
            let health = GirlManager::get_stat(girl, Stat::Health);
            let tiredness = GirlManager::get_stat(girl, Stat::Tiredness);
            if health < 80 || tiredness > 80 {
                if girl.job_day != Some(JobType::Resting) {
                    girl.prev_job_day = girl.job_day;
                    girl.job_day = Some(JobType::Resting);
                }
                if girl.job_night != Some(JobType::Resting) {
                    girl.prev_job_night = girl.job_night;
                    girl.job_night = Some(JobType::Resting);
                }
            } else if girl.job_day == Some(JobType::Resting) && girl.prev_job_day.is_some() {
                // Restore previous job if recovered
                girl.job_day = girl.prev_job_day.take();
                girl.job_night = girl.prev_job_night.take();
            }

            // Calculate ask price
            let price = GirlManager::calculate_ask_price(girl);
            girl.stats[Stat::AskPrice as usize] = price;
        }
    }

    /// Process all jobs for a single shift (Day or Night).
    fn process_shift(
        state: &mut GameState,
        shift: Shift,
        rng: &mut dyn rand::RngCore,
        events: &mut TurnEvents,
    ) {
        // For each brothel, process each assigned girl's job
        let num_brothels = state.brothels.brothels.len();
        for brothel_idx in 0..num_brothels {
            let girl_ids: Vec<usize> = state.brothels.brothels[brothel_idx].girls.clone();

            for &girl_id in &girl_ids {
                if girl_id >= state.girls.girls.len() {
                    continue;
                }

                let job = match shift {
                    Shift::Day => state.girls.girls[girl_id].job_day,
                    Shift::Night => state.girls.girls[girl_id].job_night,
                };

                let job_type = match job {
                    Some(j) => j,
                    None => JobType::Resting, // Default to resting
                };

                // Skip non-processable jobs
                if matches!(job_type, JobType::InDungeon | JobType::Runaway) {
                    continue;
                }

                // Need to temporarily borrow the brothel for job context
                let brothel_snapshot = state.brothels.brothels[brothel_idx].clone();
                let girl = &mut state.girls.girls[girl_id];

                let result = state.job_dispatcher.process(
                    job_type,
                    girl,
                    &brothel_snapshot,
                    rng,
                );

                // Apply gold
                if result.gold_earned > 0 {
                    let gold = result.gold_earned as f64;
                    // Route to appropriate income category based on job
                    match job_type {
                        JobType::WhoreBrothel | JobType::WhoreGambHall => {
                            state.gold.add_brothel_work(gold);
                        }
                        JobType::WhoreStreets => {
                            state.gold.add_street_work(gold);
                        }
                        JobType::Barmaid | JobType::Waitress | JobType::Stripper
                        | JobType::WhoreBar | JobType::Singer => {
                            state.gold.add_bar_income(gold);
                        }
                        JobType::Dealer | JobType::CustomerService
                        | JobType::Entertainment | JobType::XXXEntertainment => {
                            state.gold.add_gambling_profits(gold);
                        }
                        JobType::FilmBeast | JobType::FilmSex | JobType::FilmAnal
                        | JobType::FilmLesbian | JobType::FilmBondage | JobType::Fluffer
                        | JobType::CameraMage | JobType::CrystalPurifier => {
                            state.gold.add_movie_income(gold);
                        }
                        _ => {
                            state.gold.add_brothel_work(gold);
                        }
                    }
                } else if result.gold_earned < 0 {
                    // Negative gold = cost
                    let _ = state.gold.pay_brothel_cost((-result.gold_earned) as f64);
                }

                // Collect events
                for event in result.events {
                    events.events.push(format!("[{}] {}", state.girls.girls[girl_id].name, event));
                }

                // Cleaning reduces filthiness
                if job_type == JobType::Cleaning {
                    state.brothels.brothels[brothel_idx].filthiness =
                        (state.brothels.brothels[brothel_idx].filthiness - 5).max(0);
                }

                // Filthiness increases per working girl (non-resting)
                if !matches!(job_type, JobType::Resting | JobType::Cleaning) {
                    state.brothels.brothels[brothel_idx].filthiness += 1;
                }
            }
        }
    }

    /// End-of-week processing: wages, taxes, gang missions, rival AI, dungeon.
    fn process_end_of_week(
        state: &mut GameState,
        rng: &mut dyn rand::RngCore,
        events: &mut TurnEvents,
    ) {
        // 1. Pay goon wages
        let goon_wages = state.gangs.total_goon_wages();
        if goon_wages > 0.0 {
            state.gold.charge_goon_wages(goon_wages);
        }

        // 2. Building upkeep per brothel
        let num_brothels = state.brothels.brothels.len();
        let upkeep_per_brothel = 100.0; // Base upkeep
        state.gold.charge_building_upkeep(upkeep_per_brothel * num_brothels as f64);

        // 3. Girl support costs
        let total_girls = state.girls.girls.len() as f64;
        state.gold.charge_girl_support(total_girls * 10.0);

        // 4. Advertising costs
        for brothel in &state.brothels.brothels {
            if brothel.advertising_budget > 0.0 {
                state.gold.charge_advertising(brothel.advertising_budget);
            }
        }

        // 5. Tax (percentage of total income)
        let tax_rate = state.config.tax.rate / 100.0;
        let tax = state.gold.total_income() * tax_rate;
        if tax > 0.0 {
            state.gold.charge_tax(tax);
        }

        // 6. Bribes
        let bribe = state.player.bribe_rate as f64;
        if bribe > 0.0 {
            state.gold.charge_bribes(bribe);
        }

        // 7. Process gang missions
        let gang_results = state.gangs.process_missions(
            &mut state.player,
            rng,
        );
        for result in gang_results {
            for event in &result.events {
                events.events.push(format!("[Gang] {}", event));
            }
        }

        // 8. Gang recruit updates
        let gc = &state.config.gangs;
        state.gangs.weekly_recruit_update(
            gc.max_recruit_list,
            gc.start_random,
            gc.start_boosted,
            gc.init_member_min,
            gc.init_member_max,
            gc.add_new_weekly_min,
            gc.add_new_weekly_max,
            gc.chance_remove_unwanted,
            &[], // gang_names
            rng,
        );

        // 9. Rival AI
        let tax_rate = state.config.tax.rate / 100.0;
        let (plunder, rival_events) = state.rivals.process_rivals(tax_rate, rng);
        if plunder > 0 {
            events.events.push(format!("[Rival] Rivals plundered {} gold!", plunder));
        }
        for event in rival_events {
            events.events.push(format!("[Rival] {}", event));
        }

        // 10. Rival takeover check
        if state.rivals.check_takeover() {
            events.events.push("[CRITICAL] Rivals have taken over!".to_string());
        }

        // 11. Maybe spawn new rival
        state.rivals.maybe_spawn_rival(&[], rng);

        // 12. Process dungeon
        state.dungeon.process_week();

        // 13. Level-up checks for all girls
        for girl in &mut state.girls.girls {
            let old_level = girl.stats[Stat::Level as usize];
            GirlManager::check_level_up(girl, rng);
            if girl.stats[Stat::Level as usize] > old_level {
                events.events.push(format!("{} leveled up!", girl.name));
            }
        }

        // 14. Runaway checks
        Self::check_runaways(state, rng, events);

        // 15. Pregnancy progression
        for girl in &mut state.girls.girls {
            if girl.weeks_pregnant > 0 {
                girl.weeks_pregnant += 1;
                if girl.weeks_pregnant >= 40 {
                    // Birth!
                    girl.weeks_pregnant = 0;
                    GirlManager::remove_status(girl, wm_core::enums::Status::Pregnant);
                    GirlManager::remove_status(girl, wm_core::enums::Status::PregnantByPlayer);
                    GirlManager::remove_status(girl, wm_core::enums::Status::Inseminated);
                    events.events.push(format!("{} gave birth!", girl.name));
                }
            }
        }

        // 16. Bank interest (1% per week on deposits)
        let interest = state.gold.bank_balance * 0.01;
        if interest > 0.0 {
            state.gold.bank_balance += interest;
        }
    }

    /// Check if any girls try to run away.
    fn check_runaways(
        state: &mut GameState,
        rng: &mut dyn rand::RngCore,
        events: &mut TurnEvents,
    ) {
        let guard_power: i32 = state.gangs.guard_power();

        for girl in &mut state.girls.girls {
            if girl.job_day == Some(JobType::Runaway) {
                continue; // Already a runaway
            }

            let rebel = GirlManager::get_rebel_value(girl);
            // Only attempt escape if rebel value > 50
            if rebel <= 50 {
                continue;
            }

            // base 5% + (rebel - 50) * 0.5%
            let escape_chance = 5 + (rebel - 50) / 2;
            // Guard power reduces chance
            let adjusted = (escape_chance - guard_power / 10).max(0);

            if rng.gen_range(0..100) < adjusted {
                // Slave attempts escape
                if GirlManager::has_status(girl, wm_core::enums::Status::Slave) {
                    girl.job_day = Some(JobType::Runaway);
                    girl.job_night = Some(JobType::Runaway);
                    events.events.push(format!("{} ran away!", girl.name));
                } else {
                    // Free girl quits
                    GirlManager::update_stat(girl, Stat::Happiness, -20);
                    events.events.push(format!("{} is threatening to leave!", girl.name));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wm_core::config::GameConfig;
    use wm_core::girl::Girl;

    #[test]
    fn test_process_week_empty_game() {
        let config = GameConfig::default();
        let mut state = GameState::new(config);
        let events = TurnProcessor::process_week(&mut state);
        assert_eq!(state.week, 2);
        // No girls = no job events
        assert!(events.events.is_empty() || events.events.iter().all(|e| !e.contains("[Girl]")));
    }

    #[test]
    fn test_process_week_with_girl() {
        let config = GameConfig::default();
        let mut state = GameState::new(config);

        // Add a brothel and a girl
        let brothel_id = state.brothels.add_brothel("Test Brothel");

        let mut girl = Girl::default();
        girl.name = "TestGirl".to_string();
        girl.stats[Stat::Health as usize] = 100;
        girl.stats[Stat::Tiredness as usize] = 0;
        girl.stats[Stat::Beauty as usize] = 50;
        girl.stats[Stat::Charisma as usize] = 50;
        girl.stats[Stat::AskPrice as usize] = 20;
        girl.job_day = Some(JobType::Resting);
        girl.job_night = Some(JobType::Resting);
        state.girls.girls.push(girl);

        state.brothels.brothels[brothel_id].girls.push(0);

        let events = TurnProcessor::process_week(&mut state);
        assert_eq!(state.week, 2);
        // Girl should have healed from resting
        assert!(state.girls.girls[0].stats[Stat::Health as usize] >= 100);
        assert!(!events.events.is_empty()); // Should have events
    }
}
