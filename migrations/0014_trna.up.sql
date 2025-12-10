CREATE TABLE trna (
    id UUID PRIMARY KEY,
    mrna_id UUID NOT NULL REFERENCES mrna(id),
    tasks JSONB NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_trna_mrna ON trna(mrna_id);
CREATE INDEX idx_trna_genome ON trna(genome_id);
