import { lazy, Suspense } from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { ConfigProvider, theme, App as AntApp, Spin } from 'antd';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { MainLayout } from './layouts/MainLayout';

const ArcadesPage = lazy(() => import('./pages/ArcadesPage').then(m => ({ default: m.ArcadesPage })));
const CustomersPage = lazy(() => import('./pages/CustomersPage').then(m => ({ default: m.CustomersPage })));
const GamesPage = lazy(() => import('./pages/GamesPage').then(m => ({ default: m.GamesPage })));
const GameVersionsPage = lazy(() => import('./pages/GameVersionsPage').then(m => ({ default: m.GameVersionsPage })));
const ChannelsPage = lazy(() => import('./pages/ChannelsPage').then(m => ({ default: m.ChannelsPage })));
const SnorlaxVersionsPage = lazy(() => import('./pages/SnorlaxVersionsPage').then(m => ({ default: m.SnorlaxVersionsPage })));
const GyrosVersionsPage = lazy(() => import('./pages/GyrosVersionsPage').then(m => ({ default: m.GyrosVersionsPage })));
const SensorsPage = lazy(() => import('./pages/SensorsPage').then(m => ({ default: m.SensorsPage })));

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchOnWindowFocus: false,
      retry: 1,
      staleTime: 5000,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ConfigProvider
        theme={{
          algorithm: theme.darkAlgorithm,
          token: {
            // Very dark blue color palette
            colorPrimary: '#1e3a8a', // Very dark blue
            colorInfo: '#1e40af', // Dark blue
            colorSuccess: '#10b981', // Emerald
            colorWarning: '#f59e0b', // Amber
            colorError: '#ef4444', // Red
            colorLink: '#2563eb', // Dark blue link
            borderRadius: 8,
            fontSize: 14,
            fontFamily:
              '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
            // Very dark theme
            colorBgBase: '#0a0a0a', // Almost black
            colorTextBase: '#f1f5f9', // Slate 100
            colorBorder: '#1e293b', // Very dark slate
            colorBorderSecondary: '#0f172a', // Almost black
            lineWidth: 1,
            controlHeight: 36,
            controlHeightLG: 42,
          },
          components: {
            Layout: {
              headerBg: '#0a0a0a',
              headerPadding: '0 24px',
              siderBg: '#0f0f0f',
              bodyBg: '#0a0a0a',
            },
            Menu: {
              darkItemBg: '#0f0f0f',
              darkSubMenuItemBg: '#0a0a0a',
              darkItemSelectedBg: '#1a1a2e',
              darkItemHoverBg: '#1a1a2e',
              itemMarginInline: 8,
              itemBorderRadius: 6,
            },
            Card: {
              colorBgContainer: '#0f0f0f',
              paddingLG: 24,
              boxShadowTertiary: '0 1px 3px 0 rgb(0 0 0 / 0.5), 0 1px 2px -1px rgb(0 0 0 / 0.5)',
            },
            Table: {
              colorBgContainer: '#0f0f0f',
              headerBg: '#1a1a1a',
              headerColor: '#f1f5f9',
              rowHoverBg: '#1a1a2e',
              borderColor: '#1e1e1e',
              headerSplitColor: '#2a2a2a',
              cellPaddingBlock: 12,
              cellPaddingInline: 16,
              fontSize: 14,
            },
            Modal: {
              contentBg: '#0f0f0f',
              headerBg: 'transparent',
              titleColor: '#f1f5f9',
              titleFontSize: 18,
            },
            Input: {
              colorBgContainer: '#0a0a0a',
              colorBorder: '#2a2a2a',
              hoverBorderColor: '#3a3a3a',
              activeBorderColor: '#1e3a8a',
              paddingBlock: 8,
              paddingInline: 12,
            },
            Select: {
              colorBgContainer: '#0a0a0a',
              colorBorder: '#2a2a2a',
              controlOutline: 'rgba(30, 58, 138, 0.3)',
            },
            Button: {
              controlHeight: 36,
              controlHeightLG: 42,
              primaryShadow: '0 2px 0 rgba(30, 58, 138, 0.2)',
              fontWeight: 500,
            },
            Tag: {
              defaultBg: '#1a1a1a',
              defaultColor: '#e2e8f0',
            },
          },
        }}
      >
        <AntApp>
          <BrowserRouter>
            <Suspense fallback={<div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}><Spin size="large" /></div>}>
              <Routes>
                <Route path="/" element={<MainLayout />}>
                  <Route index element={<Navigate to="/customers" replace />} />
                  <Route path="customers" element={<CustomersPage />} />
                  <Route path="arcades" element={<ArcadesPage />} />
                  <Route path="games" element={<GamesPage />} />
                  <Route path="versions" element={<GameVersionsPage />} />
                  <Route path="channels" element={<ChannelsPage />} />
                  <Route path="snorlax" element={<SnorlaxVersionsPage />} />
                  <Route path="gyros" element={<GyrosVersionsPage />} />
                  <Route path="sensors" element={<SensorsPage />} />
                </Route>
              </Routes>
            </Suspense>
          </BrowserRouter>
        </AntApp>
      </ConfigProvider>
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  );
}

export default App;
