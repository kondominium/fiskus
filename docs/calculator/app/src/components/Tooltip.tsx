import { useState } from 'react';

export function Tooltip({ label, text, german }: { label: React.ReactNode; text: string; german: string }) {
  const [open, setOpen] = useState(false);
  return (
    <span className="tooltip" onMouseEnter={() => setOpen(true)} onMouseLeave={() => setOpen(false)}>
      <span className="tooltip-label">{label}</span>
      {open && (
        <span className="tooltip-bubble" role="tooltip">
          <span>{text}</span>
          <span className="tooltip-de">DE: {german}</span>
        </span>
      )}
    </span>
  );
}