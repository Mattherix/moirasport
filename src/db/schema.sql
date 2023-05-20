-- Tables list:
-- Coachs, Teams, Players, Leagues, Seasons, Stages, Groups, Rounds, Referees, Fixtures
-- playing in, arbitrate

CREATE DATABASE IF NOT EXISTS football;
USE football;

CREATE TABLE IF NOT EXISTS Teams (
    id int NOT NULL,
    name varchar(35) NOT NULL,
    short_code varchar(5),
    type set('domestic') NOT NULL,
    image_path varchar(255) NOT NULL,
    founded int,

    PRIMARY KEY (id),

    -- First sets of rules invented in 1845
    -- https://en.wikipedia.org/wiki/History_of_association_football#The_Football_Association
    CHECK (founded >= 1845)
);

CREATE TABLE IF NOT EXISTS Coachs (
    id int NOT NULL,
    firstname varchar(35) NOT NULL,
    lastname varchar(35) NOT NULL,
    image_path varchar(255) NOT NULL,
    height int,
    weight int,
    date_of_birth date,
    gender set('male', 'female', 'neutral') NOT NULL,
    team_id int NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (team_id) REFERENCES Teams(id),

    CHECK (height > 100 AND height < 220),
    CHECK (weight > 30 AND weight < 120)
);

CREATE TABLE IF NOT EXISTS Players (
    id int NOT NULL,
    role set('goalkeeper', 'defender', 'midfielder', 'attacker', 'coach', 'unknown') NOT NULL, -- position field include
    position set('defender', 'goalkeeping-coach', 'coach', 'assistant-coach', 'secondary_striker', 'goalkeeper', 'centre-back', 'defensive-midfied', 'attacking-midfied',
                 'centre-forward', 'left-wing', 'central-midfied', 'right-back',
                 'left-back', 'right-wing', 'left-midfield', 'right-midfield') NOT NULL, -- detailed position include
    firstname varchar(35) NOT NULL,
    lastname varchar(35) NOT NULL,
    image_path varchar(255) NOT NULL,
    height int,
    weight int,
    date_of_birth date,
    gender set('male', 'female', 'neutral') NOT NULL,

    PRIMARY KEY (id),

    CHECK (height < 220),
    CHECK (weight < 120)
);

CREATE TABLE IF NOT EXISTS Leagues (
    id int NOT NULL,
    name varchar(35) NOT NULL,
    short_code varchar(5),
    active boolean NOT NULL,
    image_path varchar(255) NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS Seasons (
    id int NOT NULL,
    name varchar(35) NOT NULL,
    state set('pending', 'current', 'finished') NOT NULL,
    starting_at date NOT NULL,
    ending_at date NOT NULL,
    tie_breaker_rule set("points", "head-to-head", "goal-difference",
        "goal-difference-goals-scored", "head-to-head-ranking-prev-stage",
        "none"),
    league_id int,

    PRIMARY KEY (id),
    FOREIGN KEY (league_id) REFERENCES Leagues(id),

    CHECK (starting_at <= ending_at)
);

CREATE TABLE IF NOT EXISTS Stages (
    id int NOT NULL,
    name varchar(35) NOT NULL,
    type set('group-stage', 'knock-out', 'qualifying'),
    sort_order int,
    state set('current', 'finished') NOT NULL,
    starting_at date,
    ending_at date,
    season_id int NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (season_id) REFERENCES Seasons(id),

    CHECK (starting_at <= ending_at)
);

CREATE TABLE IF NOT EXISTS Rounds (
    id int NOT NULL,
    name varchar(35) NOT NULL,
    state set('current', 'finished') NOT NULL,
    starting_at date,
    ending_at date,
    stage_id int NOT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (stage_id) REFERENCES Stages(id),

    CHECK (starting_at <= ending_at)
);

CREATE TABLE IF NOT EXISTS Referees (
    id int NOT NULL,
    name varchar(100) NOT NULL,
    image_path varchar(255),
    height int,
    weight int,
    date_of_birth date,
    gender set('male', 'female', 'neutral'),

    PRIMARY KEY (id),

    CHECK (height > 100 AND height < 220),
    CHECK (weight > 30 AND weight < 120)
);

CREATE TABLE IF NOT EXISTS Fixtures (
    id int NOT NULL,
    name varchar(50),
    starting_at date,
    length int, 
    home_score int,
    away_score int,
    home_team_id int,
    away_team_id int,
    stage_id int NOT NULL,
    round_id int,

    PRIMARY KEY (id),
    FOREIGN KEY (home_team_id) REFERENCES Teams(id),
    FOREIGN KEY (away_team_id) REFERENCES Teams(id),
    FOREIGN KEY (stage_id) REFERENCES Stages(id),
    FOREIGN KEY (round_id) REFERENCES Rounds(id)
    -- events included is always empty
); 

-- Won't implement, don't know how
CREATE TABLE IF NOT EXISTS arbitrate (
    fixture_id int NOT NULL,
    referee_id int NOT NULL,

    FOREIGN KEY (fixture_id) REFERENCES Fixtures(id),
    FOREIGN KEY (referee_id) REFERENCES Referees(id)
);

CREATE TABLE IF NOT EXISTS playing_in (
    player_id int NOT NULL,
    team_id int NOT NULL,
    starting_at date NOT NULL,
    ending_at date,

    FOREIGN KEY (player_id) REFERENCES Players(id),
    FOREIGN KEY (team_id) REFERENCES Teams(id),

    CHECK (starting_at <= IFNULL(ending_at, starting_at))
);
