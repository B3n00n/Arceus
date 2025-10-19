import { Link, useLocation } from 'react-router-dom';
import {
  LayoutDashboard,
  Glasses,
  Package,
  Settings,
  ChevronLeft,
  ChevronRight
} from 'lucide-react';
import { cn } from '@/lib/cn';
import { useUIStore } from '@/stores/uiStore';

const navigation = [
  { name: 'Dashboard', href: '/', icon: LayoutDashboard },
  { name: 'Devices', href: '/devices', icon: Glasses },
  { name: 'APK Manager', href: '/apk-manager', icon: Package },
  { name: 'Settings', href: '/settings', icon: Settings },
];

export function Sidebar() {
  const location = useLocation();
  const { sidebarCollapsed, toggleSidebar } = useUIStore();

  return (
    <aside
      className={cn(
        'fixed left-0 top-0 z-40 h-screen bg-discord-dark-3 border-r border-discord-dark-2 transition-all duration-300',
        sidebarCollapsed ? 'w-16' : 'w-64'
      )}
    >
      <div className="flex h-full flex-col">
        {/* Header */}
        <div className="flex h-16 items-center justify-between px-4 border-b border-discord-dark-2">
          {!sidebarCollapsed && (
            <span className="text-white font-semibold text-lg">Arceus</span>
          )}
          <button
            onClick={toggleSidebar}
            className="p-1.5 rounded-md hover:bg-discord-dark-2 text-gray-400 hover:text-white transition-colors ml-auto"
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
                  'flex items-center gap-3 rounded-md px-3 py-2.5 text-sm font-medium transition-colors group',
                  isActive
                    ? 'bg-discord-blurple text-white'
                    : 'text-gray-300 hover:bg-discord-dark-2 hover:text-white'
                )}
                title={sidebarCollapsed ? item.name : undefined}
              >
                <Icon size={20} />
                {!sidebarCollapsed && <span>{item.name}</span>}
              </Link>
            );
          })}
        </nav>

        {/* Footer */}
        <div className="border-t border-discord-dark-2 p-4">
          <div className={cn(
            'flex items-center justify-center',
            sidebarCollapsed && 'flex-col gap-1'
          )}>
            {!sidebarCollapsed ? (
              <p className="text-sm font-medium text-white">Arceus v0.1.2</p>
            ) : (
              <div className="h-10 w-10 rounded-full bg-discord-dark flex items-center justify-center">
                <span className="text-[10px] text-gray-400">v0.1.2</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </aside>
  );
}
