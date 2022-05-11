package gogo

import (
	"context"
	"log"
	"sync/atomic"
	"testing"
	"time"
)

func TestDebounceFirst(t *testing.T) {
	var counter uint64

	do := func(ctx context.Context) (*uint64, error) {
		atomic.AddUint64(&counter, 1)
		log.Println("do")
		return &counter, nil
	}

	f := DebounceFirst(do, 100*time.Millisecond)

	for i := 0; i < 10; i++ {
		for j := 0; j < 5; j++ {
			r, err := f(context.Background())
			if err != nil {
				log.Printf("result=nil, err=%s", err)
			} else {
				log.Printf("result=%d, err=%s", *r, err)
			}
		}
		time.Sleep(100 * time.Millisecond)
	}
}

func TestDebounceLast(t *testing.T) {
	var counter uint64

	do := func(ctx context.Context) (*uint64, error) {
		atomic.AddUint64(&counter, 1)
		log.Println("do")
		return &counter, nil
	}

	f := DebounceLast(do, 100*time.Millisecond)

	for i := 0; i < 10; i++ {
		for j := 0; j < 5; j++ {
			r, err := f(context.Background())
			if err != nil {
				log.Printf("result=nil, err=%s", err)
			} else {
				log.Printf("result=%d, err=%s", *r, err)
			}
		}
		time.Sleep(200 * time.Millisecond)
	}
}
