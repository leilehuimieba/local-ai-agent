import '@testing-library/jest-dom'
import { vi } from 'vitest'

Object.defineProperty(global, "ResizeObserver", {
  value: vi.fn(function () {
    return { observe: vi.fn(), unobserve: vi.fn(), disconnect: vi.fn() }
  }),
  writable: true,
})
