"use client"

import { useRef, useEffect, forwardRef } from "react"
import { cn } from "@/lib/utils"

interface TextareaAutoProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  maxRows?: number
}

export const TextareaAuto = forwardRef<HTMLTextAreaElement, TextareaAutoProps>(
  ({ maxRows = 6, className, onChange, ...props }, ref) => {
    const innerRef = useRef<HTMLTextAreaElement>(null)

    useEffect(() => {
      const el = innerRef.current
      if (!el) return
      el.style.height = "auto"
      const lineHeight = parseInt(getComputedStyle(el).lineHeight) || 20
      const maxHeight = lineHeight * maxRows
      el.style.height = Math.min(el.scrollHeight, maxHeight) + "px"
    }, [props.value, maxRows])

    return (
      <textarea
        ref={(node) => {
          innerRef.current = node
          if (typeof ref === "function") ref(node)
          else if (ref) (ref as React.MutableRefObject<HTMLTextAreaElement | null>).current = node
        }}
        onChange={(e) => {
          const el = e.target
          el.style.height = "auto"
          const lineHeight = parseInt(getComputedStyle(el).lineHeight) || 20
          const maxHeight = lineHeight * maxRows
          el.style.height = Math.min(el.scrollHeight, maxHeight) + "px"
          onChange?.(e)
        }}
        className={cn(
          "flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background",
          "placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
          "disabled:cursor-not-allowed disabled:opacity-50 resize-none overflow-y-auto",
          className
        )}
        rows={1}
        {...props}
      />
    )
  }
)
TextareaAuto.displayName = "TextareaAuto"
