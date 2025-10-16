import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { DeviceService } from "../../services/deviceService";
import type { DeviceState } from "../../types/device.types";
import "./DeviceList.css";

export function DeviceList() {
  const [devices, setDevices] = useState<DeviceState[]>([]);
  const [selectedDevices, setSelectedDevices] = useState<Set<string>>(
    new Set()
  );
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<string>("");

  useEffect(() => {
    loadDevices();

    // Listen for device events
    const unlistenPromises = [
      listen("device-connected", () => loadDevices()),
      listen("device-disconnected", () => loadDevices()),
      listen("device-updated", () => loadDevices()),
      listen("battery-updated", () => loadDevices()),
      listen("volume-updated", () => loadDevices()),
    ];

    return () => {
      Promise.all(unlistenPromises).then((unlisteners) => {
        unlisteners.forEach((unlisten) => unlisten());
      });
    };
  }, []);

  const loadDevices = async () => {
    try {
      const deviceList = await DeviceService.getDevices();
      setDevices(deviceList);
    } catch (error) {
      console.error("Failed to load devices:", error);
      showMessage("Failed to load devices: " + error);
    }
  };

  const showMessage = (msg: string) => {
    setMessage(msg);
    setTimeout(() => setMessage(""), 3000);
  };

  const toggleDevice = (id: string) => {
    const newSelection = new Set(selectedDevices);
    if (newSelection.has(id)) {
      newSelection.delete(id);
    } else {
      newSelection.add(id);
    }
    setSelectedDevices(newSelection);
  };

  const selectAll = () => {
    if (selectedDevices.size === devices.length) {
      setSelectedDevices(new Set());
    } else {
      setSelectedDevices(new Set(devices.map((d) => d.info.id)));
    }
  };

  const getSelectedIds = (): string[] => Array.from(selectedDevices);

  const handleShutdown = async () => {
    if (selectedDevices.size === 0) {
      showMessage("Please select at least one device");
      return;
    }

    if (
      !confirm(
        `Are you sure you want to shutdown ${selectedDevices.size} device(s)?`
      )
    ) {
      return;
    }

    setLoading(true);
    try {
      await DeviceService.shutdownDevices(getSelectedIds());
      showMessage(`Shutdown command sent to ${selectedDevices.size} device(s)`);
    } catch (error) {
      console.error("Shutdown failed:", error);
      showMessage("Shutdown failed: " + error);
    } finally {
      setLoading(false);
    }
  };

  const handleRequestBattery = async () => {
    if (selectedDevices.size === 0) {
      showMessage("Please select at least one device");
      return;
    }

    setLoading(true);
    try {
      await DeviceService.requestBattery(getSelectedIds());
      showMessage("Battery request sent");
    } catch (error) {
      console.error("Battery request failed:", error);
      showMessage("Battery request failed: " + error);
    } finally {
      setLoading(false);
    }
  };

  const handleGetVolume = async () => {
    if (selectedDevices.size === 0) {
      showMessage("Please select at least one device");
      return;
    }

    setLoading(true);
    try {
      await DeviceService.getVolume(getSelectedIds());
      showMessage("Volume request sent");
    } catch (error) {
      console.error("Volume request failed:", error);
      showMessage("Volume request failed: " + error);
    } finally {
      setLoading(false);
    }
  };

  const handleSetVolume = async () => {
    if (selectedDevices.size === 0) {
      showMessage("Please select at least one device");
      return;
    }

    const levelStr = prompt("Enter volume level (0-100):", "50");
    if (!levelStr) return;

    const level = parseInt(levelStr);
    if (isNaN(level) || level < 0 || level > 100) {
      showMessage("Invalid volume level. Must be 0-100");
      return;
    }

    setLoading(true);
    try {
      await DeviceService.setVolume(getSelectedIds(), level);
      showMessage(`Volume set to ${level}%`);
    } catch (error) {
      console.error("Set volume failed:", error);
      showMessage("Set volume failed: " + error);
    } finally {
      setLoading(false);
    }
  };

  const handlePing = async () => {
    if (selectedDevices.size === 0) {
      showMessage("Please select at least one device");
      return;
    }

    setLoading(true);
    try {
      const ids = getSelectedIds();
      console.log("Sending ping to device IDs:", ids);
      console.log("Type check:", Array.isArray(ids), ids.length, typeof ids[0]);
      await DeviceService.pingDevices(ids);
      showMessage("Ping sent");
    } catch (error) {
      console.error("Ping failed:", error);
      showMessage("Ping failed: " + error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="device-list-container">
      <div className="header">
        <h2>Connected Devices ({devices.length})</h2>
        <button onClick={loadDevices} disabled={loading}>
          🔄 Refresh
        </button>
      </div>

      {message && <div className="message">{message}</div>}

      <div className="device-controls">
        <button onClick={selectAll} disabled={devices.length === 0}>
          {selectedDevices.size === devices.length ? "Deselect All" : "Select All"}
        </button>
        <button
          onClick={handleShutdown}
          disabled={loading || selectedDevices.size === 0}
          className="danger"
        >
          ⚡ Shutdown ({selectedDevices.size})
        </button>
        <button
          onClick={handleRequestBattery}
          disabled={loading || selectedDevices.size === 0}
        >
          🔋 Get Battery
        </button>
        <button
          onClick={handleGetVolume}
          disabled={loading || selectedDevices.size === 0}
        >
          🔊 Get Volume
        </button>
        <button
          onClick={handleSetVolume}
          disabled={loading || selectedDevices.size === 0}
        >
          🔊 Set Volume
        </button>
        <button
          onClick={handlePing}
          disabled={loading || selectedDevices.size === 0}
        >
          📡 Ping
        </button>
      </div>

      <div className="device-grid">
        {devices.length === 0 ? (
          <div className="no-devices">
            <p>No devices connected</p>
            <p className="hint">
              Make sure your Android device has the SnorlaxClient app running
              and connected to this server.
            </p>
          </div>
        ) : (
          devices.map((device) => (
            <div
              key={device.info.id}
              className={`device-card ${selectedDevices.has(device.info.id) ? "selected" : ""}`}
              onClick={() => toggleDevice(device.info.id)}
            >
              <div className="device-header">
                <input
                  type="checkbox"
                  checked={selectedDevices.has(device.info.id)}
                  onChange={() => toggleDevice(device.info.id)}
                  onClick={(e) => e.stopPropagation()}
                />
                <div className="device-status">
                  <span
                    className={`status-dot ${device.is_connected ? "connected" : "disconnected"}`}
                  ></span>
                  <strong>{device.info.custom_name || device.info.model}</strong>
                </div>
              </div>

              <div className="device-info">
                <div className="info-row">
                  <span className="label">Serial:</span>
                  <span className="value">{device.info.serial}</span>
                </div>
                <div className="info-row">
                  <span className="label">IP:</span>
                  <span className="value">{device.info.ip}</span>
                </div>
                {device.battery && (
                  <div className="info-row">
                    <span className="label">Battery:</span>
                    <span className="value">
                      {device.battery.headset_level}%
                      {device.battery.is_charging && " ⚡"}
                    </span>
                  </div>
                )}
                {device.volume && (
                  <div className="info-row">
                    <span className="label">Volume:</span>
                    <span className="value">
                      {device.volume.volume_percentage}% ({device.volume.current_volume}/
                      {device.volume.max_volume})
                    </span>
                  </div>
                )}
              </div>

              {device.command_history.length > 0 && (
                <div className="command-history">
                  <div className="history-header">Last Command:</div>
                  <div
                    className={`history-item ${device.command_history[0].success ? "success" : "error"}`}
                  >
                    <span className="command-type">
                      {device.command_history[0].command_type}
                    </span>
                    <span className="command-message">
                      {device.command_history[0].message}
                    </span>
                  </div>
                </div>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
