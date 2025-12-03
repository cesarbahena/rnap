-- V004: DNA (structured requirements)

CREATE TABLE dna (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dna_genome ON dna(genome_id);