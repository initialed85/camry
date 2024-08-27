package scratch

import (
	"context"
	"log"
	"sync"
	"testing"

	"github.com/initialed85/camry/internal"
	"github.com/initialed85/djangolang/pkg/helpers"
	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
	"github.com/stretchr/testify/require"
)

func TestScratch(t *testing.T) {
	desiredCount := 1000

	startWg := new(sync.WaitGroup)
	startWg.Add(2)

	stopWg := new(sync.WaitGroup)
	stopWg.Add(2)

	consume := func(i int, cons jetstream.Consumer) {
		startWg.Done()
		defer stopWg.Done()

		actualCount := 0
		for actualCount < desiredCount/2 {
			msg, err := cons.Next()
			require.NoError(t, err)

			actualCount++

			log.Printf("cons%d msg: %#+v", i, string(msg.Data()))
		}

		log.Printf("cons%d done", i)
	}

	natsUrl, err := internal.GetEnvironment("NATS_URL", false, helpers.Ptr(nats.DefaultURL), false)
	require.NoError(t, err)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	nc, err := nats.Connect(natsUrl)
	require.NoError(t, err)

	defer func() {
		_ = nc.Drain()
		nc.Close()
	}()

	js, err := jetstream.New(nc)
	require.NoError(t, err)

	cfg := jetstream.StreamConfig{
		Name:      "EVENTS",
		Retention: jetstream.WorkQueuePolicy,
		Subjects:  []string{"events.>"},
	}

	stream, err := js.CreateStream(ctx, cfg)
	require.NoError(t, err)

	cons1, err := stream.CreateOrUpdateConsumer(ctx, jetstream.ConsumerConfig{
		Name: "event-consumer",
	})
	require.NoError(t, err)
	go consume(1, cons1)

	cons2, err := stream.CreateOrUpdateConsumer(ctx, jetstream.ConsumerConfig{
		Name: "event-consumer",
	})
	require.NoError(t, err)
	go consume(2, cons2)

	startWg.Wait()

	for i := 0; i < 1024; i++ {
		_, err = js.Publish(ctx, "events.ping", []byte("Hello, world."))
		require.NoError(t, err)
	}

	stopWg.Wait()
}
