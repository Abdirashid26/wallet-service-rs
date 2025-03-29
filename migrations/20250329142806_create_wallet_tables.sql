-- Create accounts table
CREATE TABLE accounts (
                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                          user_id VARCHAR(255) UNIQUE NOT NULL,
                          balance DECIMAL NOT NULL DEFAULT 0.0,
                          status VARCHAR(50) NOT NULL DEFAULT 'active',
                          created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create blocked_amounts table
CREATE TABLE blocked_amounts (
                                 id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                                 account_id UUID REFERENCES accounts(id) ON DELETE CASCADE,
                                 amount DECIMAL NOT NULL,
                                 reason VARCHAR(255),
                                 created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- Add migration script here
