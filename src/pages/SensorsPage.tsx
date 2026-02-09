import { useState, useEffect, useCallback } from 'react';
import { Cpu, RefreshCw, Upload, Usb, AlertCircle, CheckCircle2, Loader2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { SensorService } from '@/services/sensorService';
import { eventService } from '@/services/eventService';
import type { Sensor } from '@/types/sensor.types';
import { cn } from '@/lib/cn';
import { toast } from '@/lib/toast';

export function SensorsPage() {
  const [sensor, setSensor] = useState<Sensor | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadingInfo, setLoadingInfo] = useState(false);
  const [uploading, setUploading] = useState(false);
  const [uploadProgress, setUploadProgress] = useState<{ stage: string; percentage: number } | null>(null);
  const [firmwarePath, setFirmwarePath] = useState('');
  const [deviceName, setDeviceName] = useState('');
  const [maxNameLength, setMaxNameLength] = useState<number | null>(null);

  const refreshSensor = useCallback(async () => {
    setLoading(true);
    try {
      const list = await SensorService.listSensors();
      const detected = list[0] ?? null;

      if (detected && detected.status === 'connected') {
        // Preserve existing details if same port
        setSensor(prev =>
          prev?.port === detected.port && prev.device_name
            ? { ...prev, status: detected.status }
            : detected
        );

        // Fetch details if we don't have them yet
        setSensor(prev => {
          if (prev?.port === detected.port && !prev.device_name) {
            fetchDetails(detected.port);
          }
          return prev;
        });
      } else {
        setSensor(detected);
      }
    } catch (error) {
      toast.error(`Failed to detect sensor: ${error}`);
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchDetails = async (port: string) => {
    setLoadingInfo(true);
    try {
      const info = await SensorService.getSensorInfo(port);
      setSensor(info);
    } catch (error) {
      toast.warning(`Could not read sensor details: ${error}`);
    } finally {
      setLoadingInfo(false);
    }
  };

  useEffect(() => {
    refreshSensor();
    SensorService.getMaxNameLength()
      .then(setMaxNameLength)
      .catch((error) => toast.error(`Failed to get max name length: ${error}`));

    const unsubscribe = eventService.subscribe((event) => {
      if (event.type === 'sensorUploadProgress') {
        if (event.stage === 'completed' || event.stage === 'failed') {
          setUploadProgress(null);
        } else {
          setUploadProgress({ stage: event.stage, percentage: event.percentage });
        }
      }
    });

    return unsubscribe;
  }, []);

  const handleUpload = async () => {
    if (!firmwarePath) {
      toast.error('Please enter a firmware file path');
      return;
    }
    if (!deviceName.trim()) {
      toast.error('Please enter a device name');
      return;
    }
    if (maxNameLength !== null && deviceName.length > maxNameLength) {
      toast.error(`Device name too long (max ${maxNameLength} characters)`);
      return;
    }

    setUploading(true);
    try {
      await SensorService.uploadFirmware(
        sensor?.port || null,
        firmwarePath,
        deviceName.trim()
      );
      toast.success('Firmware uploaded successfully!');

      // Optimistically update the device name
      const newName = deviceName.trim();
      setSensor(prev => prev ? { ...prev, device_name: newName } : prev);

      // Refresh after device reboots
      setTimeout(() => refreshSensor(), 5000);
    } catch (error) {
      toast.error(`Upload failed: ${error}`);
    } finally {
      setUploading(false);
    }
  };

  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-white">Sensor Management</h1>
          <p className="text-grey-300 mt-1">Flash firmware to Combatica sensors</p>
        </div>
        <Button
          variant="outline"
          onClick={refreshSensor}
          disabled={loading}
        >
          <RefreshCw className={cn("h-4 w-4 mr-2", loading && "animate-spin")} />
          Refresh
        </Button>
      </div>

      {/* Sensor Status */}
      <div className="rounded-lg border bg-grey-800 border-grey-600 shadow">
        <div className="p-4 border-b border-grey-600">
          <h2 className="text-lg font-semibold text-white flex items-center gap-2">
            <Usb className="h-5 w-5" />
            Connected Sensor
          </h2>
        </div>
        <div className="p-4">
          {loading ? (
            <div className="flex items-center justify-center py-6">
              <Loader2 className="h-6 w-6 animate-spin text-grey-400" />
            </div>
          ) : !sensor ? (
            <div className="text-center py-6">
              <Cpu className="h-10 w-10 mx-auto text-grey-500 mb-2" />
              <p className="text-grey-400">No sensor detected</p>
              <p className="text-grey-500 text-sm mt-1">Connect a XIAO BLE board via USB</p>
            </div>
          ) : (
            <div className="flex items-center justify-between">
              <div className="space-y-1">
                <div className="flex items-center gap-2">
                  <span className="font-medium text-white text-lg">
                    {sensor.device_name || sensor.port}
                  </span>
                  {loadingInfo && (
                    <Loader2 className="h-3 w-3 animate-spin text-grey-400" />
                  )}
                </div>
                <div className="flex items-center gap-4 text-sm text-grey-400">
                  <span>{sensor.port}</span>
                  {sensor.firmware_version && (
                    <span>Firmware: <span className="text-grey-300">{sensor.firmware_version}</span></span>
                  )}
                </div>
              </div>
              <div className="flex items-center gap-1.5">
                {sensor.status === 'connected' ? (
                  <CheckCircle2 className="h-4 w-4 text-green-400" />
                ) : (
                  <AlertCircle className="h-4 w-4 text-yellow-400" />
                )}
                <span className={cn(
                  "text-sm capitalize",
                  sensor.status === 'connected' ? 'text-green-400' : 'text-yellow-400'
                )}>
                  {sensor.status}
                </span>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Upload Firmware */}
      <div className="rounded-lg border bg-grey-800 border-grey-600 shadow">
        <div className="p-4 border-b border-grey-600">
          <h2 className="text-lg font-semibold text-white flex items-center gap-2">
            <Upload className="h-5 w-5" />
            Upload Firmware
          </h2>
        </div>
        <div className="p-4 space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-grey-300 mb-2">
                Firmware File (.bin)
              </label>
              <Input
                value={firmwarePath}
                onChange={(e) => setFirmwarePath(e.target.value)}
                placeholder="/path/to/firmware.bin"
                disabled={uploading}
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-grey-300 mb-2">
                Device Name (BLE)
              </label>
              <Input
                value={deviceName}
                onChange={(e) => setDeviceName(e.target.value)}
                placeholder="Enter device name..."
                maxLength={maxNameLength ?? undefined}
                disabled={uploading}
              />
              {maxNameLength !== null && (
                <p className="text-xs text-grey-500 mt-1">
                  {deviceName.length}/{maxNameLength} characters
                </p>
              )}
            </div>
          </div>

          {uploading && uploadProgress && (
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-grey-300 capitalize">{uploadProgress.stage}...</span>
                <span className="text-grey-300 font-mono">{Math.round(uploadProgress.percentage)}%</span>
              </div>
              <div className="w-full h-2 bg-grey-700 rounded-full overflow-hidden">
                <div
                  className="h-full bg-blue-500 rounded-full transition-all duration-300 ease-out"
                  style={{ width: `${Math.min(uploadProgress.percentage, 100)}%` }}
                />
              </div>
            </div>
          )}

          <Button
            className="w-full"
            onClick={handleUpload}
            disabled={uploading || loadingInfo || !firmwarePath || !deviceName.trim()}
          >
            {uploading ? (
              <>
                <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                Uploading — do not disconnect...
              </>
            ) : (
              <>
                <Upload className="h-4 w-4 mr-2" />
                Upload Firmware
              </>
            )}
          </Button>
        </div>
      </div>
    </div>
  );
}
