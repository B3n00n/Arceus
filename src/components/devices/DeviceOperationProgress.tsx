import { Progress } from '@/components/ui/progress';
import type { DeviceOperationProgress } from '@/types/device.types';

interface DeviceOperationProgressProps {
  progress: DeviceOperationProgress;
}

export function DeviceOperationProgressComponent({ progress }: DeviceOperationProgressProps) {
  const getStatusColor = () => {
    switch (progress.stage) {
      case 'completed':
        return 'text-system-success';
      case 'failed':
        return 'text-system-error';
      case 'inprogress':
        return 'text-discord-blurple';
      default:
        return 'text-system-gray';
    }
  };

  const getStatusLabel = () => {
    switch (progress.stage) {
      case 'started':
        return 'Starting...';
      case 'inprogress':
        return `${progress.percentage.toFixed(0)}%`;
      case 'completed':
        return 'Complete';
      case 'failed':
        return 'Failed';
      default:
        return '';
    }
  };

  return (
    <div className="flex flex-col gap-1.5 w-full">
      <div className="flex items-center justify-between gap-2">
        <span className="text-xs text-gray-300">
          {progress.operationType === 'download' ? 'Downloading' : 'Installing'}
        </span>
        <span className={`text-xs font-medium ${getStatusColor()}`}>
          {getStatusLabel()}
        </span>
      </div>
      <Progress value={progress.percentage} className="h-1.5" />
    </div>
  );
}
