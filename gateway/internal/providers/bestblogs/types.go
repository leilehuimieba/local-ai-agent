package bestblogs

type ReadArticleRequest struct {
	ArticleURL      string `json:"article_url"`
	Language        string `json:"language"`
	IncludeHTML     bool   `json:"include_html"`
	IncludeMarkdown bool   `json:"include_markdown"`
	IncludeImages   bool   `json:"include_images"`
}

type ArticleResponse struct {
	OK        bool           `json:"ok"`
	Provider  string         `json:"provider"`
	Strategy  string         `json:"strategy"`
	ArticleID string         `json:"article_id"`
	Meta      ArticleMeta    `json:"meta"`
	Summary   ArticleSummary `json:"summary"`
	Content   ArticleContent `json:"content"`
}

type ArticleMeta struct {
	Title       string   `json:"title"`
	Author      string   `json:"author"`
	PublishTime string   `json:"publish_time"`
	Tags        []string `json:"tags"`
	SourceURL   string   `json:"source_url,omitempty"`
	SourceName  string   `json:"source_name,omitempty"`
}

type ArticleSummary struct {
	OneSentence string      `json:"one_sentence"`
	Full        string      `json:"full"`
	MainPoints  []MainPoint `json:"main_points"`
	KeyQuotes   []string    `json:"key_quotes"`
}

type MainPoint struct {
	Point       string `json:"point"`
	Explanation string `json:"explanation,omitempty"`
}

type ArticleContent struct {
	HTML     string   `json:"html"`
	Markdown string   `json:"markdown"`
	Images   []string `json:"images"`
}

type upstreamEnvelope struct {
	Success bool         `json:"success"`
	Code    any          `json:"code"`
	Message any          `json:"message"`
	Data    upstreamData `json:"data"`
}

type upstreamData struct {
	MetaData    upstreamMetaData    `json:"metaData"`
	ContentData upstreamContentData `json:"contentData"`
}

type upstreamMetaData struct {
	Title              string              `json:"title"`
	OneSentenceSummary string              `json:"oneSentenceSummary"`
	Summary            string              `json:"summary"`
	Tags               []string            `json:"tags"`
	MainPoints         []upstreamMainPoint `json:"mainPoints"`
	KeyQuotes          []string            `json:"keyQuotes"`
	URL                string              `json:"url"`
	SourceName         string              `json:"sourceName"`
	Authors            []string            `json:"authors"`
	PublishDateTimeStr string              `json:"publishDateTimeStr"`
	Cover              string              `json:"cover"`
}

type upstreamMainPoint struct {
	Point       string `json:"point"`
	Explanation string `json:"explanation"`
}

type upstreamContentData struct {
	DisplayDocument string `json:"displayDocument"`
	UpdateTime      string `json:"updateTime"`
}

// === Feed / List ===

type ListArticlesRequest struct {
	Language string `json:"language"`
	Page     int    `json:"page"`
	PageSize int    `json:"page_size"`
}

type ListArticlesResponse struct {
	OK         bool              `json:"ok"`
	Provider   string            `json:"provider"`
	CurrentPage int              `json:"current_page"`
	PageSize   int              `json:"page_size"`
	TotalCount int              `json:"total_count"`
	PageCount  int              `json:"page_count"`
	Items      []ArticleListItem `json:"items"`
}

type ArticleListItem struct {
	ArticleID          string   `json:"article_id"`
	Title              string   `json:"title"`
	OneSentenceSummary string   `json:"one_sentence_summary"`
	Summary            string   `json:"summary"`
	Tags               []string `json:"tags"`
	URL                string   `json:"url"`
	ReadURL            string   `json:"read_url"`
	Domain             string   `json:"domain"`
	Cover              string   `json:"cover"`
	SourceName         string   `json:"source_name"`
	Authors            []string `json:"authors"`
	PublishTime        string   `json:"publish_time"`
	Category           string   `json:"category"`
	ResourceType       string   `json:"resource_type"`
	WordCount          int      `json:"word_count"`
	ReadCount          int      `json:"read_count"`
}

type upstreamListEnvelope struct {
	Success bool           `json:"success"`
	Data    upstreamListData `json:"data"`
}

type upstreamListData struct {
	CurrentPage int                `json:"currentPage"`
	PageSize    int                `json:"pageSize"`
	TotalCount  int                `json:"totalCount"`
	PageCount   int                `json:"pageCount"`
	DataList    []upstreamListItem `json:"dataList"`
}

type upstreamListItem struct {
	ID                  string              `json:"id"`
	Title               string              `json:"title"`
	OneSentenceSummary  string              `json:"oneSentenceSummary"`
	Summary             string              `json:"summary"`
	Tags                []string            `json:"tags"`
	MainPoints          []upstreamMainPoint `json:"mainPoints"`
	KeyQuotes           []string            `json:"keyQuotes"`
	URL                 string              `json:"url"`
	ReadURL             string              `json:"readUrl"`
	Domain              string              `json:"domain"`
	Cover               string              `json:"cover"`
	SourceName          string              `json:"sourceName"`
	Authors             []string            `json:"authors"`
	PublishDateTimeStr  string              `json:"publishDateTimeStr"`
	CategoryDesc        string              `json:"categoryDesc"`
	ResourceTypeDesc    string              `json:"resourceTypeDesc"`
	WordCount           int                 `json:"wordCount"`
	ReadCount           int                 `json:"readCount"`
}
