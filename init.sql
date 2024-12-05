-- Create the modernlibre database if it does not exist
DO
$$
BEGIN
   IF NOT EXISTS (SELECT 1 FROM pg_database WHERE datname = 'modernlibre') THEN
      PERFORM dblink_exec('dbname=postgres', 'CREATE DATABASE modernlibre');
   END IF;
END
$$;

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