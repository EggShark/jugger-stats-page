-- Create tables and table scheemas will also drop them
DROP TABLE IF EXISTS plays;
DROP TABLE IF EXISTS matches;
DROP TABLE IF EXISTS events;
DROP TABLE IF EXISTS leauge_registrations;
DROP TABLE IF EXISTS teams;
DROP TABLE IF EXISTS players;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    username VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    description VARCHAR,
    user_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY
);

CREATE TABLE players (
	name VARCHAR NOT NULL,
	user_id INTEGER REFERENCES users (user_id),
	player_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY
);

CREATE TABLE teams (
	name VARCHAR NOT NULL,
	-- make geo location type
	current_location VARCHAR NOT NULL,
	description VARCHAR,
	captain_id INTEGER NOT NULL REFERENCES players (player_id),
	team_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY
);

CREATE TABLE leauge_registrations (
	year INTEGER NOT NULL,
	jersey_number INTEGER NOT NULL,
	jersey_name VARCHAR NOT NULL,
	player_id INTEGER NOT NULL REFERENCES players (player_id),
	team_id INTEGER NOT NULL REFERENCES teams (team_id)
);

CREATE TABLE events (
	name VARCHAR NOT NULL,
	start_time TIMESTAMPTZ NOT NULL,
	end_time TIMESTAMPTZ NOT NULL,
	-- use postGIS for this later
	location VARCHAR NOT NULL,
	event_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY
);

CREATE TABLE matches (
	team1_id INTEGER NOT NULL REFERENCES teams (team_id),
	team2_id INTEGER NOT NULL REFERENCES teams (team_id),
	event_id INTEGER NOT NULL REFERENCES events (event_id),
	t1_score INTEGER NOT NULL,
	t2_score INTEGER NOT NULL,
	match_id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY
);

-- no data on what players were enforcing in the CJL as of NOW I have plans,,,
CREATE TABLE plays (
	stones_elapsed INTEGER NOT NULL,
	match_id INTEGER NOT NULL REFERENCES matches (match_id),
	scoring_player INTEGER NOT NULL REFERENCES players (player_id)
);