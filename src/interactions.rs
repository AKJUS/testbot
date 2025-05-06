use crate::models::{
    InteractionLog, InteractionStats, NewInteractionLog, NewInteractionStats, NewRateLimit,
    RateLimit, UpdateInteractionStats, UpdateRateLimit,
};
use crate::schema::{interaction_logs, interaction_stats, rate_limits};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serenity::model::id::{GuildId, UserId};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::autocomplete::AutocompleteInteraction;
use serenity::model::interactions::message_component::MessageComponentInteraction;
use serenity::model::interactions::modal::ModalSubmitInteraction;
use serenity::model::interactions::InteractionType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct InteractionTracker {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    rate_limits: Arc<RwLock<HashMap<(String, i64), RateLimit>>>,
}

impl InteractionTracker {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            db_pool,
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn track_slash_command(
        &self,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Utc::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.name.clone();

        // Check rate limit
        if !self.check_rate_limit("slash_command", guild_id).await {
            return Ok(());
        }

        // Record interaction
        let conn = self.db_pool.get().unwrap();
        let new_log = NewInteractionLog {
            interaction_type: "slash_command".to_string(),
            interaction_id: interaction_id.clone(),
            guild_id,
            user_id,
            executed_at: start_time.naive_utc(),
            duration: 0.0,
            success: true,
            error_type: None,
        };

        diesel::insert_into(interaction_logs::table)
            .values(&new_log)
            .execute(&conn)?;

        // Update stats
        self.update_interaction_stats("slash_command", &interaction_id, guild_id, 0.0, true)
            .await?;

        Ok(())
    }

    pub async fn track_button_click(
        &self,
        interaction: &MessageComponentInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Utc::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.custom_id.clone();

        // Check rate limit
        if !self.check_rate_limit("button", guild_id).await {
            return Ok(());
        }

        // Record interaction
        let conn = self.db_pool.get().unwrap();
        let new_log = NewInteractionLog {
            interaction_type: "button".to_string(),
            interaction_id: interaction_id.clone(),
            guild_id,
            user_id,
            executed_at: start_time.naive_utc(),
            duration: 0.0,
            success: true,
            error_type: None,
        };

        diesel::insert_into(interaction_logs::table)
            .values(&new_log)
            .execute(&conn)?;

        // Update stats
        self.update_interaction_stats("button", &interaction_id, guild_id, 0.0, true)
            .await?;

        Ok(())
    }

    pub async fn track_modal_submit(
        &self,
        interaction: &ModalSubmitInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Utc::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.custom_id.clone();

        // Check rate limit
        if !self.check_rate_limit("modal", guild_id).await {
            return Ok(());
        }

        // Record interaction
        let conn = self.db_pool.get().unwrap();
        let new_log = NewInteractionLog {
            interaction_type: "modal".to_string(),
            interaction_id: interaction_id.clone(),
            guild_id,
            user_id,
            executed_at: start_time.naive_utc(),
            duration: 0.0,
            success: true,
            error_type: None,
        };

        diesel::insert_into(interaction_logs::table)
            .values(&new_log)
            .execute(&conn)?;

        // Update stats
        self.update_interaction_stats("modal", &interaction_id, guild_id, 0.0, true)
            .await?;

        Ok(())
    }

    pub async fn track_autocomplete(
        &self,
        interaction: &AutocompleteInteraction,
    ) -> Result<(), diesel::result::Error> {
        let start_time = Utc::now();
        let guild_id = interaction.guild_id.map(|id| id.0 as i64).unwrap_or(0);
        let user_id = interaction.user.id.0 as i64;
        let interaction_id = interaction.data.name.clone();

        // Check rate limit
        if !self.check_rate_limit("autocomplete", guild_id).await {
            return Ok(());
        }

        // Record interaction
        let conn = self.db_pool.get().unwrap();
        let new_log = NewInteractionLog {
            interaction_type: "autocomplete".to_string(),
            interaction_id: interaction_id.clone(),
            guild_id,
            user_id,
            executed_at: start_time.naive_utc(),
            duration: 0.0,
            success: true,
            error_type: None,
        };

        diesel::insert_into(interaction_logs::table)
            .values(&new_log)
            .execute(&conn)?;

        // Update stats
        self.update_interaction_stats("autocomplete", &interaction_id, guild_id, 0.0, true)
            .await?;

        Ok(())
    }

    async fn check_rate_limit(&self, interaction_type: &str, guild_id: i64) -> bool {
        let mut rate_limits = self.rate_limits.write().await;
        let key = (interaction_type.to_string(), guild_id);

        let now = Utc::now();
        let limit = rate_limits.entry(key.clone()).or_insert_with(|| RateLimit {
            id: 0,
            interaction_type: interaction_type.to_string(),
            guild_id,
            hits: 0,
            reset_at: (now + Duration::minutes(1)).naive_utc(),
        });

        if now.naive_utc() > limit.reset_at {
            limit.hits = 0;
            limit.reset_at = (now + Duration::minutes(1)).naive_utc();
        }

        if limit.hits >= 5 {
            return false;
        }

        limit.hits += 1;
        true
    }

