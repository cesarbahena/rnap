CREATE TABLE phenotypes (
    id UUID PRIMARY KEY,
    mrna_id UUID NOT NULL REFERENCES mrna(id),
    commit_sha TEXT NOT NULL,
    root_git_directory TEXT NOT NULL DEFAULT '',
    branch TEXT NOT NULL DEFAULT '',
    remote TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_phenotypes_mrna ON phenotypes(mrna_id);
