package runtime

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"time"

	"local-agent/gateway/internal/contracts"
)

type Client struct {
	baseURL    string
	httpClient *http.Client
}

func NewClient(port int) *Client {
	return &Client{
		baseURL: fmt.Sprintf("http://127.0.0.1:%d", port),
		httpClient: &http.Client{
			Timeout: 120 * time.Second,
		},
	}
}

func (c *Client) Run(ctx context.Context, request contracts.RunRequest) (contracts.RuntimeRunResponse, error) {
	body, err := json.Marshal(request)
	if err != nil {
		return contracts.RuntimeRunResponse{}, fmt.Errorf("marshal run request: %w", err)
	}

	httpReq, err := http.NewRequestWithContext(ctx, http.MethodPost, c.baseURL+"/v1/runtime/run", bytes.NewReader(body))
	if err != nil {
		return contracts.RuntimeRunResponse{}, fmt.Errorf("create runtime request: %w", err)
	}
	httpReq.Header.Set("Content-Type", "application/json")

	return c.doRunRequest(httpReq)
}

func (c *Client) doRunRequest(httpReq *http.Request) (contracts.RuntimeRunResponse, error) {
	resp, err := c.httpClient.Do(httpReq)
	if err != nil {
		return contracts.RuntimeRunResponse{}, fmt.Errorf("call runtime: %w", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return contracts.RuntimeRunResponse{}, fmt.Errorf("runtime returned %s", resp.Status)
	}
	var payload contracts.RuntimeRunResponse
	if err := json.NewDecoder(resp.Body).Decode(&payload); err != nil {
		return contracts.RuntimeRunResponse{}, fmt.Errorf("decode runtime response: %w", err)
	}
	return payload, nil
}

func (c *Client) Capabilities(ctx context.Context, mode string) (contracts.CapabilityListResponse, error) {
	path := "/v1/runtime/capabilities"
	if mode != "" {
		path += "?mode=" + url.QueryEscape(mode)
	}
	return getJSON[contracts.CapabilityListResponse](ctx, c.httpClient, c.baseURL+path)
}

func (c *Client) Connectors(ctx context.Context) (contracts.ConnectorListResponse, error) {
	return getJSON[contracts.ConnectorListResponse](ctx, c.httpClient, c.baseURL+"/v1/runtime/connectors")
}

func getJSON[T any](ctx context.Context, client *http.Client, target string) (T, error) {
	var payload T
	httpReq, err := http.NewRequestWithContext(ctx, http.MethodGet, target, nil)
	if err != nil {
		return payload, fmt.Errorf("create runtime request: %w", err)
	}
	resp, err := client.Do(httpReq)
	if err != nil {
		return payload, fmt.Errorf("call runtime: %w", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return payload, fmt.Errorf("runtime returned %s", resp.Status)
	}
	err = json.NewDecoder(resp.Body).Decode(&payload)
	return payload, err
}
