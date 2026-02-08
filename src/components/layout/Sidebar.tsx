import { Link, useLocation } from 'react-router-dom';
import {
  LayoutDashboard,
  Package,
  Settings,
  ChevronLeft,
  ChevronRight,
  RectangleGoggles,
  Cpu
} from 'lucide-react';
import { cn } from '@/lib/cn';
import { useUIStore } from '@/stores/uiStore';
import { useEffect, useState } from 'react';
import { getVersion } from '@tauri-apps/api/app';

const navigation = [
  { name: 'Dashboard', href: '/', icon: LayoutDashboard },
  { name: 'Devices', href: '/devices', icon: RectangleGoggles },
  { name: 'Sensors', href: '/sensors', icon: Cpu },
  { name: 'APK Manager', href: '/apk-manager', icon: Package },
  { name: 'Settings', href: '/settings', icon: Settings },
];

export function Sidebar() {
  const location = useLocation();
  const { sidebarCollapsed, toggleSidebar } = useUIStore();
  const [version, setVersion] = useState<string>('');

  useEffect(() => {
    getVersion().then(setVersion);
  }, []);

  return (
    <aside
      className={cn(
        'fixed left-0 top-0 z-40 h-screen border-r border-grey-700 transition-all duration-300 box-content',
        sidebarCollapsed ? 'w-16' : 'w-64'
      )}
    >
      <div className="flex h-full flex-col">
        {/* Header */}
        <div className="flex h-16 items-center justify-between px-4 border-grey-700">
          {!sidebarCollapsed && (
            <span className="text-white font-semibold text-lg text-nowrap">Combatica Hub</span>
          )}
          <button
            onClick={toggleSidebar}
            className="p-1.5 rounded-md text-grey-300 hover:bg-grey-700 hover:text-white active:bg-grey-600 transition-colors ml-auto"
          >
            {sidebarCollapsed ? <ChevronRight size={20} /> : <ChevronLeft size={20} />}
          </button>
        </div>

        {/* Navigation */}
        <nav className="flex-1 overflow-y-auto px-2 py-4 space-y-1">
          {navigation.map((item) => {
            const isActive = location.pathname === item.href;
            const Icon = item.icon;

            return (
              <Link
                key={item.name}
                to={item.href}
                className={cn(
                  'flex items-center rounded-md h-12 text-sm font-medium transition-all duration-300 group overflow-hidden px-3 gap-3',
                
                  isActive
                    ? 'bg-grey-600 text-white font-bold'
                    : 'text-grey-200 hover:bg-grey-700 hover:text-white'
                )}
                title={sidebarCollapsed ? item.name : undefined}
              >
                <Icon size={24} className="shrink-0" />
                <span
                  className={cn(
                    'whitespace-nowrap overflow-hidden transition-all duration-300',
                    sidebarCollapsed ? 'max-w-0 opacity-0 ml-0' : 'max-w-[160px] opacity-100 ml-1'
                  )}
                >
                  {item.name}
                </span>
              </Link>
            );
          })}
        </nav>

        {/* Footer */}
        <div className="border-t border-grey-700 p-4 h-[53px]">
          <div className={cn(
            'flex items-center justify-center',
            sidebarCollapsed && 'flex-col gap-1'
          )}>
            {!sidebarCollapsed && version && (
              <span className="text-sm font-medium text-white whitespace-nowrap overflow-hidden">Hub v{version}</span>
            )}
          </div>
        </div>
      </div>
    </aside>
  );
}
