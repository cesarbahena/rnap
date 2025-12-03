-- V013: mRNA (frozen mutation snapshots)

CREATE TABLE mrna (
    id UUID PRIMARY KEY,
    gene_id UUID NOT NULL REFERENCES genes(id),
    version INT NOT NULL,
    mutation_ids JSONB NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mrna_gene ON mrna(gene_id);
CREATE INDEX idx_mrna_genome ON mrna(genome_id);