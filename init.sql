-- Connect to the modernlibre database
\c modernlibre;

-- Create tables
CREATE TABLE BOOK (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    description TEXT,
    status INT,
    rating FLOAT,
    addedDate DATE
);

CREATE TABLE CATEGORY (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE BOOKCATEGORY (
    bookId INT REFERENCES BOOK(id),
    categoryId INT REFERENCES CATEGORY(id),
    PRIMARY KEY (bookId, categoryId)
);

CREATE TABLE "USER" (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
);

CREATE TABLE READINGLIST (
    id SERIAL PRIMARY KEY,
    userId INT REFERENCES "USER"(id),
    name VARCHAR(255) NOT NULL,
    isPublic BOOLEAN
);

CREATE TABLE READINGLISTBOOK (
    readingListId INT REFERENCES READINGLIST(id),
    bookId INT REFERENCES BOOK(id),
    PRIMARY KEY (readingListId, bookId)
);

CREATE TABLE SUBSCRIPTION (
    userId INT REFERENCES "USER"(id),
    readingListId INT REFERENCES READINGLIST(id),
    PRIMARY KEY (userId, readingListId)
);


-- Insert test data
INSERT INTO BOOK (title, author, description, status, rating, addedDate) VALUES
('Book One', 'Author One', 'Description One', 1, 4.5, '2023-01-01'),
('Book Two', 'Author Two', 'Description Two', 1, 4.0, '2023-02-01');

INSERT INTO CATEGORY (name) VALUES
('Fiction'),
('Non-Fiction');

INSERT INTO BOOKCATEGORY (bookId, categoryId) VALUES
(1, 1),
(2, 2);

INSERT INTO "USER" (username, email) VALUES
('user1', 'user1@example.com'),
('user2', 'user2@example.com');

INSERT INTO READINGLIST (userId, name, isPublic) VALUES
(1, 'Reading List One', TRUE),
(2, 'Reading List Two', FALSE);

INSERT INTO READINGLISTBOOK (readingListId, bookId) VALUES
(1, 1),
(2, 2);

INSERT INTO SUBSCRIPTION (userId, readingListId) VALUES
(1, 2),
(2, 1);