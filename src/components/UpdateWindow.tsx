import { useEffect, useState } from 'react';
import { check } from '@tauri-apps/plugin-updater';
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
      const update = await check();

      if (update?.available) {
        setStatus(`Update available: ${update.version}`);
        await downloadAndInstall();
      } else {
        setStatus('No updates available');
        setTimeout(() => {
          invoke('close_updater_and_show_main');
        }, 1000);
      }
    } catch (err) {
      setError(String(err));
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
      setError(String(err));
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
