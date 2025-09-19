package cache

import (
	"testing"

	"github.com/giftig/hellgo/models"
	"github.com/stretchr/testify/assert"
)

// Should be able to write and reread a lemming from the db
func TestCache(t *testing.T) {
	lem := models.NewLemming("Hodor", models.Happy)
	c := NewCache("localhost:6379")

	err := c.StoreLemming(lem)
	if !assert.Nil(t, err, "should store lemming ok") {
		return
	}

	result, err := c.GetLemming(lem.Name)

	if !assert.Nil(t, err, "should retrieve lemming ok") {
		return
	}

	assert.Equal(t, result, lem, "Lemming retrieved from cache should match lemming stored")
}
