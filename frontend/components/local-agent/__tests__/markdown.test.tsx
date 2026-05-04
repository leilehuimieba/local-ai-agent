import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { LightweightMarkdown } from '../markdown'

describe('LightweightMarkdown', () => {
  it('renders plain paragraph', () => {
    render(<LightweightMarkdown content="Hello world" />)
    expect(screen.getByText('Hello world')).toBeInTheDocument()
  })

  it('renders heading', () => {
    render(<LightweightMarkdown content="# Title" />)
    expect(screen.getByText('Title')).toBeInTheDocument()
  })

  it('renders bold text', () => {
    render(<LightweightMarkdown content="**bold**" />)
    expect(screen.getByText('bold')).toBeInTheDocument()
  })

  it('renders code block', () => {
    const code = '```python\nprint(1)\n```'
    render(<LightweightMarkdown content={code} />)
    expect(screen.getByText('复制')).toBeInTheDocument()
    // Code text is inside <code> element
    expect(document.querySelector('code')?.textContent).toBe('print(1)')
  })

  it('renders list items', () => {
    const list = '- item 1\n- item 2'
    render(<LightweightMarkdown content={list} />)
    const items = screen.getAllByText(/item/)
    expect(items.length).toBeGreaterThanOrEqual(2)
  })

  it('renders link', () => {
    render(<LightweightMarkdown content="[link](https://example.com)" />)
    expect(screen.getByText('link')).toHaveAttribute('href', 'https://example.com')
  })

  it('highlights search text with mark', () => {
    render(<LightweightMarkdown content="hello world" highlightText="world" />)
    const mark = document.querySelector('mark')
    expect(mark?.textContent).toBe('world')
  })
})
