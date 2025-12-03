-- V017: Ribosomes (QA pipelines)

CREATE TABLE ribosomes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    phenome_id UUID NOT NULL REFERENCES phenomes(id),
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ribosomes_phenome ON ribosomes(phenome_id);
CREATE INDEX idx_ribosomes_genome ON ribosomes(genome_id);