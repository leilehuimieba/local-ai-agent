package knowledge

type Item struct {
	ID            string    `json:"id"`
	Title         string    `json:"title"`
	Summary       string    `json:"summary"`
	Content       string    `json:"content"`
	Category      string    `json:"category"`
	Tags          []string  `json:"tags"`
	Metadata      string    `json:"metadata,omitempty"`
	Source        string    `json:"source,omitempty"`
	CitationCount int       `json:"citation_count"`
	Embedding     []float32 `json:"embedding,omitempty"`
	CreatedAt     string    `json:"created_at"`
	UpdatedAt     string    `json:"updated_at"`
}

type ListResponse struct {
	Items      []Item         `json:"items"`
	Categories []string       `json:"categories"`
	CategoryTree []CategoryNode `json:"category_tree,omitempty"`
	Tags       []string       `json:"tags"`
}

type CategoryNode struct {
	Name     string         `json:"name"`
	Path     string         `json:"path"`
	Count    int            `json:"count"`
	Children []CategoryNode `json:"children,omitempty"`
}

type CreateRequest struct {
	Title    string   `json:"title"`
	Summary  string   `json:"summary"`
	Content  string   `json:"content"`
	Category string   `json:"category"`
	Tags     []string `json:"tags"`
	Metadata string   `json:"metadata,omitempty"`
	Source   string   `json:"source,omitempty"`
}

type Chunk struct {
	ID         string    `json:"id"`
	ItemID     string    `json:"item_id"`
	ChunkIndex int       `json:"chunk_index"`
	Content    string    `json:"content"`
	Embedding  []float32 `json:"embedding,omitempty"`
	CreatedAt  string    `json:"created_at"`
}

type UpdateRequest struct {
	Title     string    `json:"title,omitempty"`
	Summary   string    `json:"summary,omitempty"`
	Content   string    `json:"content,omitempty"`
	Category  string    `json:"category,omitempty"`
	Tags      []string  `json:"tags,omitempty"`
	Metadata  string    `json:"metadata,omitempty"`
	Source    string    `json:"source,omitempty"`
	Embedding []float32 `json:"embedding,omitempty"`
}
