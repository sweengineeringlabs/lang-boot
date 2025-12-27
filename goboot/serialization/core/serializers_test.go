package core

import (
	"testing"
)

func TestJSONSerializer(t *testing.T) {
	s := NewJSONSerializer()

	type User struct {
		Name  string `json:"name"`
		Email string `json:"email"`
	}

	t.Run("Marshal", func(t *testing.T) {
		user := User{Name: "John", Email: "john@example.com"}
		data, err := s.Marshal(user)
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if len(data) == 0 {
			t.Error("Expected non-empty data")
		}
	})

	t.Run("Unmarshal", func(t *testing.T) {
		data := []byte(`{"name":"Jane","email":"jane@example.com"}`)
		var user User
		err := s.Unmarshal(data, &user)
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if user.Name != "Jane" {
			t.Errorf("Expected Jane, got %s", user.Name)
		}
	})

	t.Run("MarshalIndent", func(t *testing.T) {
		user := User{Name: "John", Email: "john@example.com"}
		data, err := s.MarshalIndent(user, "", "  ")
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if len(data) == 0 {
			t.Error("Expected non-empty data")
		}
	})
}

func TestXMLSerializer(t *testing.T) {
	s := NewXMLSerializer()

	type Item struct {
		Name  string `xml:"name"`
		Value int    `xml:"value"`
	}

	t.Run("Marshal", func(t *testing.T) {
		item := Item{Name: "test", Value: 42}
		data, err := s.Marshal(item)
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if len(data) == 0 {
			t.Error("Expected non-empty data")
		}
	})

	t.Run("Unmarshal", func(t *testing.T) {
		data := []byte(`<Item><name>test</name><value>42</value></Item>`)
		var item Item
		err := s.Unmarshal(data, &item)
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if item.Name != "test" {
			t.Errorf("Expected 'test', got %s", item.Name)
		}
	})
}

func TestRegistry(t *testing.T) {
	registry := NewRegistry()

	type Data struct {
		Value string `json:"value" xml:"value"`
	}

	t.Run("JSON", func(t *testing.T) {
		data := Data{Value: "test"}
		bytes, err := registry.Marshal("json", data)
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		
		var result Data
		err = registry.Unmarshal("json", bytes, &result)
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if result.Value != "test" {
			t.Error("Round-trip failed")
		}
	})

	t.Run("UnknownFormat", func(t *testing.T) {
		_, err := registry.Get("unknown")
		if err == nil {
			t.Error("Expected error for unknown format")
		}
	})
}
