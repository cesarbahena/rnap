-- V018: rRNA (acceptance criteria per Gene/Ribosome)

CREATE TABLE rrna (
    id UUID PRIMARY KEY,
    ribosome_id UUID NOT NULL REFERENCES ribosomes(id),
    gene_id UUID NOT NULL REFERENCES genes(id),
    criteria TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_rrna_ribosome ON rrna(ribosome_id);
CREATE INDEX idx_rrna_gene ON rrna(gene_id);
CREATE UNIQUE INDEX idx_rrna_unique ON rrna(ribosome_id, gene_id);