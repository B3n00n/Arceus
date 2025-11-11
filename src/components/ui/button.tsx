import * as React from "react"
import { cn } from "@/lib/cn"

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link"
  size?: "default" | "sm" | "lg" | "icon"
}

const buttonVariants = {
  default: "bg-discord-blurple text-white shadow hover:bg-discord-blurple/80 active:bg-discord-blurple/70",
  destructive: "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/80 active:bg-destructive/70",
  outline: "border-2 border-gray-600/50 bg-transparent text-gray-300 hover:bg-[#7289da]/20 hover:border-[#7289da] hover:text-white active:bg-[#7289da]/30",
  secondary: "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/70 active:bg-secondary/60",
  ghost: "hover:bg-accent hover:text-accent-foreground active:bg-accent/70",
  link: "text-discord-blurple underline-offset-4 hover:underline",
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
          "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium cursor-pointer",
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
