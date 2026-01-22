import { X, Maximize2, Minimize2 } from 'lucide-react';
import { useEffect, useState } from 'react';

interface MobileMapModalProps {
  isOpen: boolean;
  onClose: () => void;
  children: React.ReactNode;
}

export function MobileMapModal({ isOpen, onClose, children }: MobileMapModalProps) {
  const [isClosing, setIsClosing] = useState(false);
  const [touchStart, setTouchStart] = useState(0);
  const [touchEnd, setTouchEnd] = useState(0);
  const [forceDesktopView, setForceDesktopView] = useState(false);

  // Handle escape key
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        handleClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      // Prevent body scroll when modal is open
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = 'unset';
    };
  }, [isOpen]);

  const handleClose = () => {
    setIsClosing(true);
    setTimeout(() => {
      setIsClosing(false);
      onClose();
    }, 300); // Match animation duration
  };

  // Handle swipe down to close
  const handleTouchStart = (e: React.TouchEvent) => {
    setTouchStart(e.targetTouches[0].clientY);
  };

  const handleTouchMove = (e: React.TouchEvent) => {
    setTouchEnd(e.targetTouches[0].clientY);
  };

  const handleTouchEnd = () => {
    if (touchStart - touchEnd < -150) {
      // Swiped down more than 150px
      handleClose();
    }
  };

  if (!isOpen && !isClosing) return null;

  return (
    <>
      {/* Overlay */}
      <div
        className={`fixed inset-0 bg-black/60 backdrop-blur-sm z-50 transition-opacity duration-300 ${
          isClosing ? 'opacity-0' : 'opacity-100'
        }`}
        onClick={handleClose}
        role="presentation"
      />

      {/* Modal Container */}
      <div
        className={`fixed inset-0 z-50 flex items-end md:items-center justify-center p-0 md:p-4 pointer-events-none`}
        role="dialog"
        aria-modal="true"
        aria-labelledby="map-modal-title"
      >
        <div
          className={`
            bg-white rounded-t-3xl md:rounded-2xl shadow-2xl 
            w-full md:max-w-6xl 
            h-[95vh] md:h-[90vh] 
            flex flex-col
            pointer-events-auto
            transform transition-transform duration-300 ease-out
            ${isClosing ? 'translate-y-full md:translate-y-0 md:scale-95 md:opacity-0' : 'translate-y-0 md:scale-100 md:opacity-100'}
          `}
          onTouchStart={handleTouchStart}
          onTouchMove={handleTouchMove}
          onTouchEnd={handleTouchEnd}
        >
          {/* Header with swipe indicator */}
          <div className="flex-shrink-0 bg-gradient-to-r from-green-600 to-emerald-700 px-4 md:px-6 py-4 rounded-t-3xl md:rounded-t-2xl" style={{
            paddingLeft: 'max(1rem, env(safe-area-inset-left))',
            paddingRight: 'max(1rem, env(safe-area-inset-right))',
            paddingTop: 'max(1rem, env(safe-area-inset-top))'
          }}>
            {/* Swipe indicator for mobile */}
            <div className="md:hidden flex justify-center mb-3">
              <div className="w-12 h-1.5 bg-white/40 rounded-full" />
            </div>

            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className="p-2 bg-white/20 rounded-lg">
                  <Maximize2 className="size-5 text-white" />
                </div>
                <div>
                  <h2 id="map-modal-title" className="text-white font-bold text-lg">
                    Peta Insiden Keracunan Makanan
                  </h2>
                  <p className="text-green-100 text-sm">Monitoring real-time seluruh Indonesia</p>
                </div>
              </div>

              {/* Close button */}
              <button
                onClick={handleClose}
                className="p-2.5 bg-white/20 hover:bg-white/30 rounded-xl transition-colors touch-manipulation min-w-[44px] min-h-[44px] flex items-center justify-center"
                aria-label="Tutup peta"
              >
                <X className="size-6 text-white" />
              </button>
            </div>

            {/* Force Desktop View Toggle */}
            <div className="mt-3 flex items-center justify-between bg-white/10 rounded-lg px-3 py-2">
              <span className="text-white text-sm">Mode Desktop (Inspector)</span>
              <button
                onClick={() => setForceDesktopView(!forceDesktopView)}
                className={`relative w-14 h-7 rounded-full transition-colors touch-manipulation ${
                  forceDesktopView ? 'bg-green-400' : 'bg-white/30'
                }`}
                role="switch"
                aria-checked={forceDesktopView}
                aria-label="Toggle desktop view mode"
              >
                <div
                  className={`absolute top-1 left-1 w-5 h-5 bg-white rounded-full transition-transform ${
                    forceDesktopView ? 'translate-x-7' : 'translate-x-0'
                  }`}
                />
              </button>
            </div>
          </div>

          {/* Map Content */}
          <div className="flex-1 overflow-hidden relative">
            <div className={`h-full w-full ${forceDesktopView ? 'overflow-auto' : 'overflow-hidden'}`}>
              {children}
            </div>
          </div>

          {/* Footer hint */}
          <div className="flex-shrink-0 bg-gray-50 px-4 py-3 border-t border-gray-200 md:hidden">
            <p className="text-xs text-gray-600 text-center">
              💡 Geser ke bawah untuk menutup • Pinch untuk zoom
            </p>
          </div>
        </div>
      </div>
    </>
  );
}