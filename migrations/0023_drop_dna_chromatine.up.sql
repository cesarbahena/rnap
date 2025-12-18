-- Migration: 0023_drop_dna_chromatine
-- Reason: DNA is not linked to Chromatine directly; Gene links to both

DROP TABLE IF EXISTS dna_chromatine;