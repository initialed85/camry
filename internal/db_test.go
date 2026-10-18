package internal

import "testing"

func TestGetDBLimitsDefaults(t *testing.T) {
	t.Setenv("POSTGRES_MAX_IDLE_CONNS", "")
	t.Setenv("POSTGRES_MAX_OPEN_CONNS", "")

	maxIdleConns, maxOpenConns, err := GetDBLimits()
	if err != nil {
		t.Fatalf("GetDBLimits: %v", err)
	}
	if maxIdleConns != 5 || maxOpenConns != 500 {
		t.Fatalf("limits = %d/%d, want 5/500 (the djangolang defaults)", maxIdleConns, maxOpenConns)
	}
}

func TestGetDBLimitsFromEnvironment(t *testing.T) {
	t.Setenv("POSTGRES_MAX_IDLE_CONNS", "2")
	t.Setenv("POSTGRES_MAX_OPEN_CONNS", "8")

	maxIdleConns, maxOpenConns, err := GetDBLimits()
	if err != nil {
		t.Fatalf("GetDBLimits: %v", err)
	}
	if maxIdleConns != 2 || maxOpenConns != 8 {
		t.Fatalf("limits = %d/%d, want 2/8", maxIdleConns, maxOpenConns)
	}
}

func TestGetDBLimitsRejectsGarbage(t *testing.T) {
	t.Setenv("POSTGRES_MAX_IDLE_CONNS", "plenty")

	if _, _, err := GetDBLimits(); err == nil {
		t.Fatal("expected an error for a non-numeric POSTGRES_MAX_IDLE_CONNS")
	}
}
