import { useState } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Search } from 'lucide-react';
import { cn } from '@/lib/cn';
import { DialogOverlay } from './DialogOverlay';
import type { ApkInfo } from '@/types/apk.types';

interface ApkListDialogProps {
  isOpen: boolean;
  onClose: () => void;
  selectedCount: number;
  availableApks: ApkInfo[];
  onSelectApk: (apk: ApkInfo) => void;
  loading?: boolean;
}

export function ApkListDialog({
  isOpen,
  onClose,
  selectedCount,
  availableApks,
  onSelectApk,
  loading = false,
}: ApkListDialogProps) {
  const [dialogSearch, setDialogSearch] = useState('');
  const [selectedApk, setSelectedApk] = useState<ApkInfo | null>(null);

  if (!isOpen) return null;

  const getFilteredApks = () => {
    if (!dialogSearch) return availableApks;
    return availableApks.filter((apk) =>
      apk.filename.toLowerCase().includes(dialogSearch.toLowerCase())
    );
  };

  const formatFileSize = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  };

  const handleSelectAndInstall = () => {
    if (selectedApk) {
      onSelectApk(selectedApk);
      handleClose();
    }
  };

  const handleClose = () => {
    setSelectedApk(null);
    setDialogSearch('');
    onClose();
  };

  const filteredApks = getFilteredApks();

  return (
    <DialogOverlay onClose={handleClose}>
      <Card className="w-[500px] max-h-[600px] flex flex-col">
        <CardHeader>
          <h3 className="text-lg font-semibold text-white">Select APK to Install</h3>
          <p className="text-sm text-gray-400">For {selectedCount} device(s)</p>
          <div className="relative mt-2">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-gray-400" />
            <Input
              value={dialogSearch}
              onChange={(e) => setDialogSearch(e.target.value)}
              placeholder="Search APKs..."
              className="pl-10"
            />
          </div>
        </CardHeader>
        <CardContent className="flex-1 overflow-y-auto p-0">
          <div className="divide-y divide-discord-dark">
            {filteredApks.map((apk) => (
              <button
                key={apk.filename}
                className={cn(
                  'w-full px-6 py-3 text-left hover:bg-discord-dark-3 transition-colors',
                  selectedApk?.filename === apk.filename &&
                    'bg-discord-blurple/20 border-l-2 border-discord-blurple'
                )}
                onClick={() => setSelectedApk(apk)}
                disabled={loading}
              >
                <div className="flex items-center justify-between">
                  <p className="text-sm text-white font-medium">{apk.filename}</p>
                  <Badge variant="secondary" className="text-xs">
                    {formatFileSize(apk.size_bytes)}
                  </Badge>
                </div>
              </button>
            ))}
            {filteredApks.length === 0 && (
              <div className="px-6 py-8 text-center text-gray-400">No APK files found</div>
            )}
          </div>
        </CardContent>
        <div className="p-4 border-t border-discord-dark flex-row-reverse flex gap-2 justify-between">
          <Button variant="default" onClick={handleSelectAndInstall} disabled={loading || !selectedApk}>
            Install
          </Button>
          <Button variant="outline" onClick={handleClose} disabled={loading}>
            Cancel
          </Button>
        </div>
      </Card>
    </DialogOverlay>
  );
}
