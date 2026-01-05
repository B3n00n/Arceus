import { useEffect, useState } from 'react';
import { relaunch } from '@tauri-apps/plugin-process';
import { invoke } from '@tauri-apps/api/core';

export function UpdateWindow() {
  const [status, setStatus] = useState('Checking for updates...');
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    checkForUpdates();
  }, []);

  const checkForUpdates = async () => {
    try {
      const updateStatus = await invoke('check_for_updates');

      if (updateStatus && typeof updateStatus === 'object' && 'type' in updateStatus) {
        const status = updateStatus as any;

        if (status.type === 'UpdateAvailable' && status.data) {
          setStatus(`Update available: ${status.data.version}`);
          await downloadAndInstall();
        } else {
          setStatus('No server updates available');
          await checkClientApk();
        }
      } else {
        setStatus('No server updates available');
        await checkClientApk();
      }
    } catch (err) {
      setError('Can not check for updates');
      setTimeout(() => {
        invoke('close_updater_and_show_main');
      }, 2000);
    }
  };

  const checkClientApk = async () => {
    try {
      setStatus('Checking for client APK updates...');
      setProgress(0);

      const updated = await invoke<boolean>('check_and_update_client_apk');

      if (updated) {
        setStatus('Client APK updated successfully');
      } else {
        setStatus('Client APK is up to date');
      }

      setTimeout(() => {
        invoke('close_updater_and_show_main');
      }, 1000);
    } catch (err) {
      setError('Failed to check client APK updates');
      setTimeout(() => {
        invoke('close_updater_and_show_main');
      }, 2000);
    }
  };

  const downloadAndInstall = async () => {
    try {
      setStatus('Downloading update...');
      await invoke('download_and_install_update', {
        onProgress: (p: number) => setProgress(p),
      });

      setStatus('Installing update...');
      await new Promise((resolve) => setTimeout(resolve, 1000));

      setStatus('Restarting application...');
      await relaunch();
    } catch (err) {
      setError('Failed to download/install update');
      setTimeout(() => {
        invoke('close_updater_and_show_main');
      }, 2000);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen w-full bg-discord-dark-4 p-8">
      <div className="text-center max-w-md w-full">
        {error ? (
          <div className="text-red-400 text-base mb-4">{error}</div>
        ) : (
          <>
            <p className="text-white text-base mb-6">{status}</p>

            {progress > 0 && (
              <div className="w-full bg-discord-dark-3 rounded-full h-3 mb-4">
                <div
                  className="bg-discord-blurple h-3 rounded-full transition-all duration-300"
                  style={{ width: `${progress}%` }}
                />
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
