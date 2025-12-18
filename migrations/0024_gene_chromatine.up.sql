-- Migration: 0024_gene_chromatine
-- Gene ↔ Chromatine relationship (for future RAG context)

CREATE TABLE gene_chromatine (
    gene_id UUID NOT NULL REFERENCES genes(id),
    chromatine_id UUID NOT NULL REFERENCES chromatine(id),
    PRIMARY KEY (gene_id, chromatine_id)
);

CREATE INDEX idx_gene_chromatine_gene ON gene_chromatine(gene_id);
CREATE INDEX idx_gene_chromatine_chromatine ON gene_chromatine(chromatine_id);