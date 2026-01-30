# Frontend Development Guide

This guide covers development workflows, architecture, and best practices for the MonitorMBG frontend.

## Quick Start with dev.sh

We provide a convenient development script at `/dev.sh` for common tasks:

```bash
# Run tests
./dev.sh frontend test

# Format code
./dev.sh frontend format

# Run linter
./dev.sh frontend lint

# Build for production
./dev.sh frontend build
```

See `./dev.sh help` for all available commands and options.

## Prerequisites

- Node.js 18+
- npm 9+

## Getting Started

### 1. Installation

```bash
cd frontend
npm install
```

### 2. Run Development Server

```bash
cd frontend
npm run dev
```

The application will start at `http://localhost:5173`.

## Architecture Overview

The frontend is a Single Page Application (SPA) built with React and TypeScript. It uses a component-based architecture with local state management for simplicity and performance.

![Frontend Architecture](./diagrams/frontend-architecture.png)

### Key Components

1.  **App Controller (`App.tsx`)**
    - Acts as the main controller and router.
    - Manages global application state (`authState`, `currentUser`).
    - Handles top-level navigation logic.

2.  **View Layer**
    - **Public Views**: Accessible without login (`PublicDashboard`, `IncidentMap`).
    - **Auth Views**: Login and Registration pages.
    - **Authenticated Layout**: The main dashboard interface for logged-in users.

3.  **Feature Components**
    - **Kitchen**: `KitchenList`, `KitchenDetail`
    - **Reviews**: `ReviewForm`, `ReviewCard`
    - **Map**: `IncidentMap`, `LocationFilter`
    - **Dashboard**: `DashboardHeader`, `NotificationPanel`

### State Management

The application uses React's `useState` for local state management. Data flows down from `App.tsx` to child components via props.

- **Auth State**: Controls which view is rendered (`public`, `login`, `authenticated`).
- **User Data**: Stores current user profile and role.
- **Application Data**: Stores kitchens, reviews, and notifications.

## UI Flow

The application follows a clear flow between public and authenticated states.

![UI Flow](./diagrams/ui-flow.png)

### Navigation States

1.  **Public State**:
    - Users can view the landing page, incident map, and search for kitchens.
    - Limited interactivity (read-only).

2.  **Authentication**:
    - Users can login or register.
    - Successful authentication transitions to the Authenticated State.

3.  **Authenticated State**:
    - Access to full dashboard features.
    - Can submit reviews, view detailed reports, and manage profile.
    - Navigation via tabs (Reviews, Incidents) and header actions.

## Development Guidelines

### Component Structure

- **`src/components/ui/`**: Reusable atomic components (Buttons, Inputs). Do not modify these unless updating the design system.
- **`src/components/{feature}/`**: Feature-specific components.
- **`src/components/`**: Shared business components.

### Styling

- Use **Tailwind CSS** for all styling.
- Avoid inline styles.
- Use `className` prop for custom classes.
- Follow the design system defined in `globals.css`.

### Testing

Run tests using:
```bash
./dev.sh frontend test
```

- Write unit tests for complex logic.
- Test components that have significant user interaction.
