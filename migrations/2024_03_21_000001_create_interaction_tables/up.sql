-- Create interaction_logs table
CREATE TABLE interaction_logs (
    id BIGSERIAL PRIMARY KEY,
    interaction_type VARCHAR(50) NOT NULL,
    interaction_id VARCHAR(255) NOT NULL,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    executed_at TIMESTAMP NOT NULL,
    duration DOUBLE PRECISION NOT NULL,
    success BOOLEAN NOT NULL,
    error_type VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create interaction_stats table
CREATE TABLE interaction_stats (
    id BIGSERIAL PRIMARY KEY,
    interaction_type VARCHAR(50) NOT NULL,
    interaction_id VARCHAR(255) NOT NULL,
    guild_id BIGINT NOT NULL,
    count BIGINT NOT NULL DEFAULT 0,
    total_duration DOUBLE PRECISION NOT NULL DEFAULT 0,
    failure_count BIGINT NOT NULL DEFAULT 0,
    last_used TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(interaction_type, interaction_id, guild_id)
);

-- Create rate_limits table
CREATE TABLE rate_limits (
    id BIGSERIAL PRIMARY KEY,
    interaction_type VARCHAR(50) NOT NULL,
    guild_id BIGINT NOT NULL,
    hits BIGINT NOT NULL DEFAULT 0,
    reset_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(interaction_type, guild_id)
);

-- Create indexes
CREATE INDEX idx_interaction_logs_type ON interaction_logs(interaction_type);
CREATE INDEX idx_interaction_logs_guild ON interaction_logs(guild_id);
CREATE INDEX idx_interaction_logs_user ON interaction_logs(user_id);
CREATE INDEX idx_interaction_logs_executed ON interaction_logs(executed_at);

CREATE INDEX idx_interaction_stats_type ON interaction_stats(interaction_type);
CREATE INDEX idx_interaction_stats_guild ON interaction_stats(guild_id);
CREATE INDEX idx_interaction_stats_last_used ON interaction_stats(last_used);

CREATE INDEX idx_rate_limits_type ON rate_limits(interaction_type);
CREATE INDEX idx_rate_limits_guild ON rate_limits(guild_id);
CREATE INDEX idx_rate_limits_reset ON rate_limits(reset_at); 