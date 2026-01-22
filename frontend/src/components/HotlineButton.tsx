import { useState } from 'react';
import { Phone, AlertCircle } from 'lucide-react';
import { HotlineModal } from './HotlineModal';

export function HotlineButton() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  return (
    <>
      {/* Floating Action Button */}
      <button
        onClick={() => setIsModalOpen(true)}
        className="fixed bottom-6 right-6 z-50 group"
        style={{ width: '64px', height: '64px' }}
      >
        {/* Pulse animation ring */}
        <div className="absolute inset-0 bg-red-500 rounded-full animate-ping opacity-75"></div>
        
        {/* Main button */}
        <div className="relative bg-gradient-to-br from-red-500 to-red-600 rounded-full w-16 h-16 flex items-center justify-center shadow-2xl hover:shadow-red-500/50 transition-all hover:scale-110">
          <AlertCircle className="size-8 text-white" />
        </div>

        {/* Tooltip */}
        <div className="absolute bottom-full right-0 mb-2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
          <div className="bg-gray-900 text-white px-4 py-2 rounded-lg text-sm whitespace-nowrap shadow-xl">
            Hotline Insiden MBG
            <div className="absolute top-full right-4 w-0 h-0 border-l-4 border-r-4 border-t-4 border-l-transparent border-r-transparent border-t-gray-900"></div>
          </div>
        </div>
      </button>

      {/* Modal */}
      {isModalOpen && (
        <HotlineModal onClose={() => setIsModalOpen(false)} />
      )}
    </>
  );
}
