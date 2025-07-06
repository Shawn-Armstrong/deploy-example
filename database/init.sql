-- Create users table if it doesn't exist
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    affiliation VARCHAR(50),
    verified BOOLEAN DEFAULT FALSE,
    verification_token UUID
);

-- Create greetings table if it doesn't exist
CREATE TABLE IF NOT EXISTS greetings (
    id SERIAL PRIMARY KEY,
    message TEXT NOT NULL
);

-- Seed greetings table with sample messages
INSERT INTO greetings (message) VALUES
    ('Hello, world!'),
    ('Welcome to UCSC!'),
    ('Express and Postgres are working together!');