import type { JSX } from 'preact';
import { useEffect, useRef, useState } from 'preact/hooks';
import { currentDesktopChrome, hasNativeDesktopRuntime, windowSetDecorations, windowSetShadow } from '../native';
import { errMessage } from '../utils';
import { Button, Toggle } from './Atoms';

interface DiagnosticOptions {
  stableScrollbar: boolean;
  forcedScrollbar: boolean;
  disableMotion: boolean;
  disableEffects: boolean;
  containView: boolean;
  squareContent: boolean;
}

interface ResizeMetrics {
  viewport: string;
  document: string;
  viewClient: string;
  viewScroll: string;
  scrollbar: string;
  overflow: string;
  resizeEvents: number;
  viewResizeEvents: number;
  layoutShift: number;
  dpr: number;
}

const defaultOptions: DiagnosticOptions = {
  stableScrollbar: false,
  forcedScrollbar: false,
  disableMotion: false,
  disableEffects: false,
  containView: false,
  squareContent: false,
};

const optionAttributes: Record<keyof DiagnosticOptions, string> = {
  stableScrollbar: 'resizeScrollbarGutter',
  forcedScrollbar: 'resizeForcedScrollbar',
  disableMotion: 'resizeDisableMotion',
  disableEffects: 'resizeDisableEffects',
  containView: 'resizeContainView',
  squareContent: 'resizeSquareContent',
};

function readOptions(): DiagnosticOptions {
  try {
    const stored = JSON.parse(sessionStorage.getItem('axial-resize-diagnostics') || '{}') as Partial<DiagnosticOptions>;
    return { ...defaultOptions, ...stored };
  } catch {
    return defaultOptions;
  }
}

function applyOptions(options: DiagnosticOptions): void {
  const root = document.documentElement;
  for (const key of Object.keys(optionAttributes) as (keyof DiagnosticOptions)[]) {
    root.dataset[optionAttributes[key]] = options[key] ? 'true' : 'false';
  }
  sessionStorage.setItem('axial-resize-diagnostics', JSON.stringify(options));
}

function emptyMetrics(): ResizeMetrics {
  return {
    viewport: '—',
    document: '—',
    viewClient: '—',
    viewScroll: '—',
    scrollbar: '—',
    overflow: '—',
    resizeEvents: 0,
    viewResizeEvents: 0,
    layoutShift: 0,
    dpr: window.devicePixelRatio,
  };
}

function useResizeMetrics(resetToken: number): ResizeMetrics {
  const [metrics, setMetrics] = useState(emptyMetrics);
  const resizeEvents = useRef(0);
  const viewResizeEvents = useRef(0);
  const layoutShift = useRef(0);

  useEffect(() => {
    resizeEvents.current = 0;
    viewResizeEvents.current = 0;
    layoutShift.current = 0;
  }, [resetToken]);

  useEffect(() => {
    const view = document.querySelector<HTMLElement>('.cp-view');
    const countWindowResize = (): void => {
      resizeEvents.current += 1;
    };
    const resizeObserver = new ResizeObserver(() => {
      viewResizeEvents.current += 1;
    });
    if (view) resizeObserver.observe(view);
    window.addEventListener('resize', countWindowResize);

    const performanceObserver =
      typeof PerformanceObserver === 'undefined'
        ? null
        : new PerformanceObserver((list) => {
            for (const entry of list.getEntries()) {
              const shift = entry as PerformanceEntry & { hadRecentInput?: boolean; value?: number };
              if (!shift.hadRecentInput && typeof shift.value === 'number') layoutShift.current += shift.value;
            }
          });
    try {
      performanceObserver?.observe({ type: 'layout-shift', buffered: true });
    } catch {
      performanceObserver?.disconnect();
    }

    const sample = (): void => {
      const currentView = document.querySelector<HTMLElement>('.cp-view');
      const documentElement = document.documentElement;
      const vertical = currentView ? currentView.scrollHeight > currentView.clientHeight + 1 : false;
      const horizontal = currentView ? currentView.scrollWidth > currentView.clientWidth + 1 : false;
      const scrollbarWidth = currentView ? Math.max(0, currentView.offsetWidth - currentView.clientWidth) : 0;
      setMetrics({
        viewport: `${window.innerWidth} × ${window.innerHeight}`,
        document: `${documentElement.clientWidth} × ${documentElement.clientHeight}`,
        viewClient: currentView ? `${currentView.clientWidth} × ${currentView.clientHeight}` : '—',
        viewScroll: currentView ? `${currentView.scrollWidth} × ${currentView.scrollHeight}` : '—',
        scrollbar: `${scrollbarWidth}px`,
        overflow: `${vertical ? 'V' : '—'} ${horizontal ? 'H' : '—'}`,
        resizeEvents: resizeEvents.current,
        viewResizeEvents: viewResizeEvents.current,
        layoutShift: layoutShift.current,
        dpr: window.devicePixelRatio,
      });
    };
    sample();
    const interval = window.setInterval(sample, 120);
    return () => {
      window.clearInterval(interval);
      window.removeEventListener('resize', countWindowResize);
      resizeObserver.disconnect();
      performanceObserver?.disconnect();
    };
  }, []);

  return metrics;
}

