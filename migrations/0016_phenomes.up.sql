-- V016: Phenomes (QA profile groups)

CREATE TABLE phenomes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_phenomes_genome ON phenomes(genome_id);