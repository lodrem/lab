package gogo

import (
	"context"
	"errors"
	"sync"
	"time"
)

func CircuitBreaker[T any](do func(ctx context.Context) (*T, error), maxFailure uint) func(ctx context.Context) (*T, error) {
	var (
		failures    int
		mtx         sync.RWMutex
		lastAttempt = time.Now()
	)

	return func(ctx context.Context) (*T, error) {
		mtx.RLock()

		d := failures - int(maxFailure)
		if d >= 0 {
			shouldRetryAt := lastAttempt.Add(time.Duration(2*d) * time.Second)
			if shouldRetryAt.After(time.Now()) {
				mtx.RUnlock()
				return nil, errors.New("service unreachable")
			}
		}

		mtx.RUnlock()

		resp, err := do(ctx)

		mtx.Lock()
		defer mtx.Unlock()

		lastAttempt = time.Now()

		if err != nil {
			failures++
			return resp, err
		}
		failures = 0
		return resp, nil
	}
}
