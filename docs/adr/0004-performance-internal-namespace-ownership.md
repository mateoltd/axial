# ADR 0004: Performance Internal Namespace Ownership
Status: Accepted

## Context
Performance graph mutations must preserve any unknown or concurrently replaced file
in the live `mods/` directory. Cleanup therefore parks an admitted managed artifact
under the instance-local `.axial-performance/` state root before settling it.

Windows can delete through a retained delete-capable handle. POSIX exposes unlink as
a pathname operation and has no primitive that conditionally unlinks a directory
entry only when it still names an expected inode. Keeping every proven park avoids
that final pathname race, but it also makes successful install, remove, and rollback
cycles accumulate launcher cleanup state and eventually exhaust the bounded
quarantine.

## Decision
Reserve the complete instance-local `.axial-performance/**` namespace for launcher
state. On POSIX its directories are owner-only, and broader permissions are
rejected. It is not a user-content namespace. The live `mods/` namespace remains
user-controlled: unknown files are user-owned, and a managed mutation may not
delete or replace a live path that no longer has the exact admitted identity.

On POSIX, a transaction may unlink a parked file by its random internal pathname
only through the reserved-directory capability and the same live file capability
that admitted, moved, and retained the exact artifact. It must reverify the parked
entry against that retained identity and digest after its final settlement hook. The
remaining race between final verification and pathname unlink is accepted only
because every possible target at that path is inside the reserved internal
namespace; this authority never extends back into `mods/`.

Parking is bound to the original admitted file handle. The anchored move rechecks
that handle immediately before the no-replace rename and returns a typed pre-move,
applied, or indeterminate outcome. If the remaining syscall window moves a different
live entry, the applied receipt can restore only into an absent live name; an
occupied live name is preserved and leaves the park latched. Any move durability
error requires an explicit synchronization of both retained directory capabilities
before settlement. After the admitted park is settled, a bounded anchored scan must
prove the entire quarantine empty, not only the expected park absent.

Identity displacement before settlement fails closed. Any quarantine residue
observed after restart also lacks the process-local proof required for deletion.
Those entries are preserved, the instance remains latched, and no new mutation is
allowed to treat them as reclaimable. Publication, commit, rollback restore, and
external target-effect boundaries all recheck this latch before effects. A normal
successful transaction leaves the quarantine empty.

## Consequences
Positive:
- successful POSIX mutations reclaim their temporary parked artifacts instead of
  consuming quarantine capacity indefinitely
- live user content keeps the exact identity and replacement protections of the
  managed-composition boundary
- restart and ambiguous-state handling remain fail-closed

Tradeoffs:
- processes with direct filesystem access must not use `.axial-performance/**` for
  user content or mutate it while the launcher is operating
- POSIX cleanup accepts a pathname race only within the explicitly reserved
  launcher namespace
- restart residue is unrecoverable by automatic reconciliation and keeps the
  instance latched until an external reset; it is never silently deleted
