-- V005: DNA ↔ Chromatine (many-to-many)

CREATE TABLE dna_chromatine (
    dna_id UUID NOT NULL REFERENCES dna(id),
    chromatine_id UUID NOT NULL REFERENCES chromatine(id),
    PRIMARY KEY (dna_id, chromatine_id)
);