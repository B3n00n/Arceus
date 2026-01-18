# Sensor Management System - Complete Architecture

## Overview

This document describes the complete sensor management system for the Arceus platform. Sensors are physical Arduino devices (XIAO BLE nRF52840) that run the **Gyros** firmware and communicate with Arceus arcade machines.

---

## Table of Contents

1. [Current State](#current-state)
2. [Goal & Vision](#goal--vision)
3. [Two Core Concepts](#two-core-concepts)
4. [System Architecture](#system-architecture)
5. [Implementation Phases](#implementation-phases)
6. [Database Schema](#database-schema)
7. [Backend API (Alakazam)](#backend-api-alakazam)
8. [Frontend (Giratina)](#frontend-giratina)
9. [Arceus Integration](#arceus-integration)
10. [Migration Path](#migration-path)
11. [Technical Details](#technical-details)

---

## Current State

### Existing Tool: SensorManager.py

Currently, sensor management is handled by a standalone Python script `SensorManager.py` that:

**Capabilities:**
- Detects XIAO BLE boards via USB (VID: 0x2886)
- Reads device information (serial number, MAC address)
- Patches firmware with BLE device names
- Flashes firmware to sensors automatically (UF2 bootloader)
- Maintains local JSON database: `C:/Combatica/SensorsDataList.json`

**Local Database Schema (JSON):**
```json
{
  "sensors": [
    {
      "serial_number": "7F7D224BF8A8393B",
      "mac_address": "DD:D3:2C:8F:F1:B9",
      "name": "Sensor_01",
      "firmware_file": "Gyros_v1.2.3.bin",
      "last_updated": "2026-01-12T15:30:00"
    }
  ]
}
```

**Usage:**
```bash
# Flash firmware to sensor
python SensorManager.py upload Gyros.bin --name "Sensor_01"

# Read sensor info
python SensorManager.py info

# List all sensors
python SensorManager.py list
```

**Limitations:**
- ❌ Local only - no cloud sync
- ❌ No central visibility across arcades
- ❌ Manual firmware distribution
- ❌ No version tracking
- ❌ Python dependency

---

## Goal & Vision

### Replace SensorManager.py with Full Integration

**Transition Plan:**
1. ✅ Keep SensorManager.py functionality
2. ➡️ Rewrite in Rust and integrate into Arceus
3. ➡️ Add cloud sync to Alakazam
4. ➡️ Create visual sensor panel in Arceus (Tauri app)
5. ➡️ Display sensor inventory in Giratina (web admin)
6. ❌ Remove SensorManager.py entirely

**End State:**
- **Arceus** handles all sensor operations (flash, configure, monitor)
- **Alakazam** stores all data (firmware versions, sensor inventory)
- **Giratina** provides admin oversight (version management, sensor dashboard)
- **No external scripts needed**

---

## Two Core Concepts

It's critical to understand these are **two separate but related entities**:

### 1. Gyros Versions (Firmware Library)

**What:** Released versions of the Gyros firmware application

**Analogy:** Like Snorlax versions (Android APK releases)

**Properties:**
- `version`: Semantic version (e.g., "1.2.3")
- `gcs_path`: GCS folder path (e.g., "Gyros/1.2.3")
- `file`: Gyros.bin binary file
- `is_current`: Recommended version flag
- `release_date`: When this version was released

**Example Versions:**
```
Gyros v1.0.0 → First release
Gyros v1.1.0 → Added calibration
Gyros v1.2.0 → Fixed IMU drift
Gyros v1.2.3 → Critical bug fix (current)
```

**Managed by:** Admins via Giratina
**Stored in:** Alakazam database + GCS

---

### 2. Physical Sensors (Device Inventory)

**What:** Individual Arduino sensor devices in the field

**Analogy:** Like mobile phones that install apps - each phone has its own identity

**Properties:**
- `serial_number`: Unique hardware ID (e.g., "7F7D224BF8A8393B")
- `mac_address`: BLE MAC address (e.g., "DD:D3:2C:8F:F1:B9")
- `name`: User-assigned label (e.g., "Sensor_01", "Left_Controller")
- `current_version`: Which Gyros firmware it's currently running
- `last_updated`: When it was last flashed

**Example Sensors:**
```
Serial: 7F7D224B → Name: "Sensor_01" → Running: Gyros v1.2.3
Serial: 8A9E1F3C → Name: "Sensor_02" → Running: Gyros v1.1.0
Serial: 2C4D5E6F → Name: "Spare_01"  → Running: Gyros v1.2.3
```

**Managed by:** Arcade operators via Arceus
**Stored in:** Alakazam database

---

## System Architecture

### Component Interaction Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         GIRATINA                            │
│                    (Web Admin Portal)                       │
│                                                             │
│  • Upload Gyros versions to GCS                            │
│  • Set recommended version                                  │
│  • View all sensors across all arcades                      │
│  • Monitor sensor fleet status                              │
└──────────────────────┬──────────────────────────────────────┘
                       │ HTTP API
                       ↓
┌─────────────────────────────────────────────────────────────┐
│                        ALAKAZAM                             │
│                   (Backend Server)                          │
│                                                             │
│  Database Tables:                                           │
│  ┌──────────────────┐     ┌────────────────────┐          │
│  │ gyros_versions   │     │     sensors        │          │
│  ├──────────────────┤     ├────────────────────┤          │
│  │ id               │     │ id                 │          │
│  │ version          │◄────┤ current_version_id │          │
│  │ gcs_path         │     │ serial             │          │
│  │ is_current       │     │ mac_address        │          │
│  │ release_date     │     │ name               │          │
│  └──────────────────┘     │ last_updated       │          │
│                            │ arcade_id (FK)     │          │
│                            └────────────────────┘          │
│                                                             │
│  Services:                                                  │
│  • Signed URL generation for GCS                           │
│  • Gyros version CRUD                                       │
│  • Sensor inventory management                              │
│  • Arcade authentication                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │ HTTP API
                       ↓
┌─────────────────────────────────────────────────────────────┐
│                         ARCEUS                              │
│                   (Arcade Application)                      │
│                                                             │
│  Sensor Panel Features:                                     │
│  ┌────────────────────────────────────────────────┐        │
│  │  Connected Sensors                 [Scan USB] │        │
│  │                                                 │        │
│  │  ┌─────────────────────────────────────────┐  │        │
│  │  │ • Sensor_01                             │  │        │
│  │  │   Serial: 7F7D22...   Version: 1.2.3   │  │        │
│  │  │   [Update] [Configure] [Rename]         │  │        │
│  │  │                                          │  │        │
│  │  │ • Sensor_02 ⚠️ Outdated                 │  │        │
│  │  │   Serial: 8A9E1F...   Version: 1.1.0   │  │        │
│  │  │   [Update to 1.2.3] [Configure]         │  │        │
│  │  └─────────────────────────────────────────┘  │        │
│  │                                                 │        │
│  │  Available Versions: [v1.2.3 ✓] [v1.2.0]      │        │
│  └────────────────────────────────────────────────┘        │
│                                                             │
│  Rust Services:                                             │
│  • USB device detection (replaces Python)                   │
│  • Serial communication with sensors                        │
│  • Firmware flashing (UF2 bootloader)                      │
│  • Download Gyros versions from Alakazam                    │
│  • Report sensor updates to Alakazam                        │
└─────────────────────────────────────────────────────────────┘
                       │ USB
                       ↓
┌─────────────────────────────────────────────────────────────┐
│              PHYSICAL SENSOR (XIAO BLE)                     │
│                                                             │
│  • Runs Gyros firmware                                      │
│  • Reports serial number & MAC via serial commands         │
│  • Accepts firmware updates via UF2 bootloader             │
└─────────────────────────────────────────────────────────────┘
```

---

## Implementation Phases

### Phase 1: Gyros Version Management (MVP)

**Goal:** Replicate Snorlax version system for Gyros firmware

**Scope:**
- ✅ Database table: `gyros_versions`
- ✅ Backend API: Upload, list, delete, set current
- ✅ Giratina page: Gyros versions management
- ✅ GCS integration: Store Gyros.bin files
- ✅ Signed URL generation for downloads

**Deliverables:**
1. Database migration for `gyros_versions` table
2. Alakazam Rust backend:
   - `models/gyros.rs`
   - `repositories/gyros_repo.rs`
   - `services/gyros_service.rs`
   - `api/handlers/admin.rs` (Gyros endpoints)
3. Giratina React frontend:
   - `types/index.ts` (Gyros types)
   - `services/api.ts` (API methods)
   - `hooks/useGyros.ts` (React Query hooks)
   - `components/GyrosModal.tsx` (Upload modal)
   - `pages/GyrosVersionsPage.tsx` (Main page)
   - Navigation menu item

**Out of Scope (Phase 1):**
- ❌ Sensor tracking
- ❌ Arceus integration
- ❌ Firmware flashing
- ❌ USB device detection

**Test Plan:**
1. Upload Gyros v1.0.0 through Giratina
2. Verify file in GCS: `Gyros/1.0.0/Gyros.bin`
3. Set as current version
4. Download signed URL (verify it works)
5. Upload v1.1.0, set as current
6. Delete v1.0.0

---

### Phase 2: Sensor Inventory Tracking

**Goal:** Track physical sensors and their firmware versions

**Scope:**
- ✅ Database table: `sensors`
- ✅ Backend API: Sensor CRUD, update reports
- ✅ Giratina page: Sensor inventory dashboard
- ⚠️ Still using SensorManager.py for flashing

**Deliverables:**
1. Database migration for `sensors` table
2. Alakazam backend:
   - `models/sensor.rs`
   - `repositories/sensor_repo.rs`
   - `services/sensor_service.rs`
   - New API endpoint: `POST /api/arcade/sensors/report-update`
3. Giratina frontend:
   - `pages/SensorsPage.tsx` (Fleet overview)
   - Display all sensors across arcades
   - Filter by version, arcade, status
4. Arceus integration:
   - Call Alakazam API after SensorManager.py completes
   - Report: sensor serial, MAC, name, version

**Workflow:**
1. Arceus downloads Gyros v1.2.3 from Alakazam
2. User runs: `python SensorManager.py upload Gyros.bin --name "Sensor_01"`
3. SensorManager.py flashes sensor, reads serial/MAC
4. Arceus reports to Alakazam:
   ```json
   POST /api/arcade/sensors/report-update
   {
     "serial": "7F7D224BF8A8393B",
     "mac_address": "DD:D3:2C:8F:F1:B9",
     "name": "Sensor_01",
     "version": "1.2.3"
   }
   ```
5. Alakazam updates sensors table
6. Giratina displays updated sensor inventory

---

### Phase 3: Arceus Sensor Panel (Full Integration)

**Goal:** Replace SensorManager.py entirely with Rust implementation in Arceus

**Scope:**
- ✅ USB device detection in Rust
- ✅ Serial communication with XIAO boards
- ✅ UF2 firmware flashing in Rust
- ✅ Visual sensor panel in Arceus (Tauri UI)
- ✅ Automatic updates
- ✅ Configuration UI

**Deliverables:**

1. **Arceus Rust Backend:**
   - `src-tauri/src/infrastructure/usb/` (NEW module)
     - `device_detector.rs` - USB device enumeration
     - `serial_communication.rs` - Serial port communication
     - `xiao_protocol.rs` - XIAO command protocol
   - `src-tauri/src/infrastructure/firmware/` (NEW module)
     - `uf2_converter.rs` - UF2 format conversion
     - `firmware_flasher.rs` - Bootloader upload
     - `name_patcher.rs` - BLE name patching
   - `src-tauri/src/application/services/sensor_service.rs` (NEW)
     - Detect connected sensors
     - Flash firmware
     - Configure sensors
     - Report to Alakazam

2. **Arceus Frontend (Tauri React):**
   - `src/pages/SensorsPage.tsx` (NEW)
     - Visual sensor panel
     - Connected sensors list
     - Update buttons
     - Configuration forms
   - `src/components/sensors/` (NEW directory)
     - `SensorCard.tsx` - Individual sensor display
     - `SensorFlashModal.tsx` - Flash progress
     - `SensorConfigModal.tsx` - Configuration form

3. **Tauri Commands:**
   ```rust
   #[tauri::command]
   async fn scan_connected_sensors() -> Result<Vec<Sensor>>

   #[tauri::command]
   async fn flash_sensor(
       serial: String,
       version: String,
       name: String
   ) -> Result<()>

   #[tauri::command]
   async fn read_sensor_info(serial: String) -> Result<SensorInfo>

   #[tauri::command]
   async fn configure_sensor(
       serial: String,
       config: SensorConfig
   ) -> Result<()>
   ```

**Visual Design (Arceus Sensor Panel):**
```
┌──────────────────────────────────────────────────────────┐
│  Sensors                                    [Scan USB]   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Available Versions:                                     │
│  ┌──────────────────────────────────────────────────┐   │
│  │ Gyros v1.2.3 (Recommended) ✓     [Download]     │   │
│  │ Gyros v1.2.0                      [Download]     │   │
│  └──────────────────────────────────────────────────┘   │
│                                                          │
│  Connected Sensors:                                      │
│  ┌──────────────────────────────────────────────────┐   │
│  │  🟢 Sensor_01                                     │   │
│  │     Serial: 7F7D224BF8A8393B                      │   │
│  │     MAC: DD:D3:2C:8F:F1:B9                        │   │
│  │     Version: 1.2.3 ✓                              │   │
│  │     Status: Up to date                            │   │
│  │     [Rename] [Configure] [Update]                 │   │
│  ├──────────────────────────────────────────────────┤   │
│  │  🟡 Sensor_02                                     │   │
│  │     Serial: 8A9E1F3C2D4B5E6F                      │   │
│  │     MAC: AA:BB:CC:DD:EE:FF                        │   │
│  │     Version: 1.1.0 ⚠️ Outdated                    │   │
│  │     Status: Update available                      │   │
│  │     [Update to v1.2.3] [Configure] [Rename]       │   │
│  └──────────────────────────────────────────────────┘   │
│                                                          │
│  ⓘ Plug in sensor via USB to configure                  │
└──────────────────────────────────────────────────────────┘
```

**Update Flow:**
1. User clicks "Update to v1.2.3" on Sensor_02
2. Arceus downloads Gyros v1.2.3 from Alakazam (signed URL)
3. Progress modal shows:
   - Downloading firmware... (50%)
   - Entering bootloader...
   - Flashing firmware... (75%)
   - Verifying... (90%)
   - Updating database... (100%)
   - ✓ Complete!
4. Sensor card updates to show v1.2.3
5. Alakazam database updated automatically

**Configuration Form:**
```
┌──────────────────────────────────────────────┐
│  Configure Sensor_01                         │
├──────────────────────────────────────────────┤
│  Device Name:                                │
│  [Sensor_01_____________]                    │
│                                              │
│  Calibration:                                │
│  [ ] Auto-calibrate on startup              │
│  Sensitivity: [====•=====] 75%               │
│                                              │
│  Communication:                              │
│  Sample Rate: [100 Hz ▼]                     │
│  BLE Power: [High ▼]                         │
│                                              │
│            [Cancel]  [Save & Flash]          │
└──────────────────────────────────────────────┘
```

---

## Database Schema

### Phase 1: `gyros_versions` Table

```sql
CREATE TABLE gyros_versions (
    id SERIAL PRIMARY KEY,
    version VARCHAR(50) UNIQUE NOT NULL,
    gcs_path VARCHAR(512) NOT NULL,
    release_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_current BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_gyros_versions_is_current ON gyros_versions(is_current);
CREATE INDEX idx_gyros_versions_release_date ON gyros_versions(release_date DESC);

-- Ensure only one current version (optional trigger)
CREATE OR REPLACE FUNCTION ensure_single_current_gyros()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_current = true THEN
        UPDATE gyros_versions SET is_current = false WHERE id != NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER gyros_version_current_trigger
    AFTER INSERT OR UPDATE ON gyros_versions
    FOR EACH ROW
    WHEN (NEW.is_current = true)
    EXECUTE FUNCTION ensure_single_current_gyros();
```

### Phase 2: `sensors` Table

```sql
CREATE TABLE sensors (
    id SERIAL PRIMARY KEY,
    serial VARCHAR(50) UNIQUE NOT NULL,
    mac_address VARCHAR(17) NOT NULL,
    name VARCHAR(255) NOT NULL,
    current_gyros_version_id INT,
    arcade_id INT NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    FOREIGN KEY (current_gyros_version_id) REFERENCES gyros_versions(id) ON DELETE SET NULL,
    FOREIGN KEY (arcade_id) REFERENCES arcades(id) ON DELETE CASCADE
);

CREATE INDEX idx_sensors_serial ON sensors(serial);
CREATE INDEX idx_sensors_arcade_id ON sensors(arcade_id);
CREATE INDEX idx_sensors_version_id ON sensors(current_gyros_version_id);
CREATE INDEX idx_sensors_last_updated ON sensors(last_updated DESC);
```

**Sample Data:**
```sql
-- Gyros versions
INSERT INTO gyros_versions (version, gcs_path, is_current) VALUES
    ('1.0.0', 'Gyros/1.0.0', false),
    ('1.1.0', 'Gyros/1.1.0', false),
    ('1.2.3', 'Gyros/1.2.3', true);

-- Sensors
INSERT INTO sensors (serial, mac_address, name, current_gyros_version_id, arcade_id) VALUES
    ('7F7D224BF8A8393B', 'DD:D3:2C:8F:F1:B9', 'Sensor_01', 3, 1),
    ('8A9E1F3C2D4B5E6F', 'AA:BB:CC:DD:EE:FF', 'Sensor_02', 2, 1);
```

---

## Backend API (Alakazam)

### Phase 1: Gyros Version Endpoints

#### Admin Endpoints (Giratina)

**List all Gyros versions:**
```
GET /api/admin/gyros/versions
Auth: API Key
Response: Vec<GyrosVersion>
```

**Generate signed upload URL:**
```
POST /api/admin/gyros/generate-upload-url
Auth: API Key
Body: { "version": "1.2.3" }
Response: {
  "upload_url": "https://storage.googleapis.com/...",
  "gcs_path": "Gyros/1.2.3"
}
```

**Confirm upload (create DB record):**
```
POST /api/admin/gyros/confirm-upload
Auth: API Key
Body: { "version": "1.2.3", "gcs_path": "Gyros/1.2.3" }
Response: GyrosVersion
```

**Set current version:**
```
PUT /api/admin/gyros/versions/{id}/set-current
Auth: API Key
Response: { "success": true }
```

**Delete version:**
```
DELETE /api/admin/gyros/versions/{id}
Auth: API Key
Response: { "success": true }
```

#### Arcade Endpoints (Arceus)

**Get latest Gyros version:**
```
GET /api/arcade/gyros/latest
Auth: MAC Key Header
Response: {
  "download_url": "https://storage.googleapis.com/...",
  "expires_at": "2026-01-12T16:00:00Z",
  "version": "1.2.3"
}
```

---

### Phase 2: Sensor Endpoints

#### Admin Endpoints (Giratina)

**List all sensors (all arcades):**
```
GET /api/admin/sensors
Auth: API Key
Query: ?arcade_id=1&version=1.2.3
Response: Vec<Sensor>
```

**Get sensor details:**
```
GET /api/admin/sensors/{id}
Auth: API Key
Response: Sensor
```

**Delete sensor:**
```
DELETE /api/admin/sensors/{id}
Auth: API Key
Response: { "success": true }
```

#### Arcade Endpoints (Arceus)

**Report sensor update:**
```
POST /api/arcade/sensors/report-update
Auth: MAC Key Header
Body: {
  "serial": "7F7D224BF8A8393B",
  "mac_address": "DD:D3:2C:8F:F1:B9",
  "name": "Sensor_01",
  "version": "1.2.3"
}
Response: Sensor (created or updated)
```

**List arcade's sensors:**
```
GET /api/arcade/sensors
Auth: MAC Key Header
Response: Vec<Sensor>
```

---

## Frontend (Giratina)

### Phase 1: Gyros Versions Page

**File Structure:**
```
giratina/src/
├── types/index.ts (add GyrosVersion interface)
├── services/api.ts (add Gyros methods)
├── hooks/useGyros.ts (React Query hooks)
├── components/GyrosModal.tsx (upload modal)
├── pages/GyrosVersionsPage.tsx (main page)
└── layouts/MainLayout.tsx (add navigation item)
```

**Page Features:**
- Table showing all Gyros versions
- Upload button → modal
- Set current button (star icon)
- Delete button (only non-current versions)
- Search/filter
- Version tags with "CURRENT" badge

**Upload Flow:**
1. Click "Upload Version"
2. Modal opens with form:
   - Version input (semantic version validation)
   - File upload (.bin files only)
3. Submit → progress modal
4. Upload to GCS via signed URL
5. Confirm upload → create DB record
6. Success → refetch data

---

### Phase 2: Sensors Dashboard

**File Structure:**
```
giratina/src/
├── types/index.ts (add Sensor interface)
├── services/api.ts (add Sensor methods)
├── hooks/useSensors.ts (React Query hooks)
├── pages/SensorsPage.tsx (main dashboard)
└── components/sensors/
    ├── SensorTable.tsx
    └── SensorDetailModal.tsx
```

**Page Features:**
- Table showing all sensors across all arcades
- Columns: Serial, MAC, Name, Version, Arcade, Last Updated
- Filters: Arcade, Version, Status (up-to-date/outdated)
- Search by serial, MAC, name
- Version badge (green if current, yellow if outdated)
- Click sensor → detail modal
- Export to CSV

**Dashboard Statistics:**
```
┌─────────────────────────────────────────────────────┐
│  Total Sensors: 24                                  │
│  Up to date: 20 (83%)    Outdated: 4 (17%)         │
│  Most common version: v1.2.3 (18 sensors)          │
└─────────────────────────────────────────────────────┘
```

---

## Arceus Integration

### Phase 2: API Reporting (with SensorManager.py)

**Workflow:**
1. User runs SensorManager.py to flash sensor
2. Arceus detects JSON file update (`C:/Combatica/SensorsDataList.json`)
3. Arceus reads new/updated sensor from JSON
4. Arceus calls Alakazam API:
   ```rust
   let update = SensorUpdate {
       serial: "7F7D224BF8A8393B".to_string(),
       mac_address: "DD:D3:2C:8F:F1:B9".to_string(),
       name: "Sensor_01".to_string(),
       version: "1.2.3".to_string(),
   };

   client.post("/api/arcade/sensors/report-update")
       .json(&update)
       .send()
       .await?;
   ```

**Implementation:**
- File watcher on `C:/Combatica/SensorsDataList.json`
- Debounced updates (don't spam API)
- Retry logic for failed uploads
- Background sync on Arceus startup

---

### Phase 3: Native Rust Implementation

**USB Detection:**
```rust
// Detect XIAO BLE boards
pub fn detect_xiao_devices() -> Result<Vec<UsbDevice>> {
    let devices = rusb::devices()?;

    devices
        .iter()
        .filter_map(|device| {
            let desc = device.device_descriptor().ok()?;

            // XIAO VID: 0x2886
            if desc.vendor_id() == 0x2886 {
                // Normal mode: PID 0x8045
                // Bootloader: PID 0x0042
                Some(UsbDevice {
                    vendor_id: desc.vendor_id(),
                    product_id: desc.product_id(),
                    mode: if desc.product_id() == 0x0042 {
                        XiaoMode::Bootloader
                    } else {
                        XiaoMode::Normal
                    },
                })
            } else {
                None
            }
        })
        .collect()
}
```

**Serial Communication:**
```rust
// Read sensor information via serial
pub async fn read_sensor_info(port: &str) -> Result<SensorInfo> {
    let mut port = serialport::new(port, 115200)
        .timeout(Duration::from_secs(3))
        .open()?;

    // Send INFO command
    port.write_all(b"INFO\n")?;
    thread::sleep(Duration::from_millis(500));

    let mut buffer = String::new();
    port.read_to_string(&mut buffer)?;

    // Parse response
    parse_sensor_info(&buffer)
}
```

**Firmware Flashing:**
```rust
// Flash firmware to sensor
pub async fn flash_firmware(
    sensor: &Sensor,
    firmware_data: Vec<u8>,
    device_name: &str,
) -> Result<()> {
    // 1. Patch device name
    let patched = patch_device_name(firmware_data, device_name)?;

    // 2. Convert to UF2
    let uf2_data = convert_to_uf2(patched)?;

    // 3. Enter bootloader
    enter_bootloader(&sensor.port)?;

    // 4. Wait for bootloader drive
    let drive = wait_for_bootloader_drive(Duration::from_secs(30))?;

    // 5. Write UF2 file
    let path = PathBuf::from(drive).join("firmware.uf2");
    fs::write(path, uf2_data)?;

    // 6. Wait for completion
    wait_for_flash_complete()?;

    Ok(())
}
```

---

## Migration Path

### Step 1: Phase 1 Implementation (2-3 days)
- [ ] Create `gyros_versions` table migration
- [ ] Implement Alakazam backend (models, repo, service, handlers)
- [ ] Create Giratina Gyros versions page
- [ ] Test upload/download flow
- [ ] Deploy to production

### Step 2: Phase 2 Implementation (3-4 days)
- [ ] Create `sensors` table migration
- [ ] Implement Alakazam backend for sensors
- [ ] Create Giratina sensors dashboard
- [ ] Add API reporting to Arceus (read JSON, call API)
- [ ] Test with SensorManager.py
- [ ] Deploy to production

### Step 3: Phase 3 Implementation (1-2 weeks)
- [ ] Research Rust USB libraries (rusb, serialport)
- [ ] Implement USB device detection
- [ ] Implement serial communication
- [ ] Port UF2 conversion logic
- [ ] Implement firmware flashing
- [ ] Create Arceus sensor panel UI
- [ ] Integration testing
- [ ] User acceptance testing
- [ ] Remove SensorManager.py
- [ ] Update documentation

---

## Technical Details

### XIAO BLE nRF52840 Hardware

**USB Identifiers:**
- Vendor ID: `0x2886` (Seeed Studio)
- Product ID (Normal Mode): `0x8045`
- Product ID (Bootloader): `0x0042`

**Serial Protocol:**
Commands are sent via serial at 115200 baud:
```
SERIAL\n   → Response: SERIAL: 7F7D224BF8A8393B
MAC\n      → Response: MAC: DD:D3:2C:8F:F1:B9
BLEMAC\n   → Response: BLE_MAC: DD:D3:2C:8F:F1:B9
INFO\n     → Response: Multi-line device info
BOOTLOADER\n → Enters UF2 bootloader mode
```

**Bootloader Mode:**
- Device resets and appears as USB mass storage
- Drive contains `INFO_UF2.TXT` and `CURRENT.UF2`
- Copy `.uf2` file to drive to flash firmware
- Device automatically restarts after flashing

**UF2 Format:**
- Block size: 512 bytes
- Payload per block: 256 bytes
- Family ID: `0xADA52840` (nRF52840)
- Application start address: `0x27000`

### Firmware Patching

Gyros firmware contains a placeholder string for BLE device name:
```c
const char device_name[] = "PLACEHOLDER_BLE_NAME_HERE";
```

Before flashing:
1. Search firmware binary for placeholder (27 bytes)
2. Replace with user-provided name (pad with null bytes if shorter)
3. Flash patched firmware

This allows dynamic BLE names without recompiling firmware.

### GCS Storage Structure

```
combatica_alakazam_development/
└── Gyros/
    ├── 1.0.0/
    │   └── Gyros.bin
    ├── 1.1.0/
    │   └── Gyros.bin
    └── 1.2.3/
        └── Gyros.bin
```

Each version gets its own folder for future expansion (release notes, checksums, etc.)

---

## Future Enhancements

### Possible Phase 4+ Features

1. **Bulk Operations**
   - Flash multiple sensors at once
   - Batch rename
   - Rollback to previous version

2. **Sensor Health Monitoring**
   - Battery level
   - Connection quality
   - Error logs
   - Calibration status

3. **Analytics**
   - Sensor usage statistics
   - Firmware adoption rates
   - Update compliance

4. **Advanced Configuration**
   - Custom sensor profiles
   - Per-game configurations
   - Calibration wizard

5. **OTA Updates**
   - Wireless firmware updates over BLE
   - Scheduled updates
   - Auto-update on idle

---

## Summary

This document provides the complete architecture for sensor management in the Arceus ecosystem. The implementation follows a phased approach:

1. **Phase 1:** Gyros version management (like Snorlax)
2. **Phase 2:** Sensor inventory tracking (still using Python tool)
3. **Phase 3:** Full Rust integration (replace Python tool entirely)

The end goal is a seamless, integrated experience where Arceus handles all sensor operations natively, with cloud sync to Alakazam and admin oversight through Giratina.

**No external scripts. No manual processes. Just plug, flash, and play.**
