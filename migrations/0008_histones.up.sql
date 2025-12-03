-- V008: Histones (architecture decision records)

CREATE TABLE histones (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    decision TEXT NOT NULL,
    context TEXT NOT NULL,
    mutation_id UUID,
    gene_id UUID,
    dna_id UUID,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_histones_genome ON histones(genome_id);
CREATE INDEX idx_histones_mutation ON histones(mutation_id);
CREATE INDEX idx_histones_gene ON histones(gene_id);
CREATE INDEX idx_histones_dna ON histones(dna_id);