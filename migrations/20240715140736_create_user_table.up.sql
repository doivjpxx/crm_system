-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (id, name, email, username, password, created_at, updated_at) VALUES
('cd187045-8b42-4b8a-8913-abaf62ea72c9', 'Quý Trần', 'dochoixehoicaocap1@gmail.com', 'dochoixehoicaocap1', '$argon2id$v=19$m=19456,t=2,p=1$yCwGU/SdnV+GCSjwy+iAmg$FoI5LhKzWNG6BsYPjTIAoDuZ9gghTU2G9DQ73Ql0ed0', '2024-08-07 16:06:54.993885', '2024-08-07 16:06:54.993885');