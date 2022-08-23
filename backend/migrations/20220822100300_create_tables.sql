CREATE TYPE "RoundType" as ENUM (
    'Warmup',
    'Normal',
    'Tie breaker'
    );

CREATE TABLE competitions
(
    id      SERIAL PRIMARY KEY,
    "level" INT NOT NULL
);

CREATE TABLE teams
(
    id                  SERIAL PRIMARY KEY,
    team_name           TEXT NOT NULL,
    entered_competition INT  NOT NULL,
    CONSTRAINT competition_fk
        FOREIGN KEY (entered_competition)
            REFERENCES competitions (id)
);

CREATE TABLE rounds
(
    id          SERIAL PRIMARY KEY,
    "type"      "RoundType" NOT NULL,
    time_limit  INTEGER     NOT NULL,
    competition INT         NOT NULL,
    CONSTRAINT competition_fk
        FOREIGN KEY (competition)
            REFERENCES competitions (id)
);

CREATE TABLE questions
(
    id             SERIAL PRIMARY KEY,
    number         INT  NOT NULL,
    correct_answer TEXT NOT NULL,
    round          INT  NOT NULL,
    CONSTRAINT round_fk
        FOREIGN KEY (round)
            REFERENCES rounds (id)
);

CREATE TABLE team_answers
(
    id       SERIAL  NOT NULL,
    team     INT     NOT NULL,
    question INT     NOT NULL,
    correct  BOOLEAN NOT NULL DEFAULT FALSE,
    "skip"   BOOLEAN NOT NULL DEFAULT FALSE,
    attempts INT     NOT NULL DEFAULT 0,
    CONSTRAINT team_fk
        FOREIGN KEY (team)
            REFERENCES teams (id)
);

CREATE TABLE competition_results
(
    id          SERIAL NOT NULL,
    "placing"   INT    NOT NULL,
    team        INT    NOT NULL,
    competition INT    NOT NULL,
    CONSTRAINT team_fk
        FOREIGN KEY (team)
            REFERENCES teams (id),

    CONSTRAINT competition_fk
        FOREIGN KEY (competition)
            REFERENCES competitions (id)
);
