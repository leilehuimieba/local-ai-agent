package api

import (
	"context"
	"errors"
	"net/http"
	"time"

	"local-agent/gateway/internal/providers/bestblogs"
)

func bestblogsArticleReadHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		payload, ok := decodeBestblogsReadRequest(w, r)
		if !ok {
			return
		}
		ctx, cancel := context.WithTimeout(r.Context(), 20*time.Second)
		defer cancel()
		result, err := bestblogsArticleReader(ctx, payload)
		if err != nil {
			writeBestblogsError(w, err)
			return
		}
		writeJSON(w, http.StatusOK, result)
	}
}

func decodeBestblogsReadRequest(
	w http.ResponseWriter,
	r *http.Request,
) (bestblogs.ReadArticleRequest, bool) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return bestblogs.ReadArticleRequest{}, false
	}
	var payload bestblogs.ReadArticleRequest
	if !decodeJSONBody(w, r, &payload) {
		http.Error(w, "invalid json body", http.StatusBadRequest)
		return bestblogs.ReadArticleRequest{}, false
	}
	return payload, true
}

func writeBestblogsError(w http.ResponseWriter, err error) {
	var providerErr bestblogs.Error
	if errors.As(err, &providerErr) {
		writeJSON(w, providerErr.Status, map[string]any{
			"ok": false, "error_code": providerErr.Code, "message": providerErr.Message,
		})
		return
	}
	http.Error(w, err.Error(), http.StatusInternalServerError)
}
