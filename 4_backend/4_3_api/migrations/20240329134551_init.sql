-- Create Users Table
CREATE TYPE sex_type AS ENUM ('male', 'female', 'other');
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    sex sex_type NOT NULL
);

-- Create Roles Table
CREATE TABLE roles (
    slug TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    permissions JSONB NOT NULL
);

-- Create Role_User Table
CREATE TABLE role_user (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) NOT NULL,
    role_slug TEXT REFERENCES roles(slug) NOT NULL
);
