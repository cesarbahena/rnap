# Report: Gene Sequence Number Generation Problem

## Executive Summary

The current implementation for generating sequential gene names (e.g., `FEAT-0001`, `FEAT-0002`) has a race condition that can cause duplicate sequence numbers under concurrent load. Additionally, there is no database-level constraint to prevent duplicates.

## Current Implementation

### Code Location
- `crates/rnap-storage/src/lib.rs` - `next_sequence_for_kind()` method
- `crates/rnap-cli/src/main.rs` - Create command wiring

### Current Logic
```rust
pub async fn next_sequence_for_kind(&self, kind: &str) -> u32 {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT MAX(CAST(SUBSTRING(name FROM 6 FOR 4) AS INTEGER)) FROM genes WHERE name LIKE $1"
    )
    .bind(format!("{}-%", kind))
    .fetch_optional(&self.pool)
    .await
    .ok()
    .flatten();

    match row {
        Some((max_seq,)) => max_seq + 1,
        None => 1,
    }
}
```

### Database Schema
```sql
CREATE TABLE genes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    genome_id UUID NOT NULL REFERENCES genomes(id),
    genotype_id UUID NOT NULL REFERENCES genotypes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- NO UNIQUE CONSTRAINT ON name
```

## Identified Problems

### 1. Race Condition (Critical)
Two concurrent `rnap create` calls can both read `MAX() = 1`, both compute `next_seq = 2`, and both attempt to insert:
- Gene A: `FEAT-0002-awesome-feature`
- Gene B: `FEAT-0002-another-feature`

Without a unique constraint, both succeed → **duplicate sequence numbers**

### 2. No Unique Constraint
The `name` column has no uniqueness enforcement at the database level.

### 3. SUBSTRING Fragility
The query `SUBSTRING(name FROM 6 FOR 4)` assumes:
- Format: `KIND-NNNN-slug`
- Position 6 is start of sequence (after "FEAT-" = 5 chars)
- Exactly 4 digits

If format changes, query breaks silently.

### 4. No Atomicity
The SELECT MAX → compute → INSERT sequence is not atomic. A transaction could see stale data.

## Impact Assessment

| Scenario | Current Behavior | Severity |
|----------|------------------|----------|
| Single-threaded create | Works correctly | None |
| Concurrent creates | May produce duplicates | High |
| Schema change (format) | Query breaks silently | Medium |
| Duplicate name insertion | Succeeds (no constraint) | High |

## Solution Options

### Option A: Sequences Table (Recommended)
```sql
CREATE TABLE gene_sequences (
    kind TEXT PRIMARY KEY,
    next_seq INT NOT NULL DEFAULT 1
);

-- Insert initial sequence
INSERT INTO gene_sequences (kind, next_seq) VALUES ('FEAT', 1);
```

Atomic increment:
```sql
UPDATE gene_sequences SET next_seq = next_seq + 1 WHERE kind = $1 RETURNING next_seq;
```

**Pros:** Clean, atomic, handles concurrency properly
**Cons:** Requires separate table, must seed initial values

### Option B: Unique Constraint + Retry
```sql
ALTER TABLE genes ADD CONSTRAINT unique_gene_name UNIQUE (name);
```

Then in application code, catch duplicate error and retry with new sequence.

**Pros:** Simple change, no new tables
**Cons:** Application must handle retries, still has race condition (just handles it)

### Option C: Advisory Locks
Use PostgreSQL's `pg_advisory_xact_lock()` to serialize access to the MAX() query.

**Pros:** In-database solution
**Cons:** Complex, less idiomatic, requires careful locking

### Option D: Trigger-Based Auto-Increment
Use a BEFORE INSERT trigger to compute and assign the sequence number.

**Pros:** Transparent to application
**Cons:** Hidden magic, harder to debug, still needs sequences table for atomicity

## Recommendation

**Implement Option A (Sequences Table)** for the following reasons:
1. Properly handles concurrent access with atomic updates
2. Clear and explicit - sequence state is visible in the database
3. Easy to debug and inspect
4. Follows PostgreSQL best practices for custom sequences
5. Can be extended to support multiple genotype kinds (FEAT, BUG, etc.)

## Implementation Plan

1. Add `gene_sequences` table to schema
2. Seed initial sequences for existing genotype kinds
3. Rewrite `next_sequence_for_kind()` to use atomic UPDATE...RETURNING
4. Add unique constraint on gene names as safety net
5. Update CLI to handle "sequence not found" error gracefully

## Testing Strategy

1. Write test for sequential creates (should get 1, 2, 3...)
2. Write test for concurrent creates (should not get duplicates)
3. Verify unique constraint catches any remaining duplicates