import os
import sqlite3
from pathlib import Path


db_path = Path(os.environ.get("SQLITE_DB_PATH", "/data/demo.sqlite"))
db_path.parent.mkdir(parents=True, exist_ok=True)

if db_path.exists():
    db_path.unlink()

conn = sqlite3.connect(db_path)
cur = conn.cursor()

cur.executescript(
    """
    CREATE TABLE customers (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        city TEXT NOT NULL,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE orders (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        customer_id INTEGER NOT NULL,
        product_name TEXT NOT NULL,
        quantity INTEGER NOT NULL,
        total_amount REAL NOT NULL,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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
    """
)

conn.commit()
conn.close()

print(f"seeded sqlite database at {db_path}")
