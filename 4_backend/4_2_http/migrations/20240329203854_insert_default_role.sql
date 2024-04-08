-- Add migration script here
INSERT INTO roles (slug, name, permissions) VALUES ('default', 'Default', '["read_files"]');
