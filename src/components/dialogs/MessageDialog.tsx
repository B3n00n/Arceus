import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { DialogOverlay } from './DialogOverlay';
import { DialogWindow, DialogHeader, DialogContent, DialogFooter } from './DialogWindow';

interface MessageDialogProps {
  isOpen: boolean;
  onClose: () => void;
  selectedCount: number;
  onExecute: (message: string) => void;
  loading?: boolean;
}

const MESSAGE_PRESETS = [
  'GO TO INSTRUCTOR IMMEDIATELY!',
  'STAY AWAY FROM THE GUARDIAN!',
  'RAISE YOUR HAND!',
  'STOP CHEATING!',
];

export function MessageDialog({
  isOpen,
  onClose,
  selectedCount,
  onExecute,
  loading = false,
}: MessageDialogProps) {
  const [message, setMessage] = useState('');

  if (!isOpen) return null;

  const handleExecute = () => {
    if (!message.trim()) return;
    onExecute(message);
  };

  return (
    <DialogOverlay onClose={onClose}>
      <DialogWindow className="w-[32rem]">
        <DialogHeader title="Send Message" subtitle={`To ${selectedCount} device(s)`} />
        <DialogContent className="space-y-4">
          <div>
            <label className="text-sm text-gray-300 mb-2 block">Quick Presets</label>
            <div className="grid grid-cols-2 gap-2">
              {MESSAGE_PRESETS.map((preset, index) => (
                <Button
                  key={index}
                  variant="outline"
                  size="sm"
                  className="justify-start text-xs h-auto py-2"
                  onClick={() => setMessage(preset)}
                  disabled={loading}
                >
                  {preset}
                </Button>
              ))}
            </div>
          </div>
          <div>
            <label className="text-sm text-gray-300 mb-2 block">Message</label>
            <textarea
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              placeholder="Type your message here..."
              rows={4}
              disabled={loading}
              onKeyDown={(e) => {
                if (e.key === 'Escape') onClose();
              }}
              className="w-full bg-discord-dark-1 border border-gray-600 rounded-md px-3 py-2 text-white text-sm focus:outline-none focus:border-[#7289da] disabled:opacity-50 resize-none"
            />
          </div>
        </DialogContent>
        <DialogFooter
          confirmText="Send"
          onConfirm={handleExecute}
          confirmDisabled={loading || !message.trim()}
          onCancel={onClose}
          cancelDisabled={loading}
        />
      </DialogWindow>
    </DialogOverlay>
  );
}
