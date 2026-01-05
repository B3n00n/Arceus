import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { DialogOverlay } from './DialogOverlay';
import { DialogWindow, DialogHeader, DialogContent, DialogFooter } from './DialogWindow';

interface ConfigureDeviceDialogProps {
  isOpen: boolean;
  onClose: () => void;
  selectedCount: number;
  onExecute: (serverIp: string, serverPort: number, wifiSsid?: string, wifiPassword?: string) => void;
  loading?: boolean;
}

export function ConfigureDeviceDialog({
  isOpen,
  onClose,
  selectedCount,
  onExecute,
  loading = false,
}: ConfigureDeviceDialogProps) {
  const [serverIp, setServerIp] = useState('');
  const [serverPort, setServerPort] = useState('');
  const [wifiSsid, setWifiSsid] = useState('');
  const [wifiPassword, setWifiPassword] = useState('');

  if (!isOpen) return null;

  const handleExecute = () => {
    const port = parseInt(serverPort, 10);
    if (!serverIp.trim() || isNaN(port) || port < 1 || port > 65535) {
      return;
    }

    const ssid = wifiSsid.trim();
    const password = wifiPassword.trim();

    if (ssid && password) {
      onExecute(serverIp, port, ssid, password);
    } else if (!ssid && !password) {
      onExecute(serverIp, port);
    } else {
      return;
    }
  };

  const isValid = () => {
    const port = parseInt(serverPort, 10);
    if (!serverIp.trim() || isNaN(port) || port < 1 || port > 65535) {
      return false;
    }

    const ssid = wifiSsid.trim();
    const password = wifiPassword.trim();

    if ((ssid && !password) || (!ssid && password)) {
      return false;
    }

    if (ssid && (ssid.length < 1 || ssid.length > 32)) {
      return false;
    }

    if (password && password.length > 0 && (password.length < 8 || password.length > 63)) {
      return false;
    }

    return true;
  };

  return (
    <DialogOverlay onClose={onClose}>
      <DialogWindow className="w-[500px]">
        <DialogHeader
          title="Configure Device"
          subtitle={`Configure ${selectedCount} device(s)`}
        />
        <DialogContent className="space-y-4">
          <div className="space-y-4">
            {/* Server Configuration */}
            <div className="space-y-3 pb-3 border-b border-discord-dark-2">
              <h4 className="text-sm font-semibold text-white">Server Connection</h4>
              <div>
                <label className="text-sm text-gray-300 mb-2 block">Server IP</label>
                <Input
                  value={serverIp}
                  onChange={(e) => setServerIp(e.target.value)}
                  placeholder="192.168.0.77"
                  disabled={loading}
                />
              </div>
              <div>
                <label className="text-sm text-gray-300 mb-2 block">Server Port</label>
                <Input
                  value={serverPort}
                  onChange={(e) => setServerPort(e.target.value)}
                  placeholder="43572"
                  type="number"
                  disabled={loading}
                />
              </div>
            </div>

            {/* WiFi Configuration */}
            <div className="space-y-3">
              <h4 className="text-sm font-semibold text-white">WiFi Settings (Optional)</h4>
              <p className="text-xs text-gray-400">Leave empty to only change server settings</p>
              <div>
                <label className="text-sm text-gray-300 mb-2 block">WiFi SSID</label>
                <Input
                  value={wifiSsid}
                  onChange={(e) => setWifiSsid(e.target.value)}
                  placeholder="Network Name"
                  disabled={loading}
                />
                <p className="text-xs text-gray-500 mt-1">1-32 characters</p>
              </div>
              <div>
                <label className="text-sm text-gray-300 mb-2 block">WiFi Password</label>
                <Input
                  value={wifiPassword}
                  onChange={(e) => setWifiPassword(e.target.value)}
                  placeholder="Network Password"
                  type="password"
                  disabled={loading}
                />
                <p className="text-xs text-gray-500 mt-1">Empty or 8-63 characters</p>
              </div>
            </div>

            {/* Warning */}
            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-3">
              <p className="text-xs text-yellow-400">
                The Device(s) will disconnect immediately after applying configuration!
              </p>
            </div>
          </div>
        </DialogContent>
        <DialogFooter
          confirmText="Configure"
          onConfirm={handleExecute}
          confirmDisabled={loading || !isValid()}
          onCancel={onClose}
          cancelDisabled={loading}
        />
      </DialogWindow>
    </DialogOverlay>
  );
}
