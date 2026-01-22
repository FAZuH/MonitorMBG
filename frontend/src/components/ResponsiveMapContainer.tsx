import { useState, useEffect, useRef } from 'react';
import { MapIcon, ZoomIn, ZoomOut, Layers, X, ExternalLink } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './ui/tooltip';

interface ResponsiveMapContainerProps {
  children: React.ReactNode;
  statistics?: React.ReactNode;
  sidebar?: React.ReactNode;
  onZoomIn?: () => void;
  onZoomOut?: () => void;
  onToggleLayer?: () => void;
  fullMapUrl?: string; // URL for opening full map in new tab
}

export function ResponsiveMapContainer({
  children,
  statistics,
  sidebar,
  onZoomIn,
  onZoomOut,
  onToggleLayer,
  fullMapUrl,
}: ResponsiveMapContainerProps) {
  const [isMobile, setIsMobile] = useState(false);
  const [showMobileSidebar, setShowMobileSidebar] = useState(false);
  const [isMapVisible, setIsMapVisible] = useState(false);
  const mapContainerRef = useRef<HTMLDivElement>(null);

  // Detect mobile viewport
  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };

    checkMobile();
    window.addEventListener('resize', checkMobile);
    
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  // Lazy load map using Intersection Observer
  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            setIsMapVisible(true);
          }
        });
      },
      {
        rootMargin: '100px', // Start loading 100px before map enters viewport
        threshold: 0.01,
      }
    );

    if (mapContainerRef.current) {
      observer.observe(mapContainerRef.current);
    }

    return () => {
      if (mapContainerRef.current) {
        observer.unobserve(mapContainerRef.current);
      }
    };
  }, []);

  return (
    <>
      {/* Statistics - Always visible */}
      {statistics && (
        <div className="mb-6">
          {statistics}
        </div>
      )}

      {/* Main Map Container */}
      <div className="relative">
        {/* Desktop: Side-by-side layout */}
        <div className={`flex ${isMobile ? 'flex-col' : 'gap-6'}`}>
          {/* Map Area */}
          <div 
            ref={mapContainerRef}
            className={`
              relative rounded-xl overflow-hidden border-2 border-gray-200 shadow-lg bg-white
              ${isMobile ? 'w-full' : 'flex-1'}
            `}
            style={isMobile ? {
              width: '100vw',
              maxWidth: '100%',
              marginLeft: 'calc(-1rem - env(safe-area-inset-left))',
              marginRight: 'calc(-1rem - env(safe-area-inset-right))',
              paddingLeft: 'env(safe-area-inset-left)',
              paddingRight: 'env(safe-area-inset-right)',
              borderRadius: isMobile ? '12px 12px 0 0' : '0.75rem',
            } : undefined}
          >
            {/* Lazy-loaded map content */}
            {isMapVisible ? (
              children
            ) : (
              <div className="flex items-center justify-center h-96 bg-gray-50">
                <div className="text-center">
                  <MapIcon className="mx-auto size-12 text-gray-400 animate-pulse mb-2" />
                  <p className="text-gray-500 text-sm">Memuat peta...</p>
                </div>
              </div>
            )}

            {/* Floating Controls (Mobile Only) */}
            {isMobile && (
              <div className="absolute bottom-4 right-4 flex flex-col gap-2 z-20">
                {/* Zoom In */}
                {onZoomIn && (
                  <button
                    onClick={onZoomIn}
                    className="min-w-[48px] min-h-[48px] bg-white hover:bg-gray-50 rounded-xl shadow-lg border border-gray-200 flex items-center justify-center touch-manipulation transition-colors active:scale-95"
                    aria-label="Zoom in"
                  >
                    <ZoomIn className="size-5 text-gray-700" />
                  </button>
                )}

                {/* Zoom Out */}
                {onZoomOut && (
                  <button
                    onClick={onZoomOut}
                    className="min-w-[48px] min-h-[48px] bg-white hover:bg-gray-50 rounded-xl shadow-lg border border-gray-200 flex items-center justify-center touch-manipulation transition-colors active:scale-95"
                    aria-label="Zoom out"
                  >
                    <ZoomOut className="size-5 text-gray-700" />
                  </button>
                )}

                {/* Toggle Layer */}
                {onToggleLayer && (
                  <button
                    onClick={onToggleLayer}
                    className="min-w-[48px] min-h-[48px] bg-white hover:bg-gray-50 rounded-xl shadow-lg border border-gray-200 flex items-center justify-center touch-manipulation transition-colors active:scale-95"
                    aria-label="Toggle layers"
                  >
                    <Layers className="size-5 text-gray-700" />
                  </button>
                )}
              </div>
            )}

            {/* Show Sidebar Button (Mobile Only) */}
            {isMobile && sidebar && (
              <button
                onClick={() => setShowMobileSidebar(true)}
                className="absolute top-4 right-4 min-w-[48px] min-h-[48px] bg-green-600 hover:bg-green-700 text-white rounded-xl shadow-lg flex items-center justify-center gap-2 px-4 touch-manipulation transition-colors z-20"
                style={{
                  paddingRight: 'calc(1rem + env(safe-area-inset-right))',
                }}
              >
                <MapIcon className="size-5" />
                <span className="text-sm font-medium">Detail</span>
              </button>
            )}
          </div>

          {/* Desktop Sidebar */}
          {!isMobile && sidebar && (
            <div className="w-80">
              {sidebar}
            </div>
          )}
        </div>

        {/* Full Map Button - Below the map */}
        {fullMapUrl && (
          <div className={`mt-4 ${isMobile ? '' : 'flex justify-center'}`}>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <a
                    href={fullMapUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className={`
                      inline-flex items-center justify-center gap-3
                      bg-green-600 hover:bg-green-700 active:bg-green-800
                      text-white
                      rounded-xl shadow-lg hover:shadow-xl
                      transition-all duration-200
                      touch-manipulation
                      group
                      ${isMobile 
                        ? 'w-full min-h-[48px] px-6' 
                        : 'px-8 py-3'
                      }
                    `}
                    style={isMobile ? {
                      width: '100%',
                    } : undefined}
                  >
                    <ExternalLink className={`${isMobile ? 'size-5' : 'size-5'} transition-transform group-hover:scale-110`} />
                    <span className={`${isMobile ? 'text-base' : 'text-sm'} font-medium`}>
                      Lihat Peta Layar Penuh
                    </span>
                  </a>
                </TooltipTrigger>
                <TooltipContent 
                  className="hidden md:block"
                  side="top"
                >
                  <p>Buka peta pada tab baru</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
        )}

        {/* Mobile Bottom Sheet Sidebar */}
        {isMobile && sidebar && showMobileSidebar && (
          <>
            {/* Overlay */}
            <div
              className="fixed inset-0 bg-black/50 z-40 animate-in fade-in duration-200"
              onClick={() => setShowMobileSidebar(false)}
            />

            {/* Bottom Sheet */}
            <div className="fixed inset-x-0 bottom-0 z-50 bg-white rounded-t-3xl shadow-2xl max-h-[85vh] overflow-y-auto animate-in slide-in-from-bottom duration-300"
              style={{
                paddingLeft: 'env(safe-area-inset-left)',
                paddingRight: 'env(safe-area-inset-right)',
                paddingBottom: 'env(safe-area-inset-bottom)',
              }}
            >
              {/* Handle bar */}
              <div className="flex justify-center pt-3 pb-2 sticky top-0 bg-white z-10">
                <div className="w-12 h-1.5 bg-gray-300 rounded-full" />
              </div>

              {/* Close button */}
              <button
                onClick={() => setShowMobileSidebar(false)}
                className="absolute top-4 right-4 p-2 bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
                aria-label="Close"
              >
                <X className="size-5 text-gray-600" />
              </button>

              {/* Sidebar Content */}
              <div className="px-4 pb-6">
                {sidebar}
              </div>
            </div>
          </>
        )}
      </div>
    </>
  );
}
