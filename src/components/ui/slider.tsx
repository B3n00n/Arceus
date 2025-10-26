import * as React from "react"
import { cn } from "@/lib/cn"

export interface SliderProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'type'> {
  value?: number
  onValueChange?: (value: number) => void
}

const Slider = React.forwardRef<HTMLInputElement, SliderProps>(
  ({ className, value = 0, onValueChange, onChange, ...props }, ref) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = parseInt(e.target.value)
      onValueChange?.(newValue)
      onChange?.(e)
    }

    return (
      <input
        type="range"
        className={cn(
          "w-full h-2 bg-gray-600 rounded-lg appearance-none cursor-pointer",
          "accent-discord-blurple",
          "[&::-webkit-slider-track]:bg-gray-600",
          "[&::-webkit-slider-track]:rounded-lg",
          "[&::-webkit-slider-thumb]:appearance-none",
          "[&::-webkit-slider-thumb]:w-5",
          "[&::-webkit-slider-thumb]:h-5",
          "[&::-webkit-slider-thumb]:rounded-full",
          "[&::-webkit-slider-thumb]:bg-white",
          "[&::-webkit-slider-thumb]:cursor-pointer",
          "[&::-webkit-slider-thumb]:hover:bg-gray-200",
          "[&::-webkit-slider-thumb]:transition-colors",
          "[&::-webkit-slider-thumb]:shadow-lg",
          "[&::-moz-range-track]:bg-gray-600",
          "[&::-moz-range-track]:rounded-lg",
          "[&::-moz-range-thumb]:w-5",
          "[&::-moz-range-thumb]:h-5",
          "[&::-moz-range-thumb]:rounded-full",
          "[&::-moz-range-thumb]:bg-white",
          "[&::-moz-range-thumb]:cursor-pointer",
          "[&::-moz-range-thumb]:hover:bg-gray-200",
          "[&::-moz-range-thumb]:transition-colors",
          "[&::-moz-range-thumb]:border-0",
          "[&::-moz-range-thumb]:shadow-lg",
          className
        )}
        value={value}
        onChange={handleChange}
        ref={ref}
        {...props}
      />
    )
  }
)
Slider.displayName = "Slider"

export { Slider }
