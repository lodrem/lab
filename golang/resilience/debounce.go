package gogo

import (
	"context"
	"errors"
	"sync"
	"time"
)

var (
	ErrDebouncing = errors.New("debouncing")
)

func DebounceFirst[T any](do func(ctx context.Context) (*T, error), d time.Duration) func(ctx context.Context) (*T, error) {
	var (
		threshold time.Time
		result    *T
		err       error
		mtx       sync.Mutex
	)

	return func(ctx context.Context) (*T, error) {
		mtx.Lock()
		defer func() {
			threshold = time.Now().Add(d)
			mtx.Unlock()
		}()

		if time.Now().Before(threshold) {
			return result, err
		}

		result, err = do(ctx)
		return result, err
	}
}

func DebounceLast[T any](do func(ctx context.Context) (*T, error), d time.Duration) func(ctx context.Context) (*T, error) {
	var (
		result *T
		err    = ErrDebouncing
		mtx    sync.Mutex
		timer  *time.Timer
	)

	return func(ctx context.Context) (*T, error) {
		mtx.Lock()
		defer mtx.Unlock()

		if timer != nil {
			timer.Stop()
		}
		timer = time.AfterFunc(d, func() {
			mtx.Lock()
			defer mtx.Unlock()

			result, err = do(ctx)
		})
		t := timer // avoid race condition
		go func() {
			select {
			case <-t.C:
			case <-ctx.Done():
				t.Stop()
			}
		}()

		return result, err
	}
}
