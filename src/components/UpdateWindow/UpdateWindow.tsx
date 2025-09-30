import React, { useEffect, useState } from 'react';
import { updateService } from '../../services/updateService';
import { UpdateStatus } from '../../types/update.types';
import './UpdateWindow.css';

interface UpdateWindowProps {
  onComplete: () => void;
}

export const UpdateWindow: React.FC<UpdateWindowProps> = ({ onComplete }) => {
  const [status, setStatus] = useState<UpdateStatus>({ type: 'Checking' });
  const [isUpdating, setIsUpdating] = useState(false);

  useEffect(() => {
    let unlistenFn: (() => void) | undefined;

    const initUpdate = async () => {
      // Listen for status updates
      unlistenFn = await updateService.listenForStatus((newStatus) => {
        setStatus(newStatus);
        
        // Auto-continue if no update or complete
        if (newStatus.type === 'NoUpdate' || newStatus.type === 'Complete') {
          setTimeout(() => onComplete(), 1500);
        }
      });

      // Check for updates
      const initialStatus = await updateService.checkForUpdates();
      setStatus(initialStatus);

      // Auto-continue if no update available
      if (initialStatus.type === 'NoUpdate') {
        setTimeout(() => onComplete(), 1500);
      }
    };

    initUpdate();

    return () => {
      if (unlistenFn) unlistenFn();
      updateService.cleanup();
    };
  }, [onComplete]);

  const handleUpdate = async () => {
    setIsUpdating(true);
    try {
      await updateService.downloadAndInstall();
    } catch (error) {
      setStatus({ 
        type: 'Error', 
        message: error instanceof Error ? error.message : 'Failed to install update' 
      } as UpdateStatus);
      setIsUpdating(false);
    }
  };

  const handleSkip = async () => {
    await updateService.skipUpdate();
    onComplete();
  };

  const renderContent = () => {
    switch (status.type) {
      case 'Checking':
        return (
          <div className="update-checking">
            <div className="spinner"></div>
            <p>Checking for updates...</p>
          </div>
        );

      case 'NoUpdate':
        return (
          <div className="update-current">
            <div className="checkmark">✓</div>
            <p>You're running the latest version!</p>
          </div>
        );

      case 'UpdateAvailable':
        return (
          <div className="update-available">
            <h3>Update Available</h3>
            <p className="version-info">
              {status.data.currentVersion} → {status.data.version}
            </p>
            {status.data.body && (
              <div className="release-notes">
                <h4>Release Notes:</h4>
                <p>{status.data.body}</p>
              </div>
            )}
            <div className="update-actions">
              <button 
                className="btn-primary" 
                onClick={handleUpdate}
                disabled={isUpdating}
              >
                Update Now
              </button>
              <button 
                className="btn-secondary" 
                onClick={handleSkip}
                disabled={isUpdating}
              >
                Skip
              </button>
            </div>
          </div>
        );

      case 'Downloading':
        return (
          <div className="update-downloading">
            <p>Downloading update...</p>
            <div className="progress-bar">
              <div 
                className="progress-fill" 
                style={{ width: `${status.progress}%` }}
              />
            </div>
            <p className="progress-text">
              {Math.round(status.progress)}% 
              ({formatBytes(status.bytesDownloaded)} / {formatBytes(status.totalBytes)})
            </p>
          </div>
        );

      case 'Installing':
        return (
          <div className="update-installing">
            <div className="spinner"></div>
            <p>Installing update...</p>
            <small>The application will restart automatically</small>
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
            <small>{status.message}</small>
            <button className="btn-secondary" onClick={handleSkip}>
              Continue Anyway
            </button>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="update-window">
      <div className="update-container">
        <h2>Arceus Updater</h2>
        {renderContent()}
      </div>
    </div>
  );
};

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}