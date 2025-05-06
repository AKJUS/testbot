use crate::models::{
    InteractionLog, InteractionStats, NewInteractionLog, NewInteractionStats, NewRateLimit,
    RateLimit, UpdateInteractionStats, UpdateRateLimit,
};
use crate::schema::{interaction_logs, interaction_stats, rate_limits};
use crate::metrics::{
    INTERACTION_REQUESTS,
    INTERACTION_ERRORS,
    INTERACTION_DURATION,
};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use poise::serenity_prelude::{
    Context, GuildId, User, UserId,
    model::application::interaction::{
        ApplicationCommandInteraction, MessageComponentInteraction, ModalSubmitInteraction,
        AutocompleteInteraction,
    },
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Instant;
use std::error::Error;
use diesel::PgConnection;

pub struct InteractionTracker {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    rate_limits: RwLock<HashMap<(i64, String), RateLimit>>,
    interactions: HashMap<String, Instant>,
}

impl InteractionTracker {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self {
            pool,
            rate_limits: RwLock::new(HashMap::new()),
            interactions: HashMap::new(),
        }
    }

    pub async fn track_interaction(
        &self,
        interaction_type: &str,
        interaction_id: &str,
        user_id: i64,
        guild_id: i64,
    ) -> Result<(), diesel::result::Error> {
        let conn = &mut self.pool.get().unwrap();
        
        let new_log = NewInteractionLog {
            interaction_type: interaction_type.to_string(),
            interaction_id: interaction_id.to_string(),
            guild_id,
            user_id,
            timestamp: Utc::now().naive_utc(),
        };

        diesel::insert_into(interaction_logs::table)
            .values(&new_log)
            .execute(conn)?;

        Ok(())
    }

    pub async fn check_rate_limit(
        &self,
        user_id: i64,
        command: &str,
    ) -> Result<bool, diesel::result::Error> {
        let conn = &mut self.pool.get().unwrap();
        let now = Utc::now();

        let mut limits = self.rate_limits.write().await;
        let key = (user_id, command.to_string());

        if let Some(limit) = limits.get_mut(&key) {
            if now.naive_utc() > limit.last_used + Duration::minutes(1) {
                limit.count = 0;
                limit.last_used = now.naive_utc();
            }

            if limit.count >= 5 {
                return Ok(false);
            }

            limit.count += 1;
            limit.last_used = now.naive_utc();

            diesel::update(rate_limits::table)
                .filter(rate_limits::user_id.eq(user_id))
                .filter(rate_limits::command.eq(command))
                .set(UpdateRateLimit {
                    last_used: now.naive_utc(),
                    count: limit.count,
                })
                .execute(conn)?;
        } else {
            let new_limit = NewRateLimit {
                user_id,
                command: command.to_string(),
                last_used: now.naive_utc(),
                count: 1,
            };

            let limit = diesel::insert_into(rate_limits::table)
                .values(&new_limit)
                .get_result::<RateLimit>(conn)?;

            limits.insert(key, limit);
        }

        Ok(true)
    }

    pub async fn track_slash_command(
        &self,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Instant::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.name.clone();

        // Record metrics
        INTERACTION_REQUESTS.with_label_values(&["slash_command"]).inc();

        // Check rate limit
        if !self.check_rate_limit(user_id, "slash_command").await? {
            INTERACTION_ERRORS.with_label_values(&["slash_command"]).inc();
            return Ok(());
        }

        // Record interaction
        self.track_interaction("slash_command", &interaction_id, user_id, guild_id)?;

        // Update stats
        self.update_interaction_stats("slash_command", &interaction_id, guild_id, 0.0, true)
            .await?;

        // Record duration
        let duration = start_time.elapsed().as_secs_f64();
        INTERACTION_DURATION.with_label_values(&["slash_command"]).observe(duration);

        Ok(())
    }

    pub async fn track_button_click(
        &self,
        interaction: &MessageComponentInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Instant::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.custom_id.clone();

        // Record metrics
        INTERACTION_REQUESTS.with_label_values(&["button"]).inc();

        // Check rate limit
        if !self.check_rate_limit(user_id, "button").await? {
            INTERACTION_ERRORS.with_label_values(&["button"]).inc();
            return Ok(());
        }

        // Record interaction
        self.track_interaction("button", &interaction_id, user_id, guild_id)?;

        // Update stats
        self.update_interaction_stats("button", &interaction_id, guild_id, 0.0, true)
            .await?;

        // Record duration
        let duration = start_time.elapsed().as_secs_f64();
        INTERACTION_DURATION.with_label_values(&["button"]).observe(duration);

        Ok(())
    }

    pub async fn track_modal_submit(
        &self,
        interaction: &ModalSubmitInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Instant::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.custom_id.clone();

        // Record metrics
        INTERACTION_REQUESTS.with_label_values(&["modal"]).inc();

        // Check rate limit
        if !self.check_rate_limit(user_id, "modal").await? {
            INTERACTION_ERRORS.with_label_values(&["modal"]).inc();
            return Ok(());
        }

        // Record interaction
        self.track_interaction("modal", &interaction_id, user_id, guild_id)?;

        // Update stats
        self.update_interaction_stats("modal", &interaction_id, guild_id, 0.0, true)
            .await?;

        // Record duration
        let duration = start_time.elapsed().as_secs_f64();
        INTERACTION_DURATION.with_label_values(&["modal"]).observe(duration);

        Ok(())
    }

    pub async fn track_autocomplete(
        &self,
        interaction: &AutocompleteInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Instant::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.name.clone();

        // Record metrics
        INTERACTION_REQUESTS.with_label_values(&["autocomplete"]).inc();

        // Check rate limit
        if !self.check_rate_limit(user_id, "autocomplete").await? {
            INTERACTION_ERRORS.with_label_values(&["autocomplete"]).inc();
            return Ok(());
        }

        // Record interaction
        self.track_interaction("autocomplete", &interaction_id, user_id, guild_id)?;

        // Update stats
        self.update_interaction_stats("autocomplete", &interaction_id, guild_id, 0.0, true)
            .await?;

        // Record duration
        let duration = start_time.elapsed().as_secs_f64();
        INTERACTION_DURATION.with_label_values(&["autocomplete"]).observe(duration);

        Ok(())
    }

    async fn update_interaction_stats(
        &self,
        interaction_type: &str,
        interaction_id: &str,
        guild_id: i64,
        duration: f64,
        success: bool,
    ) -> Result<(), diesel::result::Error> {
        let conn = self.pool.get().unwrap();
        let now = Utc::now().naive_utc();

        // Try to update existing stats
        let result = diesel::update(
            interaction_stats::table
                .filter(interaction_stats::interaction_type.eq(interaction_type))
                .filter(interaction_stats::interaction_id.eq(interaction_id))
                .filter(interaction_stats::guild_id.eq(guild_id)),
        )
        .set((
            interaction_stats::count.eq(interaction_stats::count + 1),
            interaction_stats::total_duration.eq(interaction_stats::total_duration + duration),
            interaction_stats::failure_count
                .eq(interaction_stats::failure_count + if !success { 1 } else { 0 }),
            interaction_stats::last_used.eq(now),
        ))
        .execute(&conn);

        // If no stats exist, create new ones
        if result.unwrap_or(0) == 0 {
            let new_stats = NewInteractionStats {
                interaction_type: interaction_type.to_string(),
                interaction_id: interaction_id.to_string(),
                guild_id,
                count: 1,
                total_duration: duration,
                failure_count: if !success { 1 } else { 0 },
                last_used: now,
            };

            diesel::insert_into(interaction_stats::table)
                .values(&new_stats)
                .execute(&conn)?;
        }

        Ok(())
    }

    pub async fn get_interaction_stats(
        &self,
        interaction_type: &str,
        guild_id: i64,
    ) -> Result<Vec<InteractionStats>, diesel::result::Error> {
        let conn = self.pool.get().unwrap();

        interaction_stats::table
            .filter(interaction_stats::interaction_type.eq(interaction_type))
            .filter(interaction_stats::guild_id.eq(guild_id))
            .load::<InteractionStats>(&conn)
    }

    pub async fn get_interaction_logs(
        &self,
        interaction_type: &str,
        guild_id: i64,
        limit: i64,
    ) -> Result<Vec<InteractionLog>, diesel::result::Error> {
        let conn = self.pool.get().unwrap();

        interaction_logs::table
            .filter(interaction_logs::interaction_type.eq(interaction_type))
            .filter(interaction_logs::guild_id.eq(guild_id))
            .order(interaction_logs::executed_at.desc())
            .limit(limit)
            .load::<InteractionLog>(&conn)
    }

    pub fn track_interaction(&mut self, interaction_id: &str) {
        self.interactions.insert(interaction_id.to_string(), Instant::now());
    }

    pub fn get_duration(&self, interaction_id: &str) -> Option<Duration> {
        self.interactions
            .get(interaction_id)
            .map(|start_time| start_time.elapsed())
    }

    pub fn remove_interaction(&mut self, interaction_id: &str) {
        self.interactions.remove(interaction_id);
    }

    pub async fn log_interaction(
        &self,
        interaction_id: &str,
        user_id: i64,
        guild_id: Option<i64>,
        interaction_type: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get()?;
        let now = Utc::now().naive_utc();

        // Log the interaction
        diesel::insert_into(interaction_logs::table)
            .values((
                interaction_logs::interaction_id.eq(interaction_id),
                interaction_logs::user_id.eq(user_id),
                interaction_logs::guild_id.eq(guild_id),
                interaction_logs::interaction_type.eq(interaction_type),
                interaction_logs::timestamp.eq(now),
            ))
            .execute(&mut *conn)?;

        // Update interaction stats
        diesel::insert_into(interaction_stats::table)
            .values((
                interaction_stats::interaction_type.eq(interaction_type),
                interaction_stats::count.eq(1),
                interaction_stats::last_used.eq(now),
            ))
            .on_conflict(interaction_stats::interaction_type)
            .do_update()
            .set((
                interaction_stats::count.eq(interaction_stats::count + 1),
                interaction_stats::last_used.eq(now),
            ))
            .execute(&mut *conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poise::serenity_prelude::UserId;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use std::time::Duration;

    fn create_test_pool() -> Pool<ConnectionManager<PgConnection>> {
        let database_url = "postgres://localhost/testbot_test";
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .max_size(1)
            .build(manager)
            .expect("Failed to create pool")
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let pool = Arc::new(create_test_pool());
        let tracker = InteractionTracker::new(pool);

        // First 5 requests should succeed
        for _ in 0..5 {
            assert!(tracker.check_rate_limit(123, "test_command").await.unwrap());
        }

        // 6th request should fail
        assert!(!tracker.check_rate_limit(123, "test_command").await.unwrap());

        // Wait for rate limit to reset
        tokio::time::sleep(Duration::from_secs(61)).await;

        // Should succeed again after reset
        assert!(tracker.check_rate_limit(123, "test_command").await.unwrap());
    }

    #[tokio::test]
    async fn test_interaction_tracking() {
        let pool = Arc::new(create_test_pool());
        let tracker = InteractionTracker::new(pool);

        // Test tracking different types of interactions
        assert!(tracker
            .track_interaction("slash_command", "cmd_123", 123, 456)
            .await
            .is_ok());
        assert!(tracker
            .track_interaction("button_click", "btn_123", 123, 456)
            .await
            .is_ok());
        assert!(tracker
            .track_interaction("modal_submit", "modal_123", 123, 456)
            .await
            .is_ok());
        assert!(tracker
            .track_interaction("autocomplete", "auto_123", 123, 456)
            .await
            .is_ok());
    }
}
