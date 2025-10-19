import { useEffect } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './components/layout/MainLayout';
import { DashboardPage } from './pages/DashboardPage';
import { DevicesPage } from './pages/DevicesPage';
import { ApkManagerPage } from './pages/ApkManagerPage';
import { SettingsPage } from './pages/SettingsPage';
import { useUIStore } from './stores/uiStore';
import './styles/globals.css';

function App() {
  const setTheme = useUIStore((state) => state.setTheme);
  const theme = useUIStore((state) => state.theme);

  useEffect(() => {
    // Initialize theme
    setTheme(theme);
  }, []);

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<DashboardPage />} />
          <Route path="devices" element={<DevicesPage />} />
          <Route path="apk-manager" element={<ApkManagerPage />} />
          <Route path="settings" element={<SettingsPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
