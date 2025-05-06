-- Drop indexes
DROP INDEX IF EXISTS idx_interaction_logs_type;
DROP INDEX IF EXISTS idx_interaction_logs_guild;
DROP INDEX IF EXISTS idx_interaction_logs_user;
DROP INDEX IF EXISTS idx_interaction_logs_executed;

DROP INDEX IF EXISTS idx_interaction_stats_type;
DROP INDEX IF EXISTS idx_interaction_stats_guild;
DROP INDEX IF EXISTS idx_interaction_stats_last_used;

DROP INDEX IF EXISTS idx_rate_limits_type;
DROP INDEX IF EXISTS idx_rate_limits_guild;
DROP INDEX IF EXISTS idx_rate_limits_reset;

-- Drop tables
DROP TABLE IF EXISTS interaction_logs;
DROP TABLE IF EXISTS interaction_stats;
DROP TABLE IF EXISTS rate_limits; 