-- Migration: 0021_alter_dna_for_path
-- Changed: content → path

-- Add path column with default empty string first (to allow migration)
ALTER TABLE dna ADD COLUMN path TEXT NOT NULL DEFAULT '';

-- Migrate existing content to path (use the content as path placeholder)
UPDATE dna SET path = 'dna/migrated/' || LEFT(id::text, 8) || '.dna';

-- Now make path not nullable with no default
ALTER TABLE dna ALTER COLUMN path DROP DEFAULT;

-- Drop old columns
ALTER TABLE dna DROP COLUMN IF EXISTS content;
ALTER TABLE dna DROP COLUMN IF EXISTS chromatine_refs;