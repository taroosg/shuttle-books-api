CREATE TABLE IF NOT EXISTS books (
  id serial PRIMARY KEY,
  title TEXT NOT NULL,
  isbn TEXT NOT NULL UNIQUE
);

INSERT INTO books (title, isbn) VALUES
('プログラミングRust 第2版', '978-4873119786'),
('Rustの練習帳', '978-4814400584'),
('ゼロから学ぶRust', '978-4065301951'),
('RustによるWebアプリケーション開発', '978-4065369579');