function ControlRow({
  label,
  detail,
  on,
  onChange,
  disabled = false,
}: {
  label: string;
  detail: string;
  on: boolean;
  onChange: () => void;
  disabled?: boolean;
}): JSX.Element {
  return (
    <div class="cp-resize-lab-control" data-disabled={disabled ? 'true' : 'false'}>
      <span>
        <strong>{label}</strong>
        <small>{detail}</small>
      </span>
      <span aria-disabled={disabled}>
        <Toggle on={on} onChange={disabled ? () => undefined : onChange} />
      </span>
    </div>
  );
}

export function ResizeDiagnostics(): JSX.Element {
  const chrome = currentDesktopChrome();
  const native = hasNativeDesktopRuntime();
  const nativeDecorationsDefault = chrome.chrome_mode !== 'custom-frameless';
  const [open, setOpen] = useState(true);
  const [options, setOptions] = useState(readOptions);
  const [nativeDecorations, setNativeDecorations] = useState(nativeDecorationsDefault);
  const [nativeShadow, setNativeShadow] = useState(true);
  const [nativeError, setNativeError] = useState('');
  const [resetToken, setResetToken] = useState(0);
  const metrics = useResizeMetrics(resetToken);

  useEffect(() => applyOptions(options), [options]);

  const toggleOption = (key: keyof DiagnosticOptions): void => {
    setOptions((current) => ({ ...current, [key]: !current[key] }));
  };

  const toggleDecorations = async (): Promise<void> => {
    const next = !nativeDecorations;
    try {
      setNativeError('');
      await windowSetDecorations(next);
      setNativeDecorations(next);
    } catch (error: unknown) {
      setNativeError(errMessage(error));
    }
  };

  const toggleShadow = async (): Promise<void> => {
    const next = !nativeShadow;
    try {
      setNativeError('');
      await windowSetShadow(next);
      setNativeShadow(next);
    } catch (error: unknown) {
      setNativeError(errMessage(error));
    }
  };

  const reset = async (): Promise<void> => {
    setOptions(defaultOptions);
    setResetToken((value) => value + 1);
    setNativeError('');
    if (!native) return;
    try {
      await Promise.all([windowSetDecorations(nativeDecorationsDefault), windowSetShadow(true)]);
      setNativeDecorations(nativeDecorationsDefault);
      setNativeShadow(true);
    } catch (error: unknown) {
      setNativeError(errMessage(error));
    }
  };

  const snapshot = [
    `platform=${chrome.platform}`,
    `chrome=${chrome.chrome_mode}`,
    `viewport=${metrics.viewport}`,
    `document=${metrics.document}`,
    `viewClient=${metrics.viewClient}`,
    `viewScroll=${metrics.viewScroll}`,
    `scrollbar=${metrics.scrollbar}`,
    `overflow=${metrics.overflow}`,
    `resizeEvents=${metrics.resizeEvents}`,
    `viewResizeEvents=${metrics.viewResizeEvents}`,
    `layoutShift=${metrics.layoutShift.toFixed(4)}`,
    `dpr=${metrics.dpr}`,
    `options=${JSON.stringify(options)}`,
    `nativeDecorations=${nativeDecorations}`,
    `nativeShadow=${nativeShadow}`,
  ].join('\n');

  if (!open) {
    return (
      <button type="button" class="cp-resize-lab-tab cp-nodrag" onClick={() => setOpen(true)}>
        Resize lab
      </button>
    );
  }

  return (
    <aside class="cp-resize-lab cp-nodrag" aria-label="Resize diagnostics">
      <div class="cp-resize-lab-head">
        <span>
          <strong>Resize lab</strong>
          <small>
            {chrome.platform} · {chrome.chrome_mode}
          </small>
        </span>
        <button type="button" onClick={() => setOpen(false)} aria-label="Collapse resize lab">
          Hide
        </button>
      </div>

      <div class="cp-resize-lab-metrics">
        <span>
          Viewport<b>{metrics.viewport}</b>
        </span>
        <span>
          Document<b>{metrics.document}</b>
        </span>
        <span>
          View client<b>{metrics.viewClient}</b>
        </span>
        <span>
          View scroll<b>{metrics.viewScroll}</b>
        </span>
        <span>
          Scrollbar<b>{metrics.scrollbar}</b>
        </span>
        <span>
          Overflow<b>{metrics.overflow}</b>
        </span>
        <span>
          Window events<b>{metrics.resizeEvents}</b>
        </span>
        <span>
          View events<b>{metrics.viewResizeEvents}</b>
        </span>
        <span>
          Layout shift<b>{metrics.layoutShift.toFixed(4)}</b>
        </span>
        <span>
          DPR<b>{metrics.dpr}</b>
        </span>
      </div>

      <div class="cp-resize-lab-controls">
        <ControlRow
          label="Stable scrollbar"
          detail="Reserve the scrollbar gutter"
          on={options.stableScrollbar}
          onChange={() => toggleOption('stableScrollbar')}
        />
        <ControlRow
          label="Forced scrollbar"
          detail="Keep the vertical track present"
          on={options.forcedScrollbar}
          onChange={() => toggleOption('forcedScrollbar')}
        />
        <ControlRow
          label="Motion off"
          detail="Disable transitions and animations"
          on={options.disableMotion}
          onChange={() => toggleOption('disableMotion')}
        />
        <ControlRow
          label="Effects off"
          detail="Disable shadows and filters"
          on={options.disableEffects}
          onChange={() => toggleOption('disableEffects')}
        />
        <ControlRow
          label="Contain view"
          detail="Isolate layout and paint"
          on={options.containView}
          onChange={() => toggleOption('containView')}
        />
        <ControlRow
          label="Square content"
          detail="Remove the large inner corner"
          on={options.squareContent}
          onChange={() => toggleOption('squareContent')}
        />
        <ControlRow
          label="Native frame"
          detail="Toggle Tauri decorations"
          on={nativeDecorations}
          onChange={() => void toggleDecorations()}
          disabled={!native}
        />
        <ControlRow
          label="Native shadow"
          detail="Toggle the DWM window shadow"
          on={nativeShadow}
          onChange={() => void toggleShadow()}
          disabled={!native}
        />
      </div>

      {nativeError && <div class="cp-resize-lab-error">{nativeError}</div>}
      <div class="cp-resize-lab-actions">
        <Button size="sm" variant="ghost" onClick={() => void reset()}>
          Reset
        </Button>
        <Button size="sm" variant="secondary" onClick={() => void navigator.clipboard.writeText(snapshot)}>
          Copy snapshot
        </Button>
      </div>
    </aside>
  );
}
