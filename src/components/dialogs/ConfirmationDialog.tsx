import { DialogOverlay } from "./DialogOverlay";
import { DialogWindow, DialogHeader, DialogContent, DialogFooter } from "./DialogWindow";

interface ConfirmationDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  message: string | React.ReactNode;
  confirmText: string;
  loading?: boolean;
}

export function ConfirmationDialog({
  isOpen,
  onClose,
  onConfirm,
  title,
  message,
  confirmText,
  loading = false,
}: ConfirmationDialogProps) {
  if (!isOpen) return null;

  return (
    <DialogOverlay onClose={onClose}>
      <DialogWindow className="w-100">
        <DialogHeader title={title} />
        <DialogContent className="space-y-8">
          <div className="text-md text-grey-200">{message}</div>
        </DialogContent>
        <DialogFooter
          confirmText={confirmText}
          onConfirm={onConfirm}
          confirmVariant="danger"
          confirmDisabled={loading}
          onCancel={onClose}
          cancelDisabled={loading}
        />
      </DialogWindow>
    </DialogOverlay>
  );
}
