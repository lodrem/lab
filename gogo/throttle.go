package gogo

import (
	"context"
	"errors"
	"time"
)

var (
	ErrRateLimit = errors.New("rate limited")
)

func TokenBucketThrottle[T any](do func(ctx context.Context) (*T, error), refill uint, d time.Duration) func(ctx context.Context) (*T, error) {

	const MaxTokens = 1024

	var (
		tokens = MaxTokens
	)

	// TODO: cancel the goroutine for good
	go func() {
		for range time.Tick(d) {
			tokens += int(refill)
			if tokens > MaxTokens {
				tokens = MaxTokens
			}
		}
	}()

	return func(ctx context.Context) (*T, error) {
		if tokens <= 0 {
			return nil, ErrRateLimit
		}
		tokens--

		return do(ctx)
	}
}
