-- V003: Chromatine (external document references)

CREATE TABLE chromatine (
    id UUID PRIMARY KEY,
    url TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chromatine_genome ON chromatine(genome_id);