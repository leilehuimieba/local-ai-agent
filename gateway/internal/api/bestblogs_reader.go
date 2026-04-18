package api

import (
	"context"

	"local-agent/gateway/internal/providers/bestblogs"
)

var bestblogsArticleReader = func(
	ctx context.Context,
	req bestblogs.ReadArticleRequest,
) (bestblogs.ArticleResponse, error) {
	return bestblogs.NewClient(nil).ReadArticle(ctx, req)
}

func swapBestblogsArticleReader(
	reader func(context.Context, bestblogs.ReadArticleRequest) (bestblogs.ArticleResponse, error),
) func() {
	previous := bestblogsArticleReader
	bestblogsArticleReader = reader
	return func() {
		bestblogsArticleReader = previous
	}
}
