-- V011: Quiasmas (typed directed edges)

CREATE TABLE quiasmas (
    id UUID PRIMARY KEY,
    source_id UUID NOT NULL,
    source_type TEXT NOT NULL,
    target_id UUID NOT NULL,
    target_type TEXT NOT NULL,
    relationship_type TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_quiasmas_genome ON quiasmas(genome_id);
CREATE INDEX idx_quiasmas_source ON quiasmas(source_id, source_type);
CREATE INDEX idx_quiasmas_target ON quiasmas(target_id, target_type);