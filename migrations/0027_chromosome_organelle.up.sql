-- Migration: 0027_chromosome_organelle
-- Add organelle_id to chromosomes (chromosomes belong to an organelle)
-- Note: Making nullable first since organelles may not exist yet.
--       Application should enforce organelle_id is set on create.

ALTER TABLE chromosomes ADD COLUMN organelle_id UUID REFERENCES organelles(id);

CREATE INDEX idx_chromosomes_organelle ON chromosomes(organelle_id);