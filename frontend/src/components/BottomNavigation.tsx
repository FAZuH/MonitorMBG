import { Home, FileText, MapIcon, BookOpen, User } from 'lucide-react';

interface BottomNavigationProps {
  activeTab: 'dashboard' | 'review' | 'map' | 'education' | 'profile';
  onTabChange: (tab: 'dashboard' | 'review' | 'map' | 'education' | 'profile') => void;
}

export function BottomNavigation({ activeTab, onTabChange }: BottomNavigationProps) {
  const tabs = [
    { id: 'dashboard' as const, label: 'Dashboard', icon: Home },
    { id: 'review' as const, label: 'Review', icon: FileText },
    { id: 'map' as const, label: 'Peta Insiden', icon: MapIcon },
    { id: 'education' as const, label: 'Edukasi', icon: BookOpen }
  ];

  return (
    <nav className="fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 shadow-lg z-50 md:hidden">
      <div className="flex items-center justify-around h-16">
        {tabs.map((tab) => {
          const Icon = tab.icon;
          const isActive = activeTab === tab.id;
          
          return (
            <button
              key={tab.id}
              onClick={() => onTabChange(tab.id)}
              className={`flex flex-col items-center justify-center flex-1 h-full transition-colors ${
                isActive 
                  ? 'text-green-600' 
                  : 'text-gray-500 hover:text-green-600'
              }`}
              style={{ minHeight: '44px', minWidth: '44px' }} // Touch target size
            >
              <Icon className={`size-6 mb-1 ${isActive ? 'text-green-600' : 'text-gray-500'}`} />
              <span className={`text-xs ${isActive ? 'text-green-600' : 'text-gray-600'}`}>
                {tab.label}
              </span>
            </button>
          );
        })}
      </div>
    </nav>
  );
}
