CREATE TABLE dna (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    chromatine_refs TEXT[] NOT NULL DEFAULT '{}',
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_dna_genome ON dna(genome_id);
