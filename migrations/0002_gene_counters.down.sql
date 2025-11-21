-- Rollback V002: Remove gene_counters and constraints

ALTER TABLE genes DROP CONSTRAINT IF EXISTS unique_gene_name_per_genome;
DROP TABLE IF EXISTS gene_counters;