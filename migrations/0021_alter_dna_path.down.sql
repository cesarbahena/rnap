-- Migration: 0021_alter_dna_path (down)

ALTER TABLE dna DROP COLUMN path;
ALTER TABLE dna ADD COLUMN content TEXT NOT NULL;
ALTER TABLE dna ADD COLUMN chromatine_refs TEXT[] NOT NULL DEFAULT '{}';