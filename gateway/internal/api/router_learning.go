package api

import "net/http"

func registerLearningRoutes(mux *http.ServeMux, deps memoryRouteDeps) {
	mux.HandleFunc("/api/v1/learning/extract", learningExtractHandler())
	mux.HandleFunc("/api/v1/learning/explain", learningExplainHandler())
	mux.HandleFunc("/api/v1/learning/translate", learningTranslateHandler())
	mux.HandleFunc("/api/v1/learning/value-score", learningValueScoreHandler())
	mux.HandleFunc("/api/v1/learning/recommend", learningRecommendHandler())
	mux.HandleFunc("/api/v1/learning/memory/write", learningMemoryWriteHandler(deps))
	mux.HandleFunc("/api/v1/learning/audit-trace", learningAuditTraceHandler(deps))
	mux.HandleFunc("/api/v1/learning/rollback-check", learningRollbackHandler())
}
