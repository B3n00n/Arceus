# Giratina - Alakazam Admin Panel

Administrative web interface for managing Alakazam backend operations.

## Features

- **Arcade Management**: Create, edit, and delete arcade machines
- **Game Management**: Manage game entries and metadata
- **Version Management**: Upload and track game versions
- **Assignment Management**: Assign specific game versions to arcades
- **Snorlax Versions**: Manage Snorlax launcher versions

## Tech Stack

- **React** with **TypeScript**
- **Vite** for fast development
- **Ant Design** for UI components
- **React Router** for navigation
- **Axios** for API calls

## Prerequisites

- Node.js 18+
- npm or yarn
- Access to Alakazam backend API

## Setup

1. Install dependencies:
```bash
npm install
```

2. Configure environment variables:
```bash
cp .env.example .env
```

Edit `.env` and set the Alakazam API URL:
```
VITE_ALAKAZAM_API_URL=http://localhost:8080
```

3. Start development server:
```bash
npm run dev
```

The app will be available at `http://localhost:5173`

## Building for Production

```bash
npm run build
```

The production build will be in the `dist/` directory.

## Project Structure

```
giratina/
├── src/
│   ├── components/     # Reusable React components
│   ├── layouts/        # Layout components (MainLayout)
│   ├── pages/          # Page components
│   │   ├── ArcadesPage.tsx
│   │   ├── GamesPage.tsx
│   │   ├── GameVersionsPage.tsx
│   │   ├── AssignmentsPage.tsx
│   │   └── SnorlaxVersionsPage.tsx
│   ├── services/       # API client and services
│   │   └── api.ts
│   ├── types/          # TypeScript type definitions
│   │   └── index.ts
│   ├── App.tsx         # Main app component with routing
│   └── main.tsx        # Entry point
├── .env                # Environment variables (git-ignored)
├── .env.example        # Example environment variables
└── package.json
```

## Authentication

Giratina relies on Google Cloud Identity-Aware Proxy (IAP) for authentication. The backend expects the `X-Goog-Authenticated-User-Email` header to be set by IAP. For local development without IAP, you may need to configure the backend to bypass IAP checks.

## Development Status

Currently, the basic structure is in place with placeholder pages. Next steps:
- Implement CRUD operations for each entity
- Add form modals for create/edit operations
- Implement file upload for game versions
- Add data tables with search and filters
- Implement assignment management UI
