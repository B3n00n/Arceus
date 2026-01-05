import { cn } from "@/lib/cn"

interface SegmentOption {
  label: string
  value: string
}

interface SegmentedControlProps {
  options: SegmentOption[]
  value: string
  onChange: (value: string) => void
  className?: string
}

export function SegmentedControl({
  options,
  value,
  onChange,
  className,
}: SegmentedControlProps) {
  const activeIndex = options.findIndex((opt) => opt.value === value)
  const widthPercent = 100 / options.length

  return (
    <div className={cn("relative flex border-b border-discord-dark-2", className)}>
      {options.map((opt) => (
        <button
          key={opt.value}
          className={cn(
            "flex-1 px-4 py-2 text-sm font-medium transition-colors duration-200",
            value === opt.value
              ? "text-white"
              : "text-gray-400 hover:text-white"
          )}
          onClick={() => onChange(opt.value)}
        >
          {opt.label}
        </button>
      ))}

      {/* Animated underline */}
      <div
        className="absolute bottom-0 h-0.5 bg-white transition-all duration-300 ease-out"
        style={{
          width: `${widthPercent}%`,
          left: `${activeIndex * widthPercent}%`,
        }}
      />
    </div>
  )
}