    async fn update_interaction_stats(
        &self,
        interaction_type: &str,
        interaction_id: &str,
        guild_id: i64,
        duration: f64,
        success: bool,
    ) -> Result<(), diesel::result::Error> {
        let conn = self.db_pool.get().unwrap();
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
        let conn = self.db_pool.get().unwrap();

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
        let conn = self.db_pool.get().unwrap();

        interaction_logs::table
            .filter(interaction_logs::interaction_type.eq(interaction_type))
            .filter(interaction_logs::guild_id.eq(guild_id))
            .order(interaction_logs::executed_at.desc())
            .limit(limit)
            .load::<InteractionLog>(&conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::model::application::interaction::application_command::ApplicationCommandData;
    use serenity::model::application::interaction::autocomplete::AutocompleteData;
    use serenity::model::application::interaction::message_component::MessageComponentData;
    use serenity::model::application::interaction::modal::ModalSubmitData;
    use serenity::model::id::{GuildId, UserId};
    use serenity::model::interactions::application_command::ApplicationCommandInteraction;
    use serenity::model::interactions::autocomplete::AutocompleteInteraction;
    use serenity::model::interactions::message_component::MessageComponentInteraction;
    use serenity::model::interactions::modal::ModalSubmitInteraction;
    use serenity::model::interactions::InteractionType;
    use serenity::model::user::User;

    #[tokio::test]
    async fn test_rate_limiting() {
        let db_pool = Pool::new(ConnectionManager::new("postgres://localhost/test")).unwrap();
        let tracker = InteractionTracker::new(db_pool);

        // Test rate limiting
        assert!(tracker.check_rate_limit("test", 123).await);
        assert!(tracker.check_rate_limit("test", 123).await);
        assert!(tracker.check_rate_limit("test", 123).await);
        assert!(tracker.check_rate_limit("test", 123).await);
        assert!(tracker.check_rate_limit("test", 123).await);
        assert!(!tracker.check_rate_limit("test", 123).await); // Should be rate limited
    }

    #[tokio::test]
    async fn test_interaction_tracking() {
        let db_pool = Pool::new(ConnectionManager::new("postgres://localhost/test")).unwrap();
        let tracker = InteractionTracker::new(db_pool);

        // Test tracking different interaction types
        let user = User {
            id: UserId(123),
            name: "test".to_string(),
            discriminator: "1234".to_string(),
            avatar: None,
            bot: false,
            system: false,
            mfa_enabled: false,
            verified: false,
            email: None,
            flags: None,
            premium_type: None,
            public_flags: None,
        };

        let guild_id = GuildId(456);

        // Test slash command tracking
        let command_data = ApplicationCommandData {
            id: 789,
            name: "test_command".to_string(),
            options: vec![],
        };

        let command_interaction = ApplicationCommandInteraction {
            id: 789,
            application_id: 123,
            data: command_data,
            guild_id: Some(guild_id),
            channel_id: 123,
            member: None,
            user,
            token: "test".to_string(),
            version: 1,
        };

        assert!(tracker
            .track_slash_command(&command_interaction)
            .await
            .is_ok());

        // Test button click tracking
        let button_data = MessageComponentData {
            custom_id: "test_button".to_string(),
            component_type: 2,
            values: vec![],
        };

        let button_interaction = MessageComponentInteraction {
            id: 789,
            application_id: 123,
            data: button_data,
            guild_id: Some(guild_id),
            channel_id: 123,
            member: None,
            user,
            token: "test".to_string(),
            version: 1,
        };

        assert!(tracker
            .track_button_click(&button_interaction)
            .await
            .is_ok());

        // Test modal submit tracking
        let modal_data = ModalSubmitData {
            custom_id: "test_modal".to_string(),
            components: vec![],
        };

        let modal_interaction = ModalSubmitInteraction {
            id: 789,
            application_id: 123,
            data: modal_data,
            guild_id: Some(guild_id),
            channel_id: 123,
            member: None,
            user,
            token: "test".to_string(),
            version: 1,
        };

        assert!(tracker.track_modal_submit(&modal_interaction).await.is_ok());

        // Test autocomplete tracking
        let autocomplete_data = AutocompleteData {
            id: 789,
            name: "test_autocomplete".to_string(),
            options: vec![],
        };

        let autocomplete_interaction = AutocompleteInteraction {
            id: 789,
            application_id: 123,
            data: autocomplete_data,
            guild_id: Some(guild_id),
            channel_id: 123,
            member: None,
            user,
            token: "test".to_string(),
            version: 1,
        };

        assert!(tracker
            .track_autocomplete(&autocomplete_interaction)
            .await
            .is_ok());
    }
}
