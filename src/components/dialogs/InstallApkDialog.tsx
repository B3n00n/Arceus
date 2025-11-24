import { useState } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Radio } from '@/components/ui/radio';
import { SegmentedControl } from '@/components/ui/SegmentedControl';
import { Package } from 'lucide-react';
import { cn } from '@/lib/cn';
import { DialogOverlay } from './DialogOverlay';
import type { ApkInfo } from '@/types/apk.types';

type InstallSource = 'local' | 'url';


interface InstallApkDialogProps {
  isOpen: boolean;
  onClose: () => void;
  selectedCount: number;
  availableApks: ApkInfo[];
  onInstallLocal: (apk: ApkInfo) => void;
  onInstallRemote: (url: string) => void;
  loading?: boolean;
}

export function InstallApkDialog({
  isOpen,
  onClose,
  selectedCount,
  availableApks,
  onInstallLocal,
  onInstallRemote,
  loading = false,
}: InstallApkDialogProps) {
  const [installSource, setInstallSource] = useState<InstallSource>('local');
  const [dialogSearch, setDialogSearch] = useState('');
  const [selectedApk, setSelectedApk] = useState<ApkInfo | null>(null);
  const [remoteUrl, setRemoteUrl] = useState('');

  if (!isOpen) return null;

  const apksToDisplay = availableApks; 

  const getFilteredApks = () => {
    if (!dialogSearch) return apksToDisplay;
    return apksToDisplay.filter((apk) =>
      apk.filename.toLowerCase().includes(dialogSearch.toLowerCase())
    );
  };

  const formatFileSize = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  };

  const handleInstall = () => {
    if (installSource === 'local') {
      if (selectedApk) {
        onInstallLocal(selectedApk);
        handleClose();
      }
    } else {
      if (remoteUrl.trim()) {
        onInstallRemote(remoteUrl);
        handleClose();
      }
    }
  };

  const handleClose = () => {
    setSelectedApk(null);
    setDialogSearch('');
    setRemoteUrl('');
    setInstallSource('local');
    onClose();
  };

  const filteredApks = getFilteredApks();
  const canInstall = installSource === 'local' ? selectedApk !== null : remoteUrl.trim() !== '';

  return (
    <DialogOverlay onClose={handleClose}>
      <Card className="w-[500px] max-h-[400px] flex flex-col">
        <CardHeader className="pb-3">
          <h3 className="text-lg font-semibold text-white">Install APK</h3>
          <p className="text-sm text-gray-400">For {selectedCount} device(s)</p>
        </CardHeader>

        {/* Segmented Control */}
        <SegmentedControl
          options={[
            { label: 'Local APK', value: 'local' },
            { label: 'From URL', value: 'url' },
          ]}
          value={installSource}
          onChange={(val) => setInstallSource(val as InstallSource)}
        />

        {/* Content based on selected tab */}
        {installSource === 'local' ? (
          <>
            {/* Local APK List */}
            <CardContent className="flex-1 overflow-y-auto p-4">
              <div>
                {filteredApks.length > 0 && (
                  <p className="text-sm font-medium text-gray-300 mb-2">Available APKs</p>
                )}
              <div className={cn(
                "divide-y divide-discord-dark",
                filteredApks.length > 0 && "border-t border-t-discord-dark"
              )}>
                {filteredApks.map((apk) => (
                  <label
                    key={apk.filename}
                    className={cn(
                      'w-full pl-[14px] pr-4 py-3 border-l-2 flex items-center gap-3 cursor-pointer transition-colors',
                      'hover:bg-discord-dark-3',
                      selectedApk?.filename === apk.filename
                        ? 'bg-discord-blurple/20 border-l-discord-blurple'
                        : 'border-l-transparent'
                    )}
                  >
                    <Radio
                      checked={selectedApk?.filename === apk.filename}
                      onChange={() => setSelectedApk(apk)}
                      disabled={loading}
                    />
                    <div className="flex-1 flex items-center justify-between">
                      <p className="text-sm text-white font-medium">{apk.filename}</p>
                      <Badge variant="secondary" className="text-xs">
                        {formatFileSize(apk.size_bytes)}
                      </Badge>
                    </div>
                  </label>
                ))}
                {filteredApks.length === 0 && (
                  <div className="p-6 text-center text-gray-400">
                    <Package className="h-12 w-12 mx-auto mb-2 opacity-50" />
                    <p>No APK files found</p>
                  </div>
                )}
              </div>
              </div>
            </CardContent>
          </>
        ) : (
          <>
            {/* Remote URL Input */}
            <CardContent className="flex-1 p-4">
              <div className="space-y-2">
                <label className="text-sm font-medium text-gray-300">APK URL</label>
                <Input
                  value={remoteUrl}
                  onChange={(e) => setRemoteUrl(e.target.value)}
                  placeholder="https://example.com/app.apk"
                  className="w-full"
                  disabled={loading}
                />
              </div>
            </CardContent>
          </>
        )}

        {/* Footer Actions */}
        <div className="p-4 border-t border-discord-dark flex-row-reverse flex gap-2 justify-between">
          <Button variant="default" onClick={handleInstall} disabled={loading || !canInstall}>
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
