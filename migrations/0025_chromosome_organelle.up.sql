-- Migration: 0025_chromosome_organelle
-- Add organelle_id to chromosomes (chromosomes always belong to an organelle)

ALTER TABLE chromosomes ADD COLUMN organelle_id UUID NOT NULL REFERENCES organelles(id);

CREATE INDEX idx_chromosomes_organelle ON chromosomes(organelle_id);