package gogo

import (
	"context"
	"time"
)

func Retry[T any](do func(ctx context.Context) (*T, error), retries int, d time.Duration) func(ctx context.Context) (*T, error) {
	return func(ctx context.Context) (*T, error) {
		for r := 0; ; r++ {
			resp, err := do(ctx)
			if err != nil || r >= retries {
				return resp, err
			}

			select {
			case <-time.After(d):
			case <-ctx.Done():
				return nil, ctx.Err()
			}
		}
	}
}
