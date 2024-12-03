-- Connect to the modernlibre database
\c modernlibre;

-- Create tables
CREATE TABLE BOOK (
    id SERIAL PRIMARY KEY,  -- 书本 ID
    title VARCHAR(255) NOT NULL, -- 书本标题
    author VARCHAR(255) NOT NULL, -- 作者
    description TEXT,   -- 书本描述
    status INT, -- 0: Inactive, 1: Active
    rating FLOAT,   -- 评分
    added_date DATE,  -- 添加日期
    file_url VARCHAR(255),  -- 书本文件的 URL
    cover_url VARCHAR(255)  -- 书本封面的 URL
);

CREATE TABLE CATEGORY (
    id SERIAL PRIMARY KEY,  -- 分类 ID
    name VARCHAR(255) NOT NULL  -- 分类名称
);

CREATE TABLE BOOKCATEGORY (
    book_id INT REFERENCES BOOK(id),    -- 书本 ID
    category_id INT REFERENCES CATEGORY(id),    -- 分类 ID
    PRIMARY KEY (book_id, category_id)
);

CREATE TABLE "USER" (
    id SERIAL PRIMARY KEY,  -- 用户 ID
    username VARCHAR(255) NOT NULL,   -- 用户名
    email VARCHAR(255) NOT NULL -- 电子邮件
);

CREATE TABLE READINGLIST (
    id SERIAL PRIMARY KEY,  -- 书单 ID
    user_id INT REFERENCES "USER"(id),  -- 用户 ID
    name VARCHAR(255) NOT NULL, -- 书单名称
    is_public BOOLEAN
);

CREATE TABLE READINGLISTBOOK (
    reading_list_id INT REFERENCES READINGLIST(id), -- 书单 ID
    book_id INT REFERENCES BOOK(id),    -- 书本 ID
    PRIMARY KEY (reading_list_id, book_id)
);

CREATE TABLE SUBSCRIPTION (
    user_id INT REFERENCES "USER"(id),  -- 用户 ID
    reading_list_id INT REFERENCES READINGLIST(id), -- 书单 ID
    PRIMARY KEY (user_id, reading_list_id)
);

CREATE TABLE chapters (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    index INT NOT NULL,
    content TEXT NOT NULL,
    level INT NOT NULL,
    parent_id INT,
    book_id INT REFERENCES BOOK(id),
    created_time DATE,
    updated_time DATE
);

-- Insert test data
INSERT INTO BOOK (title, author, description, status, rating, added_date, file_url, cover_url) VALUES
('Book One', 'Author One', 'Description One', 1, 4.5, '2023-01-01', 'file_url_1', 'cover_url_1'),
('Book Two', 'Author Two', 'Description Two', 1, 4.0, '2023-02-01', 'file_url_2', 'cover_url_2');

INSERT INTO CATEGORY (name) VALUES
('Fiction'),
('Non-Fiction');

INSERT INTO BOOKCATEGORY (book_id, category_id) VALUES
(1, 1),
(2, 2);

INSERT INTO "USER" (username, email) VALUES
('user1', 'user1@example.com'),
('user2', 'user2@example.com');

INSERT INTO READINGLIST (user_id, name, is_public) VALUES
(1, 'Reading List One', TRUE),
(2, 'Reading List Two', FALSE);

INSERT INTO READINGLISTBOOK (reading_list_id, book_id) VALUES
(1, 1),
(2, 2);

INSERT INTO SUBSCRIPTION (user_id, reading_list_id) VALUES
(1, 2),
(2, 1);