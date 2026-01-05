import * as React from 'react';
import { Check } from 'lucide-react';
import { cn } from '@/lib/cn';

interface CheckboxProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  checked?: boolean;
  onCheckedChange?: (checked: boolean) => void;
  disabled?: boolean;
}

export const Checkbox = React.forwardRef<HTMLButtonElement, CheckboxProps>(
  ({ checked = false, onCheckedChange, className, disabled, ...props }, ref) => {
    const toggle = () => {
      if (disabled) return;
      onCheckedChange?.(!checked);
    };

    return (
      <button
        ref={ref}
        type="button"
        role="checkbox"
        aria-checked={checked}
        aria-disabled={disabled}
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();         
          toggle();
        }}
        onKeyDown={(e) => {
          if (e.key === ' ' || e.key === 'Enter') {
            e.preventDefault();
            e.stopPropagation();    
            toggle();
          }
        }}
        className={cn(
          'h-4 w-4 rounded-sm cursor-pointer flex items-center justify-center transition-all',
          'outline outline-2 outline-offset-[-2px]',
          checked
            ? 'bg-primary-default outline-primary-default shadow-primary-glow hover:outline-primary-400 hover:bg-primary-400'
            : 'bg-transparent outline-grey-300 hover:outline-primary-default hover:bg-primary-900',
          disabled && 'opacity-50 cursor-not-allowed',
          className
        )}
        {...props}
      >
        {checked && <Check className="h-3 w-3 text-black" strokeWidth={3} />}
      </button>
    );
  }
);

Checkbox.displayName = 'Checkbox';