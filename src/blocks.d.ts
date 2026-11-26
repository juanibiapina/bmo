/**
 * Every color by name, for the appearance the Mac is in now. Values are
 * `#rrggbb` or `#rrggbbaa`. Light and dark name the same colors, so use
 * these and never a literal color.
 *
 * background and surface (the window), canvas (behind cards), chrome (bars
 * and sidebars), popover, raised, hover, selected, border, input (a stronger
 * border), foreground, muted, dim, accent, accent_foreground, solid and
 * solid_foreground (a primary button), ring, selection, working, blocked,
 * done, idle, ansi0..ansi15.
 */
export function theme(): Record<string, string>;
