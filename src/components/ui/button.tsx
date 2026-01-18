import * as React from "react"
import { cn } from "@/lib/cn"

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "outline" | "outline_yellow" | "secondary" | "ghost" | "link" | "danger" | "danger-outline"
  size?: "default" | "sm" | "lg" | "icon"
}

const buttonVariants = {
  default: "bg-primary-default text-grey-900 uppercase shadow hover:bg-primary-400 hover:shadow-primary-glow active:bg-primary-300 active:sshadow-primary-glow",
  outline: "border-1 border-grey-600 bg-transparent text-grey-200 font-medium hover:bg-grey-700 hover:border-grey-200 hover:text-white active:bg-grey-600",
  outline_yellow: "border-1 border-primary-default bg-transparent text-primary-default uppercase hover:bg-primary-800 hover:text-primary-400  hover:border-primary-400 active:bg-primary-700 disabled:opacity-50",
  secondary: "border-1 border-grey-500 bg-grey-800 text-grey-100 uppercase shadow-sm hover:bg-grey-700 hover:text-white active:bg-grey-600",
  ghost: "hover:bg-accent hover:text-accent-foreground active:bg-accent/70",
  link: "text-primary-default underline-offset-4 hover:underline",
  danger:
    "bg-error-default text-white shadow uppercase hover:bg-error-600 active:bg-error-700",
  "danger-outline":
    "border-1 border-error-default text-error-default uppercase bg-transparent" +
    "hover:border-error-400 hover:text-error-400 hover:bg-error-900 active:bg-error-800 transition-colors",
}

const buttonSizes = {
  default: "h-9 px-4 py-2",
  sm: "h-9 py-2 rounded-md px-3 text-xs",
  lg: "h-10 rounded-md px-8",
  icon: "h-9 w-9",
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "default", size = "default", ...props }, ref) => {
    return (
      <button
        className={cn(
          "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-bold cursor-pointer",
          "transition-all duration-150 ease-in-out",
          "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#7289da] focus-visible:ring-offset-2 focus-visible:ring-offset-[#1e2124]",
          "disabled:pointer-events-none disabled:opacity-50",
          buttonVariants[variant],
          buttonSizes[size],
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Button.displayName = "Button"

export { Button }
