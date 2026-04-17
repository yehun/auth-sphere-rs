
CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kind INTEGER NOT NULL DEFAULT 1,
    nickname VARCHAR(32) NOT NULL,
    username VARCHAR(32) NOT NULL UNIQUE,
    is_mfa INTEGER CHECK (is_mfa IN (0, 1))  DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_user_username ON user(username);
CREATE INDEX IF NOT EXISTS idx_user_kind ON user(kind);
CREATE INDEX IF NOT EXISTS idx_user_status ON user(status);
CREATE INDEX IF NOT EXISTS idx_user_deleted ON user(deleted);


CREATE TABLE IF NOT EXISTS user_password (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    password VARCHAR(64) NOT NULL,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_user_password_user_id ON user_password(user_id);
CREATE INDEX IF NOT EXISTS idx_user_password_deleted ON user_password(deleted);


CREATE TABLE IF NOT EXISTS user_email (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    email VARCHAR(64) NOT NULL UNIQUE,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_user_email_user_id ON user_email(user_id);
CREATE INDEX IF NOT EXISTS idx_user_email_email ON user_email(email);
CREATE INDEX IF NOT EXISTS idx_user_email_deleted ON user_email(deleted);


CREATE TABLE IF NOT EXISTS user_phone (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    phone VARCHAR(20) NOT NULL UNIQUE,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_user_phone_user_id ON user_phone(user_id);
CREATE INDEX IF NOT EXISTS idx_user_phone_phone ON user_phone(phone);
CREATE INDEX IF NOT EXISTS idx_user_phone_deleted ON user_phone(deleted);



CREATE TABLE IF NOT EXISTS user_verify_code (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    source_kind INTEGER NOT NULL,
    source VARCHAR(64) NOT NULL,
    code VARCHAR(20) NOT NULL,
    verify_at DATETIME DEFAULT NULL,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_user_verify_code_user_id ON user_phone(user_id);
CREATE INDEX IF NOT EXISTS idx_user_verify_code_deleted ON user_phone(deleted);


CREATE TABLE IF NOT EXISTS user_session (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    device INTEGER NOT NULL,
    token VARCHAR(64) NOT NULL UNIQUE,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_user_session_user_id ON user_session(user_id);
CREATE INDEX IF NOT EXISTS idx_user_session_token ON user_session(token);
CREATE INDEX IF NOT EXISTS idx_user_session_deleted ON user_session(deleted);


CREATE TABLE IF NOT EXISTS user_mfa (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    secret VARCHAR(64) NOT NULL UNIQUE,
    active INTEGER CHECK (active IN (0, 1))  DEFAULT 0,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    update_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_user_mfa_user_id ON user_mfa(user_id);
CREATE INDEX IF NOT EXISTS idx_user_mfa_secret ON user_mfa(secret);
CREATE INDEX IF NOT EXISTS idx_user_mfa_deleted ON user_mfa(deleted);
