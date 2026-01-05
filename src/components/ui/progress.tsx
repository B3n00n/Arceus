import * as React from "react"
import { cn } from "@/lib/cn"

interface ProgressProps extends React.HTMLAttributes<HTMLDivElement> {
  value?: number;
}

const Progress = React.forwardRef<HTMLDivElement, ProgressProps>(
  ({ className, value = 0, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        "relative h-1.5 w-full overflow-hidden rounded-full bg-grey-600",
        className
      )}
      {...props}
    >
      <div
        className="h-full bg-gradient-to-r from-primary-400 to-primary-600 rounded-full transition-all duration-300 ease-out"
        style={{ width: `${Math.min(100, Math.max(0, value))}%` }}
      />
    </div>
  )
)
Progress.displayName = "Progress"

export { Progress }
