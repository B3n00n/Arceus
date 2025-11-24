import * as React from 'react';
import { cn } from '@/lib/cn';

interface RadioProps {
  checked?: boolean;
  onChange?: (checked: boolean) => void;
  disabled?: boolean;
  className?: string;
}

export function Radio({
  checked = false,
  onChange,
  disabled = false,
  className,
}: RadioProps) {
  const handleClick = () => {
    if (!disabled && onChange) {
      onChange(!checked);
    }
  };

  return (
    <button
      type="button"
      role="radio"
      aria-checked={checked}
      disabled={disabled}
      onClick={handleClick}
      className={cn(
        'relative h-5 w-5 rounded-full border-2 transition-all duration-200',
        'focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2',
        checked
          ? 'border-white'
          : 'border-gray-400 bg-transparent hover:border-gray-300',
        disabled && 'cursor-not-allowed opacity-50',
        !disabled && 'cursor-pointer',
        className
      )}
    >
      {/* Inner circle with scale animation */}
      <span
        className={cn(
          'absolute inset-0 flex items-center justify-center transition-transform duration-200',
          checked ? 'scale-100' : 'scale-0'
        )}
      >
        <span className="h-2 w-2 rounded-full bg-white" />
      </span>

      {/* Ripple effect on click */}
      <span
        className={cn(
          'absolute inset-0 -m-2 rounded-full transition-opacity duration-300',
          'bg-discord-blurple/20',
          checked ? 'animate-ping opacity-0' : 'opacity-0'
        )}
      />
    </button>
  );
}

interface RadioGroupProps {
  value?: string;
  onValueChange?: (value: string) => void;
  disabled?: boolean;
  children: React.ReactNode;
  className?: string;
}

export function RadioGroup({
  value,
  onValueChange,
  disabled = false,
  children,
  className,
}: RadioGroupProps) {
  return (
    <div role="radiogroup" className={cn('space-y-2', className)}>
      {React.Children.map(children, (child) => {
        if (React.isValidElement<RadioItemProps>(child)) {
          return React.cloneElement(child, {
            checked: child.props.value === value,
            onChange: () => onValueChange?.(child.props.value),
            disabled: disabled || child.props.disabled,
          });
        }
        return child;
      })}
    </div>
  );
}

interface RadioItemProps {
  value: string;
  checked?: boolean;
  onChange?: () => void;
  disabled?: boolean;
  children: React.ReactNode;
  className?: string;
}

export function RadioItem({
  value,
  checked = false,
  onChange,
  disabled = false,
  children,
  className,
}: RadioItemProps) {
  return (
    <label
      className={cn(
        'flex items-center gap-3 cursor-pointer',
        disabled && 'cursor-not-allowed opacity-50',
        className
      )}
    >
      <Radio
        checked={checked}
        onChange={onChange}
        disabled={disabled}
      />
      <span className="text-sm text-white select-none">{children}</span>
    </label>
  );
}
