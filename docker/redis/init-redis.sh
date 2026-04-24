#!/bin/sh
set -eu

redis-cli -h redis FLUSHALL
redis-cli -h redis SET app:name "SuperTable Demo"
redis-cli -h redis SET app:env "local"
redis-cli -h redis HSET user:1001 name "Alice Chen" email "alice@example.com" city "Shanghai"
redis-cli -h redis HSET user:1002 name "Bob Li" email "bob@example.com" city "Hangzhou"
redis-cli -h redis LPUSH recent:orders "order-1004" "order-1003" "order-1002" "order-1001"
redis-cli -h redis SADD feature_flags "preview" "import" "team-mode"
