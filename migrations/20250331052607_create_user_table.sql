-- Add migration script here
-- Create user table
CREATE TABLE tb_users (
                          user_id BIGSERIAL PRIMARY KEY,
                          username VARCHAR(50) UNIQUE NOT NULL,
                          email VARCHAR(100) UNIQUE NOT NULL,
                          password_hash VARCHAR(255) NOT NULL,
                          first_name VARCHAR(50),
                          last_name VARCHAR(50),
                          is_active BOOLEAN NOT NULL DEFAULT TRUE,
                          is_verified BOOLEAN NOT NULL DEFAULT FALSE,
                          last_login TIMESTAMPTZ,
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          phone_number VARCHAR(20),
                          date_of_birth DATE,
                          profile_picture_url VARCHAR(255),
                          bio TEXT,
                          role VARCHAR(20) NOT NULL DEFAULT 'user'
);

-- Create indexes
CREATE INDEX idx_tb_users_email ON tb_users(email);
CREATE INDEX idx_tb_users_username ON tb_users(username);

-- Create update trigger
CREATE OR REPLACE FUNCTION update_tb_users_modified_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_tb_users_modtime
    BEFORE UPDATE ON tb_users
    FOR EACH ROW
EXECUTE FUNCTION update_tb_users_modified_column();