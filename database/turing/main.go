package main

import (
	"log/slog"
	"math/rand"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	labels = []string{"label"}

	counters = promauto.NewCounterVec(prometheus.CounterOpts{
		Name: "counter",
	}, labels)
	gauges = promauto.NewGaugeVec(prometheus.GaugeOpts{
		Name: "gauge",
	}, labels)
	histograms = promauto.NewHistogramVec(prometheus.HistogramOpts{
		Name:    "histogram",
		Buckets: []float64{0.01, 0.05, 0.1, 0.5, 1, 5, 10},
	}, labels)
	summaries = promauto.NewSummaryVec(prometheus.SummaryOpts{
		Name: "summary",
	}, labels)
)

func main() {
	go func() {
		ticker := time.NewTicker(time.Duration(rand.Intn(1000)) * time.Millisecond)
		defer ticker.Stop()

		labelValues := []string{"dummy"}

		for range ticker.C {
			counters.WithLabelValues(labelValues...).Inc()
			if rand.Intn(2) == 1 {
				gauges.WithLabelValues(labelValues...).Inc()
			} else {
				gauges.WithLabelValues(labelValues...).Dec()
			}

			t1 := time.Now()
			time.Sleep(time.Duration(rand.Intn(300)) * time.Millisecond)
			d := time.Since(t1)
			histograms.WithLabelValues(labelValues...).Observe(d.Seconds())
			summaries.WithLabelValues(labelValues...).Observe(d.Seconds())
		}
	}()

	mux := http.NewServeMux()
	mux.Handle("/metrics", promhttp.Handler())

	slog.Info("Listen and serving on 0.0.0.0:7777")

	if err := http.ListenAndServe("0.0.0.0:7777", mux); err != nil {
		slog.Error("Failed to listen and serve server", "error", err)
	}
}
