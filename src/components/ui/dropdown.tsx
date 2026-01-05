import * as React from "react"
import { ChevronDown } from "lucide-react"
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
        className={cn(
          "w-full flex justify-between items-center rounded-md px-3 py-2 text-sm transition-colors",
          "bg-discord-dark-2 border border-discord-dark-2 text-gray-200 hover:border-[#7289da] focus:outline-none focus:ring-2 focus:ring-[#7289da]",
          disabled && "opacity-50 cursor-not-allowed"
        )}
      >
        <span>{value || placeholder}</span>
        <ChevronDown
          className={cn(
            "h-4 w-4 ml-2 transition-transform",
            isOpen && "rotate-180"
          )}
        />
      </button>

      {/* Menu */}
      {isOpen && (
        <div
          className="absolute z-50 mt-2 w-full rounded-md border border-discord-dark-2 bg-discord-dark-3 shadow-lg"
        >
          <ul className="max-h-36 overflow-y-auto space-y-1 p-2 text-sm">
            {options.map((option) => (
              <li key={option}>
                <button
                  className={cn(
                    "w-full rounded-sm text-left px-3 py-2 hover:bg-[#7289da]/20 text-gray-200 hover:text-white",
                    option === value && "bg-[#7289da]/30 text-white"
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
