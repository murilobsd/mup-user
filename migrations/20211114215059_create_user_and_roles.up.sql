-- Add migration script here

-- roles table
CREATE TABLE IF NOT EXISTS roles (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    description		text DEFAULT NULL,
    name 		citext NOT NULL UNIQUE,
    created_at		timestamptz NOT NULL DEFAULT now(),
    updated_at		timestamptz DEFAULT NULL
);
CREATE INDEX idx_name ON roles(name);

-- users table
CREATE TABLE IF NOT EXISTS users (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    active		BOOLEAN DEFAULT FALSE,
    email		citext NOT NULL UNIQUE,
    updated_at		timestamptz DEFAULT NULL,
    confirmed_at	timestamptz DEFAULT NULL,
    created_at		timestamptz NOT NULL DEFAULT now(),
    username		VARCHAR(255) DEFAULT NULL,
    password		VARCHAR(255) NOT NULL,
    salt		VARCHAR(32) NOT NULL
);
CREATE INDEX idx_email ON users(email);
CREATE INDEX idx_username ON users(username);
CREATE INDEX idx_active ON users(active);

-- access table
CREATE TABLE IF NOT EXISTS access (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    login_count		INTEGER DEFAULT 0,
    updated_at		timestamptz DEFAULT NULL,
    current_login_at	timestamptz DEFAULT NULL,
    last_login_at	timestamptz DEFAULT NULL,
    created_at		timestamptz NOT NULL DEFAULT now(),
    user_id             UUID NOT NULL UNIQUE,
    current_login_ip	VARCHAR(100) DEFAULT NULL,
    last_login_ip	VARCHAR(100) DEFAULT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE INDEX idx_user_id ON access(user_id);

-- roles and users table
CREATE TABLE IF NOT EXISTS roles_users (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id             UUID NOT NULL,
    user_id             UUID NOT NULL,
    created_at		timestamptz NOT NULL DEFAULT now(),
    updated_at		timestamptz DEFAULT NULL,

    FOREIGN KEY (role_id) REFERENCES roles(id),
    FOREIGN KEY (user_id) REFERENCES users(id),

    UNIQUE (role_id, user_id)
);

--
-- some default values
--

-- insert roles
INSERT INTO
	roles (id, name, description)
VALUES
	('c1c29c18-c428-405a-819d-300a0f7d46f5', 'admin', 'admin role'),
	('d0ac9fff-4470-4114-98d8-3111704074d4', 'user', 'user role'),
	('893211a9-1407-48e1-ab6d-0cdaffd5f750', 'anonymous', 'anonymous role');

-- insert user admin "mbsd"
-- default password: 123456
INSERT INTO
    	users (id, active, confirmed_at, email, password, salt, username)
VALUES
	('f45e47d7-ff31-4730-af6e-a84fa94e747a', TRUE, now(), 'mbsd@m0x.ru',
	 '$argon2i$v=19$m=256,t=2,p=8$Q01VT0F2d0xpaVR0MExScg$xZUdFoGb1A8ekMt+eNpHwA', 'CMUOAvwLiiTt0LRr', 'mbsd');

-- insert user to admin role
INSERT INTO
	roles_users (role_id, user_id)
VALUES
	('c1c29c18-c428-405a-819d-300a0f7d46f5', 'f45e47d7-ff31-4730-af6e-a84fa94e747a');
