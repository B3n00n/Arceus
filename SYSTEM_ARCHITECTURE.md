# VR Arcade System Architecture

## Overview
A distributed VR arcade management system deployed globally, managing Oculus Quest devices and game version distribution.

## System Components

### 1. Snorlax (Client APK)
- **Platform**: Oculus Quest devices
- **Mode**: Device Owner APK
- **Purpose**: Client application running on VR headsets
- **Connectivity**: Connects to local Arceus server

### 2. Arceus (Arcade Server)
- **Platform**: PC (Rust Tauri application)
- **Deployment**: One instance per VR arcade
- **Responsibilities**:
  - Manage local Oculus Quest devices (Snorlax clients)
  - Provide version updates for games (both PC and Quest)
  - Download arcade-specific content from Alakazam
  - Run/manage Unity netcode game servers (PC-side)
- **Connectivity**:
  - Serves multiple Snorlax clients (local network)
  - Connects to Alakazam (central server)

### 3. Alakazam (Central Management Server)
- **Platform**: Server application (new)
- **Deployment**: Single central instance
- **Responsibilities**:
  - Store and serve arcade-specific information
  - Manage game version distribution
  - Connect to database for all arcade data
  - Connect to GCS (Google Cloud Storage) for game files
  - Provide version-controlled game downloads to Arceus instances
- **Connectivity**:
  - Serves multiple Arceus instances worldwide
  - Connected to database
  - Connected to GCS

## Deployment Model

```
Global Deployment:
┌─────────────────────────────────────────────────────────┐
│                   Alakazam (Central)                    │
│                 ┌──────────────────┐                    │
│                 │   Database       │                    │
│                 └──────────────────┘                    │
│                 ┌──────────────────┐                    │
│                 │   GCS Storage    │                    │
│                 └──────────────────┘                    │
└────────────┬────────────────────────┬───────────────────┘
             │                        │
             │                        │
    ┌────────▼────────┐      ┌───────▼─────────┐
    │  Arcade #1      │      │  Arcade #2      │  ... (Many arcades worldwide)
    │                 │      │                 │
    │  ┌───────────┐  │      │  ┌───────────┐  │
    │  │  Arceus   │  │      │  │  Arceus   │  │
    │  │  (PC)     │  │      │  │  (PC)     │  │
    │  └─────┬─────┘  │      │  └─────┬─────┘  │
    │        │        │      │        │        │
    │   ┌────┴────┐   │      │   ┌────┴────┐   │
    │   │ Snorlax │   │      │   │ Snorlax │   │
    │   │ Quest 1 │   │      │   │ Quest 1 │   │
    │   └─────────┘   │      │   └─────────┘   │
    │   ┌─────────┐   │      │   ┌─────────┐   │
    │   │ Snorlax │   │      │   │ Snorlax │   │
    │   │ Quest 2 │   │      │   │ Quest 2 │   │
    │   └─────────┘   │      │   └─────────┘   │
    │      ...        │      │      ...        │
    └─────────────────┘      └─────────────────┘
```

## Game Architecture

### Game Example: "Combatica Platform"
Each game consists of two components with the same concept:

1. **PC Server Component**
   - Unity application with Netcode for GameObjects
   - Runs on Arceus PC
   - Serves as game server

2. **Quest Client Component**
   - Unity APK application
   - Runs on Snorlax Quest devices
   - Connects to PC Unity Netcode server for gameplay

### Game Version Management Requirements

- **Per-Arcade Configuration**: Each arcade can run different game versions
- **Version Control**: Alakazam manages which version each arcade should use
- **Dual Distribution**:
  - PC Unity server builds (for Arceus to run)
  - Quest APK builds (for Snorlax devices to install)

## New Feature Requirements

### Version Update System

1. **Alakazam responsibilities**:
   - Store game files in GCS
   - Maintain database of:
     - Available game versions
     - Arcade-specific configurations
     - Which version each arcade should run
   - Serve game files to Arceus instances

2. **Arceus responsibilities**:
   - Query Alakazam for assigned game versions
   - Download PC Unity server builds from Alakazam/GCS
   - Download Quest APK builds from Alakazam/GCS
   - Distribute APKs to connected Snorlax clients
   - Run appropriate version of Unity server

3. **Snorlax responsibilities**:
   - Receive and install APK updates from Arceus
   - Connect to appropriate game server version

## Open Questions

1. **Update Triggers**: How are updates initiated?
   - Automatic polling by Arceus?
   - Push notifications from Alakazam?
   - Manual trigger from admin panel?

2. **Rollback Strategy**: How to handle failed updates?

3. **Partial Updates**: Can games update incrementally or only full downloads?

4. **Admin Interface**: Where is the admin panel for managing versions per arcade?
   - Part of Alakazam?
   - Separate web interface?

5. **Authentication**: How does Arceus authenticate with Alakazam?

6. **Download Resumption**: Support for resuming interrupted downloads?

7. **Storage Management**: How to handle limited storage on Arceus PC?
   - Keep only current version?
   - Keep previous version for rollback?

8. **Update Scheduling**: Can arcades schedule updates for off-hours?
