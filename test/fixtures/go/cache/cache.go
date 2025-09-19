package cache

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/redis/go-redis/v9"

	"github.com/giftig/hellgo/models"
)

var ctx = context.Background()

type Cache struct {
	client *redis.Client
}

func NewCache(addr string) Cache {
	return Cache{
		client: redis.NewClient(&redis.Options{
			Addr:     addr,
			Password: "",
			DB:       0,
		}),
	}
}

// Store lemming keyed by name
func (c Cache) StoreLemming(lemming models.Lemming) (err error) {
	data, err := json.Marshal(lemming)
	if err != nil {
		return
	}

	err = c.client.Set(ctx, fmt.Sprintf("lemming:%s", lemming.Name), string(data), 0).Err()
	return
}

func (c Cache) GetLemming(name string) (lemming models.Lemming, err error) {
	data, err := c.client.Get(ctx, fmt.Sprintf("lemming:%s", name)).Result()

	if err != nil {
		return
	}

	err = json.Unmarshal([]byte(data), &lemming)
	return
}
