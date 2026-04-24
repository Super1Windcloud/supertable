db = db.getSiblingDB("app");

db.customers.insertMany([
  {
    name: "Alice Chen",
    email: "alice@example.com",
    city: "Shanghai",
    tags: ["vip", "beta"],
  },
  {
    name: "Bob Li",
    email: "bob@example.com",
    city: "Hangzhou",
    tags: ["trial"],
  },
  {
    name: "Carol Wang",
    email: "carol@example.com",
    city: "Shenzhen",
    tags: ["vip"],
  },
]);

db.orders.insertMany([
  {
    customerEmail: "alice@example.com",
    productName: "Mechanical Keyboard",
    quantity: 1,
    totalAmount: 599,
  },
  {
    customerEmail: "bob@example.com",
    productName: "4K Monitor",
    quantity: 1,
    totalAmount: 2299,
  },
  {
    customerEmail: "carol@example.com",
    productName: "Ergonomic Mouse",
    quantity: 3,
    totalAmount: 417,
  },
]);
