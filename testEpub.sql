USE epubTest;

CREATE TABLE chapter (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    `index` INT NOT NULL,
    content_index INT NOT NULL,
    `level` INT NOT NULL,
    parent_id INT,
    book_id INT,
    created_time DATE ,
    updated_time DATE
);