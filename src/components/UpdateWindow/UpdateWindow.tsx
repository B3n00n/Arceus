import { useEffect, useState, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { updateService } from '../../services/updateService';
import { UpdateStatus } from '../../types/update.types';
import './UpdateWindow.css';

const CLOSE_DELAY_MS = 1500;
const ERROR_CLOSE_DELAY_MS = 3000;

export const UpdateWindow = () => {
  const [status, setStatus] = useState<UpdateStatus>({ type: 'Checking' });
  const closeTimerRef = useRef<number | undefined>(undefined);

  const closeUpdaterWindow = useCallback(async () => {
    try {
      await invoke('close_updater_and_show_main');
    } catch (error) {
      console.error('Failed to close updater:', error);
    }
  }, []);

  const scheduleClose = useCallback((delay: number) => {
    if (closeTimerRef.current) {
      clearTimeout(closeTimerRef.current);
    }
    closeTimerRef.current = window.setTimeout(closeUpdaterWindow, delay);
  }, [closeUpdaterWindow]);

  const handleStatusChange = useCallback((newStatus: UpdateStatus) => {
    setStatus(newStatus);

    if (newStatus.type === 'NoUpdate') {
      scheduleClose(CLOSE_DELAY_MS);
    } else if (newStatus.type === 'Error') {
      scheduleClose(ERROR_CLOSE_DELAY_MS);
    }
  }, [scheduleClose]);

  const startUpdate = useCallback(async () => {
    try {
      await updateService.downloadAndInstall();
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to install update';
      setStatus({
        type: 'Error',
        data: { message: errorMessage }
      });
    }
  }, []);

  useEffect(() => {
    let unlistenFn: (() => void) | undefined;

    const initUpdate = async () => {
      try {
        unlistenFn = await updateService.listenForStatus(handleStatusChange);

        const initialStatus = await updateService.checkForUpdates();
        handleStatusChange(initialStatus);

        if (initialStatus.type === 'UpdateAvailable') {
          await startUpdate();
        }
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Failed to check for updates';
        setStatus({
          type: 'Error',
          data: { message: errorMessage }
        });
      }
    };

    initUpdate();

    return () => {
      if (unlistenFn) unlistenFn();
      if (closeTimerRef.current) clearTimeout(closeTimerRef.current);
      updateService.cleanup();
    };
  }, [handleStatusChange, startUpdate]);

  const renderContent = () => {
    switch (status.type) {
      case 'Checking':
        return (
          <div className="update-checking">
            <p>Checking for updates...</p>
          </div>
        );

      case 'NoUpdate':
        return (
          <div className="update-current">
            <p>Starting...</p>
          </div>
        );

      case 'UpdateAvailable':
        return (
          <div className="update-downloading">
            <p>Preparing update...</p>
          </div>
        );

      case 'Downloading':
        return (
          <div className="update-downloading">
            <p>Downloading...</p>
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{ width: `${status.data.progress}%` }}
              />
            </div>
          </div>
        );

      case 'Downloaded':
        return (
          <div className="update-installing">
            <p>Download complete</p>
            <small>Preparing installation...</small>
          </div>
        );

      case 'Installing':
        return (
          <div className="update-installing">
            <p>Installing...</p>
            <small>App will restart automatically</small>
          </div>
        );

      case 'Installed':
        return (
          <div className="update-complete">
            <div className="checkmark">✓</div>
            <p>Installation complete!</p>
            <small>Restarting...</small>
          </div>
        );

      case 'Complete':
        return (
          <div className="update-complete">
            <div className="checkmark">✓</div>
            <p>Update complete!</p>
            <small>Restarting...</small>
          </div>
        );

      case 'Error':
        return (
          <div className="update-error">
            <div className="error-icon">⚠</div>
            <p>Update failed</p>
            <small>{status.data.message}</small>
            <small>Continuing to app...</small>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="update-window">
      <div className="update-container">
        {renderContent()}
      </div>
    </div>
  );
};