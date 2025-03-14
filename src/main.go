package main

import (
	_ "embed"
	"fmt"
	"math/rand"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

//go:embed static/index.html
var Index string

var Seed = rand.New(rand.NewSource(time.Now().UnixNano()))

func main() {
	router := chi.NewRouter()
	router.Use(middleware.Logger)
	router.Get("/", index)
	router.Get("/index", index)
	router.Get("/fact", fact)

	fmt.Println("Listening on port 8080")
	http.ListenAndServe(":8080", router)
}

func index(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte(Index))
	return
}

func fact(w http.ResponseWriter, r *http.Request) {
	randomIndex := Seed.Intn(len(Facts))
	fact := Facts[randomIndex]
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(fact))
	return
}
	return
}
