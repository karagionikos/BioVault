# sync-sim: the sync simulator

> **Human-owned and mandatory.** Phase 2 isn't done, and no sync change ships, unless this passes (D69). AI agents may help write it; you design the assertions and review every line.

A Rust test crate that runs 2–3 in-memory BioVault devices on the real `biovault-core` and checks that they always converge without losing anything. Spec: [05 · The sync simulator](../../docs/05-sync-spec.md#the-sync-simulator-mandatory-d69).

## What it does

1. Generates random, realistic edits on each device while it's "offline": create papers, edit takeaways, link, move categories, delete and restore.
2. Syncs random pairs of devices, including interrupted syncs and out-of-order delivery.
3. After a full sync, asserts that:
   - every device has an identical vault (byte-identical canonical JSON export);
   - no op is lost, and every concurrent text edit survives as the winner or as a conflict version;
   - there are no duplicate entities and no category cycles;
   - re-applying all ops changes nothing (idempotence).
4. Runs property-based with `proptest`, thousands of cases locally. Every failing seed is kept as a regression test: proptest writes them to `proptest-regressions/` files, and those files are committed.

## When

Built **first** in Phase 2 (step 2.1, test-first), before any networking ([10](../../docs/10-roadmap.md)). It relies on `biovault-core`'s op log, which records from P1's first build.
