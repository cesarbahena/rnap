-- V012: Loci (view projections)

CREATE TABLE loci (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    view_type TEXT NOT NULL,
    scope JSONB NOT NULL DEFAULT '{}',
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_loci_genome ON loci(genome_id);