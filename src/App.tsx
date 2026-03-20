import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import {
  Play,
  MousePointer,
  Clock,
  Activity,
  Zap,
  Info,
  Square
} from "lucide-react";

import "./App.css";

type AwakeMode = "PreventSleep" | "JiggleCursor" | "Smart";
type MovementPattern = "Line" | "Square" | "Circle" | "ZigZag";

type AppSettings = {
  mode: AwakeMode;
  interval_seconds: number;
  jiggle_pixels: number;
  movement_duration_ms: number;
  movement_pattern: MovementPattern;
};

type AppStatus = {
  is_running: boolean;
  active_mode: AwakeMode;
};

function App() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [status, setStatus] = useState<AppStatus | null>(null);
  const [message, setMessage] = useState("");

  const [isStarting, setIsStarting] = useState(false);
  const [isStopping, setIsStopping] = useState(false);
  const [secondsLeft, setSecondsLeft] = useState<number | null>(null);

  useEffect(() => {
    async function loadData() {
      try {
        const loadedSettings = await invoke<AppSettings>("load_app_settings");
        const runtimeStatus = await invoke<AppStatus>("get_runtime_status_command");

        setSettings(loadedSettings);
        setStatus(runtimeStatus);
      } catch (error) {
        setMessage(`Error loading app data: ${String(error)}`);
      }
    }

    loadData();
  }, []);

  useEffect(() => {
    const handler = (e: MouseEvent) => e.preventDefault();
    document.addEventListener("contextmenu", handler);
    return () => document.removeEventListener("contextmenu", handler);
  }, []);

  useEffect(() => {
    if (!settings) return;

    const timeout = setTimeout(async () => {
      try {
        await invoke("save_app_settings", { settings });
      } catch (error) {
        setMessage(`Error saving settings: ${String(error)}`);
      }
    }, 500);

    return () => clearTimeout(timeout);
  }, [settings]);

  useEffect(() => {
    if (!status?.is_running) {
      setSecondsLeft(null);
      return;
    }

    const interval = setInterval(async () => {
      try {
        const value = await invoke<number | null>("get_next_movement_seconds");
        setSecondsLeft(value);
      } catch {}
    }, 1000);

    return () => clearInterval(interval);
  }, [status?.is_running]);

  async function refreshStatus() {
    try {
      const runtimeStatus = await invoke<AppStatus>("get_runtime_status_command");
      setStatus(runtimeStatus);
    } catch (error) {
      setMessage(`Error loading status: ${String(error)}`);
    }
  }

  async function handleStart() {
    if (!settings) return;

    try {
      setIsStarting(true);
      await invoke("start_awake");
      await refreshStatus();
      setMessage("Awake engine started");
    } catch (error) {
      setMessage(`Error starting engine: ${String(error)}`);
    } finally {
      setIsStarting(false);
    }
  }

  async function handleStop() {
    try {
      setIsStopping(true);
      await invoke("stop_awake");
      await refreshStatus();
      setSecondsLeft(null);
      setMessage("Awake engine stopped");
    } catch (error) {
      setMessage(`Error stopping engine: ${String(error)}`);
    } finally {
      setIsStopping(false);
    }
  }

  if (!settings) {
    return (
      <main className="app-shell">
        <header className="topbar">
          <div className="topbar-center">
            <h1>Aurora Awake</h1>
            <p>Keep your desktop active</p>
          </div>
        </header>

        <section className="panel">
          <div className="empty-state">{message || "Loading..."}</div>
        </section>
      </main>
    );
  }

  const isRunning = status?.is_running ?? false;

  return (
    <main className="app-shell">
      <header className="topbar">
        <div className="topbar-center">
          <p>Keep your desktop active</p>
        </div>
      </header>

      <section className="panel">
        {/* {message && <div className="message-bar">{message}</div>} */}

        <div className="settings-list">
          <SettingRow
            label="Mode"
            icon={<Zap size={18} />}
            description="Controls how Aurora keeps the system awake."
          >
            <div className="choice-group">
              {(["JiggleCursor", "Smart", "PreventSleep"] as AwakeMode[]).map((mode) => {
                const available = mode === "JiggleCursor";
                return available ? (
                  <button
                    key={mode}
                    className={`choice-btn ${settings.mode === mode ? "active" : ""}`}
                    onClick={() => setSettings({ ...settings, mode })}
                    disabled={isRunning}
                  >
                    {mode}
                  </button>
                ) : (
                  <div key={mode} className="coming-soon-wrapper">
                    <button className="choice-btn" disabled>
                      {mode}
                    </button>
                    <span className="coming-soon-tooltip">Coming soon</span>
                  </div>
                );
              })}
            </div>
          </SettingRow>

          <SettingRow
            label="Pattern"
            icon={<Activity size={18} />}
            description="Defines the shape of the cursor movement during each cycle."
          >
            <div className="choice-group">
              {(["Line", "Square", "Circle", "ZigZag"] as MovementPattern[]).map((pattern) => (
                <button
                  key={pattern}
                  className={`choice-btn ${settings.movement_pattern === pattern ? "active" : ""}`}
                  onClick={() =>
                    setSettings({
                      ...settings,
                      movement_pattern: pattern
                    })
                  }
                  disabled={isRunning}
                >
                  {pattern}
                </button>
              ))}
            </div>
          </SettingRow>

          <StepperRow
            label="Interval"
            icon={<Clock size={18} />}
            description="Time the app waits before triggering the next cursor movement."
            value={settings.interval_seconds}
            unit="s"
            min={1}
            max={3600}
            step={1}
            disabled={isRunning}
            onChange={(v) => setSettings({ ...settings, interval_seconds: v })}
          />

          <StepperRow
            label="Distance"
            icon={<MousePointer size={18} />}
            description="Maximum cursor movement distance in pixels."
            value={settings.jiggle_pixels}
            unit="px"
            min={1}
            max={500}
            step={1}
            disabled={isRunning}
            onChange={(v) => setSettings({ ...settings, jiggle_pixels: v })}
          />

          <StepperRow
            label="Duration"
            icon={<Clock size={18} />}
            description="How long each movement animation lasts."
            value={settings.movement_duration_ms}
            unit="ms"
            min={50}
            max={5000}
            step={50}
            disabled={isRunning}
            onChange={(v) => setSettings({ ...settings, movement_duration_ms: v })}
          />

          <SettingRow
            label="Status"
            icon={<Activity size={18} />}
            description="Shows whether the awake engine is currently running."
          >
            <div className={`status-badge ${isRunning ? "running" : "stopped"}`}>
              {isRunning
                ? `Running • ${status?.active_mode ?? settings.mode}`
                : "Stopped"}
            </div>
          </SettingRow>

          <SettingRow
            label="Next move"
            icon={<Clock size={18} />}
            description="Countdown until the next scheduled movement."
          >
            <div className="status-badge stopped">
              {status?.is_running
                ? secondsLeft !== null
                  ? `${secondsLeft}s`
                  : "..."
                : "Not scheduled"}
            </div>
          </SettingRow>
        </div>
      </section>

      <footer className="footer-bar">
        <button
          className={`action-btn ${isRunning ? "stop" : "start"}`}
          onClick={isRunning ? handleStop : handleStart}
          disabled={isStarting || isStopping}
        >
          {isRunning ? <Square size={18} /> : <Play size={18} />}
          {isRunning
            ? isStopping
              ? "Stopping..."
              : "Stop Awake"
            : isStarting
              ? "Starting..."
              : "Start Awake"}
        </button>
      </footer>
    </main>
  );
}

