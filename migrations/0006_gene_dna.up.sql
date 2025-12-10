CREATE TABLE gene_dna (
    gene_id UUID NOT NULL REFERENCES genes(id),
    dna_id UUID NOT NULL REFERENCES dna(id),
    PRIMARY KEY (gene_id, dna_id)
);
