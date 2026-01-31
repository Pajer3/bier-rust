#!/bin/bash
# Probeer de migratie te draaien als de 'biertjeapp' user of via sudo postgres

DB_NAME="biertjeapp"
DB_USER="biertjeapp"
FILE="migrations/0001_initial_schema.sql"

echo "🍺 Applying migration $FILE to database $DB_NAME..."

# Try 1: Connect via TCP (often cleaner for non-peer auth)
if psql -U "$DB_USER" -h 127.0.0.1 -d "$DB_NAME" -f "$FILE" 2>/dev/null; then
    echo "✅ Migration successful (via TCP)!"
    exit 0
fi

# Try 2: Connect via local socket (might fail if peer auth mismatch)
if psql -U "$DB_USER" -d "$DB_NAME" -f "$FILE" 2>/dev/null; then
    echo "✅ Migration successful (via Socket)!"
    exit 0
fi

# Try 3: Fallback to sudo postgres (requires user interaction potentially)
echo "⚠️ Could not connect as $DB_USER directly. Trying via sudo postgres..."
if sudo -u postgres psql -d "$DB_NAME" -f "$FILE"; then
    echo "✅ Migration successful (via sudo postgres)!"
    exit 0
fi

echo "❌ Failed to run migration. Please ensure the database '$DB_NAME' exists and you have access."
exit 1
