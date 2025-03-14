package main

import (
	_ "embed"
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

//go:embed static/index.html
var Index string
func main() {
	router := chi.NewRouter()
	router.Use(middleware.Logger)
	router.Get("/", index)
	router.Get("/index", index)

	fmt.Println("Listening on port 8080")
	http.ListenAndServe(":8080", router)
}

func index(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte(Index))
	return
}
	return
}
