interface LoaderProps {
  className?: string;
  size?: number;
}

export default function CodeIcon({ className, size = 24 }: LoaderProps) {
  return (
    <svg className={className} height={size} width={size} viewBox="0 0 22 18" xmlns="http://www.w3.org/2000/svg">
      <path d="M14.0467 2.08254L9.93244 16.4829C9.77494 17.0293 9.20601 17.3443 8.65958 17.19C8.11315 17.0325 7.79815 16.4636 7.95244 15.9171L12.0667 1.51747C12.2242 0.97123 12.7931 0.654944 13.3396 0.81103C13.886 0.967084 14.201 1.53643 14.0467 2.08254ZM17.3832 4.67357L20.9831 8.27357C21.3849 8.67536 21.3849 9.32464 20.9831 9.72643L17.3832 13.3264C16.9814 13.7282 16.3321 13.7282 15.9303 13.3264C15.5285 12.9246 15.5285 12.2754 15.9303 11.8736L18.8006 9L15.9303 6.12643C15.5285 5.72464 15.5285 5.07536 15.9303 4.67357C16.3321 4.27179 16.9814 4.27179 17.3832 4.67357ZM6.06887 6.12643L3.1969 9L6.06887 11.8736C6.47065 12.2754 6.47065 12.9246 6.06887 13.3264C5.66708 13.7282 5.0178 13.7282 4.61601 13.3264L1.01511 9.72643C0.613453 9.32464 0.613453 8.67536 1.01511 8.27357L4.61601 4.67357C5.0178 4.27179 5.66708 4.27179 6.06887 4.67357C6.47065 5.07536 6.47065 5.72464 6.06887 6.12643Z" />
    </svg>
  );
}
