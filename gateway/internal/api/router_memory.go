package api

import "net/http"

func registerMemoryRoutes(mux *http.ServeMux, deps memoryRouteDeps) {
	mux.HandleFunc("/api/v1/memories", deps.handleMemories)
	mux.HandleFunc("/api/v1/memories/delete", deps.handleMemoryDelete)
}
