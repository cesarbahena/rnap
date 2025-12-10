CREATE TABLE organisms (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('Human', 'Team', 'Service')),
    description TEXT NOT NULL DEFAULT '',
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_organisms_genome ON organisms(genome_id);
