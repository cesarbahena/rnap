-- Migration: 0023_drop_dna_chromatine (down)

CREATE TABLE dna_chromatine (
    dna_id UUID NOT NULL REFERENCES dna(id),
    chromatine_id UUID NOT NULL REFERENCES chromatine(id),
    PRIMARY KEY (dna_id, chromatine_id)
);