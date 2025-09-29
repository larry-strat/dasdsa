# dasdsa activity workspace

This repository bundles automation scripts for generating GitHub activity.

- `backfill_commits.py`: appends dated lines to `keep.log` for scripted commits.
- `auto_activity.py`: minimal PR/issue generator using the GitHub CLI.
- `activity_automation.py`: orchestrates batches of commits, PRs, and issues.

Run `python3 activity_automation.py --help` for usage instructions.
