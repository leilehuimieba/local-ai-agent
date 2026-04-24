package knowledge

import (
	"encoding/json"
	"net/http"
	"path/filepath"
	"strings"

	"local-agent/gateway/internal/config"
	"local-agent/gateway/internal/state"
)

type Handler struct {
	store         *Store
	repoRoot      string
	cfg           config.AppConfig
	settingsStore *state.SettingsStore
}

func NewHandler(repoRoot string) *Handler {
	return &Handler{store: NewStore(repoRoot), repoRoot: repoRoot}
}

func (h *Handler) RegisterRoutes(mux *http.ServeMux, settingsStore *state.SettingsStore, repoRoot string, cfg config.AppConfig) {
	h.repoRoot = repoRoot
	h.cfg = cfg
	h.settingsStore = settingsStore
	mux.HandleFunc("/api/v1/knowledge/items", func(w http.ResponseWriter, r *http.Request) {
		workspaceID, ok := currentWorkspaceID(settingsStore)
		if !ok {
			http.Error(w, "workspace not found", http.StatusNotFound)
			return
		}
		switch r.Method {
		case http.MethodGet:
			h.handleList(w, r, workspaceID)
		case http.MethodPost:
			h.handleCreate(w, r, workspaceID)
		default:
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		}
	})
	mux.HandleFunc("/api/v1/knowledge/items/", func(w http.ResponseWriter, r *http.Request) {
		workspaceID, ok := currentWorkspaceID(settingsStore)
		if !ok {
			http.Error(w, "workspace not found", http.StatusNotFound)
			return
		}
		id := strings.TrimPrefix(r.URL.Path, "/api/v1/knowledge/items/")
		if id == "" {
			http.Error(w, "id required", http.StatusBadRequest)
			return
		}
		switch r.Method {
		case http.MethodGet:
			h.handleGet(w, r, workspaceID, id)
		case http.MethodPut:
			h.handleUpdate(w, r, workspaceID, id)
		case http.MethodDelete:
			h.handleDelete(w, r, workspaceID, id)
		default:
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		}
	})
	mux.HandleFunc("/api/v1/knowledge/upload", func(w http.ResponseWriter, r *http.Request) {
		workspaceID, ok := currentWorkspaceID(settingsStore)
		if !ok {
			http.Error(w, "workspace not found", http.StatusNotFound)
			return
		}
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		h.handleUpload(w, r, workspaceID)
	})
	mux.HandleFunc("/api/v1/knowledge/ask", func(w http.ResponseWriter, r *http.Request) {
		workspaceID, ok := currentWorkspaceID(settingsStore)
		if !ok {
			http.Error(w, "workspace not found", http.StatusNotFound)
			return
		}
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		h.handleAsk(w, r, workspaceID, cfg, settingsStore)
	})
	mux.HandleFunc("/api/v1/knowledge/search", func(w http.ResponseWriter, r *http.Request) {
		workspaceID, ok := currentWorkspaceID(settingsStore)
		if !ok {
			http.Error(w, "workspace not found", http.StatusNotFound)
			return
		}
		if r.Method != http.MethodGet {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		h.handleSearch(w, r, workspaceID)
	})
}

func (h *Handler) handleList(w http.ResponseWriter, _ *http.Request, workspaceID string) {
	items, err := h.store.List(workspaceID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	cats, _ := h.store.Categories(workspaceID)
	tags, _ := h.store.Tags(workspaceID)
	writeJSON(w, http.StatusOK, ListResponse{Items: items, Categories: cats, Tags: tags})
}

func (h *Handler) handleGet(w http.ResponseWriter, _ *http.Request, workspaceID string, id string) {
	item, err := h.store.Get(workspaceID, id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}
	writeJSON(w, http.StatusOK, item)
}

func (h *Handler) handleCreate(w http.ResponseWriter, r *http.Request, workspaceID string) {
	var req CreateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid json", http.StatusBadRequest)
		return
	}
	if strings.TrimSpace(req.Title) == "" {
		http.Error(w, "title is required", http.StatusBadRequest)
		return
	}
	item, err := h.store.Create(workspaceID, req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	go h.generateEmbedding(workspaceID, item.ID)
	writeJSON(w, http.StatusCreated, item)
}

func (h *Handler) handleUpdate(w http.ResponseWriter, r *http.Request, workspaceID string, id string) {
	var req UpdateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid json", http.StatusBadRequest)
		return
	}
	item, err := h.store.Update(workspaceID, id, req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}
	go h.generateEmbedding(workspaceID, item.ID)
	writeJSON(w, http.StatusOK, item)
}

func (h *Handler) handleDelete(w http.ResponseWriter, _ *http.Request, workspaceID string, id string) {
	if err := h.store.Delete(workspaceID, id); err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}
	writeJSON(w, http.StatusOK, map[string]bool{"ok": true})
}

func (h *Handler) handleSearch(w http.ResponseWriter, r *http.Request, workspaceID string) {
	query := r.URL.Query().Get("q")
	items, err := h.store.Search(workspaceID, query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	writeJSON(w, http.StatusOK, ListResponse{Items: items})
}

func (h *Handler) handleUpload(w http.ResponseWriter, r *http.Request, workspaceID string) {
	file, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, "file required", http.StatusBadRequest)
		return
	}
	defer file.Close()

	uploadDir := filepath.Join(h.repoRoot, "data", "knowledge_base", "uploads", safeName(workspaceID))
	savedPath := filepath.Join(uploadDir, safeName(header.Filename))
	if err := SaveUploadedFile(file, savedPath); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	extracted := ExtractText(savedPath)
	if extracted.Error != nil {
		http.Error(w, extracted.Error.Error(), http.StatusUnprocessableEntity)
		return
	}

	summary := extracted.Content
	if len(summary) > 200 {
		summary = summary[:200] + "..."
	}
	item, err := h.store.Create(workspaceID, CreateRequest{
		Title:    extracted.Title,
		Summary:  summary,
		Content:  extracted.Content,
		Category: "文档",
		Tags:     []string{"上传"},
		Source:   header.Filename,
	})
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	go h.generateEmbedding(workspaceID, item.ID)
	writeJSON(w, http.StatusCreated, item)
}

func (h *Handler) generateEmbedding(workspaceID, itemID string) {
	if h.settingsStore == nil {
		return
	}
	_, currentModel, _, _, _, _, _, _ := h.settingsStore.Snapshot()
	provider := findProvider(h.cfg, currentModel.ProviderID)
	item, err := h.store.Get(workspaceID, itemID)
	if err != nil {
		return
	}
	text := item.Title + "\n" + item.Summary + "\n" + item.Content
	embed, err := GetEmbedding(text, provider, currentModel.ModelID)
	if err != nil {
		return
	}
	_, _ = h.store.Update(workspaceID, itemID, UpdateRequest{Embedding: embed})
}

func currentWorkspaceID(store *state.SettingsStore) (string, bool) {
	_, _, _, workspace, _, _, _, _ := store.Snapshot()
	if workspace.WorkspaceID == "" {
		return "", false
	}
	return workspace.WorkspaceID, true
}

func writeJSON(w http.ResponseWriter, status int, payload any) {
	w.Header().Set("Content-Type", "application/json; charset=utf-8")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(payload)
}
