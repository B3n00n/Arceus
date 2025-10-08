# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Arceus is a Tauri-based desktop application built with React, TypeScript, and Rust. It's an auto-updater application that provides self-updating capabilities for desktop software.

## Architecture

### Frontend (React/TypeScript)
- **Entry Point**: `src/main.tsx` - React application entry point
- **Main App**: `src/App.tsx` - Main application component that displays an update window by default
- **Components**: `src/components/UpdateWindow/` - Update UI components
- **Services**: `src/services/updateService.ts` - Frontend service for communicating with Tauri backend
- **Types**: `src/types/update.types.ts` - TypeScript type definitions for update-related data

### Backend (Rust/Tauri)
- **Entry Point**: `src-tauri/src/main.rs` - Calls the library's run function
- **Library**: `src-tauri/src/lib.rs` - Main Tauri application setup with plugin initialization
- **Commands**: `src-tauri/src/commands/update_commands.rs` - Tauri commands exposed to frontend
- **Services**: `src-tauri/src/services/update_service.rs` - Backend update service implementation
- **Models**: `src-tauri/src/core/models/update.rs` - Rust data structures for update functionality

### Update System
The application uses Tauri's updater plugin to check for updates from GitHub releases. The frontend communicates with the backend through Tauri's invoke system and listens for update status events.

## Development Commands

### Frontend Development
- `npm run dev` - Start Vite development server
- `npm run build` - Build frontend for production (TypeScript compilation + Vite build)
- `npm run preview` - Preview production build

### Tauri Development
- `npm run tauri dev` - Start Tauri development mode (builds Rust backend + starts frontend)
- `npm run tauri build` - Build complete application for distribution

### Tauri CLI
- `npm run tauri` - Access Tauri CLI commands directly

## Key Configuration Files

- `tauri.conf.json` - Main Tauri configuration including updater endpoints
- `package.json` - Frontend dependencies and scripts
- `src-tauri/Cargo.toml` - Rust dependencies and project metadata
- `vite.config.ts` - Vite bundler configuration

## Update Configuration

The application is configured to check for updates from:
- GitHub repository: `B3n00n/arceus`
- Endpoint: `https://api.github.com/repos/B3n00n/arceus/releases/latest`
- Uses public key verification (currently set to "TEMP" placeholder)

## Development Notes

- The application shows an update window by default on startup
- Update checking is handled asynchronously with proper error handling
- The frontend uses modern React patterns (hooks, TypeScript)
- Backend uses Rust async/await with proper state management through Tauri's State system