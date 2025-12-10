CREATE TABLE loci (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    view_type TEXT NOT NULL CHECK (view_type IN ('SystemLandscape', 'Context', 'Container', 'Component')),
    scope JSONB NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_loci_genome ON loci(genome_id);
