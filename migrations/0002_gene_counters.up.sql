-- V002: Add gene_counters table for atomic sequence generation

CREATE TABLE gene_counters (
    genome_id UUID NOT NULL REFERENCES genomes(id),
    kind TEXT NOT NULL,
    next_seq BIGINT NOT NULL DEFAULT 2,
    PRIMARY KEY (genome_id, kind)
);

-- Add unique constraint on gene names (genome-scoped)
ALTER TABLE genes ADD CONSTRAINT unique_gene_name_per_genome UNIQUE (genome_id, name);