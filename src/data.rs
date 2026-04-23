pub const CONNECTIONS: [Connection; 5] = [
    Connection::new("Production", "mysql://prod-us-east", "12 schemas", true, "PROD"),
    Connection::new("Analytics", "postgres://warehouse-main", "8 schemas", false, "LIVE"),
    Connection::new("Staging", "mysql://staging-shanghai", "11 schemas", false, "STAGE"),
    Connection::new("Local Dev", "sqlite://workspace.db", "3 schemas", false, "LOCAL"),
    Connection::new("Redis Cache", "redis://cache-eu-1", "6 dbs", false, "CACHE"),
];

pub const SCHEMA_ITEMS: [SchemaItem; 8] = [
    SchemaItem::new("Tables", 23, true),
    SchemaItem::new("Views", 5, false),
    SchemaItem::new("Functions", 12, false),
    SchemaItem::new("Triggers", 7, false),
    SchemaItem::new("Users", 4, false),
    SchemaItem::new("Migrations", 18, false),
    SchemaItem::new("Favorites", 9, false),
    SchemaItem::new("Pins", 3, false),
];

pub const QUERY_LINES: [&str; 8] = [
    "SELECT",
    "    o.id,",
    "    o.customer_name,",
    "    o.status,",
    "    o.total_amount,",
    "    o.updated_at",
    "FROM orders o",
    "WHERE o.status IN ('paid', 'refunded') ORDER BY o.updated_at DESC LIMIT 200;",
];

pub const ROWS: [ResultRow; 8] = [
    ResultRow::new("84721", "Pixel Union", "paid", "$12,480.00", "3 items", "2026-04-23 18:02"),
    ResultRow::new("84720", "Acme Labs", "refunded", "$420.00", "1 item", "2026-04-23 17:48"),
    ResultRow::new("84719", "Northwind", "paid", "$2,190.40", "9 items", "2026-04-23 17:30"),
    ResultRow::new("84718", "Neon Works", "paid", "$98.00", "1 item", "2026-04-23 17:08"),
    ResultRow::new("84717", "Delta Freight", "pending", "$4,800.00", "2 items", "2026-04-23 16:54"),
    ResultRow::new("84716", "Sora Health", "paid", "$1,244.10", "5 items", "2026-04-23 16:15"),
    ResultRow::new("84715", "Blue Harbor", "cancelled", "$640.00", "2 items", "2026-04-23 15:49"),
    ResultRow::new("84714", "Orbit Studio", "paid", "$8,020.00", "4 items", "2026-04-23 15:20"),
];

#[derive(Clone, Copy)]
pub struct Connection {
    pub name: &'static str,
    pub endpoint: &'static str,
    pub meta: &'static str,
    pub active: bool,
    pub badge: &'static str,
}

impl Connection {
    pub const fn new(
        name: &'static str,
        endpoint: &'static str,
        meta: &'static str,
        active: bool,
        badge: &'static str,
    ) -> Self {
        Self {
            name,
            endpoint,
            meta,
            active,
            badge,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SchemaItem {
    pub name: &'static str,
    pub count: usize,
    pub active: bool,
}

impl SchemaItem {
    pub const fn new(name: &'static str, count: usize, active: bool) -> Self {
        Self {
            name,
            count,
            active,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ResultRow {
    pub id: &'static str,
    pub customer: &'static str,
    pub status: &'static str,
    pub amount: &'static str,
    pub items: &'static str,
    pub updated_at: &'static str,
}

impl ResultRow {
    pub const fn new(
        id: &'static str,
        customer: &'static str,
        status: &'static str,
        amount: &'static str,
        items: &'static str,
        updated_at: &'static str,
    ) -> Self {
        Self {
            id,
            customer,
            status,
            amount,
            items,
            updated_at,
        }
    }
}
