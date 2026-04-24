CREATE TABLE IF NOT EXISTS customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(120) NOT NULL UNIQUE,
    city VARCHAR(80) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    customer_id INT NOT NULL REFERENCES customers(id),
    product_name VARCHAR(120) NOT NULL,
    quantity INT NOT NULL,
    total_amount NUMERIC(10, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO customers (name, email, city) VALUES
    ('Alice Chen', 'alice@example.com', 'Shanghai'),
    ('Bob Li', 'bob@example.com', 'Hangzhou'),
    ('Carol Wang', 'carol@example.com', 'Shenzhen');

INSERT INTO orders (customer_id, product_name, quantity, total_amount) VALUES
    (1, 'Mechanical Keyboard', 1, 599.00),
    (1, 'USB-C Dock', 2, 798.00),
    (2, '4K Monitor', 1, 2299.00),
    (3, 'Ergonomic Mouse', 3, 417.00);
