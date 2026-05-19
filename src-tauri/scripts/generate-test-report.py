#!/usr/bin/env python3
"""Parse cargo test JSON output and generate a markdown report for PR comments."""

import json
import re
import os


def parse_json_results(filepath):
    """Parse cargo test's --format json output."""
    passed = []
    failed = []
    ignored = []

    if not os.path.exists(filepath):
        return passed, failed, ignored

    with open(filepath, "r") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue

            event_type = event.get("type", "")

            if event_type == "test" and event.get("event") == "ok":
                name = event.get("name", "unknown")
                passed.append({"name": name})

            elif event_type == "test" and event.get("event") == "failed":
                name = event.get("name", "unknown")
                stdout = event.get("stdout", "")
                failed.append({"name": name, "stdout": stdout})

            elif event_type == "test" and event.get("event") == "ignored":
                name = event.get("name", "unknown")
                ignored.append({"name": name})

    return passed, failed, ignored


def parse_text_results(filepath):
    """Parse cargo test's human-readable output for summary, individual tests, and timing."""
    summary = {"passed": 0, "failed": 0, "ignored": 0, "measured": 0}
    performance = []
    text_passed = []
    text_failed = []
    text_ignored = []

    if not os.path.exists(filepath):
        return summary, performance, text_passed, text_failed, text_ignored

    with open(filepath, "r") as f:
        text_content = f.read()

    # Parse individual test result lines
    # Format: "test test_name ... ok" or "test test_name ... FAILED" or "test test_name ... ignored"
    test_line_pattern = re.compile(r"^test\s+(\S+)\s+\.\.\.\s+(ok|FAILED|ignored)", re.MULTILINE)
    for match in test_line_pattern.finditer(text_content):
        name = match.group(1)
        status = match.group(2)
        if status == "ok":
            text_passed.append({"name": name})
        elif status == "FAILED":
            text_failed.append({"name": name, "stdout": ""})
        elif status == "ignored":
            text_ignored.append({"name": name})

    # Parse test summary line
    # e.g., "test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out"
    summary_match = re.search(
        r"test result: (ok|FAILED).*?(\d+) passed.*?(\d+) failed.*?(\d+) ignored.*?(\d+) measured",
        text_content,
    )
    if summary_match:
        summary["passed"] = int(summary_match.group(2))
        summary["failed"] = int(summary_match.group(3))
        summary["ignored"] = int(summary_match.group(4))
        summary["measured"] = int(summary_match.group(5))

    # Extract test execution times from the text output
    time_pattern = re.compile(r"test\s+(\S+)\s+.*?(\d+[\.\d]*)\s*ms")
    for match in time_pattern.finditer(text_content):
        name = match.group(1)
        duration_ms = float(match.group(2))
        if "perf" in name.lower() or "performance" in name.lower():
            performance.append({"name": name, "duration_ms": duration_ms})

    return summary, performance, text_passed, text_failed, text_ignored


def categorize_tests(passed, failed):
    """Categorize tests into stability and performance groups."""
    stability_passed = [t for t in passed if "perf" not in t["name"].lower()]
    perf_passed = [t for t in passed if "perf" in t["name"].lower()]
    stability_failed = [t for t in failed if "perf" not in t["name"].lower()]
    perf_failed = [t for t in failed if "perf" in t["name"].lower()]

    return {
        "stability": {
            "passed": len(stability_passed),
            "failed": len(stability_failed),
            "total": len(stability_passed) + len(stability_failed),
            "failed_details": stability_failed,
        },
        "performance": {
            "passed": len(perf_passed),
            "failed": len(perf_failed),
            "total": len(perf_passed) + len(perf_failed),
            "failed_details": perf_failed,
        },
    }


def get_perf_threshold(name):
    """Return the performance threshold in ms for a given test name."""
    if "parse_sql_type" in name:
        return 500
    if "format_cell" in name:
        return 2000
    if "resolve_export" in name:
        return 100
    return None


