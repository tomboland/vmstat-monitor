# Vmstat Monitor

*The* solution to a problem no one has.  I wrote this just so I always had the last line of vmstat on tap to display on my i3status-rs [https://github.com/greshake/i3status-rust] i3bar.  It uses `renameat2()` (linux specific) to atomically swap a text file containing the last line of output.  I wrote it for the sake of it really, and because it annoys me how many things out there do this sort of thing whilst consuming lots of CPU and memory for no good reason.

It relies on my fork of nix [https://github.com/tomboland/nix] until such time as my PR to add `renameat2()` is merged [https://github.com/nix-rust/nix/pull/1458].