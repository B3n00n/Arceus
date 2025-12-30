import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { ConfigProvider, theme, App as AntApp } from 'antd';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { MainLayout } from './layouts/MainLayout';
import { ArcadesPage } from './pages/ArcadesPage';
import { GamesPage } from './pages/GamesPage';
import { GameVersionsPage } from './pages/GameVersionsPage';
import { AssignmentsPage } from './pages/AssignmentsPage';
import { SnorlaxVersionsPage } from './pages/SnorlaxVersionsPage';

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
            colorPrimary: '#dc2626',
            colorInfo: '#dc2626',
            colorSuccess: '#16a34a',
            colorWarning: '#ea580c',
            colorError: '#dc2626',
            colorLink: '#f97316',
            borderRadius: 6,
            fontSize: 14,
            colorBgBase: '#0a0a0a',
            colorTextBase: '#e5e5e5',
          },
          components: {
            Layout: {
              headerBg: '#0a0a0a',
              siderBg: '#141414',
              bodyBg: '#0a0a0a',
            },
            Menu: {
              darkItemBg: '#141414',
              darkSubMenuItemBg: '#0a0a0a',
              darkItemSelectedBg: '#1f1f1f',
            },
            Card: {
              colorBgContainer: '#141414',
            },
            Table: {
              colorBgContainer: '#141414',
              headerBg: '#1a1a1a',
            },
            Modal: {
              contentBg: '#141414',
              headerBg: '#1a1a1a',
            },
            Input: {
              colorBgContainer: '#1a1a1a',
            },
            Select: {
              colorBgContainer: '#1a1a1a',
            },
          },
        }}
      >
        <AntApp>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<MainLayout />}>
                <Route index element={<Navigate to="/arcades" replace />} />
                <Route path="arcades" element={<ArcadesPage />} />
                <Route path="games" element={<GamesPage />} />
                <Route path="versions" element={<GameVersionsPage />} />
                <Route path="assignments" element={<AssignmentsPage />} />
                <Route path="snorlax" element={<SnorlaxVersionsPage />} />
              </Route>
            </Routes>
          </BrowserRouter>
        </AntApp>
      </ConfigProvider>
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  );
}

export default App;