type SettingRowProps = {
  label: string;
  icon: React.ReactNode;
  description?: string;
  children: React.ReactNode;
};

function SettingRow({ label, icon, description, children }: SettingRowProps) {
  return (
    <div className="setting-row">
      <div className="setting-label">
        <span className="setting-icon">{icon}</span>
        <span>{label}</span>

        {description && (
          <span className="info-wrapper" tabIndex={0}>
            <Info size={14} className="info-icon" />
            <span className="tooltip">{description}</span>
          </span>
        )}
      </div>

      <div className="setting-content">{children}</div>
    </div>
  );
}

type StepperRowProps = {
  label: string;
  icon: React.ReactNode;
  description?: string;
  value: number;
  unit: string;
  min: number;
  max: number;
  step: number;
  disabled?: boolean;
  onChange: (value: number) => void;
};

function StepperRow({
  label,
  icon,
  description,
  value,
  unit,
  min,
  max,
  step,
  disabled,
  onChange
}: StepperRowProps) {
  const [draft, setDraft] = useState(String(value));

  const [rangeError, setRangeError] = useState(false);

  useEffect(() => {
    setDraft(String(value));
  }, [value]);

  function commit(raw: string) {
    const num = parseInt(raw, 10);
    if (isNaN(num) || raw === "") {
      setDraft(String(value));
      return;
    }
    if (num < min || num > max) {
      setDraft(String(value));
      setRangeError(true);
      setTimeout(() => setRangeError(false), 2500);
      return;
    }
    onChange(num);
    setDraft(String(num));
  }

  return (
    <SettingRow label={label} icon={icon} description={description}>
      <div className="stepper-inline">
        <button
          className="step-btn"
          onClick={() => onChange(Math.max(min, value - step))}
          disabled={disabled}
        >
          –
        </button>
        <div className="step-input-wrapper">
          {rangeError && (
            <span className="range-error-tooltip">
              {min}–{max} {unit}
            </span>
          )}
          <input
            className="step-input"
            type="text"
            inputMode="numeric"
            value={draft}
            disabled={disabled}
            onChange={(e) => setDraft(e.target.value.replace(/\D/g, ""))}
            onFocus={(e) => { const t = e.target; setTimeout(() => t.select(), 0); }}
            onBlur={(e) => commit(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && e.currentTarget.blur()}
          />
          <span className="step-unit">{unit}</span>
        </div>
        <button
          className="step-btn"
          onClick={() => onChange(Math.min(max, value + step))}
          disabled={disabled}
        >
          +
        </button>
      </div>
    </SettingRow>
  );
}

export default App;