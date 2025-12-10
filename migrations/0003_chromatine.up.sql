CREATE TABLE chromatine (
    id UUID PRIMARY KEY,
    url TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_chromatine_genome ON chromatine(genome_id);
