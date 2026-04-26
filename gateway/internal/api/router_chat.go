package api

import "net/http"

func registerChatRoutes(mux *http.ServeMux, chat *ChatHandler) {
	mux.HandleFunc("/api/v1/chat/run", chat.Run)
	mux.HandleFunc("/api/v1/chat/retry", chat.Retry)
	mux.HandleFunc("/api/v1/chat/confirm", chat.Confirm)
	mux.HandleFunc("/api/v1/chat/cancel", chat.Cancel)
	mux.HandleFunc("/api/v1/events/stream", chat.Stream)
}
