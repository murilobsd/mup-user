-- Add migration script here

CREATE TABLE IF NOT EXISTS roles (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    description		text DEFAULT NULL,
    name 		text NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS users (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    active		boolean DEFAULT FALSE,
    confirmed_at	timestamptz DEFAULT NULL,
    created_at		timestamptz NOT NULL,
    current_login_at	timestamptz DEFAULT NULL,
    current_login_ip	varchar(100) DEFAULT NULL,
    email		citext NOT NULL UNIQUE,
    last_login_at	timestamptz DEFAULT NULL,
    last_login_ip	varchar(100) DEFAULT NULL,
    login_count		INTEGER DEFAULT 0,
    password		varchar(255) NOT NULL,
    salt		varchar(32) NOT NULL,
    updated_at		timestamptz DEFAULT NULL,
    username		varchar(255) DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS roles_users (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id             UUID NOT NULL,
    user_id             UUID NOT NULL,
    FOREIGN KEY (role_id) REFERENCES roles(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO
	roles (name, description)
VALUES
	('admin', 'admin role'),
	('user', 'user role'),
	('anonymous', 'anonymous role');
