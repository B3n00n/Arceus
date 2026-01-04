import * as React from "react"
import { ChevronDown, ChevronUp } from "lucide-react"
import { cn } from "@/lib/cn"

interface DropdownProps {
  options: string[]
  value?: string
  onChange: (value: string) => void
  placeholder?: string
  disabled?: boolean
  className?: string
}

export function Dropdown({
  options,
  value,
  onChange,
  placeholder = "Select an option",
  disabled = false,
  className,
}: DropdownProps) {
  const [isOpen, setIsOpen] = React.useState(false)

  const toggleOpen = () => {
    if (!disabled) setIsOpen((prev) => !prev)
  }

  const handleSelect = (option: string) => {
    onChange(option)
    setIsOpen(false)
  }

  React.useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (!(e.target as HTMLElement).closest(".dropdown-container")) {
        setIsOpen(false)
      }
    }
    document.addEventListener("mousedown", handleClickOutside)
    return () => document.removeEventListener("mousedown", handleClickOutside)
  }, [])

  return (
    <div
      className={cn("relative dropdown-container", className)}
    >
      {/* Trigger */}
      <button
        type="button"
        onClick={toggleOpen}
        disabled={disabled}
        data-state={isOpen ? "active" : "default"}
        className={cn(
           "w-full flex justify-between items-center rounded-md px-3 py-2 text-sm transition-colors",
          "bg-grey-900 outline outline-grey-600 text-grey-200 hover:outline-primary-default focus:outline-none focus:ring-2 focus:ring-primary-default",
          disabled && "opacity-50 cursor-not-allowed"
        )}
      >
        <span className="text-white">
          {value || placeholder}
        </span>
        {isOpen ? (
          <ChevronUp className="w-4 h-4 text-white" />
        ) : (
          <ChevronDown className="w-4 h-4 text-grey-200" />
        )}
      </button>

      {/* Menu */}
      {isOpen && (
        <div
          className="absolute z-50 mt-1 w-full rounded-md border border-grey-500 bg-grey-800 shadow-lg"
        >
          <ul className="max-h-36 overflow-y-auto space-y-0 py-2 text-sm">
            {options.map((option) => (
              <li key={option}>
                <button
                  className={cn(
                    "w-full text-left px-3 py-2 hover:bg-grey-700 text-grey-200 hover:text-white",
                    option === value && "bg-primary-800 text-white hover:bg-primary-700"
                  )}
                  onClick={() => handleSelect(option)}
                >
                  {option}
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  )
}
