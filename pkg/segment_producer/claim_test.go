package segment_producer

import (
	"context"
	"os"
	"testing"

	"github.com/google/uuid"
	"github.com/initialed85/camry/internal"
	"github.com/jackc/pgx/v5/pgxpool"
)

// The fence is the whole claim design, and its semantics belong to Postgres, so this is tested
// against a real one: SEGMENT_PRODUCER_DB_TEST=1 with the usual POSTGRES_* environment.
func fencingDB(t *testing.T) *pgxpool.Pool {
	t.Helper()

	if os.Getenv("SEGMENT_PRODUCER_DB_TEST") != "1" {
		t.Skip("set SEGMENT_PRODUCER_DB_TEST=1 (plus POSTGRES_* ) to test the claim fence")
	}

	db, err := internal.GetDBFromEnvironment(context.Background())
	if err != nil {
		t.Fatalf("GetDBFromEnvironment: %v", err)
	}

	t.Cleanup(func() { db.Close() })

	return db
}

// advisoryLocks counts this connection's grants of cameraID's fence, so we can see the depth the
// refresh path is leaving behind.
func advisoryLocks(t *testing.T, fence *claimFence, cameraID uuid.UUID) int {
	t.Helper()

	var count int
	err := fence.conn.
		QueryRow(context.Background(), `
			select count(*)
			from pg_locks
			where
				locktype = 'advisory'
				and granted
				and pid = pg_backend_pid()
				and classid = $1::oid
				and objid = $2::int4`,
			claimLockClassID,
			claimLockKey(cameraID),
		).
		Scan(&count)
	if err != nil {
		t.Fatalf("counting advisory locks: %v", err)
	}

	return count
}

func TestClaimFence(t *testing.T) {
	db := fencingDB(t)
	ctx := context.Background()

	cameraID := uuid.New()
	otherID := uuid.New()

	fenceA, err := newClaimFence(ctx, db, cameraID)
	if err != nil {
		t.Fatalf("newClaimFence: %v", err)
	}
	fenceB, err := newClaimFence(ctx, db, cameraID)
	if err != nil {
		t.Fatalf("newClaimFence: %v", err)
	}
	fenceC, err := newClaimFence(ctx, db, otherID)
	if err != nil {
		t.Fatalf("newClaimFence: %v", err)
	}

	defer func() {
		_ = fenceA.release(ctx)
		_ = fenceB.release(ctx)
		_ = fenceC.release(ctx)
	}()

	// A producer takes the camera it is about to write.
	locked, err := fenceA.take(ctx)
	if err != nil {
		t.Fatalf("fenceA.take: %v", err)
	}
	if !locked {
		t.Fatal("fenceA.take = false, want true on an unfenced camera")
	}

	// The owner keeps proving it owns it, and a refresh never stacks the lock: the owner's session
	// holds exactly one grant of this camera's fence, no matter how many segments it has refreshed
	// through.
	owned, err := fenceA.owns(ctx)
	if err != nil {
		t.Fatalf("fenceA.owns: %v", err)
	}
	if !owned {
		t.Fatal("fenceA.owns = false, want true for the owner")
	}

	if got := advisoryLocks(t, fenceA, cameraID); got != 1 {
		t.Fatalf("advisory locks after a refresh = %v, want 1", got)
	}

	locked, err = fenceB.take(ctx)
	if err != nil {
		t.Fatalf("fenceB.take: %v", err)
	}
	if locked {
		t.Fatal("fenceB took a camera fenceA is writing")
	}

	if got := advisoryLocks(t, fenceB, cameraID); got != 0 {
		t.Fatalf("fenceB holds %v advisory locks on a camera it failed to take, want 0", got)
	}

	// Released is claimable: this is the handover a restarted producer relies on.
	if err := fenceA.release(ctx); err != nil {
		t.Fatalf("fenceA.release: %v", err)
	}

	locked, err = fenceB.take(ctx)
	if err != nil {
		t.Fatalf("fenceB.take after release: %v", err)
	}
	if !locked {
		t.Fatal("fenceB.take = false after fenceA released the camera")
	}

	// A released fence reports itself as gone instead of using a connection that is back in the
	// pool - and the segment producer's own exit path closes the pool, which is what frees a
	// camera when a pod dies.
	if _, err := fenceA.owns(ctx); err == nil {
		t.Fatal("a released fence reported no error")
	}

	// Fences are per camera: B owning this one does not fence a producer on another.
	locked, err = fenceC.take(ctx)
	if err != nil {
		t.Fatalf("fenceC.take: %v", err)
	}
	if !locked {
		t.Fatalf("fenceC was fenced by a producer on camera %s", cameraID)
	}
}
