-- V001: Initial schema

-- Genome is the tenant boundary
CREATE TABLE genomes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Genotype is the document type definition (generation-versioned schema)
CREATE TABLE genotypes (
    id UUID PRIMARY KEY,
    kind TEXT NOT NULL,
    name TEXT NOT NULL,
    generation INT NOT NULL DEFAULT 1,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    traits JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Gene is the primary SDLC artifact (mutation log)
CREATE TABLE genes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    genotype_id UUID NOT NULL REFERENCES genotypes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Mutation is the source of truth (append-only)
CREATE TABLE mutations (
    id UUID PRIMARY KEY,
    gene_id UUID NOT NULL REFERENCES genes(id),
    trait_key TEXT NOT NULL,
    value JSONB NOT NULL,
    by TEXT NOT NULL,
    context TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_genotypes_kind ON genotypes(kind);
CREATE INDEX idx_genotypes_genome ON genotypes(genome_id);
CREATE INDEX idx_genes_name ON genes(name);
CREATE INDEX idx_genes_genome ON genes(genome_id);
CREATE INDEX idx_genes_genotype ON genes(genotype_id);
CREATE INDEX idx_mutations_gene ON mutations(gene_id);
CREATE INDEX idx_mutations_trait ON mutations(gene_id, trait_key);