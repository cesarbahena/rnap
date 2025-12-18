-- Migration: 0022_alter_chromatine_path (down)

ALTER TABLE chromatine DROP COLUMN path;
ALTER TABLE chromatine ADD COLUMN url TEXT NOT NULL;