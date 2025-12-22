-- Migration: 0026_cells
-- Create cells table (C4 Software System level)

CREATE TABLE cells (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_cells_genome ON cells(genome_id);