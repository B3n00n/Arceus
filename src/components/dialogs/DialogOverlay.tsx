interface DialogOverlayProps {
  children: React.ReactNode;
  onClose: () => void;
}

export function DialogOverlay({ children, onClose }: DialogOverlayProps) {
  return (
    <div
      className="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div onClick={(e) => e.stopPropagation()}>{children}</div>
    </div>
  );
}