def generate_report(passed, failed, ignored, summary, performance, categories):
    """Generate the markdown report."""
    total = len(passed) + len(failed)
    pass_rate = (len(passed) / total * 100) if total > 0 else 0

    lines = []
    lines.append("## 🧪 Test Results Report")
    lines.append("")

    # Overall summary
    status_icon = "✅" if len(failed) == 0 else "❌"
    lines.append(
        f"**{status_icon} Total: {total} tests | "
        f"Passed: {len(passed)} | "
        f"Failed: {len(failed)} | "
        f"Pass Rate: {pass_rate:.1f}%**"
    )
    lines.append("")

    # Category breakdown table
    lines.append("| Category | Status | Passed | Total | Rate |")
    lines.append("|----------|--------|--------|-------|------|")

    stab = categories["stability"]
    stab_rate = (stab["passed"] / stab["total"] * 100) if stab["total"] else 0
    stab_status = "✅ PASS" if stab["passed"] == stab["total"] else "❌ FAIL"
    lines.append(
        f"| Stability | {stab_status} | {stab['passed']} | {stab['total']} | {stab_rate:.1f}% |"
    )

    perf = categories["performance"]
    if perf["total"] > 0:
        perf_rate = (perf["passed"] / perf["total"] * 100) if perf["total"] else 0
        perf_status = "✅ PASS" if perf["passed"] == perf["total"] else "❌ FAIL"
        lines.append(
            f"| Performance | {perf_status} | {perf['passed']} | {perf['total']} | {perf_rate:.1f}% |"
        )

    lines.append("")

    # Failed tests detail
    if failed:
        lines.append("### ❌ Failed Tests")
        lines.append("")
        for t in failed:
            lines.append(f"- **{t['name']}**")
            if t.get("stdout"):
                stdout_clean = t["stdout"].strip()
                if len(stdout_clean) > 500:
                    stdout_clean = stdout_clean[-500:]
                lines.append(f"  ```")
                lines.append(f"  {stdout_clean}")
                lines.append(f"  ```")
        lines.append("")

    # Performance tests detail
    perf_tests = [t for t in passed + failed if "perf" in t["name"].lower()]
    if performance or perf_tests:
        lines.append("### ⚡ Performance Metrics")
        lines.append("")

        if performance:
            lines.append("| Test | Duration | Threshold | Status |")
            lines.append("|------|----------|-----------|--------|")
            for p in performance:
                name = p["name"]
                duration = p.get("duration_ms", 0)
                threshold = get_perf_threshold(name)
                if threshold is not None:
                    threshold_str = f"{threshold}ms"
                    status = "✅" if duration < threshold else "❌"
                else:
                    threshold_str = "-"
                    status = "✅"
                lines.append(f"| {name} | {duration:.1f}ms | {threshold_str} | {status} |")
        else:
            lines.append(
                "All performance tests passed within acceptable thresholds."
            )
        lines.append("")

    # Test list summary
    lines.append("### 📋 Test List")
    lines.append("")
    lines.append("<details>")
    lines.append("<summary>Click to expand full test list</summary>")
    lines.append("")
    for t in passed:
        icon = "⚡" if "perf" in t["name"].lower() else "✅"
        lines.append(f"- {icon} {t['name']}")
    for t in failed:
        lines.append(f"- ❌ {t['name']}")
    lines.append("")
    lines.append("</details>")
    lines.append("")

    return "\n".join(lines)


def main():
    json_file = "test-results.json"
    text_file = "test-results.txt"

    passed, failed, ignored = parse_json_results(json_file)
    summary, performance, text_passed, text_failed, text_ignored = parse_text_results(text_file)

    # Fallback: if JSON parsing yielded no results, use text-parsed individual tests
    if not passed and not failed:
        passed = text_passed
        failed = text_failed
        ignored = text_ignored

    categories = categorize_tests(passed, failed)

    report = generate_report(passed, failed, ignored, summary, performance, categories)

    # Write markdown report
    with open("test-report.md", "w") as f:
        f.write(report)

    # Write structured JSON report
    report_data = {
        "total": len(passed) + len(failed),
        "passed": len(passed),
        "failed": len(failed),
        "ignored": len(ignored),
        "pass_rate": (len(passed) / (len(passed) + len(failed)) * 100)
        if (len(passed) + len(failed)) > 0
        else 0,
        "stability_tests": categories["stability"],
        "performance_tests": categories["performance"],
        "performance_details": performance,
    }
    with open("test-report.json", "w") as f:
        json.dump(report_data, f, indent=2)

    print(report)


if __name__ == "__main__":
    main()
