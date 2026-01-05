import * as React from "react"
import { cn } from "@/lib/cn"

export interface SliderProps
  extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "type"> {
  value?: number
  onValueChange?: (value: number) => void
}

const Slider = React.forwardRef<HTMLInputElement, SliderProps>(
  ({ className, value = 0, onValueChange, onChange, max = 100, ...props }, ref) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = parseInt(e.target.value)
      onValueChange?.(newValue)
      onChange?.(e)
    }

    const fillPercent = Math.min(Math.max((Number(value) / Number(max)) * 100, 0), 100)

    return (
      <input
        type="range"
        value={value}
        max={max}
        onChange={handleChange}
        ref={ref}
        {...props}
        className={cn(
          // Base track
          "w-full h-1 appearance-none cursor-pointer rounded-full",
          "bg-grey-600",
          // Thumb (Chrome / Edge / Safari)
          "[&::-webkit-slider-thumb]:appearance-none",
          "[&::-webkit-slider-thumb]:h-3",
          "[&::-webkit-slider-thumb]:w-3",
          "[&::-webkit-slider-thumb]:rounded-full",
          "[&::-webkit-slider-thumb]:bg-white",
          "[&::-webkit-slider-thumb]:shadow",
          "[&::-webkit-slider-thumb]:cursor-pointer",
          "[&::-webkit-slider-thumb]:transition-all",
          // Thumb (Firefox)
          "[&::-moz-range-thumb]:h-3",
          "[&::-moz-range-thumb]:w-3",
          "[&::-moz-range-thumb]:rounded-full",
          "[&::-moz-range-thumb]:bg-white",
          "[&::-moz-range-thumb]:border-0",
          "[&::-moz-range-thumb]:shadow",
          "[&::-moz-range-thumb]:cursor-pointer",
          "[&::-moz-range-thumb]:transition-all",
          className
        )}
        style={{
          background: `linear-gradient(to right, var(--color-primary-400) 0%, var(--color-primary-600) ${fillPercent}%, #3b3b3b ${fillPercent}%)`,
          transition: "background 0.15s ease-out",
        }}
      />
    )
  }
)

Slider.displayName = "Slider"
export { Slider }
