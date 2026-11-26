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

/**
 * Runs a program and resolves when it ends, whatever its exit code. It gets
 * the user's real environment, so a command that works in their terminal
 * works here. Use this rather than `process.run`, which starts programs with
 * an empty environment.
 *
 * `timeout` is in milliseconds and defaults to 30,000. `input` is written to
 * the program's stdin. Rejects when the program cannot start or runs past
 * the timeout.
 */
export function exec(
  command: string,
  args?: string[],
  options?: { cwd?: string; input?: string; timeout?: number },
): Promise<{ code: number; stdout: string; stderr: string }>;
