import { User } from '../../types';
import { LogOut, User as UserIcon } from 'lucide-react';
import { NotificationPanel, Notification } from '../NotificationPanel';
import logoMBG from 'figma:asset/51e94428001de32b8adf84d690546c9b5cfc362e.png';

interface DashboardHeaderProps {
  user: User;
  notifications?: Notification[];
  onMarkAsRead?: (id: string) => void;
  onMarkAsResolved?: (id: string) => void;
  onViewDetail?: (reviewId: string) => void;
  onFollowUp?: (id: string) => void;
  onLogout: () => void;
}

export function DashboardHeader({ 
  user, 
  notifications = [],
  onMarkAsRead = () => {},
  onMarkAsResolved = () => {},
  onViewDetail = () => {},
  onFollowUp = () => {},
  onLogout 
}: DashboardHeaderProps) {
  const getRoleLabel = (role: string) => {
    switch (role) {
      case 'kitchen': return 'Dapur';
      case 'supplier': return 'Supplier';
      case 'school': return 'Sekolah';
      default: return role;
    }
  };

  return (
    <header className="bg-white border-b border-green-100 sticky top-0 z-40 shadow-sm">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-20">
          {/* Logo and Title */}
          <div className="flex items-center gap-4">
            <img src={logoMBG} alt="Logo MBG" className="w-14 h-14" />
            <div>
              <h1 className="text-green-900 font-[Aclonica]">MonitorMBG</h1>
              <p className="text-sm text-green-600">Makan Bergizi Gratis - HACCP Platform</p>
            </div>
          </div>

          {/* User Info and Actions */}
          <div className="flex items-center gap-4">
            {/* Notifications */}
            <NotificationPanel
              user={user}
              notifications={notifications}
              onMarkAsRead={onMarkAsRead}
              onMarkAsResolved={onMarkAsResolved}
              onViewDetail={onViewDetail}
              onFollowUp={onFollowUp}
            />

            {/* User Profile */}
            <div className="flex items-center gap-3 bg-green-50 pl-3 pr-4 py-2 rounded-xl">
              <div className="p-2 bg-green-100 rounded-lg">
                <UserIcon className="size-5 text-green-600" />
              </div>
              <div>
                <p className="text-sm text-green-900">{user.name}</p>
                <p className="text-xs text-green-600">{getRoleLabel(user.role)} - {user.institutionName}</p>
              </div>
            </div>

            {/* Logout */}
            <button
              onClick={onLogout}
              className="p-2 hover:bg-red-50 rounded-xl transition-colors group"
              title="Logout"
            >
              <LogOut className="size-6 text-green-600 group-hover:text-red-600 transition-colors" />
            </button>
          </div>
        </div>
      </div>
    </header>
  );
}