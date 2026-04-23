package knowledge

type Item struct {
	ID            string    `json:"id"`
	Title         string    `json:"title"`
	Summary       string    `json:"summary"`
	Content       string    `json:"content"`
	Category      string    `json:"category"`
	Tags          []string  `json:"tags"`
	Source        string    `json:"source,omitempty"`
	CitationCount int       `json:"citation_count"`
	Embedding     []float32 `json:"embedding,omitempty"`
	CreatedAt     string    `json:"created_at"`
	UpdatedAt     string    `json:"updated_at"`
}

type ListResponse struct {
	Items      []Item   `json:"items"`
	Categories []string `json:"categories"`
	Tags       []string `json:"tags"`
}

type CreateRequest struct {
	Title    string   `json:"title"`
	Summary  string   `json:"summary"`
	Content  string   `json:"content"`
	Category string   `json:"category"`
	Tags     []string `json:"tags"`
	Source   string   `json:"source,omitempty"`
}

type UpdateRequest struct {
	Title     string    `json:"title,omitempty"`
	Summary   string    `json:"summary,omitempty"`
	Content   string    `json:"content,omitempty"`
	Category  string    `json:"category,omitempty"`
	Tags      []string  `json:"tags,omitempty"`
	Source    string    `json:"source,omitempty"`
	Embedding []float32 `json:"embedding,omitempty"`
}
