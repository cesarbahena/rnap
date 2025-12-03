-- V007: Gene ↔ Chromosome (many-to-many)

CREATE TABLE gene_chromosome (
    gene_id UUID NOT NULL REFERENCES genes(id),
    chromosome_id UUID NOT NULL REFERENCES chromosomes(id),
    PRIMARY KEY (gene_id, chromosome_id)
);