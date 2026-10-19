package segment_producer

import (
	"context"
	"encoding/binary"
	"errors"
	"fmt"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgxpool"
)

const (
	// djangolang namespaces advisory locks per table as (1337 + n, key). This is the segment
	// producer's own namespace, and these locks are session- rather than transaction-scoped: they
	// outlive every transaction and die with the connection that holds them.
	claimLockClassID = int32(1337 + 40)
)

// errClaimLost is terminal. A producer that cannot prove it owns its camera has to stop writing,
// because some live producer owns it and is writing the same files.
var errClaimLost = errors.New("lost the claim on this camera")

func claimLockKey(cameraID uuid.UUID) int32 {
	return int32(binary.BigEndian.Uint32(cameraID[:4]))
}

// claimFence is the proof that this process owns one camera: a session-scoped advisory lock held on
// a dedicated connection for as long as we write that camera's segments. A live producer's lock is
// visible to every other producer immediately, so a camera is never written by two of them at once,
// and if our connection dies the lock dies with it and the camera is free again.
type claimFence struct {
	conn     *pgxpool.Conn
	cameraID uuid.UUID
	released bool
}

func newClaimFence(ctx context.Context, db *pgxpool.Pool, cameraID uuid.UUID) (*claimFence, error) {
	conn, err := db.Acquire(ctx)
	if err != nil {
		return nil, err
	}

	return &claimFence{conn: conn, cameraID: cameraID}, nil
}

func (f *claimFence) take(ctx context.Context) (bool, error) {
	if f.released {
		return false, fmt.Errorf("fence on camera %v is released", f.cameraID)
	}

	var locked bool
	err := f.conn.
		QueryRow(ctx, "select pg_try_advisory_lock($1, $2)", claimLockClassID, claimLockKey(f.cameraID)).
		Scan(&locked)
	if err != nil {
		return false, err
	}

	return locked, nil
}

// owns reports whether this process is the one that took the fence. Taking is re-entrant for the
// owner, so we take and immediately release to keep the session's lock depth at one.
func (f *claimFence) owns(ctx context.Context) (bool, error) {
	locked, err := f.take(ctx)
	if err != nil || !locked {
		return false, err
	}

	var released bool
	err = f.conn.
		QueryRow(ctx, "select pg_advisory_unlock($1, $2)", claimLockClassID, claimLockKey(f.cameraID)).
		Scan(&released)
	if err != nil {
		return false, err
	}

	return true, nil
}

func (f *claimFence) release(ctx context.Context) error {
	if f.released {
		return nil
	}
	f.released = true

	var released bool
	err := f.conn.
		QueryRow(ctx, "select pg_advisory_unlock($1, $2)", claimLockClassID, claimLockKey(f.cameraID)).
		Scan(&released)

	f.conn.Release()

	return err
}
