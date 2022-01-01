CREATE TABLE users (
        id CHAR(18) PRIMARY KEY,
        name VARCHAR NOT NULL
);

CREATE TABLE ratings (
        rating DOUBLE PRECISION NOT NULL,
        deviation DOUBLE PRECISION NOT NULL,
        character INT NOT NULL,
        user_id CHAR(18) REFERENCES users (id) NOT NULL
);

CREATE TABLE matches (
        "timestamp" TIMESTAMP,
        winner_char INT,
        loser_char INT,
        winner CHAR(18) REFERENCES users (id) NOT NULL,
        loser CHAR(18) REFERENCES users (id) NOT NULL,
        CONSTRAINT PK_M PRIMARY KEY ( "timestamp", winner_char, loser_char )
);
