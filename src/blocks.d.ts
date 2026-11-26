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

/**
 * An HTTP request to any host, with any headers, cookies included. Use this
 * rather than the global `fetch`, which only reaches hosts the manifest
 * lists.
 *
 * Resolves for every status; check `ok` (true for 200 to 299). `body` is the
 * response as text: parse JSON with `JSON.parse(response.body)`. Header names
 * in the response are lower case. `timeout` is in milliseconds and defaults
 * to 30,000. Rejects when the server cannot be reached.
 */
export function fetch(
  url: string,
  options?: { method?: string; headers?: Record<string, string>; body?: string; timeout?: number },
): Promise<{ status: number; ok: boolean; headers: Record<string, string>; body: string }>;

/**
 * Fetches a URL and saves the response to a file, creating its folder. Use it
 * for images and other files that are not text. Nothing is saved unless `ok`.
 *
 * `image(path)` draws only files inside the app's own folder, so save
 * pictures under it, in a folder Git ignores.
 */
export function download(
  url: string,
  file: string,
  options?: { method?: string; headers?: Record<string, string>; body?: string; timeout?: number },
): Promise<{ status: number; ok: boolean; headers: Record<string, string> }>;

/** One of Herdr's terminals: a coding agent or a shell the user has open. */
export interface Session {
  /** What every other call takes, such as "t20d". */
  address: string;
  /** The program in it: "pi", "claude", "zsh". */
  agent: string;
  cwd: string;
  /** "working", "blocked", "done" or "idle"; missing for a plain shell. */
  status?: string;
  /** Where Herdr shows it, as "space / tab". Use it as the label. */
  where: string;
  state: string;
}

/**
 * The sessions Herdr has open. These five calls reject with "Herdr is not
 * installed" on a Mac without Herdr, so catch that and leave the feature out.
 */
export function sessions(): Promise<Session[]>;

/** The address of the session the user is looking at in Herdr, or null. */
export function focusedSession(): Promise<string | null>;

/** Types the text into a session and presses enter, as the user would. */
export function send(address: string, text: string): Promise<null>;

/** Brings Herdr to the front with that session showing. */
export function show(address: string): Promise<null>;

/**
 * Opens a new session in Herdr: a space in `folder`, running `agent`
 * (default "pi"), with `task` as its first message.
 */
export function start(options: {
  folder?: string;
  task?: string;
  name?: string;
  agent?: string;
}): Promise<null>;
