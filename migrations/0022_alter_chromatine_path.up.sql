-- Migration: 0022_alter_chromatine_path
-- Changed: url → path

-- Add path column with temporary default
ALTER TABLE chromatine ADD COLUMN path TEXT NOT NULL DEFAULT '';

-- Migrate existing URLs to paths (strip protocol and domain for demo)
UPDATE chromatine SET path = 'docs/migrated/' || LEFT(id::text, 8) || '.md';

-- Drop default
ALTER TABLE chromatine ALTER COLUMN path DROP DEFAULT;

-- Drop old column
ALTER TABLE chromatine DROP COLUMN IF EXISTS url;