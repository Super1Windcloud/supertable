CREATE TABLE IF NOT EXISTS customers (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(120) NOT NULL UNIQUE,
    city VARCHAR(80) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS orders (
    id INT PRIMARY KEY AUTO_INCREMENT,
    customer_id INT NOT NULL,
    product_name VARCHAR(120) NOT NULL,
    quantity INT NOT NULL,
    total_amount DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_orders_customer
        FOREIGN KEY (customer_id) REFERENCES customers(id)
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
