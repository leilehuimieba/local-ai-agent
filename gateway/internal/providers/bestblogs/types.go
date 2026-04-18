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
