export async function readErrorText(response: Response): Promise<string> {
  const text = (await response.text()).trim();
  return text || `HTTP ${response.status}`;
}
