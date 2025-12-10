CREATE TABLE srna (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    task_context TEXT NOT NULL,
    promoted BOOLEAN NOT NULL DEFAULT FALSE,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_srna_genome ON srna(genome_id);
CREATE INDEX idx_srna_promoted ON srna(promoted);
