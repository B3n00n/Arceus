import { ReactNode } from "react";
import { cn } from "@/lib/cn";
import { Button } from "@/components/ui/button";

interface DialogWindowProps {
  children: ReactNode;
  className?: string;
  /**
   * Optional max height for the dialog content area
   * Useful for scrollable dialogs
   */
  maxHeight?: string;
}

/**
 * DialogWindow - The base container for all dialog content
 *
 * Provides consistent styling for dialogs including:
 * - Background color (discord-dark-4)
 * - Border and shadow
 * - Rounded corners
 * - Proper spacing
 *
 * Should be used inside DialogOverlay
 */
export function DialogWindow({ children, className, maxHeight }: DialogWindowProps) {
  return (
    <div
      className={cn(
        "rounded-lg border bg-discord-dark-4 border-discord-dark shadow flex flex-col",
        maxHeight && maxHeight,
        className
      )}
    >
      {children}
    </div>
  );
}

interface DialogHeaderProps {
  title: string;
  subtitle?: string;
  className?: string;
}

/**
 * DialogHeader - Consistent header for all dialogs
 *
 * Displays the dialog title and optional subtitle
 * Includes proper padding and spacing
 */
export function DialogHeader({ title, subtitle, className }: DialogHeaderProps) {
  return (
    <div className={cn("flex flex-col space-y-1.5 p-4", subtitle && "pb-3", className)}>
      <h3 className="text-lg font-semibold text-white">{title}</h3>
      {subtitle && <p className="text-sm text-gray-400">{subtitle}</p>}
    </div>
  );
}

interface DialogContentProps {
  children: ReactNode;
  className?: string;
  /**
   * Whether the content should scroll
   * When true, adds overflow-y-auto and flex-1
   */
  scrollable?: boolean;
}

/**
 * DialogContent - Main content area of the dialog
 *
 * Provides consistent padding and optional scrolling
 */
export function DialogContent({ children, className, scrollable = false }: DialogContentProps) {
  return (
    <div
      className={cn(
        "p-4 pt-0",
        scrollable && "flex-1 overflow-y-auto",
        className
      )}
    >
      {children}
    </div>
  );
}

interface DialogFooterProps {
  /**
   * Confirm button text
   */
  confirmText: string;
  /**
   * Confirm button click handler
   */
  onConfirm: () => void;
  /**
   * Confirm button variant
   * @default "default"
   */
  confirmVariant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link" | "danger" | "danger-outline";
  /**
   * Whether the confirm button should be disabled
   */
  confirmDisabled?: boolean;
  /**
   * Cancel button click handler
   * When provided, a cancel button will be automatically rendered
   */
  onCancel?: () => void;
  /**
   * Whether the cancel button should be disabled
   */
  cancelDisabled?: boolean;
  className?: string;
}

/**
 * DialogFooter - Consistent footer for dialog actions
 *
 * Displays action buttons in a consistent layout:
 * - Confirm button on the right (with customizable variant and text)
 * - Cancel button on the left (automatically rendered with standard styling)
 * - Proper spacing and border
 */
export function DialogFooter({
  confirmText,
  onConfirm,
  confirmVariant = "default",
  confirmDisabled = false,
  onCancel,
  cancelDisabled = false,
  className
}: DialogFooterProps) {
  return (
    <div className={cn(
      "p-4 border-t border-discord-dark flex gap-2",
      onCancel ? "flex-row-reverse justify-between" : "flex-row-reverse",
      className
    )}>
      <Button variant={confirmVariant} onClick={onConfirm} disabled={confirmDisabled}>
        {confirmText}
      </Button>
      {onCancel && (
        <Button variant="outline" onClick={onCancel} disabled={cancelDisabled}>
          Cancel
        </Button>
      )}
    </div>
  );
}
