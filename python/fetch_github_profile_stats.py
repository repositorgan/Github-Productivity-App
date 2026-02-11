# fetch_github_profile_stats.py
# FINAL MATERIAL VERSION — production ready

import os
import requests
import subprocess
import json
from datetime import datetime, timedelta
from pathlib import Path

GITHUB_API = "https://api.github.com"

def get_username(token):
    r = requests.get(
        f"{GITHUB_API}/user",
        headers={"Authorization": f"token {token}"}
    )
    r.raise_for_status()
    return r.json()["login"]

def get_repos(token, username):
    repos = []
    page = 1
    while True:
        r = requests.get(
            f"{GITHUB_API}/users/{username}/repos",
            headers={"Authorization": f"token {token}"},
            params={"per_page": 100, "page": page, "type": "owner"},
        )
        r.raise_for_status()
        data = r.json()
        if not data:
            break
        repos.extend(data)
        page += 1
    return repos

def get_weeks(num_weeks=12):
    today = datetime.utcnow().date()
    weeks = []
    for i in range(num_weeks):
        end = today - timedelta(days=today.weekday() + 7 * i)
        start = end - timedelta(days=6)
        weeks.append((start, end))
    return list(reversed(weeks))

def run_git_stats(repo_dir, start, end):
    def run(cmd):
        result = subprocess.run(
            cmd,
            cwd=repo_dir,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=True,
        )
        return result.stdout.strip()

    date_range = [f"--since={start}", f"--until={end + timedelta(days=1)}"]

    # commits
    try:
        commits = int(run(["git", "rev-list", "--count", "HEAD"] + date_range))
    except:
        commits = 0

    # numstat
    try:
        output = run(["git", "log", "--numstat", "--pretty=format:%H"] + date_range)
    except:
        output = ""

    lines_added = 0
    lines_deleted = 0
    files_changed = set()

    for line in output.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[0].isdigit() and parts[1].isdigit():
            lines_added += int(parts[0])
            lines_deleted += int(parts[1])
            files_changed.add(parts[2])

    return {
        "commits": commits,
        "lines_added": lines_added,
        "lines_deleted": lines_deleted,
        "files_changed": len(files_changed),
    }

def main():
    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        raise RuntimeError("GITHUB_TOKEN is required")

    username = get_username(token)
    repos = get_repos(token, username)

    base = Path(".").resolve()
    work = base / "repos"
    work.mkdir(exist_ok=True)

    weeks = get_weeks(12)
    weekly = [
        {
            "week_start": s.isoformat(),
            "week_end": e.isoformat(),
            "commits": 0,
            "lines_added": 0,
            "lines_deleted": 0,
            "files_changed": 0,
        }
        for s, e in weeks
    ]

    for repo in repos:
        url = repo["clone_url"]
        name = repo["name"]
        path = work / name

        if not path.exists():
            subprocess.run(["git", "clone", "--depth", "200", url, str(path)], check=True)
        else:
            subprocess.run(["git", "fetch", "--all"], cwd=path, check=True)
            subprocess.run(["git", "pull"], cwd=path, check=True)

        for i, (s, e) in enumerate(weeks):
            stats = run_git_stats(path, s, e)
            weekly[i]["commits"] += stats["commits"]
            weekly[i]["lines_added"] += stats["lines_added"]
            weekly[i]["lines_deleted"] += stats["lines_deleted"]
            weekly[i]["files_changed"] += stats["files_changed"]

    out = Path("profile_productivity.json")
    with out.open("w", encoding="utf-8") as f:
        json.dump(weekly, f, indent=2)

    print(f"Wrote {out}")

if __name__ == "__main__":
    main()

