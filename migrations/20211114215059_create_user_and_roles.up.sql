-- Add migration script here

CREATE TABLE IF NOT EXISTS roles (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name 		text NOT NULL UNIQUE,
    description		text DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    active		boolean DEFAULT FALSE,
    confirmed_at	timestamptz NOT NULL,
    created_at		timestamptz NOT NULL,
    current_login_at	timestamptz NOT NULL,
    current_login_ip	text DEFAULT NULL,
    email		text NOT NULL UNIQUE,
    last_login_at	timestamptz NOT NULL,
    last_login_ip	text DEFAULT NULL,
    login_count		INTEGER DEFAULT 0,
    password		text NOT NULL,
    updated_at		timestamptz DEFAULT NULL,
    username		text DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS roles_users (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id             UUID NOT NULL, 
    user_id             UUID NOT NULL,
FOREIGN KEY (role_id) REFERENCES roles(id),
FOREIGN KEY (user_id) REFERENCES users(id)
);
