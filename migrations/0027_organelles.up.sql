-- Migration: 0027_organelles
-- Create organelles table (C4 Container level, inside a Cell)

CREATE TABLE organelles (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    kind TEXT NOT NULL CHECK (kind IN ('Service', 'Worker', 'Database', 'Queue')),
    technology TEXT NOT NULL DEFAULT '',
    cell_id UUID NOT NULL REFERENCES cells(id),
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_organelles_cell ON organelles(cell_id);
CREATE INDEX idx_organelles_genome ON organelles(genome_id);