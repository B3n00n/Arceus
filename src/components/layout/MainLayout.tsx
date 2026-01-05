import { Outlet } from 'react-router-dom';
import { Sidebar } from './Sidebar';
import { Header } from './Header';
import { useUIStore } from '@/stores/uiStore';
import { cn } from '@/lib/cn';
import { Toaster } from 'sonner';

export function MainLayout() {
  const sidebarCollapsed = useUIStore((state) => state.sidebarCollapsed);

  return (
    <div className="min-h-screen bg-grey-900">
      <Sidebar />
      <div
        className={cn(
          'transition-all duration-300',
          sidebarCollapsed ? 'pl-16' : 'pl-64'
        )}
      >
        <Header />
        <main>
          <Outlet />
        </main>
      </div>
      <Toaster theme="dark" position="top-right" richColors offset={{ top: '75px' }}
 />
    </div>
  );
}
