package bestblogs

import "fmt"

const (
	ErrInvalidInput       = "BESTBLOGS_INVALID_INPUT"
	ErrUpstreamHTTP       = "BESTBLOGS_UPSTREAM_HTTP_ERROR"
	ErrUpstreamNotSuccess = "BESTBLOGS_UPSTREAM_NOT_SUCCESS"
	ErrEmptyContent       = "BESTBLOGS_EMPTY_CONTENT"
	ErrDecode             = "BESTBLOGS_DECODE_ERROR"
)

type Error struct {
	Code    string `json:"code"`
	Message string `json:"message"`
	Status  int    `json:"status"`
}

func (e Error) Error() string {
	return e.Message
}

func newInvalidInputError(message string) error {
	return Error{Code: ErrInvalidInput, Message: message, Status: 400}
}

func newUpstreamHTTPError(status int, err error) error {
	message := fmt.Sprintf("BestBlogs 上游请求失败: %d", status)
	if err != nil {
		message = fmt.Sprintf("%s: %v", message, err)
	}
	return Error{Code: ErrUpstreamHTTP, Message: message, Status: 502}
}

func newUpstreamNotSuccessError() error {
	return Error{Code: ErrUpstreamNotSuccess, Message: "BestBlogs 上游返回 success=false", Status: 502}
}

func newEmptyContentError() error {
	return Error{Code: ErrEmptyContent, Message: "BestBlogs 正文为空", Status: 502}
}

func newDecodeError(err error) error {
	return Error{Code: ErrDecode, Message: fmt.Sprintf("BestBlogs 解码失败: %v", err), Status: 502}
}
