-- V020: Proteins (evaluation results)

CREATE TABLE proteins (
    id UUID PRIMARY KEY,
    phenotype_id UUID NOT NULL REFERENCES phenotypes(id),
    phenome_id UUID NOT NULL REFERENCES phenomes(id),
    gene_id UUID NOT NULL REFERENCES genes(id),
    result TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_proteins_phenotype ON proteins(phenotype_id);
CREATE INDEX idx_proteins_phenome ON proteins(phenome_id);
CREATE INDEX idx_proteins_gene ON proteins(gene_id);
CREATE INDEX idx_proteins_genome ON proteins(genome_id);